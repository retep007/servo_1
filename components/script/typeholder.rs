use dom::servoparser::ServoParser;
use dom::bindings::trace::JSTraceable;

pub trait TypeHolderTrait: JSTraceable {
    type ServoParser: ServoParser<Self>;
}
