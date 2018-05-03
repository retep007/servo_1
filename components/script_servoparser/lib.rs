/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use document_loader::{DocumentLoader, LoadType};
use script::dom::bindings::cell::DomRefCell;
use script::dom::bindings::codegen::Bindings::DocumentBinding::{DocumentMethods, DocumentReadyState};
use script::dom::bindings::codegen::Bindings::HTMLImageElementBinding::HTMLImageElementMethods;
use script::dom::bindings::codegen::Bindings::HTMLTemplateElementBinding::HTMLTemplateElementMethods;
use script::dom::bindings::codegen::Bindings::NodeBinding::NodeMethods;
use script::dom::bindings::codegen::Bindings::ServoParserBinding;
use script::dom::bindings::inheritance::Castable;
use script::dom::bindings::refcounted::Trusted;
use script::dom::bindings::reflector::{Reflector, reflect_dom_object};
use script::dom::bindings::root::{Dom, DomRoot, MutNullableDom, RootedReference};
use script::dom::bindings::settings_stack::is_execution_stack_empty;
use script::dom::bindings::str::DOMString;
use script::dom::characterdata::CharacterData;
use script::dom::comment::Comment;
use script::dom::document::{Document, DocumentSource, HasBrowsingContext, IsHTMLDocument};
use script::dom::documenttype::DocumentType;
use script::dom::element::{Element, ElementCreator, CustomElementCreationMode};
use script::dom::globalscope::GlobalScope;
use script::dom::htmlformelement::{FormControlElementHelpers, HTMLFormElement};
use script::dom::htmlimageelement::HTMLImageElement;
use script::dom::htmlscriptelement::{HTMLScriptElement, ScriptResult};
use script::dom::htmltemplateelement::HTMLTemplateElement;
use script::dom::node::Node;
use script::dom::processinginstruction::ProcessingInstruction;
use script::dom::text::Text;
use script::dom::virtualmethods::vtable_for;
use script::dom::servoparser::ServoParser as ServoParserTrait;
use dom_struct::dom_struct;
use embedder_traits::resources::{self, Resource};
use html5ever::{Attribute, ExpandedName, LocalName, QualName};
use html5ever::buffer_queue::BufferQueue;
use html5ever::tendril::{StrTendril, ByteTendril, IncompleteUtf8};
use html5ever::tree_builder::{NodeOrText, TreeSink, NextParserState, QuirksMode, ElementFlags};
use hyper::header::ContentType;
use hyper::mime::{Mime, SubLevel, TopLevel};
use hyper_serde::Serde;
use msg::constellation_msg::PipelineId;
use net_traits::{FetchMetadata, FetchResponseListener, Metadata, NetworkError};
use network_listener::PreInvoke;
use profile_traits::time::{TimerMetadata, TimerMetadataFrameType};
use profile_traits::time::{TimerMetadataReflowType, ProfilerCategory, profile};
use script_thread::ScriptThread;
use script_traits::DocumentActivity;
use servo_config::prefs::PREFS;
use servo_url::ServoUrl;
use std::borrow::Cow;
use std::cell::Cell;
use std::mem;
use style::context::QuirksMode as ServoQuirksMode;

mod async_html;
mod html;
mod xml;

#[dom_struct]
/// The parser maintains two input streams: one for input from script through
/// document.write(), and one for input from network.
///
/// There is no concrete representation of the insertion point, instead it
/// always points to just before the next character from the network input,
/// with all of the script input before itself.
///
/// ```text
///     ... script input ... | ... network input ...
///                          ^
///                 insertion point
/// ```
pub struct ServoParser {
    reflector: Reflector,
    /// The document associated with this parser.
    document: Dom<Document>,
    /// Input received from network.
    #[ignore_malloc_size_of = "Defined in html5ever"]
    network_input: DomRefCell<BufferQueue>,
    /// Part of an UTF-8 code point spanning input chunks
    #[ignore_malloc_size_of = "Defined in html5ever"]
    incomplete_utf8: DomRefCell<Option<IncompleteUtf8>>,
    /// Input received from script. Used only to support document.write().
    #[ignore_malloc_size_of = "Defined in html5ever"]
    script_input: DomRefCell<BufferQueue>,
    /// The tokenizer of this parser.
    tokenizer: DomRefCell<Tokenizer>,
    /// Whether to expect any further input from the associated network request.
    last_chunk_received: Cell<bool>,
    /// Whether this parser should avoid passing any further data to the tokenizer.
    suspended: Cell<bool>,
    /// <https://html.spec.whatwg.org/multipage/#script-nesting-level>
    script_nesting_level: Cell<usize>,
    /// <https://html.spec.whatwg.org/multipage/#abort-a-parser>
    aborted: Cell<bool>,
    /// <https://html.spec.whatwg.org/multipage/#script-created-parser>
    script_created_parser: bool,
}

