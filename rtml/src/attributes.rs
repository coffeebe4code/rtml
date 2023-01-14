use std::fmt::Display;

//pub trait Attribute: Display {
//    fn render(&self) -> fmt::Arguments {
//        format_args!("{}=", self)
//    }
//}

pub trait GlobalAttribute {}

pub trait Attribute: Display + 'static {}

pub trait AttributeValue: ToString {
    fn render(&self) -> String {
        self.to_string()
    }
}
