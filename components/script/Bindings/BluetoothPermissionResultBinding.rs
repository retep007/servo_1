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
use dom::bindings::codegen::Bindings::BluetoothBinding::BluetoothLEScanFilterInit;
use dom::bindings::codegen::Bindings::PermissionStatusBinding::PermissionName;
use dom::bindings::codegen::Bindings::PermissionStatusBinding::PermissionNameValues;
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

#[derive(JSTraceable)]
pub struct BluetoothPermissionDescriptor {
    pub parent: dom::bindings::codegen::Bindings::PermissionStatusBinding::PermissionDescriptor,
    pub acceptAllDevices: bool,
    pub deviceId: Option<DOMString>,
    pub filters: Option<Vec<dom::bindings::codegen::Bindings::BluetoothBinding::BluetoothLEScanFilterInit>>,
    pub optionalServices: Option<Vec<UnionTypes::StringOrUnsignedLong>>,
}
impl BluetoothPermissionDescriptor {
    pub unsafe fn empty(cx: *mut JSContext) -> BluetoothPermissionDescriptor {
        match BluetoothPermissionDescriptor::new(cx, HandleValue::null()) {
            Ok(ConversionResult::Success(v)) => v,
            _ => unreachable!(),
        }
    }
    pub unsafe fn new(cx: *mut JSContext, val: HandleValue)
                      -> Result<ConversionResult<BluetoothPermissionDescriptor>, ()> {
        let object = if val.get().is_null_or_undefined() {
            ptr::null_mut()
        } else if val.get().is_object() {
            val.get().to_object()
        } else {
            return Ok(ConversionResult::Failure("Value is not an object.".into()));
        };
        rooted!(in(cx) let object = object);
            let dictionary = BluetoothPermissionDescriptor {
            parent: {
                match try!(dom::bindings::codegen::Bindings::PermissionStatusBinding::PermissionDescriptor::new(cx, val)) {
                    ConversionResult::Success(v) => v,
                    ConversionResult::Failure(error) => {
                        throw_type_error(cx, &error);
                        return Err(());
                    }
                }
            },
            acceptAllDevices: {
                rooted!(in(cx) let mut rval = UndefinedValue());
                match try!(get_dictionary_property(cx, object.handle(), "acceptAllDevices", rval.handle_mut())) {
                    true => {
                        match FromJSValConvertible::from_jsval(cx, rval.handle(), ()) {
                            Ok(ConversionResult::Success(v)) => v,
                            Ok(ConversionResult::Failure(error)) => {
                                throw_type_error(cx, &error);
                                return Err(());
                            }
                            _ => { return Err(()); }
                        }
                    },
                    false => {
                        false
                    },
                }
            },
            deviceId: {
                rooted!(in(cx) let mut rval = UndefinedValue());
                match try!(get_dictionary_property(cx, object.handle(), "deviceId", rval.handle_mut())) {
                    true => {
                        Some(match FromJSValConvertible::from_jsval(cx, rval.handle(), StringificationBehavior::Default) {
                            Ok(ConversionResult::Success(strval)) => strval,
                            Ok(ConversionResult::Failure(error)) => {
                                throw_type_error(cx, &error);
                                return Err(());
                            }
                            _ => { return Err(()); },
                        })
                    },
                    false => {
                        None
                    },
                }
            },
            filters: {
                rooted!(in(cx) let mut rval = UndefinedValue());
                match try!(get_dictionary_property(cx, object.handle(), "filters", rval.handle_mut())) {
                    true => {
                        Some(match FromJSValConvertible::from_jsval(cx, rval.handle(), ()) {
                            Ok(ConversionResult::Success(value)) => value,
                            Ok(ConversionResult::Failure(error)) => {
                                throw_type_error(cx, &error);
                                return Err(());
                            }
                            _ => { return Err(()); },
                        })
                    },
                    false => {
                        None
                    },
                }
            },
            optionalServices: {
                rooted!(in(cx) let mut rval = UndefinedValue());
                match try!(get_dictionary_property(cx, object.handle(), "optionalServices", rval.handle_mut())) {
                    true => {
                        Some(match FromJSValConvertible::from_jsval(cx, rval.handle(), ()) {
                            Ok(ConversionResult::Success(value)) => value,
                            Ok(ConversionResult::Failure(error)) => {
                                throw_type_error(cx, &error);
                                return Err(());
                            }
                            _ => { return Err(()); },
                        })
                    },
                    false => {
                        None
                    },
                }
            },
            };
        Ok(ConversionResult::Success(dictionary))
    }
}

