/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::DOMStringMapBinding;
use dom::bindings::codegen::Bindings::DOMStringMapBinding::DOMStringMapMethods;
use dom::bindings::error::ErrorResult;
use dom::bindings::reflector::{Reflector, reflect_dom_object};
use dom::bindings::root::{Dom, DomRoot};
use dom::bindings::str::DOMString;
use dom::htmlelement::HTMLElement;
use dom::node::window_from_node;
use dom_struct::dom_struct;
use typeholder::TypeHolderTrait;

#[dom_struct]
pub struct DOMStringMap<TH: TypeHolderTrait> {
    reflector_: Reflector<TH>,
    element: Dom<HTMLElement<TH>>,
}

impl<TH> DOMStringMap<TH> {
    fn new_inherited(element: &HTMLElement<TH>) -> DOMStringMap<TH> {
        DOMStringMap {
            reflector_: Reflector::new(),
            element: Dom::from_ref(element),
        }
    }

    pub fn new(element: &HTMLElement<TH>) -> DomRoot<DOMStringMap<TH>> {
        let window = window_from_node(element);
        reflect_dom_object(Box::new(DOMStringMap::new_inherited(element)),
                           &*window,
                           DOMStringMapBinding::Wrap)
    }
}

// https://html.spec.whatwg.org/multipage/#domstringmap
impl<TH> DOMStringMapMethods<TH> for DOMStringMap<TH> {
    // https://html.spec.whatwg.org/multipage/#dom-domstringmap-removeitem
    fn NamedDeleter(&self, name: DOMString) {
        self.element.delete_custom_attr(name)
    }

    // https://html.spec.whatwg.org/multipage/#dom-domstringmap-setitem
    fn NamedSetter(&self, name: DOMString, value: DOMString) -> ErrorResult<TH> {
        self.element.set_custom_attr(name, value)
    }

    // https://html.spec.whatwg.org/multipage/#dom-domstringmap-nameditem
    fn NamedGetter(&self, name: DOMString) -> Option<DOMString> {
        self.element.get_custom_attr(name)
    }

    // https://html.spec.whatwg.org/multipage/#the-domstringmap-interface:supported-property-names
    fn SupportedPropertyNames(&self) -> Vec<DOMString> {
        self.element.supported_prop_names_custom_attr().iter().cloned().collect()
    }
}
