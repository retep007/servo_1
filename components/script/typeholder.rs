use dom::servoparser::ServoParser;
use dom::bindings::trace::JSTraceable;
use malloc_size_of::MallocSizeOf;
use std::marker::Sized;
use std::marker::Send;
use dom::bindings::inheritance::Castable;
use std::clone::Clone;
use std::marker::Copy;
use std::fmt::Debug;
use std::cmp::PartialEq;

pub trait TypeHolderTrait: MallocSizeOf + JSTraceable  + 'static + Sized + Default + Send  + Clone + Copy + Debug + PartialEq {
    type ServoParser: ServoParser<Self>;
}