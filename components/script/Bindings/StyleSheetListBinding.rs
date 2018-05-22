/* THIS FILE IS AUTOGENERATED - DO NOT EDIT */

#![allow(non_camel_case_types,non_upper_case_globals,unused_imports,unused_variables,unused_assignments,unused_mut)]
use dom;
use dom::bindings;
use dom::bindings::callback::CallSetup;
use dom::bindings::callback::CallbackContainer;
use dom::bindings::callback::CallbackFunction;
use dom::bindings::callback::CallbackInterface;
use dom::bindings::callback::CallbackObject;
use dom::bindings::callback::ExceptionHandling;
use dom::bindings::callback::wrap_call_this_object;
use dom::bindings::codegen::InterfaceObjectMap;
use dom::bindings::codegen::PrototypeList;
use dom::bindings::codegen::RegisterBindings;
use dom::bindings::codegen::UnionTypes;
use dom::bindings::constant::ConstantSpec;
use dom::bindings::constant::ConstantVal;
use dom::bindings::conversions::ConversionBehavior;
use dom::bindings::conversions::ConversionResult;
use dom::bindings::conversions::DOM_OBJECT_SLOT;
use dom::bindings::conversions::FromJSValConvertible;
use dom::bindings::conversions::IDLInterface;
use dom::bindings::conversions::StringificationBehavior;
use dom::bindings::conversions::ToJSValConvertible;
use dom::bindings::conversions::is_array_like;
use dom::bindings::conversions::jsid_to_string;
use dom::bindings::conversions::native_from_handlevalue;
use dom::bindings::conversions::native_from_object;
use dom::bindings::conversions::private_from_object;
use dom::bindings::conversions::root_from_handleobject;
use dom::bindings::conversions::root_from_handlevalue;
use dom::bindings::conversions::root_from_object;
use dom::bindings::error::Error;
use dom::bindings::error::Error::JSFailed;
use dom::bindings::error::ErrorResult;
use dom::bindings::error::Fallible;
use dom::bindings::error::throw_dom_exception;
use dom::bindings::guard::Condition;
use dom::bindings::guard::Guard;
use dom::bindings::htmlconstructor::html_constructor;
use dom::bindings::htmlconstructor::pop_current_element_queue;
use dom::bindings::htmlconstructor::push_new_element_queue;
use dom::bindings::inheritance::Castable;
use dom::bindings::interface::ConstructorClassHook;
use dom::bindings::interface::InterfaceConstructorBehavior;
use dom::bindings::interface::NonCallbackInterfaceObjectClass;
use dom::bindings::interface::create_callback_interface_object;
use dom::bindings::interface::create_global_object;
use dom::bindings::interface::create_interface_prototype_object;
use dom::bindings::interface::create_named_constructors;
use dom::bindings::interface::create_noncallback_interface_object;
use dom::bindings::interface::define_guarded_constants;
use dom::bindings::interface::define_guarded_methods;
use dom::bindings::interface::define_guarded_properties;
use dom::bindings::interface::is_exposed_in;
use dom::bindings::iterable::Iterable;
use dom::bindings::iterable::IteratorType;
use dom::bindings::mozmap::MozMap;
use dom::bindings::namespace::NamespaceObjectClass;
use dom::bindings::namespace::create_namespace_object;
use dom::bindings::num::Finite;
use dom::bindings::proxyhandler;
use dom::bindings::proxyhandler::ensure_expando_object;
use dom::bindings::proxyhandler::fill_property_descriptor;
use dom::bindings::proxyhandler::get_expando_object;
use dom::bindings::proxyhandler::get_property_descriptor;
use dom::bindings::reflector::DomObject;
use dom::bindings::reflector::MutDomObject;
use dom::bindings::root::Dom;
use dom::bindings::root::DomRoot;
use dom::bindings::root::OptionalHeapSetter;
use dom::bindings::root::RootedReference;
use dom::bindings::str::ByteString;
use dom::bindings::str::DOMString;
use dom::bindings::str::USVString;
use dom::bindings::trace::JSTraceable;
use dom::bindings::trace::RootedTraceable;
use dom::bindings::trace::RootedTraceableBox;
use dom::bindings::utils::AsVoidPtr;
use dom::bindings::utils::DOMClass;
use dom::bindings::utils::DOMJSClass;
use dom::bindings::utils::DOM_PROTO_UNFORGEABLE_HOLDER_SLOT;
use dom::bindings::utils::JSCLASS_DOM_GLOBAL;
use dom::bindings::utils::ProtoOrIfaceArray;
use dom::bindings::utils::enumerate_global;
use dom::bindings::utils::finalize_global;
use dom::bindings::utils::find_enum_value;
use dom::bindings::utils::generic_getter;
use dom::bindings::utils::generic_lenient_getter;
use dom::bindings::utils::generic_lenient_setter;
use dom::bindings::utils::generic_method;
use dom::bindings::utils::generic_setter;
use dom::bindings::utils::get_array_index_from_id;
use dom::bindings::utils::get_dictionary_property;
use dom::bindings::utils::get_property_on_prototype;
use dom::bindings::utils::get_proto_or_iface_array;
use dom::bindings::utils::has_property_on_prototype;
use dom::bindings::utils::is_platform_object;
use dom::bindings::utils::resolve_global;
use dom::bindings::utils::set_dictionary_property;
use dom::bindings::utils::trace_global;
use dom::bindings::weakref::DOM_WEAK_SLOT;
use dom::bindings::weakref::WeakBox;
use dom::bindings::weakref::WeakReferenceable;
use dom::globalscope::GlobalScope;
use dom::windowproxy::WindowProxy;
use js;
use js::JSCLASS_GLOBAL_SLOT_COUNT;
use js::JSCLASS_IS_DOMJSCLASS;
use js::JSCLASS_IS_GLOBAL;
use js::JSCLASS_RESERVED_SLOTS_MASK;
use js::JS_CALLEE;
use js::error::throw_internal_error;
use js::error::throw_type_error;
use js::glue::AppendToAutoIdVector;
use js::glue::CallJitGetterOp;
use js::glue::CallJitMethodOp;
use js::glue::CallJitSetterOp;
use js::glue::CreateProxyHandler;
use js::glue::GetProxyPrivate;
use js::glue::ProxyTraps;
use js::glue::RUST_JSID_IS_INT;
use js::glue::RUST_JSID_IS_STRING;
use js::glue::RUST_SYMBOL_TO_JSID;
use js::glue::UnwrapObject;
use js::glue::int_to_jsid;
use js::jsapi::AutoIdVector;
use js::jsapi::CallArgs;
use js::jsapi::CurrentGlobalOrNull;
use js::jsapi::FreeOp;
use js::jsapi::GetWellKnownSymbol;
use js::jsapi::Handle as RawHandle;
use js::jsapi::HandleId as RawHandleId;
use js::jsapi::HandleObject as RawHandleObject;
use js::jsapi::HandleValue as RawHandleValue;
use js::jsapi::HandleValueArray;
use js::jsapi::Heap;
use js::jsapi::INTERNED_STRING_TO_JSID;
use js::jsapi::IsCallable;
use js::jsapi::JSAutoCompartment;
use js::jsapi::JSCLASS_RESERVED_SLOTS_SHIFT;
use js::jsapi::JSClass;
use js::jsapi::JSContext;
use js::jsapi::JSFreeOp;
use js::jsapi::JSFunctionSpec;
use js::jsapi::JSITER_HIDDEN;
use js::jsapi::JSITER_OWNONLY;
use js::jsapi::JSITER_SYMBOLS;
use js::jsapi::JSJitGetterCallArgs;
use js::jsapi::JSJitInfo;
use js::jsapi::JSJitInfo_AliasSet;
use js::jsapi::JSJitInfo_ArgType;
use js::jsapi::JSJitInfo_OpType;
use js::jsapi::JSJitMethodCallArgs;
use js::jsapi::JSJitSetterCallArgs;
use js::jsapi::JSNative;
use js::jsapi::JSNativeWrapper;
use js::jsapi::JSObject;
use js::jsapi::JSPROP_ENUMERATE;
use js::jsapi::JSPROP_PERMANENT;
use js::jsapi::JSPROP_READONLY;
use js::jsapi::JSPROP_SHARED;
use js::jsapi::JSPropertySpec;
use js::jsapi::JSString;
use js::jsapi::JSTracer;
use js::jsapi::JSType;
use js::jsapi::JSTypedMethodJitInfo;
use js::jsapi::JSValueType;
use js::jsapi::JS_AtomizeAndPinString;
use js::jsapi::JS_ForwardGetPropertyTo;
use js::jsapi::JS_GetErrorPrototype;
use js::jsapi::JS_GetGlobalForObject;
use js::jsapi::JS_GetIteratorPrototype;
use js::jsapi::JS_GetPropertyById;
use js::jsapi::JS_GetPropertyDescriptorById;
use js::jsapi::JS_GetReservedSlot;
use js::jsapi::JS_HasProperty;
use js::jsapi::JS_HasPropertyById;
use js::jsapi::JS_NewObject;
use js::jsapi::JS_SetReservedSlot;
use js::jsapi::MutableHandle as RawMutableHandle;
use js::jsapi::MutableHandleObject as RawMutableHandleObject;
use js::jsapi::MutableHandleValue as RawMutableHandleValue;
use js::jsapi::ObjectOpResult;
use js::jsapi::PropertyDescriptor;
use js::jsapi::Rooted;
use js::jsapi::RootedId;
use js::jsapi::RootedObject;
use js::jsapi::RootedString;
use js::jsapi::SymbolCode;
use js::jsapi::jsid;
use js::jsval::JSVal;
use js::jsval::NullValue;
use js::jsval::ObjectOrNullValue;
use js::jsval::ObjectValue;
use js::jsval::PrivateValue;
use js::jsval::UndefinedValue;
use js::panic::maybe_resume_unwind;
use js::panic::wrap_panic;
use js::rust::CustomAutoRooterGuard;
use js::rust::GCMethods;
use js::rust::Handle;
use js::rust::HandleId;
use js::rust::HandleObject;
use js::rust::HandleValue;
use js::rust::MutableHandle;
use js::rust::MutableHandleObject;
use js::rust::MutableHandleValue;
use js::rust::define_methods;
use js::rust::define_properties;
use js::rust::get_object_class;
use js::rust::wrappers::Call;
use js::rust::wrappers::GetPropertyKeys;
use js::rust::wrappers::JS_CallFunctionValue;
use js::rust::wrappers::JS_CopyPropertiesFrom;
use js::rust::wrappers::JS_DefineProperty;
use js::rust::wrappers::JS_DefinePropertyById2;
use js::rust::wrappers::JS_GetFunctionPrototype;
use js::rust::wrappers::JS_GetObjectPrototype;
use js::rust::wrappers::JS_GetProperty;
use js::rust::wrappers::JS_InitializePropertiesFromCompatibleNativeObject;
use js::rust::wrappers::JS_NewObjectWithGivenProto;
use js::rust::wrappers::JS_NewObjectWithoutMetadata;
use js::rust::wrappers::JS_ObjectIsDate;
use js::rust::wrappers::JS_SetImmutablePrototype;
use js::rust::wrappers::JS_SetProperty;
use js::rust::wrappers::JS_SetPrototype;
use js::rust::wrappers::JS_SplicePrototype;
use js::rust::wrappers::JS_WrapObject;
use js::rust::wrappers::JS_WrapValue;
use js::rust::wrappers::NewProxyObject;
use js::typedarray;
use libc;
use malloc_size_of::MallocSizeOfOps;
use mem::malloc_size_of_including_raw_self;
use servo_config::prefs::PREFS;
use std::borrow::ToOwned;
use std::cmp;
use std::default::Default;
use std::ffi::CString;
use std::mem;
use std::num;
use std::os;
use std::os::raw::c_void;
use std::panic;
use std::ptr;
use std::ptr::NonNull;
use std::rc;
use std::rc::Rc;
use std::str;
use typeholder::TypeHolderTrait;

