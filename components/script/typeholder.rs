use dom::servoparser::ServoParser;
use dom::bindings::trace::JSTraceable;
use malloc_size_of::MallocSizeOf;

pub trait TypeHolderTrait: MallocSizeOf + JSTraceable {
    type ServoParser: ServoParser<Self>;
}
