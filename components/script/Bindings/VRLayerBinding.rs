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
use dom::bindings::codegen::Bindings::HTMLElementBinding::HTMLElementBinding;
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
use dom::types::HTMLCanvasElement;
use dom::types::HTMLElement;
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
pub struct VRLayer<TH: TypeHolderTrait<TH> + 'static> {
    pub leftBounds: Option<Vec<Finite<f32>>>,
    pub rightBounds: Option<Vec<Finite<f32>>>,
    pub source: Option<DomRoot<HTMLCanvasElement<TH>>>,
}
impl<TH: TypeHolderTrait<TH>> VRLayer<TH> {
    pub unsafe fn empty(cx: *mut JSContext) -> VRLayer<TH> {
        match VRLayer::new(cx, HandleValue::null()) {
            Ok(ConversionResult::Success(v)) => v,
            _ => unreachable!(),
        }
    }
    pub unsafe fn new(cx: *mut JSContext, val: HandleValue)
                      -> Result<ConversionResult<VRLayer<TH>>, ()> {
        let object = if val.get().is_null_or_undefined() {
            ptr::null_mut()
        } else if val.get().is_object() {
            val.get().to_object()
        } else {
            return Ok(ConversionResult::Failure("Value is not an object.".into()));
        };
        rooted!(in(cx) let object = object);
            let dictionary = VRLayer {
            leftBounds: {
                rooted!(in(cx) let mut rval = UndefinedValue());
                match try!(get_dictionary_property(cx, object.handle(), "leftBounds", rval.handle_mut())) {
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
            rightBounds: {
                rooted!(in(cx) let mut rval = UndefinedValue());
                match try!(get_dictionary_property(cx, object.handle(), "rightBounds", rval.handle_mut())) {
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
            source: {
                rooted!(in(cx) let mut rval = UndefinedValue());
                match try!(get_dictionary_property(cx, object.handle(), "source", rval.handle_mut())) {
                    true => {
                        Some(if rval.handle().get().is_object() {
                            match root_from_handlevalue(rval.handle()) {
                                Ok(val) => val,
                                Err(()) => {
                                    throw_type_error(cx, "value does not implement interface HTMLCanvasElement.");
                                    return Err(());
                                }
                            }

                        } else {
                            throw_type_error(cx, "Value is not an object.");
                            return Err(());
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

impl<TH: TypeHolderTrait<TH>> FromJSValConvertible for VRLayer<TH> {
    type Config = ();
    unsafe fn from_jsval(cx: *mut JSContext, value: HandleValue, _option: ())
                         -> Result<ConversionResult<VRLayer<TH>>, ()> {
        VRLayer::new(cx, value)
    }
}

impl<TH: TypeHolderTrait<TH>> ToJSValConvertible for VRLayer<TH> {
    unsafe fn to_jsval(&self, cx: *mut JSContext, mut rval: MutableHandleValue) {
        rooted!(in(cx) let obj = JS_NewObject(cx, ptr::null()));
        if let Some(ref leftBounds) = self.leftBounds {
            rooted!(in(cx) let mut leftBounds_js = UndefinedValue());
            leftBounds.to_jsval(cx, leftBounds_js.handle_mut());
            set_dictionary_property(cx, obj.handle(), "leftBounds", leftBounds_js.handle()).unwrap();
        }
        if let Some(ref rightBounds) = self.rightBounds {
            rooted!(in(cx) let mut rightBounds_js = UndefinedValue());
            rightBounds.to_jsval(cx, rightBounds_js.handle_mut());
            set_dictionary_property(cx, obj.handle(), "rightBounds", rightBounds_js.handle()).unwrap();
        }
        if let Some(ref source) = self.source {
            rooted!(in(cx) let mut source_js = UndefinedValue());
            source.to_jsval(cx, source_js.handle_mut());
            set_dictionary_property(cx, obj.handle(), "source", source_js.handle()).unwrap();
        }
        rval.set(ObjectOrNullValue(obj.get()))
    }
}


