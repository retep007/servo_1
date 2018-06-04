/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::domexception::DOMErrorName;
use dom::filereader::{FileReader, TrustedFileReader, GenerationId, ReadMetaData};
use msg::constellation_msg::PipelineId;
use script_runtime::{CommonScriptMsg, ScriptThreadEventCategory, ScriptChan};
use std::sync::Arc;
use task::{TaskCanceller, TaskOnce};
use task_source::TaskSource;
use typeholder::TypeHolderTrait;

#[derive(JSTraceable)]
pub struct FileReadingTaskSource(pub Box<ScriptChan + Send + 'static>, pub PipelineId);

impl Clone for FileReadingTaskSource {
    fn clone(&self) -> FileReadingTaskSource {
        FileReadingTaskSource(self.0.clone(), self.1.clone())
    }
}

impl<TH: TypeHolderTrait<TH>> TaskSource<TH> for FileReadingTaskSource {
    fn queue_with_canceller<T>(
        &self,
        task: T,
        canceller: &TaskCanceller,
    ) -> Result<(), ()>
    where
        T: TaskOnce + 'static,
    {
        self.0.send(CommonScriptMsg::Task(
            ScriptThreadEventCategory::FileRead,
            Box::new(canceller.wrap_task(task)),
            Some(self.1),
        ))
    }
}

impl<TH: TypeHolderTrait<TH>> TaskOnce for FileReadingTask<TH> {
    fn run_once(self) {
        self.handle_task();
    }
}

#[allow(dead_code)]
pub enum FileReadingTask<TH: TypeHolderTrait<TH> + 'static> {
    ProcessRead(TrustedFileReader<TH>, GenerationId),
    ProcessReadData(TrustedFileReader<TH>, GenerationId),
    ProcessReadError(TrustedFileReader<TH>, GenerationId, DOMErrorName),
    ProcessReadEOF(TrustedFileReader<TH>, GenerationId, ReadMetaData, Arc<Vec<u8>>),
}

impl<TH: TypeHolderTrait<TH>> FileReadingTask<TH> {
    pub fn handle_task(self) {
        use self::FileReadingTask::*;

        match self {
            ProcessRead(reader, gen_id) =>
                FileReader::process_read(reader, gen_id),
            ProcessReadData(reader, gen_id) =>
                FileReader::process_read_data(reader, gen_id),
            ProcessReadError(reader, gen_id, error) =>
                FileReader::process_read_error(reader, gen_id, error),
            ProcessReadEOF(reader, gen_id, metadata, blob_contents) =>
                FileReader::process_read_eof(reader, gen_id, metadata, blob_contents),
        }
    }
}
