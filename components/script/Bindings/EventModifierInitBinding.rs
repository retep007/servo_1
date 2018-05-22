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
use dom::bindings::codegen::Bindings::EventTargetBinding::EventTargetBinding;
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
use dom::types::EventTarget;
use dom::types::Window;
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
pub struct EventModifierInit {
    pub parent: dom::bindings::codegen::Bindings::UIEventBinding::UIEventInit,
    pub altKey: bool,
    pub ctrlKey: bool,
    pub keyModifierStateAltGraph: bool,
    pub keyModifierStateCapsLock: bool,
    pub keyModifierStateFn: bool,
    pub keyModifierStateFnLock: bool,
    pub keyModifierStateHyper: bool,
    pub keyModifierStateNumLock: bool,
    pub keyModifierStateOS: bool,
    pub keyModifierStateScrollLock: bool,
    pub keyModifierStateSuper: bool,
    pub keyModifierStateSymbol: bool,
    pub keyModifierStateSymbolLock: bool,
    pub metaKey: bool,
    pub shiftKey: bool,
}
impl EventModifierInit {
    pub unsafe fn empty(cx: *mut JSContext) -> EventModifierInit {
        match EventModifierInit::new(cx, HandleValue::null()) {
            Ok(ConversionResult::Success(v)) => v,
            _ => unreachable!(),
        }
    }
    pub unsafe fn new(cx: *mut JSContext, val: HandleValue)
                      -> Result<ConversionResult<EventModifierInit>, ()> {
        let object = if val.get().is_null_or_undefined() {
            ptr::null_mut()
        } else if val.get().is_object() {
            val.get().to_object()
        } else {
            return Ok(ConversionResult::Failure("Value is not an object.".into()));
        };
        rooted!(in(cx) let object = object);
            let dictionary = EventModifierInit {
            parent: {
                match try!(dom::bindings::codegen::Bindings::UIEventBinding::UIEventInit::new(cx, val)) {
                    ConversionResult::Success(v) => v,
                    ConversionResult::Failure(error) => {
                        throw_type_error(cx, &error);
                        return Err(());
                    }
                }
            },
            altKey: {
                rooted!(in(cx) let mut rval = UndefinedValue());
                match try!(get_dictionary_property(cx, object.handle(), "altKey", rval.handle_mut())) {
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
            ctrlKey: {
                rooted!(in(cx) let mut rval = UndefinedValue());
                match try!(get_dictionary_property(cx, object.handle(), "ctrlKey", rval.handle_mut())) {
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
            keyModifierStateAltGraph: {
                rooted!(in(cx) let mut rval = UndefinedValue());
                match try!(get_dictionary_property(cx, object.handle(), "keyModifierStateAltGraph", rval.handle_mut())) {
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
            keyModifierStateCapsLock: {
                rooted!(in(cx) let mut rval = UndefinedValue());
                match try!(get_dictionary_property(cx, object.handle(), "keyModifierStateCapsLock", rval.handle_mut())) {
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
            keyModifierStateFn: {
                rooted!(in(cx) let mut rval = UndefinedValue());
                match try!(get_dictionary_property(cx, object.handle(), "keyModifierStateFn", rval.handle_mut())) {
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
            keyModifierStateFnLock: {
                rooted!(in(cx) let mut rval = UndefinedValue());
                match try!(get_dictionary_property(cx, object.handle(), "keyModifierStateFnLock", rval.handle_mut())) {
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
            keyModifierStateHyper: {
                rooted!(in(cx) let mut rval = UndefinedValue());
                match try!(get_dictionary_property(cx, object.handle(), "keyModifierStateHyper", rval.handle_mut())) {
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
            keyModifierStateNumLock: {
                rooted!(in(cx) let mut rval = UndefinedValue());
                match try!(get_dictionary_property(cx, object.handle(), "keyModifierStateNumLock", rval.handle_mut())) {
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
            keyModifierStateOS: {
                rooted!(in(cx) let mut rval = UndefinedValue());
                match try!(get_dictionary_property(cx, object.handle(), "keyModifierStateOS", rval.handle_mut())) {
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
            keyModifierStateScrollLock: {
                rooted!(in(cx) let mut rval = UndefinedValue());
                match try!(get_dictionary_property(cx, object.handle(), "keyModifierStateScrollLock", rval.handle_mut())) {
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
            keyModifierStateSuper: {
                rooted!(in(cx) let mut rval = UndefinedValue());
                match try!(get_dictionary_property(cx, object.handle(), "keyModifierStateSuper", rval.handle_mut())) {
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
            keyModifierStateSymbol: {
                rooted!(in(cx) let mut rval = UndefinedValue());
                match try!(get_dictionary_property(cx, object.handle(), "keyModifierStateSymbol", rval.handle_mut())) {
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
            keyModifierStateSymbolLock: {
                rooted!(in(cx) let mut rval = UndefinedValue());
                match try!(get_dictionary_property(cx, object.handle(), "keyModifierStateSymbolLock", rval.handle_mut())) {
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
            metaKey: {
                rooted!(in(cx) let mut rval = UndefinedValue());
                match try!(get_dictionary_property(cx, object.handle(), "metaKey", rval.handle_mut())) {
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
            shiftKey: {
                rooted!(in(cx) let mut rval = UndefinedValue());
                match try!(get_dictionary_property(cx, object.handle(), "shiftKey", rval.handle_mut())) {
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
            };
        Ok(ConversionResult::Success(dictionary))
    }
}

impl FromJSValConvertible for EventModifierInit {
    type Config = ();
    unsafe fn from_jsval(cx: *mut JSContext, value: HandleValue, _option: ())
                         -> Result<ConversionResult<EventModifierInit>, ()> {
        EventModifierInit::new(cx, value)
    }
}

impl ToJSValConvertible for EventModifierInit {
    unsafe fn to_jsval(&self, cx: *mut JSContext, mut rval: MutableHandleValue) {
        rooted!(in(cx) let obj = JS_NewObject(cx, ptr::null()));
        let altKey = &self.altKey;
        rooted!(in(cx) let mut altKey_js = UndefinedValue());
        altKey.to_jsval(cx, altKey_js.handle_mut());
        set_dictionary_property(cx, obj.handle(), "altKey", altKey_js.handle()).unwrap();
        let ctrlKey = &self.ctrlKey;
        rooted!(in(cx) let mut ctrlKey_js = UndefinedValue());
        ctrlKey.to_jsval(cx, ctrlKey_js.handle_mut());
        set_dictionary_property(cx, obj.handle(), "ctrlKey", ctrlKey_js.handle()).unwrap();
        let keyModifierStateAltGraph = &self.keyModifierStateAltGraph;
        rooted!(in(cx) let mut keyModifierStateAltGraph_js = UndefinedValue());
        keyModifierStateAltGraph.to_jsval(cx, keyModifierStateAltGraph_js.handle_mut());
        set_dictionary_property(cx, obj.handle(), "keyModifierStateAltGraph", keyModifierStateAltGraph_js.handle()).unwrap();
        let keyModifierStateCapsLock = &self.keyModifierStateCapsLock;
        rooted!(in(cx) let mut keyModifierStateCapsLock_js = UndefinedValue());
        keyModifierStateCapsLock.to_jsval(cx, keyModifierStateCapsLock_js.handle_mut());
        set_dictionary_property(cx, obj.handle(), "keyModifierStateCapsLock", keyModifierStateCapsLock_js.handle()).unwrap();
        let keyModifierStateFn = &self.keyModifierStateFn;
        rooted!(in(cx) let mut keyModifierStateFn_js = UndefinedValue());
        keyModifierStateFn.to_jsval(cx, keyModifierStateFn_js.handle_mut());
        set_dictionary_property(cx, obj.handle(), "keyModifierStateFn", keyModifierStateFn_js.handle()).unwrap();
        let keyModifierStateFnLock = &self.keyModifierStateFnLock;
        rooted!(in(cx) let mut keyModifierStateFnLock_js = UndefinedValue());
        keyModifierStateFnLock.to_jsval(cx, keyModifierStateFnLock_js.handle_mut());
        set_dictionary_property(cx, obj.handle(), "keyModifierStateFnLock", keyModifierStateFnLock_js.handle()).unwrap();
        let keyModifierStateHyper = &self.keyModifierStateHyper;
        rooted!(in(cx) let mut keyModifierStateHyper_js = UndefinedValue());
        keyModifierStateHyper.to_jsval(cx, keyModifierStateHyper_js.handle_mut());
        set_dictionary_property(cx, obj.handle(), "keyModifierStateHyper", keyModifierStateHyper_js.handle()).unwrap();
        let keyModifierStateNumLock = &self.keyModifierStateNumLock;
        rooted!(in(cx) let mut keyModifierStateNumLock_js = UndefinedValue());
        keyModifierStateNumLock.to_jsval(cx, keyModifierStateNumLock_js.handle_mut());
        set_dictionary_property(cx, obj.handle(), "keyModifierStateNumLock", keyModifierStateNumLock_js.handle()).unwrap();
        let keyModifierStateOS = &self.keyModifierStateOS;
        rooted!(in(cx) let mut keyModifierStateOS_js = UndefinedValue());
        keyModifierStateOS.to_jsval(cx, keyModifierStateOS_js.handle_mut());
        set_dictionary_property(cx, obj.handle(), "keyModifierStateOS", keyModifierStateOS_js.handle()).unwrap();
        let keyModifierStateScrollLock = &self.keyModifierStateScrollLock;
        rooted!(in(cx) let mut keyModifierStateScrollLock_js = UndefinedValue());
        keyModifierStateScrollLock.to_jsval(cx, keyModifierStateScrollLock_js.handle_mut());
        set_dictionary_property(cx, obj.handle(), "keyModifierStateScrollLock", keyModifierStateScrollLock_js.handle()).unwrap();
        let keyModifierStateSuper = &self.keyModifierStateSuper;
        rooted!(in(cx) let mut keyModifierStateSuper_js = UndefinedValue());
        keyModifierStateSuper.to_jsval(cx, keyModifierStateSuper_js.handle_mut());
        set_dictionary_property(cx, obj.handle(), "keyModifierStateSuper", keyModifierStateSuper_js.handle()).unwrap();
        let keyModifierStateSymbol = &self.keyModifierStateSymbol;
        rooted!(in(cx) let mut keyModifierStateSymbol_js = UndefinedValue());
        keyModifierStateSymbol.to_jsval(cx, keyModifierStateSymbol_js.handle_mut());
        set_dictionary_property(cx, obj.handle(), "keyModifierStateSymbol", keyModifierStateSymbol_js.handle()).unwrap();
        let keyModifierStateSymbolLock = &self.keyModifierStateSymbolLock;
        rooted!(in(cx) let mut keyModifierStateSymbolLock_js = UndefinedValue());
        keyModifierStateSymbolLock.to_jsval(cx, keyModifierStateSymbolLock_js.handle_mut());
        set_dictionary_property(cx, obj.handle(), "keyModifierStateSymbolLock", keyModifierStateSymbolLock_js.handle()).unwrap();
        let metaKey = &self.metaKey;
        rooted!(in(cx) let mut metaKey_js = UndefinedValue());
        metaKey.to_jsval(cx, metaKey_js.handle_mut());
        set_dictionary_property(cx, obj.handle(), "metaKey", metaKey_js.handle()).unwrap();
        let shiftKey = &self.shiftKey;
        rooted!(in(cx) let mut shiftKey_js = UndefinedValue());
        shiftKey.to_jsval(cx, shiftKey_js.handle_mut());
        set_dictionary_property(cx, obj.handle(), "shiftKey", shiftKey_js.handle()).unwrap();
        rval.set(ObjectOrNullValue(obj.get()))
    }
}


