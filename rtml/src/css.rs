use paste::paste;
use std::fmt;
use std::fmt::Display;

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

pub trait CssProperty {
    fn is_prop(&self) {}
}

macro_rules! propit {
    ($ident:ident) => {
        #[allow(non_camel_case_types)]
        #[derive(Clone)]
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
propit!(break_after);
propit!(break_before);
propit!(break_inside);

propit!(caption_side);
propit!(caret_color);
propit!(clear);
propit!(clip_path);
propit!(color);
propit!(color_scheme);
propit!(column_count);
propit!(column_fill);
propit!(column_gap);
propit!(column_rule);
propit!(column_rule_color);
propit!(column_rule_style);
propit!(column_rule_width);
propit!(column_span);
propit!(column_width);
propit!(columns);
propit!(container);
propit!(container_name);
propit!(container_type);
propit!(counter_increment);
propit!(counter_reset);
propit!(counter_set);
propit!(content);
propit!(content_visibility);
propit!(cursor);
propit!(direction);
propit!(display);
propit!(empty_cells);
propit!(filter);
propit!(flex);
propit!(flex_basis);
propit!(flex_direction);
propit!(flex_flow);
propit!(flex_grow);
propit!(flex_shrink);
propit!(flex_wrap);
propit!(float);
propit!(font);
propit!(font_family);
propit!(font_feature_setting);
propit!(font_kerning);
propit!(font_language_override);
propit!(font_optical_sizing);
propit!(font_palette);
propit!(font_size);
propit!(font_size_adjust);
propit!(font_stretch);
propit!(font_style);
propit!(font_synthesis);
propit!(font_variant);
propit!(font_variant_alternatives);
propit!(font_variant_caps);
propit!(font_variant_east_asian);
propit!(font_variant_emoji);
propit!(font_variant_ligatures);
propit!(font_variant_numeric);
propit!(font_variant_position);
propit!(font_variant_settings);
propit!(font_weight);
propit!(forced_color_adjust);
propit!(gap);
propit!(grid);
propit!(grid_area);
propit!(grid_auto_columns);
propit!(grid_auto_flow);
propit!(grid_auto_rows);
propit!(grid_column);
propit!(grid_column_end);
propit!(grid_column_start);
propit!(grid_row);
propit!(grid_row_end);
propit!(grid_row_start);
propit!(grid_template);
propit!(grid_template_areas);
propit!(grid_template_columns);
propit!(grid_template_rows);
propit!(hanging_punctuation);
propit!(hyphenate_character);
propit!(hyphenate_limit_chars);
propit!(hyphens);
propit!(inline_size);
propit!(image_orientation);
propit!(image_rendering);
propit!(isolation);
propit!(inset);
propit!(inset_block);
propit!(inset_block_end);
propit!(inset_block_start);
propit!(inset_inline);
propit!(inset_inline_end);
propit!(inset_inline_start);
propit!(justify_content);
propit!(justify_items);
propit!(justify_self);
propit!(left);
propit!(letter_spacing);
propit!(line_break);
propit!(line_height);
propit!(list_style);
propit!(list_style_image);
propit!(list_style_position);
propit!(list_style_type);
propit!(margin);
propit!(margin_block);
propit!(margin_block_end);
propit!(margin_block_start);
propit!(margin_bottom);
propit!(margin_inline);
propit!(margin_inline_end);
propit!(margin_inline_start);
propit!(margin_left);
propit!(margin_right);
propit!(margin_top);
propit!(mask);
propit!(mask_border);
propit!(mask_border_mode);
propit!(mask_border_outset);
propit!(mask_border_repeat);
propit!(mask_border_slice);
propit!(mask_border_source);
propit!(mask_border_width);
propit!(mask_clip);
propit!(mask_composite);
propit!(mask_image);
propit!(mask_mode);
propit!(mask_origin);
propit!(mask_position);
propit!(mask_repeat);
propit!(mask_size);
propit!(mask_type);
propit!(math_style);
propit!(max_block_size);
propit!(max_height);
propit!(max_inline_size);
propit!(max_width);
propit!(min_block_size);
propit!(min_height);
propit!(min_inline_size);
propit!(min_width);

propit!(offset);
propit!(offset_anchar);
propit!(offset_distance);
propit!(offset_path);
propit!(offset_rotate);

propit!(mix_blend_mode);
propit!(object_fit);
propit!(object_position);
propit!(opacity);
propit!(order);
propit!(orphans);
propit!(outline);
propit!(outline_color);
propit!(outline_offset);
propit!(outline_style);
propit!(outline_width);
propit!(overflow);
propit!(overflow_anchor);
propit!(overflow_block);
propit!(overflow_clip_margin);
propit!(overflow_inline);
propit!(overflow_wrap);
propit!(overflow_x);
propit!(overflow_y);
propit!(overscoll_behavior);
propit!(overscoll_behavior_block);
propit!(overscoll_behavior_inline);
propit!(overscoll_behavior_x);
propit!(overscoll_behavior_y);
propit!(padding);
propit!(padding_block);
propit!(padding_block_end);
propit!(padding_block_start);
propit!(padding_bottom);
propit!(padding_inline);
propit!(padding_inline_end);
propit!(padding_inline_start);
propit!(padding_left);
propit!(padding_right);
propit!(padding_top);
propit!(page_break_after);
propit!(page_break_before);
propit!(page_break_inside);
propit!(paint_order);
propit!(perspective);
propit!(perspective_origin);
propit!(place_content);
propit!(place_items);
propit!(place_self);
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
propit!(scroll_behavior);
propit!(scroll_margin);
propit!(scroll_margin_block);
propit!(scroll_margin_block_end);
propit!(scroll_margin_block_start);
propit!(scroll_margin_bottom);
propit!(scroll_margin_inline);
propit!(scroll_margin_inline_end);
propit!(scroll_margin_inline_start);
propit!(scroll_margin_left);
propit!(scroll_margin_right);
propit!(scroll_margin_top);
propit!(scroll_padding);
propit!(scroll_padding_block);
propit!(scroll_padding_block_end);
propit!(scroll_padding_block_start);
propit!(scroll_padding_bottom);
propit!(scroll_padding_inline);
propit!(scroll_padding_inline_end);
propit!(scroll_padding_inline_start);
propit!(scroll_padding_left);
propit!(scroll_padding_right);
propit!(scroll_padding_top);
propit!(scroll_snap_align);
propit!(scroll_snap_stop);
propit!(scroll_snap_type);
propit!(scrollbar_color);
propit!(scrollbar_gutter);
propit!(scrollbar_width);
propit!(shape_image_threshold);
propit!(shape_margin);
propit!(shape_outside);
propit!(tab_size);
propit!(table_layout);
propit!(text_align);
propit!(text_align_last);
propit!(text_combine_upright);
propit!(text_decoration);
propit!(text_decoration_color);
propit!(text_decoration_line);
propit!(text_decoration_skip_ink);
propit!(text_decoration_style);
propit!(text_decoration_thickness);
propit!(text_emphasis);
propit!(text_emphasis_color);
propit!(text_emphasis_position);
propit!(text_emphasis_style);
propit!(text_indent);
propit!(text_justify);
propit!(text_orientation);
propit!(text_overflow);
propit!(text_rendering);
propit!(text_shadow);
propit!(text_transform);
propit!(text_underline_offset);
propit!(text_underline_position);
propit!(top);
propit!(touch_action);
propit!(transform);
propit!(transform_box);
propit!(transform_origin);
propit!(transform_style);
propit!(transition);
propit!(transition_delay);
propit!(transition_duration);
propit!(transition_property);
propit!(transition_timing_function);
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

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::css::*;
///
/// let css = css!(
///     p > div {
///         background-color: "green",
///     }
///     p div {
///         float: "left"
///     }
///    )
///    .render();
/// assert_eq!(
///     css,
///     "p > div {\n  background-color: green;\n  }\np div {\n  float: left;\n  }\n"
/// );
///
/// # }
/// ```
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
    ($head:ident : $val:literal) => {{
        $head.is_prop();
        format_args!("{}: {};\n  ", $head.clone(), property_value!($val))
    }};
    (: $val:literal) => {
        format_args!(": {};\n  ", property_value!($val))
    };
    (: $val:literal, $($next:tt)*) => {
        format_args!(": {};\n  {}", property_value!($val), property!($($next)*))
    };
    ($head:ident : $val:literal, $($next:tt)*) => {
        format_args!("{}: {};\n  {}", $head, property_value!($val), property!($($next)*))
    };
    ($head:ident$(-$next:ident)+: $($rest:tt)*) => {
        {
            let ident = paste!{[<$head $(_$next)*>]};
            ident.is_prop();
            format_args!("{}{}", ident.clone(), property!(: $($rest)*))
        }
    };
    (-$head:ident$(-$next:ident)+: $($rest:tt)*) => {
        {
            let ident = paste!{[<_$head $(_$next)*>]};
            ident.is_prop();
        format_args!("{}{}", ident.clone(), property!(: $($rest)*))
        }
    };
}
