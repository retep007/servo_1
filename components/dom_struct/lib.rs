/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#![feature(proc_macro)]

extern crate proc_macro;
extern crate syn;
#[macro_use] extern crate quote;

use proc_macro::{TokenStream};
use syn::DeriveInput;

#[proc_macro_attribute]
pub fn dom_struct(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut base = quote! {};
    if !args.is_empty() {
        let args = args.to_string();
        let crater = args.trim_matches(&['(', ')', ' '][..]);
        base = quote! {#[base = #crater]};
    }
    let attributes = quote! {
        #[derive(DomObject, DenyPublicFields, JSTraceable, MallocSizeOf)]
        #base
        #[must_root]
        #[repr(C)]
    };

    // Work around https://github.com/rust-lang/rust/issues/46489
    let attributes: TokenStream = attributes.to_string().parse().unwrap();

    attributes.into_iter().chain(input.into_iter()).collect()
}
