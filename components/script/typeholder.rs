use dom::servoparser::ServoParser;

pub trait TypeHolderTrait {
    type ServoParser: ServoParser;
}