impl FromJSValConvertible for BluetoothPermissionDescriptor {
    type Config = ();
    unsafe fn from_jsval(cx: *mut JSContext, value: HandleValue, _option: ())
                         -> Result<ConversionResult<BluetoothPermissionDescriptor>, ()> {
        BluetoothPermissionDescriptor::new(cx, value)
    }
}

impl ToJSValConvertible for BluetoothPermissionDescriptor {
    unsafe fn to_jsval(&self, cx: *mut JSContext, mut rval: MutableHandleValue) {
        rooted!(in(cx) let obj = JS_NewObject(cx, ptr::null()));
        let acceptAllDevices = &self.acceptAllDevices;
        rooted!(in(cx) let mut acceptAllDevices_js = UndefinedValue());
        acceptAllDevices.to_jsval(cx, acceptAllDevices_js.handle_mut());
        set_dictionary_property(cx, obj.handle(), "acceptAllDevices", acceptAllDevices_js.handle()).unwrap();
        if let Some(ref deviceId) = self.deviceId {
            rooted!(in(cx) let mut deviceId_js = UndefinedValue());
            deviceId.to_jsval(cx, deviceId_js.handle_mut());
            set_dictionary_property(cx, obj.handle(), "deviceId", deviceId_js.handle()).unwrap();
        }
        if let Some(ref filters) = self.filters {
            rooted!(in(cx) let mut filters_js = UndefinedValue());
            filters.to_jsval(cx, filters_js.handle_mut());
            set_dictionary_property(cx, obj.handle(), "filters", filters_js.handle()).unwrap();
        }
        if let Some(ref optionalServices) = self.optionalServices {
            rooted!(in(cx) let mut optionalServices_js = UndefinedValue());
            optionalServices.to_jsval(cx, optionalServices_js.handle_mut());
            set_dictionary_property(cx, obj.handle(), "optionalServices", optionalServices_js.handle()).unwrap();
        }
        rval.set(ObjectOrNullValue(obj.get()))
    }
}


