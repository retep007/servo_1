/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::EventBinding::EventMethods;
use dom::bindings::codegen::Bindings::HashChangeEventBinding;
use dom::bindings::codegen::Bindings::HashChangeEventBinding::HashChangeEventMethods;
use dom::bindings::error::Fallible;
use dom::bindings::inheritance::Castable;
use dom::bindings::reflector::reflect_dom_object;
use dom::bindings::root::DomRoot;
use dom::bindings::str::{DOMString, USVString};
use dom::event::Event;
use dom::window::Window;
use dom_struct::dom_struct;
use servo_atoms::Atom;
use typeholder::TypeHolderTrait;

// https://html.spec.whatwg.org/multipage/#hashchangeevent
#[dom_struct]
pub struct HashChangeEvent<TH: TypeHolderTrait> {
    event: Event<TH>,
    old_url: String,
    new_url: String,
}

impl<TH: TypeHolderTrait> HashChangeEvent<TH> {
    fn new_inherited(old_url: String, new_url: String) -> HashChangeEvent<TH> {
        HashChangeEvent {
            event: Event::new_inherited(),
            old_url: old_url,
            new_url: new_url,
        }
    }

    pub fn new_uninitialized(window: &Window<TH>) -> DomRoot<HashChangeEvent<TH>> {
        reflect_dom_object(Box::new(HashChangeEvent::new_inherited(String::new(), String::new())),
                           window,
                           HashChangeEventBinding::Wrap)
    }

    pub fn new(window: &Window<TH>,
               type_: Atom,
               bubbles: bool,
               cancelable: bool,
               old_url: String,
               new_url: String)
               -> DomRoot<HashChangeEvent<TH>> {
        let ev = reflect_dom_object(Box::new(HashChangeEvent::new_inherited(old_url, new_url)),
                                    window,
                                    HashChangeEventBinding::Wrap);
        {
            let event = ev.upcast::<Event<TH>>();
            event.init_event(type_, bubbles, cancelable);
        }
        ev
    }

    pub fn Constructor(window: &Window<TH>,
                       type_: DOMString,
                       init: &HashChangeEventBinding::HashChangeEventInit)
                       -> Fallible<DomRoot<HashChangeEvent<TH>>, TH> {
        Ok(HashChangeEvent::new(window,
                                Atom::from(type_),
                                init.parent.bubbles,
                                init.parent.cancelable,
                                init.oldURL.0.clone(),
                                init.newURL.0.clone()))
    }
}

impl<TH> HashChangeEventMethods for HashChangeEvent<TH> {
    // https://html.spec.whatwg.org/multipage/#dom-hashchangeevent-oldurl
    fn OldURL(&self) -> USVString {
        USVString(self.old_url.clone())
    }

    // https://html.spec.whatwg.org/multipage/#dom-hashchangeevent-newurl
    fn NewURL(&self) -> USVString {
        USVString(self.new_url.clone())
    }

    // https://dom.spec.whatwg.org/#dom-event-istrusted
    fn IsTrusted(&self) -> bool {
        self.event.IsTrusted()
    }
}
