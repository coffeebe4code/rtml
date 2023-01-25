use crate::Render;
use std::fmt;

pub trait CssSelector {}
pub trait CssClass: CssSelector {}
pub trait CssId: CssSelector {}
pub trait CssElement: CssSelector {}
pub trait CssAttribute: CssSelector {}
pub trait CssPseudo: CssSelector {}
pub trait CssGlobal: CssSelector {}

#[macro_export]
macro_rules! class {
    ($ident:ident) => {
        #[allow(non_camel_case_types)]
        pub struct $ident;
        impl CssClass for $ident {}
        impl CssSelector for $ident {}

        impl fmt::Display for $ident {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                return write!(f, "{}", stringify!($ident));
            }
        }
    };
}

#[macro_export]
macro_rules! id {
    ($ident:ident) => {
        #[allow(non_camel_case_types)]
        pub struct $ident;
        impl CssId for $ident {}
        impl CssSelector for $ident {}

        impl fmt::Display for $ident {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                return write!(f, "{}", stringify!($ident));
            }
        }
    };
}

#[macro_export]
macro_rules! selectorit {
    ($ident:ident, $trait:ident) => {
        #[allow(non_camel_case_types)]
        pub struct $ident;
        impl $trait for $ident {}
        impl CssSelector for $ident {}

        impl fmt::Display for $ident {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                return write!(f, "{}", stringify!($ident));
            }
        }
    };
}

selectorit!(p, CssElement);
selectorit!(div, CssElement);
selectorit!(abbr, CssElement);

pub trait CssProperty {}

#[macro_export]
macro_rules! css {
    () => {
        ""
    };
    (,$head:expr $(,$blocks:expr)*) => {
        format_args!("{}\n\n{}", $head, css!($(,$blocks)*))
    };
    ($head:expr $(,$blocks:expr)*) => {
        format_args!("{}\n\n{}", $head, css!($(,$blocks)*))
    };
}

#[macro_export]
macro_rules! selector {
    (.$class:ident $($inner:tt)*) => {
        format_args!(".{}{}", $class, combinator!($($inner)*))
    };
    (#$id:ident $($inner:tt)*) => {
        format_args!("#{}{}", $ident, combinator!($($inner)*))
    };
    ($tag:ident $($inner:tt)*) => {
        format_args!("{}{}", $ident, combinator!($($inner)*))
    };
    (* $($inner:tt)*) => {
        format_args!("*{}", combinator!($($inner)*))
    };
    ({ $($inner:tt)* }) => {
        format_args!("{}", css_body!($($inner)*))
    };
}

#[macro_export]
macro_rules! combinator {
    (~ $($selector:tt)+) => {
        format_args!("~ {}", selector!($($selector)*))
    };
    (+ $($selector:tt)+) => {
        format_args!("~ {}", selector!($($selector)*))
    };
    (, $($selector:tt)+) => {
        format_args!(",\n{}", selector!($($selector)*))
    };
    (> $($selector:tt)+) => {
        format_args!("> {}", selector!($($selector)*))
    };
    (_ $($selector:tt)+) => {
        format_args!(" {}", selector!($($selector)*))
    };
    ({$($selector:tt)*}) => {
        format_args!("{}", css_body!($($selector)*))
    };
}

#[macro_export]
macro_rules! css_body {
    ($($inner:tt)*) => {
        format_args!(" {{\n{}}}\n", property!($($inner)*))
    };
}

#[macro_export]
macro_rules! property_value {
    ($val:literal) => {
        format_args!(": {};\n", $val)
    };
}

#[macro_export]
macro_rules! property {
    () => { "" };
    ($prop:literal: $val:literal) => {
        format_args!("  {}{}", $prop, property_value!($val))
    };
    ($prop:literal: $val:literal,) => {
        format_args!("  {}{}", $prop, property_value!($val))
    };
    ($prop:literal: $val:literal, $($inner:tt)*) => {
        format_args!("  {}{}{}", $prop, property_value!($val), property!($($inner)*))
    };
}

#[test]
fn test_value() {
    assert_eq!(property_value!("red").render(), ": red;\n");
}

#[test]
fn test_property() {
    assert_eq!(
        property!("background-color": "red",).render(),
        "  background-color: red;\n"
    );
    assert_eq!(
        property!("background-color": "red").render(),
        "  background-color: red;\n"
    );
    assert_eq!(
        property!("background-color": "red","float": "left",).render(),
        "  background-color: red;\n  float: left;\n"
    );
}

#[test]
fn test_css_body() {
    assert_eq!(
        css_body!("background-color": "red",).render(),
        " {\n  background-color: red;\n}\n"
    );
}

#[test]
fn test_selector_and_class() {
    class!(my_class);
    assert_eq!(selector!(.my_class {}).render(), ".my_class {\n}\n");
    assert_eq!(
        selector!(.my_class {"background-color": "red"}).render(),
        ".my_class {\n  background-color: red;\n}\n"
    );
}