pub use self::StyleSheetListBinding::{Wrap, StyleSheetListMethods, GetProtoObject, DefineDOMInterface};
pub mod StyleSheetListBinding {
#![allow(non_camel_case_types,non_upper_case_globals,unused_imports,unused_variables,unused_assignments,unused_mut)]
use dom;
use dom::bindings;
use dom::bindings::callback::CallSetup;
use dom::bindings::callback::CallbackContainer;
use dom::bindings::callback::CallbackFunction;
use dom::bindings::callback::CallbackInterface;
use dom::bindings::callback::CallbackObject;
use dom::bindings::callback::ExceptionHandling;
use dom::bindings::callback::wrap_call_this_object;
use dom::bindings::codegen::InterfaceObjectMap;
use dom::bindings::codegen::PrototypeList;
use dom::bindings::codegen::RegisterBindings;
use dom::bindings::codegen::UnionTypes;
use dom::bindings::constant::ConstantSpec;
use dom::bindings::constant::ConstantVal;
use dom::bindings::conversions::ConversionBehavior;
use dom::bindings::conversions::ConversionResult;
use dom::bindings::conversions::DOM_OBJECT_SLOT;
use dom::bindings::conversions::FromJSValConvertible;
use dom::bindings::conversions::IDLInterface;
use dom::bindings::conversions::StringificationBehavior;
use dom::bindings::conversions::ToJSValConvertible;
use dom::bindings::conversions::is_array_like;
use dom::bindings::conversions::jsid_to_string;
use dom::bindings::conversions::native_from_handlevalue;
use dom::bindings::conversions::native_from_object;
use dom::bindings::conversions::private_from_object;
use dom::bindings::conversions::root_from_handleobject;
use dom::bindings::conversions::root_from_handlevalue;
use dom::bindings::conversions::root_from_object;
use dom::bindings::error::Error;
use dom::bindings::error::Error::JSFailed;
use dom::bindings::error::ErrorResult;
use dom::bindings::error::Fallible;
use dom::bindings::error::throw_dom_exception;
use dom::bindings::guard::Condition;
use dom::bindings::guard::Guard;
use dom::bindings::htmlconstructor::html_constructor;
use dom::bindings::htmlconstructor::pop_current_element_queue;
use dom::bindings::htmlconstructor::push_new_element_queue;
use dom::bindings::inheritance::Castable;
use dom::bindings::interface::ConstructorClassHook;
use dom::bindings::interface::InterfaceConstructorBehavior;
use dom::bindings::interface::NonCallbackInterfaceObjectClass;
use dom::bindings::interface::create_callback_interface_object;
use dom::bindings::interface::create_global_object;
use dom::bindings::interface::create_interface_prototype_object;
use dom::bindings::interface::create_named_constructors;
use dom::bindings::interface::create_noncallback_interface_object;
use dom::bindings::interface::define_guarded_constants;
use dom::bindings::interface::define_guarded_methods;
use dom::bindings::interface::define_guarded_properties;
use dom::bindings::interface::is_exposed_in;
use dom::bindings::iterable::Iterable;
use dom::bindings::iterable::IteratorType;
use dom::bindings::mozmap::MozMap;
use dom::bindings::namespace::NamespaceObjectClass;
use dom::bindings::namespace::create_namespace_object;
use dom::bindings::num::Finite;
use dom::bindings::proxyhandler;
use dom::bindings::proxyhandler::ensure_expando_object;
use dom::bindings::proxyhandler::fill_property_descriptor;
use dom::bindings::proxyhandler::get_expando_object;
use dom::bindings::proxyhandler::get_property_descriptor;
use dom::bindings::reflector::DomObject;
use dom::bindings::reflector::MutDomObject;
use dom::bindings::root::Dom;
use dom::bindings::root::DomRoot;
use dom::bindings::root::OptionalHeapSetter;
use dom::bindings::root::RootedReference;
use dom::bindings::str::ByteString;
use dom::bindings::str::DOMString;
use dom::bindings::str::USVString;
use dom::bindings::trace::JSTraceable;
use dom::bindings::trace::RootedTraceable;
use dom::bindings::trace::RootedTraceableBox;
use dom::bindings::utils::AsVoidPtr;
use dom::bindings::utils::DOMClass;
use dom::bindings::utils::DOMJSClass;
use dom::bindings::utils::DOM_PROTO_UNFORGEABLE_HOLDER_SLOT;
use dom::bindings::utils::JSCLASS_DOM_GLOBAL;
use dom::bindings::utils::ProtoOrIfaceArray;
use dom::bindings::utils::enumerate_global;
use dom::bindings::utils::finalize_global;
use dom::bindings::utils::find_enum_value;
use dom::bindings::utils::generic_getter;
use dom::bindings::utils::generic_lenient_getter;
use dom::bindings::utils::generic_lenient_setter;
use dom::bindings::utils::generic_method;
use dom::bindings::utils::generic_setter;
use dom::bindings::utils::get_array_index_from_id;
use dom::bindings::utils::get_dictionary_property;
use dom::bindings::utils::get_property_on_prototype;
use dom::bindings::utils::get_proto_or_iface_array;
use dom::bindings::utils::has_property_on_prototype;
use dom::bindings::utils::is_platform_object;
use dom::bindings::utils::resolve_global;
use dom::bindings::utils::set_dictionary_property;
use dom::bindings::utils::trace_global;
use dom::bindings::weakref::DOM_WEAK_SLOT;
use dom::bindings::weakref::WeakBox;
use dom::bindings::weakref::WeakReferenceable;
use dom::globalscope::GlobalScope;
use dom::types::StyleSheet;
use dom::types::StyleSheetList;
use dom::windowproxy::WindowProxy;
use js;
use js::JSCLASS_GLOBAL_SLOT_COUNT;
use js::JSCLASS_IS_DOMJSCLASS;
use js::JSCLASS_IS_GLOBAL;
use js::JSCLASS_RESERVED_SLOTS_MASK;
use js::JS_CALLEE;
use js::error::throw_internal_error;
use js::error::throw_type_error;
use js::glue::AppendToAutoIdVector;
use js::glue::CallJitGetterOp;
use js::glue::CallJitMethodOp;
use js::glue::CallJitSetterOp;
use js::glue::CreateProxyHandler;
use js::glue::GetProxyPrivate;
use js::glue::ProxyTraps;
use js::glue::RUST_JSID_IS_INT;
use js::glue::RUST_JSID_IS_STRING;
use js::glue::RUST_SYMBOL_TO_JSID;
use js::glue::UnwrapObject;
use js::glue::int_to_jsid;
use js::jsapi::AutoIdVector;
use js::jsapi::CallArgs;
use js::jsapi::CurrentGlobalOrNull;
use js::jsapi::FreeOp;
use js::jsapi::GetWellKnownSymbol;
use js::jsapi::Handle as RawHandle;
use js::jsapi::HandleId as RawHandleId;
use js::jsapi::HandleObject as RawHandleObject;
use js::jsapi::HandleValue as RawHandleValue;
use js::jsapi::HandleValueArray;
use js::jsapi::Heap;
use js::jsapi::INTERNED_STRING_TO_JSID;
use js::jsapi::IsCallable;
use js::jsapi::JSAutoCompartment;
use js::jsapi::JSCLASS_RESERVED_SLOTS_SHIFT;
use js::jsapi::JSClass;
use js::jsapi::JSContext;
use js::jsapi::JSFreeOp;
use js::jsapi::JSFunctionSpec;
use js::jsapi::JSITER_HIDDEN;
use js::jsapi::JSITER_OWNONLY;
use js::jsapi::JSITER_SYMBOLS;
use js::jsapi::JSJitGetterCallArgs;
use js::jsapi::JSJitInfo;
use js::jsapi::JSJitInfo_AliasSet;
use js::jsapi::JSJitInfo_ArgType;
use js::jsapi::JSJitInfo_OpType;
use js::jsapi::JSJitMethodCallArgs;
use js::jsapi::JSJitSetterCallArgs;
use js::jsapi::JSNative;
use js::jsapi::JSNativeWrapper;
use js::jsapi::JSObject;
use js::jsapi::JSPROP_ENUMERATE;
use js::jsapi::JSPROP_PERMANENT;
use js::jsapi::JSPROP_READONLY;
use js::jsapi::JSPROP_SHARED;
use js::jsapi::JSPropertySpec;
use js::jsapi::JSString;
use js::jsapi::JSTracer;
use js::jsapi::JSType;
use js::jsapi::JSTypedMethodJitInfo;
use js::jsapi::JSValueType;
use js::jsapi::JS_AtomizeAndPinString;
use js::jsapi::JS_ForwardGetPropertyTo;
use js::jsapi::JS_GetErrorPrototype;
use js::jsapi::JS_GetGlobalForObject;
use js::jsapi::JS_GetIteratorPrototype;
use js::jsapi::JS_GetPropertyById;
use js::jsapi::JS_GetPropertyDescriptorById;
use js::jsapi::JS_GetReservedSlot;
use js::jsapi::JS_HasProperty;
use js::jsapi::JS_HasPropertyById;
use js::jsapi::JS_NewObject;
use js::jsapi::JS_SetReservedSlot;
use js::jsapi::MutableHandle as RawMutableHandle;
use js::jsapi::MutableHandleObject as RawMutableHandleObject;
use js::jsapi::MutableHandleValue as RawMutableHandleValue;
use js::jsapi::ObjectOpResult;
use js::jsapi::PropertyDescriptor;
use js::jsapi::Rooted;
use js::jsapi::RootedId;
use js::jsapi::RootedObject;
use js::jsapi::RootedString;
use js::jsapi::SymbolCode;
use js::jsapi::jsid;
use js::jsval::JSVal;
use js::jsval::NullValue;
use js::jsval::ObjectOrNullValue;
use js::jsval::ObjectValue;
use js::jsval::PrivateValue;
use js::jsval::UndefinedValue;
use js::panic::maybe_resume_unwind;
use js::panic::wrap_panic;
use js::rust::CustomAutoRooterGuard;
use js::rust::GCMethods;
use js::rust::Handle;
use js::rust::HandleId;
use js::rust::HandleObject;
use js::rust::HandleValue;
use js::rust::MutableHandle;
use js::rust::MutableHandleObject;
use js::rust::MutableHandleValue;
use js::rust::define_methods;
use js::rust::define_properties;
use js::rust::get_object_class;
use js::rust::wrappers::Call;
use js::rust::wrappers::GetPropertyKeys;
use js::rust::wrappers::JS_CallFunctionValue;
use js::rust::wrappers::JS_CopyPropertiesFrom;
use js::rust::wrappers::JS_DefineProperty;
use js::rust::wrappers::JS_DefinePropertyById2;
use js::rust::wrappers::JS_GetFunctionPrototype;
use js::rust::wrappers::JS_GetObjectPrototype;
use js::rust::wrappers::JS_GetProperty;
use js::rust::wrappers::JS_InitializePropertiesFromCompatibleNativeObject;
use js::rust::wrappers::JS_NewObjectWithGivenProto;
use js::rust::wrappers::JS_NewObjectWithoutMetadata;
use js::rust::wrappers::JS_ObjectIsDate;
use js::rust::wrappers::JS_SetImmutablePrototype;
use js::rust::wrappers::JS_SetProperty;
use js::rust::wrappers::JS_SetPrototype;
use js::rust::wrappers::JS_SplicePrototype;
use js::rust::wrappers::JS_WrapObject;
use js::rust::wrappers::JS_WrapValue;
use js::rust::wrappers::NewProxyObject;
use js::typedarray;
use libc;
use malloc_size_of::MallocSizeOfOps;
use mem::malloc_size_of_including_raw_self;
use servo_config::prefs::PREFS;
use std::borrow::ToOwned;
use std::cmp;
use std::default::Default;
use std::ffi::CString;
use std::mem;
use std::num;
use std::os;
use std::os::raw::c_void;
use std::panic;
use std::ptr;
use std::ptr::NonNull;
use std::rc;
use std::rc::Rc;
use std::str;
use typeholder::TypeHolderTrait;

unsafe extern fn item<TH: TypeHolderTrait>
(cx: *mut JSContext, _obj: HandleObject, this: *const StyleSheetList<TH>, args: *const JSJitMethodCallArgs) -> bool {
    return wrap_panic(panic::AssertUnwindSafe(|| {
        let this = &*this;
        let args = &*args;
        let argc = args._base.argc_;

        if argc < 1 {
            throw_type_error(cx, "Not enough arguments to \"StyleSheetList.item\".");
            return false;
        }
        let arg0: u32 = match FromJSValConvertible::from_jsval(cx, args.get(0), ConversionBehavior::Default) {
            Ok(ConversionResult::Success(v)) => v,
            Ok(ConversionResult::Failure(error)) => {
                throw_type_error(cx, &error);
                return false;

            }
            _ => { return false;
         }
        };
        let result: Option<DomRoot<StyleSheet<TH>>> = this.Item(arg0);

        (result).to_jsval(cx, args.rval());
        return true;
    }), false);
}


const item_methodinfo: JSJitInfo = JSJitInfo {
    call: item as *const os::raw::c_void,
    protoID: PrototypeList::ID::StyleSheetList as u16,
    depth: 0,
    _bitfield_1: new_jsjitinfo_bitfield_1!(
        JSJitInfo_OpType::Method as u8,
        JSJitInfo_AliasSet::AliasEverything as u8,
        JSValueType::JSVAL_TYPE_UNKNOWN as u8,
        false,
        false,
        false,
        false,
        false,
        false,
        0,
    ),
};

unsafe extern fn get_length<TH: TypeHolderTrait>
(cx: *mut JSContext, _obj: HandleObject, this: *const StyleSheetList<TH>, args: JSJitGetterCallArgs) -> bool {
    return wrap_panic(panic::AssertUnwindSafe(|| {
        let this = &*this;
        let result: u32 = this.Length();

        (result).to_jsval(cx, args.rval());
        return true;
    }), false);
}


const length_getterinfo: JSJitInfo = JSJitInfo {
    call: get_length as *const os::raw::c_void,
    protoID: PrototypeList::ID::StyleSheetList as u16,
    depth: 0,
    _bitfield_1: new_jsjitinfo_bitfield_1!(
        JSJitInfo_OpType::Getter as u8,
        JSJitInfo_AliasSet::AliasEverything as u8,
        JSValueType::JSVAL_TYPE_DOUBLE as u8,
        true,
        false,
        false,
        false,
        false,
        false,
        0,
    ),
};

unsafe extern fn _finalize<TH: TypeHolderTrait>
(_fop: *mut JSFreeOp, obj: *mut JSObject) {
    return wrap_panic(panic::AssertUnwindSafe(|| {

        let this = native_from_object::<StyleSheetList<TH>>(obj).unwrap();
            if !this.is_null() {
                // The pointer can be null if the object is the unforgeable holder of that interface.
                let _ = Box::from_raw(this as *mut StyleSheetList<TH>);
            }
            debug!("StyleSheetList<TH> finalize: {:p}", this);
    }), ());
}

unsafe extern fn _trace<TH: TypeHolderTrait>
(trc: *mut JSTracer, obj: *mut JSObject) {
    return wrap_panic(panic::AssertUnwindSafe(|| {

        let this = native_from_object::<StyleSheetList<TH>>(obj).unwrap();
        if this.is_null() { return; } // GC during obj creation
        (*this).trace(trc);
    }), ());
}

pub unsafe fn DefineProxyHandler<TH: TypeHolderTrait>
() -> *const libc::c_void {
    let traps = ProxyTraps {
        enter: None,
        getOwnPropertyDescriptor: Some(getOwnPropertyDescriptor),
        defineProperty: Some(proxyhandler::define_property),
        ownPropertyKeys: Some(own_property_keys),
        delete_: Some(proxyhandler::delete),
        enumerate: None,
        getPrototypeIfOrdinary: Some(proxyhandler::get_prototype_if_ordinary),
        preventExtensions: Some(proxyhandler::prevent_extensions),
        isExtensible: Some(proxyhandler::is_extensible),
        has: None,
        get: Some(get),
        set: None,
        call: None,
        construct: None,
        getPropertyDescriptor: Some(get_property_descriptor),
        hasOwn: Some(hasOwn),
        getOwnEnumerablePropertyKeys: Some(own_property_keys),
        nativeCall: None,
        hasInstance: None,
        objectClassIs: None,
        className: Some(className),
        fun_toString: None,
        boxedValue_unbox: None,
        defaultValue: None,
        trace: Some(_trace),
        finalize: Some(_finalize),
        objectMoved: None,
        isCallable: None,
        isConstructor: None,
    };

    CreateProxyHandler(&traps, Class.as_void_ptr())
}

#[inline] unsafe fn UnwrapProxy<TH: TypeHolderTrait>
(obj: RawHandleObject) -> *const StyleSheetList<TH> {
    /*if (xpc::WrapperFactory::IsXrayWrapper(obj)) {
        obj = js::UnwrapObject(obj);
    }*/
    //MOZ_ASSERT(IsProxy(obj));
    let box_ = GetProxyPrivate(obj.get()).to_private() as *const StyleSheetList<TH>;
    return box_;
}

static Class: DOMClass = DOMClass {
    interface_chain: [ PrototypeList::ID::StyleSheetList, PrototypeList::ID::Last, PrototypeList::ID::Last, PrototypeList::ID::Last, PrototypeList::ID::Last, PrototypeList::ID::Last ],
    type_id: ::dom::bindings::codegen::InheritTypes::TopTypeId { alone: () },
    global: InterfaceObjectMap::Globals::EMPTY,
};

unsafe extern fn own_property_keys<TH: TypeHolderTrait>
(cx: *mut JSContext, proxy: RawHandleObject, props: *mut AutoIdVector) -> bool {
    return wrap_panic(panic::AssertUnwindSafe(|| {
        let unwrapped_proxy = UnwrapProxy(proxy);
        for i in 0..(*unwrapped_proxy).Length() {
            rooted!(in(cx) let rooted_jsid = int_to_jsid(i as i32));
            AppendToAutoIdVector(props, rooted_jsid.handle().get());
        }
        rooted!(in(cx) let mut expando = ptr::null_mut::<JSObject>());
        get_expando_object(proxy, expando.handle_mut());
        if !expando.is_null() {
            GetPropertyKeys(cx, expando.handle(), JSITER_OWNONLY | JSITER_HIDDEN | JSITER_SYMBOLS, props);
        }

        return true;

    }), false);
}

unsafe extern fn getOwnPropertyDescriptor<TH: TypeHolderTrait>
(cx: *mut JSContext, proxy: RawHandleObject, id: RawHandleId, desc: RawMutableHandle<PropertyDescriptor>) -> bool {
    return wrap_panic(panic::AssertUnwindSafe(|| {
        let index = get_array_index_from_id(cx, Handle::from_raw(id));
        if let Some(index) = index {
            let this = UnwrapProxy(proxy);
            let this = &*this;
            let result: Option<DomRoot<StyleSheet<TH>>> = this.IndexedGetter(index);

            if let Some(result) = result {
                rooted!(in(cx) let mut result_root = UndefinedValue());
                (result).to_jsval(cx, result_root.handle_mut());
                desc.get().value = result_root.get();
                fill_property_descriptor(MutableHandle::from_raw(desc), proxy.get(), JSPROP_ENUMERATE | JSPROP_READONLY);
                return true;
            }
        }
        rooted!(in(cx) let mut expando = ptr::null_mut::<JSObject>());
        get_expando_object(proxy, expando.handle_mut());
        //if (!xpc::WrapperFactory::IsXrayWrapper(proxy) && (expando = GetExpandoObject(proxy))) {
        let proxy_lt = Handle::from_raw(proxy);
        let id_lt = Handle::from_raw(id);
        if !expando.is_null() {
            if !JS_GetPropertyDescriptorById(cx, expando.handle().into(), id, desc) {
                return false;
            }
            if !desc.obj.is_null() {
                // Pretend the property lives on the wrapper.
                desc.get().obj = proxy.get();
                return true;
            }
        }
        desc.get().obj = ptr::null_mut();
        return true;
    }), false);
}

unsafe extern fn className<TH: TypeHolderTrait>
(cx: *mut JSContext, _proxy: RawHandleObject) -> *const i8 {
    b"StyleSheetList\0" as *const u8 as *const i8
}

unsafe extern fn get<TH: TypeHolderTrait>
(cx: *mut JSContext, proxy: RawHandleObject, receiver: RawHandleValue, id: RawHandleId, vp: RawMutableHandleValue) -> bool {
    return wrap_panic(panic::AssertUnwindSafe(|| {
        //MOZ_ASSERT(!xpc::WrapperFactory::IsXrayWrapper(proxy),
        //"Should not have a XrayWrapper here");
        let proxy_lt = Handle::from_raw(proxy);
        let vp_lt = MutableHandle::from_raw(vp);
        let id_lt = Handle::from_raw(id);
        let receiver_lt = Handle::from_raw(receiver);

        let index = get_array_index_from_id(cx, id_lt);
        if let Some(index) = index {
            let this = UnwrapProxy(proxy);
            let this = &*this;
            let result: Option<DomRoot<StyleSheet<TH>>> = this.IndexedGetter(index);

            if let Some(result) = result {

                (result).to_jsval(cx, vp_lt);
                return true;
            }    // Even if we don't have this index, we don't forward the
            // get on to our expando object.
        } else {
            rooted!(in(cx) let mut expando = ptr::null_mut::<JSObject>());
            get_expando_object(proxy, expando.handle_mut());
            if !expando.is_null() {
                let mut hasProp = false;
                if !JS_HasPropertyById(cx, expando.handle().into(), id, &mut hasProp) {
                    return false;
                }

                if hasProp {
                    return JS_ForwardGetPropertyTo(cx, expando.handle().into(), id, receiver, vp);
                }
            }
        }

        let mut found = false;
        if !get_property_on_prototype(cx, proxy_lt, receiver_lt, id_lt, &mut found, vp_lt) {
            return false;
        }

        if found {
            return true;
        }

        vp.set(UndefinedValue());
        return true;
    }), false);
}

unsafe extern fn hasOwn<TH: TypeHolderTrait>
(cx: *mut JSContext, proxy: RawHandleObject, id: RawHandleId, bp: *mut bool) -> bool {
    return wrap_panic(panic::AssertUnwindSafe(|| {
        let index = get_array_index_from_id(cx, Handle::from_raw(id));
        if let Some(index) = index {
            let this = UnwrapProxy(proxy);
            let this = &*this;
            let result: Option<DomRoot<StyleSheet<TH>>> = this.IndexedGetter(index);

            *bp = result.is_some();
            return true;
        }

        rooted!(in(cx) let mut expando = ptr::null_mut::<JSObject>());
        let proxy_lt = Handle::from_raw(proxy);
        let id_lt = Handle::from_raw(id);
        get_expando_object(proxy, expando.handle_mut());
        if !expando.is_null() {
            let ok = JS_HasPropertyById(cx, expando.handle().into(), id, bp);
            if !ok || *bp {
                return ok;
            }
        }
        *bp = false;
        return true;
    }), false);
}

pub unsafe fn Wrap<TH: TypeHolderTrait>
(cx: *mut JSContext, scope: &GlobalScope<TH>, object: Box<StyleSheetList<TH>>) -> DomRoot<StyleSheetList<TH>> {
    let scope = scope.reflector().get_jsobject();
    assert!(!scope.get().is_null());
    assert!(((*get_object_class(scope.get())).flags & JSCLASS_IS_GLOBAL) != 0);

    rooted!(in(cx) let mut proto = ptr::null_mut::<JSObject>());
    let _ac = JSAutoCompartment::new(cx, scope.get());
    GetProtoObject(cx, scope, proto.handle_mut());
    assert!(!proto.is_null());

    let raw = Box::into_raw(object);
    let _rt = RootedTraceable::new(&*raw);

    let handler = RegisterBindings::PROXY_HANDLERS[PrototypeList::Proxies::StyleSheetList as usize];
    rooted!(in(cx) let private = PrivateValue(raw as *const libc::c_void));
    let obj = NewProxyObject(cx, handler,
                             private.handle(),
                             proto.get(), scope.get(),
                             ptr::null_mut(), ptr::null_mut());
    assert!(!obj.is_null());
    rooted!(in(cx) let obj = obj);


    (*raw).init_reflector(obj.get());

    DomRoot::from_ref(&*raw)
}

impl<TH: TypeHolderTrait> IDLInterface for StyleSheetList<TH> {
    #[inline]
    fn derives(class: &'static DOMClass) -> bool {
        class as *const _ == &Class as *const _
    }
}

impl<TH: TypeHolderTrait> PartialEq for StyleSheetList<TH> {
    fn eq(&self, other: &StyleSheetList<TH>) -> bool {
        self as *const StyleSheetList<TH> == &*other
    }
}

pub trait StyleSheetListMethods<TH: TypeHolderTrait> {
    fn Item(&self, index: u32) -> Option<DomRoot<StyleSheet<TH>>>;
    fn Length(&self) -> u32;
    fn IndexedGetter(&self, index: u32) -> Option<DomRoot<StyleSheet<TH>>>;
}
const sMethods_specs: &'static [&'static[JSFunctionSpec]] = &[
&[
    JSFunctionSpec {
        name: b"item\0" as *const u8 as *const libc::c_char,
        call: JSNativeWrapper { op: Some(generic_method), info: &item_methodinfo as *const _ as *const JSJitInfo },
        nargs: 1,
        flags: (JSPROP_ENUMERATE) as u16,
        selfHostedName: 0 as *const libc::c_char
    },
    JSFunctionSpec {
        name: (SymbolCode::iterator as i32 + 1) as *const u8 as *const libc::c_char,
        call: JSNativeWrapper { op: None, info: 0 as *const JSJitInfo },
        nargs: 0,
        flags: (JSPROP_ENUMERATE) as u16,
        selfHostedName: b"ArrayValues\0" as *const u8 as *const libc::c_char
    },
    JSFunctionSpec {
        name: 0 as *const libc::c_char,
        call: JSNativeWrapper { op: None, info: 0 as *const JSJitInfo },
        nargs: 0,
        flags: 0,
        selfHostedName: 0 as *const libc::c_char
    }]

];
const sMethods: &'static [Guard<&'static [JSFunctionSpec]>] = &[
    Guard::new(Condition::Satisfied, sMethods_specs[0])
];
const sAttributes_specs: &'static [&'static[JSPropertySpec]] = &[
&[
    JSPropertySpec {
        name: b"length\0" as *const u8 as *const libc::c_char,
        flags: (JSPROP_ENUMERATE | JSPROP_SHARED) as u8,
        getter: JSNativeWrapper { op: Some(generic_getter), info: &length_getterinfo },
        setter: JSNativeWrapper { op: None, info: 0 as *const JSJitInfo }
    },
    JSPropertySpec {
        name: 0 as *const libc::c_char,
        flags: 0,
        getter: JSNativeWrapper { op: None, info: 0 as *const JSJitInfo },
        setter: JSNativeWrapper { op: None, info: 0 as *const JSJitInfo }
    }]

];
const sAttributes: &'static [Guard<&'static [JSPropertySpec]>] = &[
    Guard::new(Condition::Satisfied, sAttributes_specs[0])
];