pub use self::BluetoothPermissionResultBinding::{Wrap, BluetoothPermissionResultMethods, GetProtoObject, DefineDOMInterface};
pub mod BluetoothPermissionResultBinding {
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
use dom::bindings::codegen::Bindings::EventTargetBinding::EventTargetBinding;
use dom::bindings::codegen::Bindings::PermissionStatusBinding::PermissionStatusBinding;
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
use dom::types::BluetoothDevice;
use dom::types::BluetoothPermissionResult;
use dom::types::EventTarget;
use dom::types::PermissionStatus;
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

unsafe extern fn devices<TH: TypeHolderTrait<TH>>
(cx: *mut JSContext, _obj: HandleObject, this: *const BluetoothPermissionResult<TH>, args: *const JSJitMethodCallArgs) -> bool {
    return wrap_panic(panic::AssertUnwindSafe(|| {
        let this = &*this;
        let args = &*args;
        let argc = args._base.argc_;
        let result: Vec<DomRoot<BluetoothDevice<TH>>> = this.Devices();

        (result).to_jsval(cx, args.rval());
        return true;
    }), false);
}


fn devices_methodinfo<TH: TypeHolderTrait<TH>>() -> JSJitInfo { JSJitInfo {
    call: devices::<TH> as *const os::raw::c_void,
    protoID: PrototypeList::ID::BluetoothPermissionResult as u16,
    depth: 2,
    _bitfield_1: new_jsjitinfo_bitfield_1!(
        JSJitInfo_OpType::Method as u8,
        JSJitInfo_AliasSet::AliasEverything as u8,
        JSValueType::JSVAL_TYPE_OBJECT as u8,
        true,
        false,
        false,
        false,
        false,
        false,
        0,
    ),
}}

unsafe extern fn _finalize<TH: TypeHolderTrait<TH>>
(_fop: *mut JSFreeOp, obj: *mut JSObject) {
    return wrap_panic(panic::AssertUnwindSafe(|| {

        let this = native_from_object::<BluetoothPermissionResult<TH>>(obj).unwrap();
            if !this.is_null() {
                // The pointer can be null if the object is the unforgeable holder of that interface.
                let _ = Box::from_raw(this as *mut BluetoothPermissionResult<TH>);
            }
            debug!("BluetoothPermissionResult<TH> finalize: {:p}", this);
    }), ());
}

unsafe extern fn _trace<TH: TypeHolderTrait<TH>>
(trc: *mut JSTracer, obj: *mut JSObject) {
    return wrap_panic(panic::AssertUnwindSafe(|| {

        let this = native_from_object::<BluetoothPermissionResult<TH>>(obj).unwrap();
        if this.is_null() { return; } // GC during obj creation
        (*this).trace(trc);
    }), ());
}

fn CLASS_OPS<TH: TypeHolderTrait<TH>>() -> js::jsapi::JSClassOps { js::jsapi::JSClassOps {
    addProperty: None,
    delProperty: None,
    getProperty: None,
    setProperty: None,
    enumerate: None,
    resolve: None,
    mayResolve: None,
    finalize: Some(_finalize::<TH>),
    call: None,
    hasInstance: None,
    construct: None,
    trace: Some(_trace::<TH>),
}}

fn Class<TH: TypeHolderTrait<TH>>() -> DOMJSClass { DOMJSClass {
    base: js::jsapi::JSClass {
        name: b"BluetoothPermissionResult\0" as *const u8 as *const libc::c_char,
        flags: JSCLASS_IS_DOMJSCLASS | 0 |
               (((1) & JSCLASS_RESERVED_SLOTS_MASK) << JSCLASS_RESERVED_SLOTS_SHIFT)
               /* JSCLASS_HAS_RESERVED_SLOTS(1) */,
        cOps: &CLASS_OPS::<TH>(),
        reserved: [0 as *mut _; 3],
    },
    dom_class: DOMClass {
    interface_chain: [ PrototypeList::ID::EventTarget, PrototypeList::ID::PermissionStatus, PrototypeList::ID::BluetoothPermissionResult, PrototypeList::ID::Last, PrototypeList::ID::Last, PrototypeList::ID::Last ],
    type_id: ::dom::bindings::codegen::InheritTypes::TopTypeId { eventtarget: (::dom::bindings::codegen::InheritTypes::EventTargetTypeId::PermissionStatus(::dom::bindings::codegen::InheritTypes::PermissionStatusTypeId::BluetoothPermissionResult)) },
    global: InterfaceObjectMap::Globals::EMPTY,
    malloc_size_of: malloc_size_of_including_raw_self::<BluetoothPermissionResult<TH>> as unsafe fn(&mut _, _) -> _,
}
}}

#[inline]
fn malloc_size<TH: TypeHolderTrait<TH>>(ops: &mut MallocSizeOfOps, obj: *const c_void) -> usize {
    malloc_size_of_including_raw_self::<BluetoothPermissionResult<TH>>(ops, obj)
}

pub unsafe fn Wrap<TH: TypeHolderTrait<TH>>
(cx: *mut JSContext, scope: &GlobalScope<TH>, object: Box<BluetoothPermissionResult<TH>>) -> DomRoot<BluetoothPermissionResult<TH>> {
    let scope = scope.reflector().get_jsobject();
    assert!(!scope.get().is_null());
    assert!(((*get_object_class(scope.get())).flags & JSCLASS_IS_GLOBAL) != 0);

    rooted!(in(cx) let mut proto = ptr::null_mut::<JSObject>());
    let _ac = JSAutoCompartment::new(cx, scope.get());
    GetProtoObject(cx, scope, proto.handle_mut());
    assert!(!proto.is_null());

    let raw = Box::into_raw(object);
    let _rt = RootedTraceable::new(&*raw);
    rooted!(in(cx) let obj = JS_NewObjectWithGivenProto(
        cx, &Class::<TH>().base as *const JSClass, proto.handle()));
    assert!(!obj.is_null());

    JS_SetReservedSlot(obj.get(), DOM_OBJECT_SLOT,
                       PrivateValue(raw as *const libc::c_void));


    (*raw).init_reflector(obj.get());

    DomRoot::from_ref(&*raw)
}

impl<TH: TypeHolderTrait<TH>> IDLInterface for BluetoothPermissionResult<TH> {
    #[inline]
    fn derives(class: &'static DOMClass) -> bool {
        class as *const _ == &Class::<TH>().dom_class as *const _
    }
}

impl<TH: TypeHolderTrait<TH>> PartialEq for BluetoothPermissionResult<TH> {
    fn eq(&self, other: &BluetoothPermissionResult<TH>) -> bool {
        self as *const BluetoothPermissionResult<TH> == &*other
    }
}

pub trait BluetoothPermissionResultMethods<TH: TypeHolderTrait<TH>> {
    fn Devices(&self) -> Vec<DomRoot<BluetoothDevice<TH>>>;
}
fn sMethods_specs<TH: TypeHolderTrait<TH>>() -> &'static [&'static[JSFunctionSpec]] { &[
&[
    JSFunctionSpec {
        name: b"devices\0" as *const u8 as *const libc::c_char,
        call: JSNativeWrapper { op: Some(generic_method), info: &devices_methodinfo::<TH>() as *const _ as *const JSJitInfo },
        nargs: 0,
        flags: (JSPROP_ENUMERATE) as u16,
        selfHostedName: 0 as *const libc::c_char
    },
    JSFunctionSpec {
        name: 0 as *const libc::c_char,
        call: JSNativeWrapper { op: None, info: 0 as *const JSJitInfo },
        nargs: 0,
        flags: 0,
        selfHostedName: 0 as *const libc::c_char
    }]

]}
fn sMethods<TH: TypeHolderTrait<TH>>() -> &'static [Guard<&'static [JSFunctionSpec]>] { &[
    Guard::new(Condition::Satisfied, sMethods_specs::<TH>()[0])
]}

