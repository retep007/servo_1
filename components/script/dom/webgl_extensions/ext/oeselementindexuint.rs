/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use canvas_traits::webgl::WebGLVersion;
use dom::bindings::codegen::Bindings::OESElementIndexUintBinding;
use dom::bindings::reflector::{DomObject, Reflector, reflect_dom_object};
use dom::bindings::root::DomRoot;
use dom::webglrenderingcontext::WebGLRenderingContext;
use dom_struct::dom_struct;
use super::{WebGLExtension, WebGLExtensions, WebGLExtensionSpec};
use typeholder::TypeHolderTrait;

#[dom_struct]
pub struct OESElementIndexUint<TH: TypeHolderTrait> {
    reflector_: Reflector<TH>,
}

impl<TH> OESElementIndexUint<TH> {
    fn new_inherited() -> Self {
        Self { reflector_: Reflector::new() }
    }
}

impl<TH> WebGLExtension<TH> for OESElementIndexUint<TH> {
    type Extension = Self;

    fn new(ctx: &WebGLRenderingContext<TH>) -> DomRoot<Self> {
        reflect_dom_object(
            Box::new(OESElementIndexUint::new_inherited()),
            &*ctx.global(),
            OESElementIndexUintBinding::Wrap,
        )
    }

    fn spec() -> WebGLExtensionSpec {
        WebGLExtensionSpec::Specific(WebGLVersion::WebGL1)
    }

    fn is_supported(ext: &WebGLExtensions<TH>) -> bool {
        ext.supports_gl_extension("GL_OES_element_index_uint")
    }

    fn enable(ext: &WebGLExtensions<TH>) {
        ext.enable_element_index_uint();
    }

    fn name() -> &'static str {
        "OES_element_index_uint"
    }
}
