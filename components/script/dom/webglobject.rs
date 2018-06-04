/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

// https://www.khronos.org/registry/webgl/specs/latest/1.0/webgl.idl
use dom::bindings::reflector::Reflector;
use dom_struct::dom_struct;
use typeholder::TypeHolderTrait;
use std::marker::PhantomData;

#[dom_struct]
pub struct WebGLObject<TH: TypeHolderTrait<TH> + 'static> {
    reflector_: Reflector<TH>,
    _p: PhantomData<TH>,
}

impl<TH: TypeHolderTrait<TH>> WebGLObject<TH> {
    pub fn new_inherited() -> WebGLObject<TH> {
        WebGLObject {
            reflector_: Reflector::new(),
            _p: Default::default(),
        }
    }
}