impl ServoParserTrait for ServoParser {
    pub fn parse_html_document(document: &Document, input: DOMString, url: ServoUrl) {
        let parser = if PREFS.get("dom.servoparser.async_html_tokenizer.enabled").as_boolean().unwrap() {
            ServoParser::new(document,
                                Tokenizer::AsyncHtml(self::async_html::Tokenizer::new(document, url, None)),
                                LastChunkState::NotReceived,
                                ParserKind::Normal)
        } else {
            ServoParser::new(document,
                                Tokenizer::Html(self::html::Tokenizer::new(document, url, None, ParsingAlgorithm::Normal)),
                                LastChunkState::NotReceived,
                                ParserKind::Normal)
        };
        parser.parse_string_chunk(String::from(input));
    }

    // https://html.spec.whatwg.org/multipage/#parsing-html-fragments
    pub fn parse_html_fragment(context: &Element, input: DOMString) -> impl Iterator<Item=DomRoot<Node>> {
        let context_node = context.upcast::<Node>();
        let context_document = context_node.owner_doc();
        let window = context_document.window();
        let url = context_document.url();

        // Step 1.
        let loader = DocumentLoader::new_with_threads(context_document.loader().resource_threads().clone(),
                                                        Some(url.clone()));
        let document = Document::new(window,
                                        HasBrowsingContext::No,
                                        Some(url.clone()),
                                        context_document.origin().clone(),
                                        IsHTMLDocument::HTMLDocument,
                                        None,
                                        None,
                                        DocumentActivity::Inactive,
                                        DocumentSource::FromParser,
                                        loader,
                                        None,
                                        None,
                                        Default::default());

        // Step 2.
        document.set_quirks_mode(context_document.quirks_mode());

        // Step 11.
        let form = context_node.inclusive_ancestors()
            .find(|element| element.is::<HTMLFormElement>());

        let fragment_context = FragmentContext {
            context_elem: context_node,
            form_elem: form.r(),
        };

        let parser = ServoParser::new(&document,
                                        Tokenizer::Html(self::html::Tokenizer::new(&document,
                                                                                    url,
                                                                                    Some(fragment_context),
                                                                                    ParsingAlgorithm::Fragment)),
                                        LastChunkState::Received,
                                        ParserKind::Normal);
        parser.parse_string_chunk(String::from(input));

        // Step 14.
        let root_element = document.GetDocumentElement().expect("no document element");
        FragmentParsingResult {
            inner: root_element.upcast::<Node>().children(),
        }
    }

    pub fn parse_html_script_input(document: &Document, url: ServoUrl, type_: &str) {
        let parser = ServoParser::new(
            document,
            Tokenizer::Html(self::html::Tokenizer::new(
                document,
                url,
                None,
                ParsingAlgorithm::Normal,
            )),
            LastChunkState::NotReceived,
            ParserKind::ScriptCreated,
        );
        document.set_current_parser(Some(&parser));
        if !type_.eq_ignore_ascii_case("text/html") {
            parser.parse_string_chunk("<pre>\n".to_owned());
            parser.tokenizer.borrow_mut().set_plaintext_state();
        }
    }

    pub fn parse_xml_document(document: &Document, input: DOMString, url: ServoUrl) {
        let parser = ServoParser::new(document,
                                        Tokenizer::Xml(self::xml::Tokenizer::new(document, url)),
                                        LastChunkState::NotReceived,
                                        ParserKind::Normal);
        parser.parse_string_chunk(String::from(input));
    }

    pub fn parser_is_not_active(&self) -> bool {
        self.can_write() || self.tokenizer.try_borrow_mut().is_ok()
    }

    pub fn script_nesting_level(&self) -> usize {
        self.script_nesting_level.get()
    }

    pub fn is_script_created(&self) -> bool {
        self.script_created_parser
    }

