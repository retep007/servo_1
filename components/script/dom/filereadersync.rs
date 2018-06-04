/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::FileReaderSyncBinding;
use dom::bindings::error::Fallible;
use dom::bindings::reflector::reflect_dom_object;
use dom::bindings::root::DomRoot;
use dom::eventtarget::EventTarget;
use dom::globalscope::GlobalScope;
use dom_struct::dom_struct;
use typeholder::TypeHolderTrait;

#[dom_struct]
pub struct FileReaderSync<TH: TypeHolderTrait<TH> + 'static> {
    eventtarget: EventTarget<TH>
}

impl<TH: TypeHolderTrait<TH>> FileReaderSync<TH> {
    pub fn new_inherited() -> FileReaderSync<TH> {
        FileReaderSync {
            eventtarget: EventTarget::new_inherited(),
        }
    }

    pub fn new(global: &GlobalScope<TH>) -> DomRoot<FileReaderSync<TH>> {
        reflect_dom_object(Box::new(FileReaderSync::new_inherited()),
                           global, FileReaderSyncBinding::Wrap)
    }

    pub fn Constructor(global: &GlobalScope<TH>) -> Fallible<DomRoot<FileReaderSync<TH>>, TH> {
        Ok(FileReaderSync::new(global))
    }
}
