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
pub trait CssPropertyPart {}

macro_rules! propitpart {
    ($ident:ident) => {
        #[allow(non_camel_case_types)]
        pub struct $ident;
        impl CssProperty for $ident {}
    };
}
// webkit
propitpart!(_webkit_line_clamp);
propitpart!(line);
propitpart!(clamp);
propitpart!(text);
propitpart!(fill);
propitpart!(color);
propitpart!(stroke);
propitpart!(width);

// accent
propitpart!(accent);

// align
propitpart!(align);
propitpart!(content);
propitpart!(items);
propitpart!(_self);
propitpart!(background);
propitpart!(float);

#[macro_export]
macro_rules! css {
    ($($next:tt)*) => {
        format_args!("{}", selector!($($next)*))
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
        format_args!("{}{}", $tag, combinator!($($inner)*))
    };
    (* $($inner:tt)*) => {
        format_args!("*{}", combinator!($($inner)*))
    };
    ({ $($inner:tt)* }) => {
        format_args!("{}", css_body!($($inner)*))
    };
    ({ $($inner:tt)* } $($next:tt)+) => {
        format_args!("{}{}", css_body!($($inner)*), selector!($($next)*))
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
    ($head:ident $($selector:tt)+) => {
        format_args!(" {}{}", $head, selector!($($selector)*))
    };
    (_ $($selector:tt)+) => {
        format_args!(" {}", selector!($($selector)*))
    };
    ({$($selector:tt)*}) => {
        format_args!("{}", css_body!($($selector)*))
    };
    ({$($selector:tt)*} $($next:tt)+) => {
        format_args!("{}{}", css_body!($($selector)*), selector!($($next)*))
    };
}

#[macro_export]
macro_rules! css_body {
    ($($inner:tt)*) => {
        format_args!(" {{\n  {}}}\n", property!($($inner)*))
    };
}

#[macro_export]
macro_rules! property_value {
    ($val:literal) => {
        format_args!("{}", $val)
    };
}

#[macro_export]
macro_rules! property {
    () => { "" };
    ($head:ident : $val:literal) => {
        format_args!("{}: {};\n  ", $head, property_value!($val))
    };
    ($head:ident : $val:literal, $($next:tt)*) => {
        format_args!("{}: {};\n  {}", $head, property_value!($val), property!($($next)*))
    };
    ($head:ident-$($next:tt)*) => {
        format_args!("{}-{}", $head, property!($($next)*))
    };
    (-$head:ident-$($next:tt)*) => {
        format_args!("-{}-{}", $head, property!($($next)*))
    };
}

#[test]
fn test_value() {
    assert_eq!(property_value!("red").render(), "red");
}

#[test]
fn test_property() {
    assert_eq!(
        property!(background-color: "red",).render(),
        "background-color: red;\n  "
    );
    assert_eq!(
        property!(background-color: "red").render(),
        "background-color: red;\n  "
    );
    assert_eq!(
        property!(background-color: "red",float: "left",).render(),
        "background-color: red;\n  float: left;\n  "
    );
}

#[test]
fn test_css_body() {
    assert_eq!(
        css_body!(background-color: "red",).render(),
        " {\n  background-color: red;\n  }\n"
    );
}

#[test]
fn test_selector_and_class() {
    class!(my_class);
    assert_eq!(selector!(.my_class {}).render(), ".my_class {\n  }\n");
    assert_eq!(
        selector!(.my_class {background-color: "red"}).render(),
        ".my_class {\n  background-color: red;\n  }\n"
    );
}

#[test]
fn test_css() {
    let css = css!(
        p > div {
            background-color: "green",
            -webkit-line-clamp: "none",
        }
        p div {
            float: "left"

        }
    )
    .render();

    assert_eq!(
        css,
        "p > div {\n  background-color: green;\n  }\np div {\n  float: left;\n  }\n"
    );
}