    /// Corresponds to the latter part of the "Otherwise" branch of the 'An end
    /// tag whose tag name is "script"' of
    /// <https://html.spec.whatwg.org/multipage/#parsing-main-incdata>
    ///
    /// This first moves everything from the script input to the beginning of
    /// the network input, effectively resetting the insertion point to just
    /// before the next character to be consumed.
    ///
    ///
    /// ```text
    ///     | ... script input ... network input ...
    ///     ^
    ///     insertion point
    /// ```
    pub fn resume_with_pending_parsing_blocking_script(&self, script: &HTMLScriptElement, result: ScriptResult) {
        assert!(self.suspended.get());
        self.suspended.set(false);

        mem::swap(&mut *self.script_input.borrow_mut(),
                  &mut *self.network_input.borrow_mut());
        while let Some(chunk) = self.script_input.borrow_mut().pop_front() {
            self.network_input.borrow_mut().push_back(chunk);
        }

        let script_nesting_level = self.script_nesting_level.get();
        assert_eq!(script_nesting_level, 0);

        self.script_nesting_level.set(script_nesting_level + 1);
        script.execute(result);
        self.script_nesting_level.set(script_nesting_level);

        if !self.suspended.get() {
            self.parse_sync();
        }
    }

    pub fn can_write(&self) -> bool {
        self.script_created_parser || self.script_nesting_level.get() > 0
    }

    /// Steps 6-8 of https://html.spec.whatwg.org/multipage/#document.write()
    pub fn write(&self, text: Vec<DOMString>) {
        assert!(self.can_write());

        if self.document.has_pending_parsing_blocking_script() {
            // There is already a pending parsing blocking script so the
            // parser is suspended, we just append everything to the
            // script input and abort these steps.
            for chunk in text {
                self.script_input.borrow_mut().push_back(String::from(chunk).into());
            }
            return;
        }

        // There is no pending parsing blocking script, so all previous calls
        // to document.write() should have seen their entire input tokenized
        // and process, with nothing pushed to the parser script input.
        assert!(self.script_input.borrow().is_empty());

        let mut input = BufferQueue::new();
        for chunk in text {
            input.push_back(String::from(chunk).into());
        }

        self.tokenize(|tokenizer| tokenizer.feed(&mut input));

        if self.suspended.get() {
            // Parser got suspended, insert remaining input at end of
            // script input, following anything written by scripts executed
            // reentrantly during this call.
            while let Some(chunk) = input.pop_front() {
                self.script_input.borrow_mut().push_back(chunk);
            }
            return;
        }

        assert!(input.is_empty());
    }

    // Steps 4-6 of https://html.spec.whatwg.org/multipage/#dom-document-close
    pub fn close(&self) {
        assert!(self.script_created_parser);

        // Step 4.
        self.last_chunk_received.set(true);

        if self.suspended.get() {
            // Step 5.
            return;
        }

        // Step 6.
        self.parse_sync();
    }

    // https://html.spec.whatwg.org/multipage/#abort-a-parser
    pub fn abort(&self) {
        assert!(!self.aborted.get());
        self.aborted.set(true);

        // Step 1.
        *self.script_input.borrow_mut() = BufferQueue::new();
        *self.network_input.borrow_mut() = BufferQueue::new();

        // Step 2.
        self.document.set_ready_state(DocumentReadyState::Interactive);

        // Step 3.
        self.tokenizer.borrow_mut().end();
        self.document.set_current_parser(None);

        // Step 4.
        self.document.set_ready_state(DocumentReadyState::Interactive);
    }

    #[allow(unrooted_must_root)]
    fn new_inherited(document: &Document,
                     tokenizer: Tokenizer,
                     last_chunk_state: LastChunkState,
                     kind: ParserKind)
                     -> Self {
        ServoParser {
            reflector: Reflector::new(),
            document: Dom::from_ref(document),
            incomplete_utf8: DomRefCell::new(None),
            network_input: DomRefCell::new(BufferQueue::new()),
            script_input: DomRefCell::new(BufferQueue::new()),
            tokenizer: DomRefCell::new(tokenizer),
            last_chunk_received: Cell::new(last_chunk_state == LastChunkState::Received),
            suspended: Default::default(),
            script_nesting_level: Default::default(),
            aborted: Default::default(),
            script_created_parser: kind == ParserKind::ScriptCreated,
        }
    }