pub unsafe fn GetProtoObject<TH: TypeHolderTrait<TH>>
(cx: *mut JSContext, global: HandleObject, mut rval: MutableHandleObject) {
    /* Get the interface prototype object for this class.  This will create the
       object as needed. */
    assert!(((*get_object_class(global.get())).flags & JSCLASS_DOM_GLOBAL) != 0);

    /* Check to see whether the interface objects are already installed */
    let proto_or_iface_array = get_proto_or_iface_array(global.get());
    rval.set((*proto_or_iface_array)[PrototypeList::ID::BluetoothPermissionResult as usize]);
    if !rval.get().is_null() {
        return;
    }

    CreateInterfaceObjects(cx, global, proto_or_iface_array);
    rval.set((*proto_or_iface_array)[PrototypeList::ID::BluetoothPermissionResult as usize]);
    assert!(!rval.get().is_null());

}

static PrototypeClass: JSClass = JSClass {
    name: b"BluetoothPermissionResultPrototype\0" as *const u8 as *const libc::c_char,
    flags:
        // JSCLASS_HAS_RESERVED_SLOTS(0)
        (0 & JSCLASS_RESERVED_SLOTS_MASK) << JSCLASS_RESERVED_SLOTS_SHIFT,
    cOps: 0 as *const _,
    reserved: [0 as *mut os::raw::c_void; 3]
};

static INTERFACE_OBJECT_CLASS: NonCallbackInterfaceObjectClass =
    NonCallbackInterfaceObjectClass::new(
        &InterfaceConstructorBehavior::throw(),
        b"function BluetoothPermissionResult() {\n    [native code]\n}",
        PrototypeList::ID::BluetoothPermissionResult,
        2);

pub unsafe fn DefineDOMInterface<TH: TypeHolderTrait<TH>>
(cx: *mut JSContext, global: HandleObject) {
    assert!(!global.get().is_null());

    if !ConstructorEnabled(cx, global) {
        return;
    }

    rooted!(in(cx) let mut proto = ptr::null_mut::<JSObject>());
    GetProtoObject(cx, global, proto.handle_mut());
    assert!(!proto.is_null());
}

unsafe fn ConstructorEnabled<TH: TypeHolderTrait<TH>>
(aCx: *mut JSContext, aObj: HandleObject) -> bool {
    is_exposed_in(aObj, InterfaceObjectMap::Globals::WINDOW) &&
    PREFS.get("dom.bluetooth.enabled").as_boolean().unwrap_or(false)
}

unsafe fn CreateInterfaceObjects<TH: TypeHolderTrait<TH>>
(cx: *mut JSContext, global: HandleObject, cache: *mut ProtoOrIfaceArray) {
    rooted!(in(cx) let mut prototype_proto = ptr::null_mut::<JSObject>());
    PermissionStatusBinding::GetProtoObject(cx, global, prototype_proto.handle_mut());
    assert!(!prototype_proto.is_null());

    rooted!(in(cx) let mut prototype = ptr::null_mut::<JSObject>());
    create_interface_prototype_object(cx,
                                      prototype_proto.handle().into(),
                                      &PrototypeClass,
                                      sMethods::<TH>(),
                                      &[],
                                      &[],
                                      &[],
                                      prototype.handle_mut().into());
    assert!(!prototype.is_null());
    assert!((*cache)[PrototypeList::ID::BluetoothPermissionResult as usize].is_null());
    (*cache)[PrototypeList::ID::BluetoothPermissionResult as usize] = prototype.get();
    <*mut JSObject>::post_barrier((*cache).as_mut_ptr().offset(PrototypeList::ID::BluetoothPermissionResult as isize),
                                  ptr::null_mut(),
                                  prototype.get());


    rooted!(in(cx) let mut interface_proto = ptr::null_mut::<JSObject>());
    PermissionStatusBinding::GetConstructorObject(cx, global, interface_proto.handle_mut());
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
                                        b"BluetoothPermissionResult\0",
                                        0,
                                        interface.handle_mut());
    assert!(!interface.is_null());
}
} // mod BluetoothPermissionResultBinding


