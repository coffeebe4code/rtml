use std::fmt::Display;

pub trait Tag: Display + 'static {}

pub trait TagValue: ToString {
    fn render(&self) -> String {
        return self.to_string();
    }
}