    #[allow(unrooted_must_root)]
    fn new(document: &Document,
           tokenizer: Tokenizer,
           last_chunk_state: LastChunkState,
           kind: ParserKind)
           -> DomRoot<Self> {
        reflect_dom_object(Box::new(ServoParser::new_inherited(document, tokenizer, last_chunk_state, kind)),
                           document.window(),
                           ServoParserBinding::Wrap)
    }

    fn push_bytes_input_chunk(&self, chunk: Vec<u8>) {
        let mut chunk = ByteTendril::from(&*chunk);
        let mut network_input = self.network_input.borrow_mut();
        let mut incomplete_utf8 = self.incomplete_utf8.borrow_mut();

        if let Some(mut incomplete) = incomplete_utf8.take() {
            let result = incomplete.try_complete(chunk, |s| network_input.push_back(s));
            match result {
                Err(()) => {
                    *incomplete_utf8 = Some(incomplete);
                    return
                }
                Ok(remaining) => {
                    chunk = remaining
                }
            }
        }

        *incomplete_utf8 = chunk.decode_utf8_lossy(|s| network_input.push_back(s));
    }

    fn push_string_input_chunk(&self, chunk: String) {
        self.network_input.borrow_mut().push_back(chunk.into());
    }

    fn parse_sync(&self) {
        let metadata = TimerMetadata {
            url: self.document.url().as_str().into(),
            iframe: TimerMetadataFrameType::RootWindow,
            incremental: TimerMetadataReflowType::FirstReflow,
        };
        let profiler_category = self.tokenizer.borrow().profiler_category();
        profile(profiler_category,
                Some(metadata),
                self.document.window().upcast::<GlobalScope>().time_profiler_chan().clone(),
                || self.do_parse_sync())
    }

    fn do_parse_sync(&self) {
        assert!(self.script_input.borrow().is_empty());

        // This parser will continue to parse while there is either pending input or
        // the parser remains unsuspended.

        if self.last_chunk_received.get() {
            if let Some(_) = self.incomplete_utf8.borrow_mut().take() {
                self.network_input.borrow_mut().push_back(StrTendril::from("\u{FFFD}"))
            }
        }
        self.tokenize(|tokenizer| tokenizer.feed(&mut *self.network_input.borrow_mut()));

        if self.suspended.get() {
            return;
        }

        assert!(self.network_input.borrow().is_empty());

        if self.last_chunk_received.get() {
            self.finish();
        }
    }

    fn parse_string_chunk(&self, input: String) {
        self.document.set_current_parser(Some(self));
        self.push_string_input_chunk(input);
        if !self.suspended.get() {
            self.parse_sync();
        }
    }

    fn parse_bytes_chunk(&self, input: Vec<u8>) {
        self.document.set_current_parser(Some(self));
        self.push_bytes_input_chunk(input);
        if !self.suspended.get() {
            self.parse_sync();
        }
    }

    fn tokenize<F>(&self, mut feed: F)
        where F: FnMut(&mut Tokenizer) -> Result<(), DomRoot<HTMLScriptElement>>,
    {
        loop {
            assert!(!self.suspended.get());
            assert!(!self.aborted.get());

            self.document.reflow_if_reflow_timer_expired();
            let script = match feed(&mut *self.tokenizer.borrow_mut()) {
                Ok(()) => return,
                Err(script) => script,
            };

            let script_nesting_level = self.script_nesting_level.get();

            self.script_nesting_level.set(script_nesting_level + 1);
            script.prepare();
            self.script_nesting_level.set(script_nesting_level);

            if self.document.has_pending_parsing_blocking_script() {
                self.suspended.set(true);
                return;
            }
        }
    }

    // https://html.spec.whatwg.org/multipage/#the-end
    fn finish(&self) {
        assert!(!self.suspended.get());
        assert!(self.last_chunk_received.get());
        assert!(self.script_input.borrow().is_empty());
        assert!(self.network_input.borrow().is_empty());
        assert!(self.incomplete_utf8.borrow().is_none());

        // Step 1.
        self.document.set_ready_state(DocumentReadyState::Interactive);

        // Step 2.
        self.tokenizer.borrow_mut().end();
        self.document.set_current_parser(None);

        // Steps 3-12 are in another castle, namely finish_load.
        let url = self.tokenizer.borrow().url().clone();
        self.document.finish_load(LoadType::PageSource(url));
    }
}