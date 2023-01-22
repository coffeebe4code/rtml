pub trait CssSelector {
    fn is_selector(&self) {}
}
pub trait CssClass: CssSelector {}
pub trait CssId: CssSelector {}
pub trait CssElement: CssSelector {}
pub trait CssAttribute: CssSelector {}
pub trait CssPseudo: CssSelector {}
pub trait CssGlobal: CssSelector {}

use crate::Render;

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

macro_rules! class {
    () => {
        ""
    };
    (.$class:ident  $head:expr $(,$blocks:expr)*) => {
        format_args!("{}\n\n{}", $head, css!($(,$blocks)*))
    };
    (,$head:expr $(,$blocks:expr)*) => {
        format_args!("{}\n\n{}", $head, css!($(,$blocks)*))
    };
    ($head:expr $(,$blocks:expr)*) => {
        format_args!("{}\n\n{}", $head, css!($(,$blocks)*))
    };
}

macro_rules! selector {
    (.$class:literal $(selector::tt)*) => {{
        format_args!(".{}", $selector)
    }};
    (#$id:literal $(selector::tt)*) => {{
        format_args!("#{}", $selector)
    }};
    ($tag:ident $(selector::tt)*) => {{
        format_args!(",\n{}", combinator!($selector))
    }};
    (> $(selector::tt)+) => {{
        format_args!("> {}", combinator$selector)
    }};
    (_ $(selector::tt)+) => {{
        format_args!(" {}", $selector)
    }};
}

#[macro_export]
macro_rules! combinator {
    (~ $(selector::tt)+) => {{
        format_args!("~ {}", selector!($selector))
    }};
    (+ $(selector::tt)+) => {{
        format_args!("~ {}", selector!($selector))
    }};
    (, $(selector::tt)+) => {{
        format_args!(",\n{}", selector!($selector))
    }};
    (> $(selector::tt)+) => {{
        format_args!("> {}", selector!($selector))
    }};
    (_ $(selector::tt)+) => {{
        format_args!(" {}", selector!($selector))
    }};
}

#[test]
fn test_css() {
    assert_eq!(css!("p {}", "div {}").render(), "p {}\n\ndiv {}");
}