pub unsafe fn GetProtoObject<TH: TypeHolderTrait>
(cx: *mut JSContext, global: HandleObject, mut rval: MutableHandleObject) {
    /* Get the interface prototype object for this class.  This will create the
       object as needed. */
    assert!(((*get_object_class(global.get())).flags & JSCLASS_DOM_GLOBAL) != 0);

    /* Check to see whether the interface objects are already installed */
    let proto_or_iface_array = get_proto_or_iface_array(global.get());
    rval.set((*proto_or_iface_array)[PrototypeList::ID::StyleSheetList as usize]);
    if !rval.get().is_null() {
        return;
    }

    CreateInterfaceObjects(cx, global, proto_or_iface_array);
    rval.set((*proto_or_iface_array)[PrototypeList::ID::StyleSheetList as usize]);
    assert!(!rval.get().is_null());

}

static PrototypeClass: JSClass = JSClass {
    name: b"StyleSheetListPrototype\0" as *const u8 as *const libc::c_char,
    flags:
        // JSCLASS_HAS_RESERVED_SLOTS(0)
        (0 & JSCLASS_RESERVED_SLOTS_MASK) << JSCLASS_RESERVED_SLOTS_SHIFT,
    cOps: 0 as *const _,
    reserved: [0 as *mut os::raw::c_void; 3]
};

