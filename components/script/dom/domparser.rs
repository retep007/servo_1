/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use document_loader::DocumentLoader;
use dom::bindings::codegen::Bindings::DOMParserBinding;
use dom::bindings::codegen::Bindings::DOMParserBinding::DOMParserMethods;
use dom::bindings::codegen::Bindings::DOMParserBinding::SupportedType::Application_xhtml_xml;
use dom::bindings::codegen::Bindings::DOMParserBinding::SupportedType::Application_xml;
use dom::bindings::codegen::Bindings::DOMParserBinding::SupportedType::Text_html;
use dom::bindings::codegen::Bindings::DOMParserBinding::SupportedType::Text_xml;
use dom::bindings::codegen::Bindings::DocumentBinding::DocumentReadyState;
use dom::bindings::codegen::Bindings::WindowBinding::WindowMethods;
use dom::bindings::error::Fallible;
use dom::bindings::reflector::{Reflector, reflect_dom_object};
use dom::bindings::root::{Dom, DomRoot};
use dom::bindings::str::DOMString;
use dom::document::{Document, HasBrowsingContext, IsHTMLDocument};
use dom::document::DocumentSource;
use dom::servoparser::ServoParser;
use dom::window::Window;
use dom_struct::dom_struct;
use script_traits::DocumentActivity;
use typeholder::TypeHolderTrait;

#[dom_struct]
pub struct DOMParser<TH: TypeHolderTrait + 'static> {
    reflector_: Reflector<TH>,
    window: Dom<Window<TH>>, // XXXjdm Document instead?
}

impl<TH: TypeHolderTrait> DOMParser<TH> {
    fn new_inherited(window: &Window<TH>) -> DOMParser<TH> {
        DOMParser {
            reflector_: Reflector::new(),
            window: Dom::from_ref(window),
        }
    }

    pub fn new(window: &Window<TH>) -> DomRoot<DOMParser<TH>> {
        reflect_dom_object(Box::new(DOMParser::new_inherited(window)),
                           window,
                           DOMParserBinding::Wrap)
    }

    pub fn Constructor(window: &Window<TH>) -> Fallible<DomRoot<DOMParser<TH>>, TH> {
        Ok(DOMParser::new(window))
    }
}

impl<TH: TypeHolderTrait> DOMParserMethods<TH> for DOMParser<TH> {
    // https://w3c.github.io/DOM-Parsing/#the-domparser-interface
    fn ParseFromString(&self,
                       s: DOMString,
                       ty: DOMParserBinding::SupportedType)
                       -> Fallible<DomRoot<Document<TH>>, TH> {
        let url = self.window.get_url();
        let content_type = ty.as_str().parse().expect("Supported type is not a MIME type");
        let doc = self.window.Document();
        let loader = DocumentLoader::new(&*doc.loader());
        match ty {
            Text_html => {
                let document = Document::new(&self.window,
                                             HasBrowsingContext::No,
                                             Some(url.clone()),
                                             doc.origin().clone(),
                                             IsHTMLDocument::HTMLDocument,
                                             Some(content_type),
                                             None,
                                             DocumentActivity::Inactive,
                                             DocumentSource::FromParser,
                                             loader,
                                             None,
                                             None,
                                             Default::default());
                ServoParser::<TH>::parse_html_document(&document, s, url);
                document.set_ready_state(DocumentReadyState::Complete);
                Ok(document)
            }
            Text_xml | Application_xml | Application_xhtml_xml => {
                let document = Document::new(&self.window,
                                             HasBrowsingContext::No,
                                             Some(url.clone()),
                                             doc.origin().clone(),
                                             IsHTMLDocument::NonHTMLDocument,
                                             Some(content_type),
                                             None,
                                             DocumentActivity::Inactive,
                                             DocumentSource::FromParser,
                                             loader,
                                             None,
                                             None,
                                             Default::default());
                ServoParser::<TH>::parse_xml_document(&document, s, url);
                document.set_ready_state(DocumentReadyState::Complete);
                Ok(document)
            }
        }
    }
}
