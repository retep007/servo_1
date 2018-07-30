/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use bluetooth_traits::BluetoothRequest;
use dom::bindings::codegen::Bindings::TestRunnerBinding;
use dom::bindings::codegen::Bindings::TestRunnerBinding::TestRunnerMethods;
use dom::bindings::error::{Error, ErrorResult};
use dom::bindings::reflector::{DomObject, Reflector, reflect_dom_object};
use dom::bindings::root::DomRoot;
use dom::bindings::str::DOMString;
use dom::globalscope::GlobalScope;
use dom_struct::dom_struct;
use ipc_channel::ipc::IpcSender;
use profile_traits::ipc;
use typeholder::TypeHolderTrait;
use std::marker::PhantomData;

// https://webbluetoothcg.github.io/web-bluetooth/tests#test-runner
 #[dom_struct]
pub struct TestRunner<TH: TypeHolderTrait + 'static> {
    reflector_: Reflector<TH>,
    _p: PhantomData<TH>,
}

impl<TH: TypeHolderTrait> TestRunner<TH> {
    pub fn new_inherited() -> TestRunner<TH> {
        TestRunner {
            reflector_: Reflector::new(),
            _p: Default::default(),
        }
    }

    pub fn new(global: &GlobalScope<TH>) -> DomRoot<TestRunner<TH>> {
        reflect_dom_object(Box::new(TestRunner::new_inherited()),
                           global,
                           TestRunnerBinding::Wrap)
    }

    fn get_bluetooth_thread(&self) -> IpcSender<BluetoothRequest> {
        self.global().as_window().bluetooth_thread()
    }
}

impl<TH: TypeHolderTrait> TestRunnerMethods for TestRunner<TH> {
    // https://webbluetoothcg.github.io/web-bluetooth/tests#setBluetoothMockDataSet
    fn SetBluetoothMockDataSet(&self, dataSetName: DOMString) -> ErrorResult {
        let (sender, receiver) = ipc::channel(self.global().time_profiler_chan().clone()).unwrap();
        self.get_bluetooth_thread().send(BluetoothRequest::Test(String::from(dataSetName), sender)).unwrap();
        match receiver.recv().unwrap().into() {
            Ok(()) => {
                Ok(())
            },
            Err(error) => {
                Err(Error::from(error))
            },
        }
    }
}
