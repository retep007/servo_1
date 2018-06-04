/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::EventBinding::EventMethods;
use dom::bindings::codegen::Bindings::StorageEventBinding;
use dom::bindings::codegen::Bindings::StorageEventBinding::StorageEventMethods;
use dom::bindings::error::Fallible;
use dom::bindings::inheritance::Castable;
use dom::bindings::reflector::reflect_dom_object;
use dom::bindings::root::{DomRoot, MutNullableDom, RootedReference};
use dom::bindings::str::DOMString;
use dom::event::{Event, EventBubbles, EventCancelable};
use dom::storage::Storage;
use dom::window::Window;
use dom_struct::dom_struct;
use servo_atoms::Atom;
use typeholder::TypeHolderTrait;

#[dom_struct]
pub struct StorageEvent<TH: TypeHolderTrait<TH> + 'static> {
    event: Event<TH>,
    key: Option<DOMString>,
    old_value: Option<DOMString>,
    new_value: Option<DOMString>,
    url: DOMString,
    storage_area: MutNullableDom<Storage<TH>>
}


impl<TH: TypeHolderTrait<TH>> StorageEvent<TH> {
    pub fn new_inherited(key: Option<DOMString>,
                         old_value: Option<DOMString>,
                         new_value: Option<DOMString>,
                         url: DOMString,
                         storage_area: Option<&Storage<TH>>) -> StorageEvent<TH> {
        StorageEvent {
            event: Event::new_inherited(),
            key: key,
            old_value: old_value,
            new_value: new_value,
            url: url,
            storage_area: MutNullableDom::new(storage_area)
        }
    }

    pub fn new_uninitialized(window: &Window<TH>,
                             url: DOMString) -> DomRoot<StorageEvent<TH>> {
        reflect_dom_object(Box::new(StorageEvent::new_inherited(None, None, None, url, None)),
                           window,
                           StorageEventBinding::Wrap)
    }

    pub fn new(global: &Window<TH>,
               type_: Atom,
               bubbles: EventBubbles,
               cancelable: EventCancelable,
               key: Option<DOMString>,
               oldValue: Option<DOMString>,
               newValue: Option<DOMString>,
               url: DOMString,
               storageArea: Option<&Storage<TH>>) -> DomRoot<StorageEvent<TH>> {
        let ev = reflect_dom_object(
            Box::new(StorageEvent::new_inherited(key, oldValue, newValue, url, storageArea)),
            global,
            StorageEventBinding::Wrap
        );
        {
            let event = ev.upcast::<Event<TH>>();
            event.init_event(type_, bool::from(bubbles), bool::from(cancelable));
        }
        ev
    }

    pub fn Constructor(global: &Window<TH>,
                       type_: DOMString,
                       init: &StorageEventBinding::StorageEventInit<TH>) -> Fallible<DomRoot<StorageEvent<TH>>, TH> {
        let key = init.key.clone();
        let oldValue = init.oldValue.clone();
        let newValue = init.newValue.clone();
        let url = init.url.clone();
        let storageArea = init.storageArea.r();
        let bubbles = EventBubbles::from(init.parent.bubbles);
        let cancelable = EventCancelable::from(init.parent.cancelable);
        let event = StorageEvent::new(global, Atom::from(type_),
                                      bubbles, cancelable,
                                      key, oldValue, newValue,
                                      url, storageArea);
        Ok(event)
    }
}

impl<TH: TypeHolderTrait<TH>> StorageEventMethods<TH> for StorageEvent<TH> {
    // https://html.spec.whatwg.org/multipage/#dom-storageevent-key
    fn GetKey(&self) -> Option<DOMString> {
        self.key.clone()
    }

    // https://html.spec.whatwg.org/multipage/#dom-storageevent-oldvalue
    fn GetOldValue(&self) -> Option<DOMString> {
        self.old_value.clone()
    }

    // https://html.spec.whatwg.org/multipage/#dom-storageevent-newvalue
    fn GetNewValue(&self) -> Option<DOMString> {
        self.new_value.clone()
    }

    // https://html.spec.whatwg.org/multipage/#dom-storageevent-url
    fn Url(&self) -> DOMString {
        self.url.clone()
    }

    // https://html.spec.whatwg.org/multipage/#dom-storageevent-storagearea
    fn GetStorageArea(&self) -> Option<DomRoot<Storage<TH>>> {
        self.storage_area.get()
    }

    // https://dom.spec.whatwg.org/#dom-event-istrusted
    fn IsTrusted(&self) -> bool {
        self.event.IsTrusted()
    }
}
