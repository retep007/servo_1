use dom::servoparser::ServoParser;
use dom::bindings::trace::JSTraceable;
use malloc_size_of::MallocSizeOf;
use std::marker::Sized;

pub trait TypeHolderTrait: MallocSizeOf + JSTraceable + 'static + Sized {
    type ServoParser: ServoParser<Self>;
}
