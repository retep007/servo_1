use dom::bindings::codegen::Bindings;
use dom::bindings::codegen::PrototypeList::Proxies;
use libc;

pub static mut PROXY_HANDLERS: [*const libc::c_void; 26] = [0 as *const libc::c_void; 26];
/// Create the global vtables used by the generated DOM bindings to implement JS proxies.
pub unsafe fn RegisterProxyHandlers() {
    PROXY_HANDLERS[Proxies::CSSRuleList as usize] = Bindings::CSSRuleListBinding::CSSRuleListBinding::DefineProxyHandler();
    PROXY_HANDLERS[Proxies::CSSStyleDeclaration as usize] = Bindings::CSSStyleDeclarationBinding::CSSStyleDeclarationBinding::DefineProxyHandler();
    PROXY_HANDLERS[Proxies::DOMStringMap as usize] = Bindings::DOMStringMapBinding::DOMStringMapBinding::DefineProxyHandler();
    PROXY_HANDLERS[Proxies::DOMTokenList as usize] = Bindings::DOMTokenListBinding::DOMTokenListBinding::DefineProxyHandler();
    PROXY_HANDLERS[Proxies::Document as usize] = Bindings::DocumentBinding::DocumentBinding::DefineProxyHandler();
    PROXY_HANDLERS[Proxies::FileList as usize] = Bindings::FileListBinding::FileListBinding::DefineProxyHandler();
    PROXY_HANDLERS[Proxies::GamepadButtonList as usize] = Bindings::GamepadButtonListBinding::GamepadButtonListBinding::DefineProxyHandler();
    PROXY_HANDLERS[Proxies::GamepadList as usize] = Bindings::GamepadListBinding::GamepadListBinding::DefineProxyHandler();
    PROXY_HANDLERS[Proxies::HTMLCollection as usize] = Bindings::HTMLCollectionBinding::HTMLCollectionBinding::DefineProxyHandler();
    PROXY_HANDLERS[Proxies::HTMLFormControlsCollection as usize] = Bindings::HTMLFormControlsCollectionBinding::HTMLFormControlsCollectionBinding::DefineProxyHandler();
    PROXY_HANDLERS[Proxies::HTMLFormElement as usize] = Bindings::HTMLFormElementBinding::HTMLFormElementBinding::DefineProxyHandler();
    PROXY_HANDLERS[Proxies::HTMLOptionsCollection as usize] = Bindings::HTMLOptionsCollectionBinding::HTMLOptionsCollectionBinding::DefineProxyHandler();
    PROXY_HANDLERS[Proxies::HTMLSelectElement as usize] = Bindings::HTMLSelectElementBinding::HTMLSelectElementBinding::DefineProxyHandler();
    PROXY_HANDLERS[Proxies::MediaList as usize] = Bindings::MediaListBinding::MediaListBinding::DefineProxyHandler();
    PROXY_HANDLERS[Proxies::MimeTypeArray as usize] = Bindings::MimeTypeArrayBinding::MimeTypeArrayBinding::DefineProxyHandler();
    PROXY_HANDLERS[Proxies::NamedNodeMap as usize] = Bindings::NamedNodeMapBinding::NamedNodeMapBinding::DefineProxyHandler();
    PROXY_HANDLERS[Proxies::NodeList as usize] = Bindings::NodeListBinding::NodeListBinding::DefineProxyHandler();
    PROXY_HANDLERS[Proxies::Plugin as usize] = Bindings::PluginBinding::PluginBinding::DefineProxyHandler();
    PROXY_HANDLERS[Proxies::PluginArray as usize] = Bindings::PluginArrayBinding::PluginArrayBinding::DefineProxyHandler();
    PROXY_HANDLERS[Proxies::RadioNodeList as usize] = Bindings::RadioNodeListBinding::RadioNodeListBinding::DefineProxyHandler();
    PROXY_HANDLERS[Proxies::Storage as usize] = Bindings::StorageBinding::StorageBinding::DefineProxyHandler();
    PROXY_HANDLERS[Proxies::StyleSheetList as usize] = Bindings::StyleSheetListBinding::StyleSheetListBinding::DefineProxyHandler();
    PROXY_HANDLERS[Proxies::TestBindingIterable as usize] = Bindings::TestBindingIterableBinding::TestBindingIterableBinding::DefineProxyHandler();
    PROXY_HANDLERS[Proxies::TestBindingProxy as usize] = Bindings::TestBindingProxyBinding::TestBindingProxyBinding::DefineProxyHandler();
    PROXY_HANDLERS[Proxies::TouchList as usize] = Bindings::TouchListBinding::TouchListBinding::DefineProxyHandler();
    PROXY_HANDLERS[Proxies::XMLDocument as usize] = Bindings::XMLDocumentBinding::XMLDocumentBinding::DefineProxyHandler();
}
