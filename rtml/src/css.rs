use crate::Render;
use paste::paste;
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

macro_rules! propit {
    ($ident:ident) => {
        #[allow(non_camel_case_types)]
        pub struct $ident;
        impl CssProperty for $ident {}

        impl fmt::Display for $ident {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                return write!(f, "{}", str::replace(stringify!($ident), "_", "-"));
            }
        }
    };
}
// webkit
propit!(_webkit_line_clamp);
propit!(_webkit_text_fill_color);
propit!(_webkit_text_stroke);
propit!(_webkit_text_stroke_color);
propit!(_webkit_text_stroke_width);
propit!(accent_color);
propit!(align_content);
propit!(align_items);
propit!(align_self);
propit!(all);
propit!(animation);
propit!(animation_delay);
propit!(animation_direction);
propit!(animation_duration);
propit!(animation_fill_mode);
propit!(animation_iteration_count);
propit!(animation_name);
propit!(animation_play_state);
propit!(animation_timing_function);
propit!(appearance);
propit!(aspect_ration);
propit!(backdrop_filter);
propit!(backface_visibility);
propit!(background);
propit!(background_attachment);
propit!(background_blend_mode);
propit!(background_clip);
propit!(background_color);
propit!(background_image);
propit!(background_origin);
propit!(background_position);
propit!(background_position_x);
propit!(background_position_y);
propit!(background_repeat);
propit!(background_size);
propit!(block_size);
propit!(border);
propit!(border_block);
propit!(border_block_color);
propit!(border_block_end);
propit!(border_block_end_color);
propit!(border_block_end_style);
propit!(border_block_end_width);
propit!(border_block_start);
propit!(border_block_start_color);
propit!(border_block_start_style);
propit!(border_block_start_width);
propit!(border_block_style);
propit!(border_block_width);
propit!(border_bottom);
propit!(border_bottom_color);
propit!(border_bottom_left_radius);
propit!(border_bottom_right_radius);
propit!(border_bottom_style);
propit!(border_bottom_width);
propit!(border_collapse);
propit!(border_color);
propit!(border_end_end_radius);
propit!(border_end_start_radius);
propit!(border_image);
propit!(border_image_outset);
propit!(border_image_repeat);
propit!(border_image_slice);
propit!(border_image_source);
propit!(border_image_width);
propit!(border_inline);
propit!(border_inline_color);
propit!(border_inline_end);
propit!(border_inline_end_color);
propit!(border_inline_end_style);
propit!(border_inline_end_width);
propit!(border_inline_start);
propit!(border_inline_start_color);
propit!(border_inline_start_style);
propit!(border_inline_start_width);
propit!(border_inline_style);
propit!(border_inline_width);
propit!(border_left);
propit!(border_left_color);
propit!(border_left_style);
propit!(border_left_width);
propit!(border_radius);
propit!(border_right);
propit!(border_right_color);
propit!(border_right_style);
propit!(border_right_wdith);
propit!(border_spacing);
propit!(border_start_end_radius);
propit!(border_start_start_radius);
propit!(border_style);
propit!(border_top);
propit!(border_top_color);
propit!(border_top_left_radius);
propit!(border_top_right_radius);
propit!(border_top_style);
propit!(border_top_width);
propit!(border_width);
propit!(bottom);

propit!(caption_side);
propit!(caret_color);
propit!(clear);
propit!(clip_path);
propit!(color);
propit!(color_scheme);
propit!(columns);
propit!(content);
propit!(content_visibility);
propit!(cursor);
propit!(direction);
propit!(display);
propit!(empty_cells);
propit!(filter);
propit!(float);
propit!(forced_color_adjust);
propit!(gap);
propit!(hanging_punctuation);
propit!(hyphenate_character);
propit!(hyphenate_limit_chars);
propit!(hyphens);
propit!(inline_size);
propit!(isolation);
propit!(left);
propit!(letter_spacing);
propit!(mix_blend_mode);
propit!(object_fit);
propit!(object_position);
propit!(opacity);
propit!(order);
propit!(orphans);
propit!(paint_order);
propit!(perspective);
propit!(perspective_origin);
propit!(pointer_events);
propit!(position);
propit!(print_color_adjust);
propit!(quotes);
propit!(resize);
propit!(right);
propit!(rotate);
propit!(row_gap);
propit!(ruby_position);
propit!(scale);
propit!(tab_size);
propit!(table_layout);
propit!(top);
propit!(touch_action);
propit!(translate);
propit!(unicode_bidi);
propit!(user_select);
propit!(vertical_align);
propit!(visibility);
propit!(white_space);
propit!(widows);
propit!(width);
propit!(will_change);
propit!(word_break);
propit!(word_spacing);
propit!(writing_mode);
propit!(z_index);

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
        format_args!(" ~ {}", selector!($($selector)*))
    };
    (+ $($selector:tt)+) => {
        format_args!(" + {}", selector!($($selector)*))
    };
    (, $($selector:tt)+) => {
        format_args!(",\n{}", selector!($($selector)*))
    };
    (> $($selector:tt)+) => {
        format_args!(" > {}", selector!($($selector)*))
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
    (: $val:literal) => {
        format_args!(": {};\n  ", property_value!($val))
    };
    (: $val:literal, $($next:tt)*) => {
        format_args!(": {};\n  {}", property_value!($val), property!($($next)*))
    };
    ($head:ident : $val:literal, $($next:tt)*) => {
        format_args!("{}: {};\n  {}", $head, property_value!($val), property!($($next)*))
    };
    ($head:ident-$($next:ident)+: $($rest:tt)*) => {
        format_args!("{}{}", paste!{[<$head _ $($next)*>]} , property!(: $($rest)*))
    };
    (-$head:ident-$($next:ident)+: $($rest:tt)*) => {
        format_args!("{}{}", paste!{[<$head _ $($next)*>]} , property!(: $($rest)*))
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
        property!(background-color: "red", float: "left",).render(),
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
