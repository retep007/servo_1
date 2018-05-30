use dom::servoparser::ServoParser;
use dom::bindings::trace::JSTraceable;
use malloc_size_of::MallocSizeOf;
use std::marker::Sized;
use std::marker::Send;
use dom::bindings::inheritance::Castable;
use std::clone::Clone;
use std::marker::Copy;

pub trait TypeHolderTrait: MallocSizeOf + JSTraceable + 'static + Sized + Default + Send + Castable + Clone + Copy {
    type ServoParser: ServoParser<Self>;
}
