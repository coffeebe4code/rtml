use std::fmt::Display;

pub trait GlobalAttribute {}
pub trait EventAttribute {}

pub trait Attribute: Display + 'static {}

pub trait AttributeValue: ToString {
    fn render(&self) -> String {
        self.to_string()
    }
}
