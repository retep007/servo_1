/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::ServiceWorkerBinding::ServiceWorkerState;
use dom::bindings::codegen::Bindings::ServiceWorkerRegistrationBinding::{ServiceWorkerRegistrationMethods, Wrap};
use dom::bindings::reflector::reflect_dom_object;
use dom::bindings::root::{Dom, DomRoot};
use dom::bindings::str::USVString;
use dom::eventtarget::EventTarget;
use dom::globalscope::GlobalScope;
use dom::serviceworker::ServiceWorker;
use dom::workerglobalscope::prepare_workerscope_init;
use dom_struct::dom_struct;
use script_traits::{WorkerScriptLoadOrigin, ScopeThings};
use servo_url::ServoUrl;
use std::cell::Cell;
use typeholder::TypeHolderTrait;

#[dom_struct]
pub struct ServiceWorkerRegistration<TH: TypeHolderTrait<TH> + 'static> {
    eventtarget: EventTarget<TH>,
    active: Option<Dom<ServiceWorker<TH>>>,
    installing: Option<Dom<ServiceWorker<TH>>>,
    waiting: Option<Dom<ServiceWorker<TH>>>,
    scope: ServoUrl,
    uninstalling: Cell<bool>
}

impl<TH: TypeHolderTrait<TH>> ServiceWorkerRegistration<TH> {
    fn new_inherited(active_sw: &ServiceWorker<TH>, scope: ServoUrl) -> ServiceWorkerRegistration<TH> {
        ServiceWorkerRegistration {
            eventtarget: EventTarget::new_inherited(),
            active: Some(Dom::from_ref(active_sw)),
            installing: None,
            waiting: None,
            scope: scope,
            uninstalling: Cell::new(false)
        }
    }
    #[allow(unrooted_must_root)]
    pub fn new(global: &GlobalScope<TH>,
               script_url: &ServoUrl,
               scope: ServoUrl) -> DomRoot<ServiceWorkerRegistration<TH>> {
        let active_worker = ServiceWorker::install_serviceworker(global, script_url.clone(), scope.clone(), true);
        active_worker.set_transition_state(ServiceWorkerState::Installed);
        reflect_dom_object(Box::new(ServiceWorkerRegistration::new_inherited(&*active_worker, scope)), global, Wrap)
    }

    pub fn get_installed(&self) -> &ServiceWorker<TH> {
        self.active.as_ref().unwrap()
    }

    pub fn get_uninstalling(&self) -> bool {
        self.uninstalling.get()
    }

    pub fn set_uninstalling(&self, flag: bool) {
        self.uninstalling.set(flag)
    }

    pub fn create_scope_things(global: &GlobalScope<TH>, script_url: ServoUrl) -> ScopeThings {
        let worker_load_origin = WorkerScriptLoadOrigin {
            referrer_url: None,
            referrer_policy: None,
            pipeline_id: Some(global.pipeline_id())
        };

        let worker_id = global.get_next_worker_id();
        let devtools_chan = global.devtools_chan().cloned();
        let init = prepare_workerscope_init(&global, None);
        ScopeThings {
            script_url: script_url,
            init: init,
            worker_load_origin: worker_load_origin,
            devtools_chan: devtools_chan,
            worker_id: worker_id
        }
    }

    // https://w3c.github.io/ServiceWorker/#get-newest-worker-algorithm
    pub fn get_newest_worker(&self) -> Option<DomRoot<ServiceWorker<TH>>> {
        if self.installing.as_ref().is_some() {
            self.installing.as_ref().map(|sw| DomRoot::from_ref(&**sw))
        } else if self.waiting.as_ref().is_some() {
            self.waiting.as_ref().map(|sw| DomRoot::from_ref(&**sw))
        } else {
            self.active.as_ref().map(|sw| DomRoot::from_ref(&**sw))
        }
    }
}

pub fn longest_prefix_match(stored_scope: &ServoUrl, potential_match: &ServoUrl) -> bool {
    if stored_scope.origin() != potential_match.origin() {
        return false;
    }
    let scope_chars = stored_scope.path().chars();
    let matching_chars = potential_match.path().chars();
    if scope_chars.count() > matching_chars.count() {
        return false;
    }

    stored_scope.path().chars().zip(potential_match.path().chars()).all(|(scope, matched)| scope == matched)
}

impl<TH: TypeHolderTrait<TH>> ServiceWorkerRegistrationMethods<TH> for ServiceWorkerRegistration<TH> {
    // https://w3c.github.io/ServiceWorker/#service-worker-registration-installing-attribute
    fn GetInstalling(&self) -> Option<DomRoot<ServiceWorker<TH>>> {
        self.installing.as_ref().map(|sw| DomRoot::from_ref(&**sw))
    }

    // https://w3c.github.io/ServiceWorker/#service-worker-registration-active-attribute
    fn GetActive(&self) -> Option<DomRoot<ServiceWorker<TH>>> {
        self.active.as_ref().map(|sw| DomRoot::from_ref(&**sw))
    }

    // https://w3c.github.io/ServiceWorker/#service-worker-registration-waiting-attribute
    fn GetWaiting(&self) -> Option<DomRoot<ServiceWorker<TH>>> {
        self.waiting.as_ref().map(|sw| DomRoot::from_ref(&**sw))
    }

    // https://w3c.github.io/ServiceWorker/#service-worker-registration-scope-attribute
    fn Scope(&self) -> USVString {
        USVString(self.scope.as_str().to_owned())
    }
}
