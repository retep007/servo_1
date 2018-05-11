/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#![allow(dead_code)]

// https://www.khronos.org/registry/webgl/specs/latest/1.0/webgl.idl
use dom::bindings::codegen::Bindings::WebGLShaderPrecisionFormatBinding;
use dom::bindings::codegen::Bindings::WebGLShaderPrecisionFormatBinding::WebGLShaderPrecisionFormatMethods;
use dom::bindings::reflector::{Reflector, reflect_dom_object};
use dom::bindings::root::DomRoot;
use dom::window::Window;
use dom_struct::dom_struct;
use typeholder::TypeHolderTrait;

#[dom_struct]
pub struct WebGLShaderPrecisionFormat<TH: TypeHolderTrait> {
    reflector_: Reflector,
    range_min: i32,
    range_max: i32,
    precision: i32,
}

impl<TH: TypeHolderTrait> WebGLShaderPrecisionFormat<TH> {
    fn new_inherited(range_min: i32, range_max: i32, precision: i32) -> WebGLShaderPrecisionFormat {
        WebGLShaderPrecisionFormat {
            reflector_: Reflector::new(),
            range_min: range_min,
            range_max: range_max,
            precision: precision,
        }
    }

    pub fn new(window: &Window<TH>,
               range_min: i32,
               range_max: i32,
               precision: i32) -> DomRoot<WebGLShaderPrecisionFormat> {
        reflect_dom_object(
            Box::new(WebGLShaderPrecisionFormat::new_inherited(range_min, range_max, precision)),
            window,
            WebGLShaderPrecisionFormatBinding::Wrap)
    }
}

impl<TH> WebGLShaderPrecisionFormatMethods for WebGLShaderPrecisionFormat<TH> {
    // https://www.khronos.org/registry/webgl/specs/1.0/#5.12.1
    fn RangeMin(&self) -> i32 {
        self.range_min
    }

    // https://www.khronos.org/registry/webgl/specs/1.0/#5.12.1
    fn RangeMax(&self) -> i32 {
        self.range_max
    }

    // https://www.khronos.org/registry/webgl/specs/1.0/#5.12.1
    fn Precision(&self) -> i32 {
        self.precision
    }
}
