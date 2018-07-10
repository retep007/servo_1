use dom::bindings::reflector::{DomObject, Reflector, reflect_dom_object};
use dom_struct::dom_struct;
use dom::bindings::trace::JSTraceable;

#[dom_struct]
pub struct Test<TH: TypeHolderTrait + 'static> {
	reflector_: Reflector,
	_p: TH::ServoParser,
	_a: i32,
}

pub trait TypeHolderTrait {
    type ServoParser: ServoParser;

}

pub trait ServoParser {
    
}

// #[allow(non_upper_case_globals)]
// const _IMPL_DOMOBJECT_FOR_Test: () = {
//     impl<TH: TypeHolderTrait + 'static> ::js::conversions::ToJSValConvertible for Test<TH> {
//         #[allow(unsafe_code)]
//         unsafe fn to_jsval(
//             &self,
//             cx: *mut ::js::jsapi::JSContext,
//             rval: ::js::rust::MutableHandleValue,
//         ) {
//             let object = ::dom::bindings::reflector::DomObject::reflector(self).get_jsobject();
//             object.to_jsval(cx, rval)
//         }
//     }
//     impl<TH: TypeHolderTrait + 'static> ::dom::bindings::reflector::DomObject for Test<TH> {
//         #[inline]
//         fn reflector(&self) -> &::dom::bindings::reflector::Reflector {
//             self.reflector_.reflector()
//         }
//     }
//     impl<TH: TypeHolderTrait + 'static> ::dom::bindings::reflector::MutDomObject for Test<TH> {
//         fn init_reflector(&mut self, obj: *mut ::js::jsapi::JSObject) {
//             self.reflector_.init_reflector(obj);
//         }
//     }
//     impl<TH: TypeHolderTrait + 'static> ShouldNotImplDomObject for ((TH), TH::ServoParser) {}
//     impl<TH: TypeHolderTrait + 'static> ShouldNotImplDomObject for ((TH), i32) {}
//     trait ShouldNotImplDomObject {}
//     impl<TH: TypeHolderTrait + 'static, __T: ::dom::bindings::reflector::DomObject>
//         ShouldNotImplDomObject for ((TH), __T)
//     {}
// };