static INTERFACE_OBJECT_CLASS: NonCallbackInterfaceObjectClass =
    NonCallbackInterfaceObjectClass::new(
        &InterfaceConstructorBehavior::throw(),
        b"function StyleSheetList() {\n    [native code]\n}",
        PrototypeList::ID::StyleSheetList,
        0);

pub unsafe fn DefineDOMInterface<TH: TypeHolderTrait>
(cx: *mut JSContext, global: HandleObject) {
    assert!(!global.get().is_null());

    if !ConstructorEnabled(cx, global) {
        return;
    }

    rooted!(in(cx) let mut proto = ptr::null_mut::<JSObject>());
    GetProtoObject(cx, global, proto.handle_mut());
    assert!(!proto.is_null());
}

unsafe fn ConstructorEnabled<TH: TypeHolderTrait>
(aCx: *mut JSContext, aObj: HandleObject) -> bool {
    is_exposed_in(aObj, InterfaceObjectMap::Globals::WINDOW)
}

unsafe fn CreateInterfaceObjects<TH: TypeHolderTrait>
(cx: *mut JSContext, global: HandleObject, cache: *mut ProtoOrIfaceArray) {
    rooted!(in(cx) let mut prototype_proto = ptr::null_mut::<JSObject>());
    prototype_proto.set(JS_GetObjectPrototype(cx, global));
    assert!(!prototype_proto.is_null());

    rooted!(in(cx) let mut prototype = ptr::null_mut::<JSObject>());
    create_interface_prototype_object(cx,
                                      prototype_proto.handle().into(),
                                      &PrototypeClass,
                                      sMethods,
                                      sAttributes,
                                      &[],
                                      &[],
                                      prototype.handle_mut().into());
    assert!(!prototype.is_null());
    assert!((*cache)[PrototypeList::ID::StyleSheetList as usize].is_null());
    (*cache)[PrototypeList::ID::StyleSheetList as usize] = prototype.get();
    <*mut JSObject>::post_barrier((*cache).as_mut_ptr().offset(PrototypeList::ID::StyleSheetList as isize),
                                  ptr::null_mut(),
                                  prototype.get());


    rooted!(in(cx) let interface_proto = JS_GetFunctionPrototype(cx, global));
    assert!(!interface_proto.is_null());

    rooted!(in(cx) let mut interface = ptr::null_mut::<JSObject>());
    create_noncallback_interface_object(cx,
                                        global.into(),
                                        interface_proto.handle(),
                                        &INTERFACE_OBJECT_CLASS,
                                        &[],
                                        &[],
                                        &[],
                                        prototype.handle(),
                                        b"StyleSheetList\0",
                                        0,
                                        interface.handle_mut());
    assert!(!interface.is_null());
}
} // mod StyleSheetListBinding


