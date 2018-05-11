/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use cssparser::RGBA;
use dom::bindings::codegen::Bindings::HTMLTableSectionElementBinding::{self, HTMLTableSectionElementMethods};
use dom::bindings::codegen::Bindings::NodeBinding::NodeMethods;
use dom::bindings::error::{ErrorResult, Fallible};
use dom::bindings::inheritance::Castable;
use dom::bindings::root::{DomRoot, LayoutDom, RootedReference};
use dom::bindings::str::DOMString;
use dom::document::Document;
use dom::element::{Element, RawLayoutElementHelpers};
use dom::htmlcollection::{CollectionFilter, HTMLCollection};
use dom::htmlelement::HTMLElement;
use dom::htmltablerowelement::HTMLTableRowElement;
use dom::node::{Node, window_from_node};
use dom::virtualmethods::VirtualMethods;
use dom_struct::dom_struct;
use html5ever::{LocalName, Prefix};
use style::attr::AttrValue;
use typeholder::TypeHolderTrait;

#[dom_struct]
pub struct HTMLTableSectionElement<TH: TypeHolderTrait> {
    htmlelement: HTMLElement,
}

impl<TH: TypeHolderTrait> HTMLTableSectionElement<TH> {
    fn new_inherited(local_name: LocalName, prefix: Option<Prefix>, document: &Document<TH>)
                     -> HTMLTableSectionElement<TH> {
        HTMLTableSectionElement {
            htmlelement: HTMLElement::new_inherited(local_name, prefix, document),
        }
    }

    #[allow(unrooted_must_root)]
    pub fn new(local_name: LocalName, prefix: Option<Prefix>, document: &Document<TH>)
               -> DomRoot<HTMLTableSectionElement<TH>> {
        Node::reflect_node(Box::new(HTMLTableSectionElement::new_inherited(local_name, prefix, document)),
                           document,
                           HTMLTableSectionElementBinding::Wrap)
    }
}

#[derive(JSTraceable)]
struct RowsFilter<TH: TypeHolderTrait>;
impl<TH: TypeHolderTrait> CollectionFilter for RowsFilter<TH> {
    fn filter(&self, elem: &Element, root: &Node<TH>) -> bool {
        elem.is::<HTMLTableRowElement>() &&
            elem.upcast::<Node<TH>>().GetParentNode().r() == Some(root)
    }
}

impl<TH: TypeHolderTrait> HTMLTableSectionElementMethods for HTMLTableSectionElement<TH> {
    // https://html.spec.whatwg.org/multipage/#dom-tbody-rows
    fn Rows(&self) -> DomRoot<HTMLCollection> {
        HTMLCollection::create(&window_from_node(self), self.upcast(), Box::new(RowsFilter))
    }

    // https://html.spec.whatwg.org/multipage/#dom-tbody-insertrow
    fn InsertRow(&self, index: i32) -> Fallible<DomRoot<HTMLElement>> {
        let node = self.upcast::<Node<TH>>();
        node.insert_cell_or_row(
            index,
            || self.Rows(),
            || HTMLTableRowElement::new(local_name!("tr"), None, &node.owner_doc()))
    }

    // https://html.spec.whatwg.org/multipage/#dom-tbody-deleterow
    fn DeleteRow(&self, index: i32) -> ErrorResult {
        let node = self.upcast::<Node<TH>>();
        node.delete_cell_or_row(
            index,
            || self.Rows(),
            |n| n.is::<HTMLTableRowElement>())
    }
}

pub trait HTMLTableSectionElementLayoutHelpers {
    fn get_background_color(&self) -> Option<RGBA>;
}

#[allow(unsafe_code)]
impl<TH> HTMLTableSectionElementLayoutHelpers for LayoutDom<HTMLTableSectionElement<TH>> {
    fn get_background_color(&self) -> Option<RGBA> {
        unsafe {
            (&*self.upcast::<Element>().unsafe_get())
                .get_attr_for_layout(&ns!(), &local_name!("bgcolor"))
                .and_then(AttrValue::as_color)
                .cloned()
        }
    }
}

impl<TH> VirtualMethods for HTMLTableSectionElement<TH> {
    fn super_type(&self) -> Option<&VirtualMethods> {
        Some(self.upcast::<HTMLElement>() as &VirtualMethods)
    }

    fn parse_plain_attribute(&self, local_name: &LocalName, value: DOMString) -> AttrValue {
        match *local_name {
            local_name!("bgcolor") => AttrValue::from_legacy_color(value.into()),
            _ => self.super_type().unwrap().parse_plain_attribute(local_name, value),
        }
    }
}
