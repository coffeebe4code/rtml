pub trait CssSelector {
    fn is_selector(&self) {}
}
pub trait CssClass: CssSelector {
    fn is_class(&self) {}
}
pub trait CssId: CssSelector {
    fn is_id(&self) {}
}
pub trait CssElement: CssSelector {}
pub trait CssAttribute: CssSelector {}
pub trait CssPseudoClass: CssSelector {
    fn is_pseudo_class(&self) {}
}
pub trait CssIsParenable {
    fn is_parenable(&self) {}
}
pub trait CssGlobal: CssSelector {}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// // create your own custom class.
/// make_class!(my_class);
/// let css = css!(
///     .my_class {
///         color: "red"
///     }).render();
/// // this css renders to
/// // .my_class {
/// //   color: "red";
/// //  }
/// assert_eq!(css, ".my_class {\n  color: red;\n  }\n");
///
/// // it can be used in conjunction with html macros
///
/// let html = p! { .class = my_class, "red text!" }.render();
/// // this html renders to
/// // <p class="my_class">red text!</p>
/// assert_eq!(html, "<p class=\"my_class\">red text!</p>");
///
/// # }
/// ```

#[macro_export]
macro_rules! make_class {
    ($ident:ident) => {
        #[allow(non_camel_case_types)]
        #[derive(Clone)]
        pub struct $ident;
        impl CssClass for $ident {}
        impl CssSelector for $ident {}

        impl std::fmt::Display for $ident {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                return write!(f, "{}", stringify!($ident));
            }
        }
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// // create your own custom id.
/// make_id!(my_id);
/// let css = css!(
///     #my_id {
///         color: "red"
///     }).render();
/// // this css renders to
/// // #my_id {
/// //   color: "red";
/// //  }
/// assert_eq!(css, "#my_id {\n  color: red;\n  }\n");
///
/// // it can be used in conjunction with html macros
///
/// let html = p! { .id = my_id, "red text!" }.render();
/// // this html renders to
/// // <p id="my_id">red text!</p>
/// assert_eq!(html, "<p id=\"my_id\">red text!</p>");
///
/// # }
/// ```
#[macro_export]
macro_rules! make_id {
    ($ident:ident) => {
        #[allow(non_camel_case_types)]
        #[derive(Clone)]
        pub struct $ident;
        impl CssId for $ident {}
        impl CssSelector for $ident {}

        impl std::fmt::Display for $ident {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                return write!(f, "{}", stringify!($ident));
            }
        }
    };
}

macro_rules! selectorit {
    ($ident:ident, $lit:literal) => {
        paste::paste! {
        #[allow(non_camel_case_types)]
        #[derive(Clone)]
        pub struct [< _ $ident _ >];
        impl CssElement for [< _ $ident _ >] {}
        impl CssSelector for [< _ $ident _ >] {}

        impl std::fmt::Display for [< _ $ident _ >] {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                return write!(f, "{}", $lit);
            }
        }
        }
    };
}

selectorit!(a, "a");
selectorit!(abbr, "abbr");
selectorit!(address, "address");
selectorit!(area, "area");
selectorit!(article, "article");
selectorit!(aside, "aside");
selectorit!(audio, "audio");
selectorit!(b, "b");
selectorit!(base, "base");
selectorit!(bdi, "bdi");
selectorit!(bdo, "bdo");
selectorit!(blockquote, "blockquote");
selectorit!(body, "body");
selectorit!(br, "br");
selectorit!(button, "button");
selectorit!(canvas, "canvas");
selectorit!(caption, "caption");
selectorit!(cite, "cite");
selectorit!(code, "code");
selectorit!(col, "col");
selectorit!(colgroup, "colgroup");
selectorit!(data, "data");
selectorit!(datalist, "datalist");
selectorit!(dd, "dd");
selectorit!(del, "del");
selectorit!(details, "details");
selectorit!(dfn, "dfn");
selectorit!(dialog, "dialog");
selectorit!(div, "div");
selectorit!(dl, "dl");
selectorit!(dt, "dt");
selectorit!(em, "em");
selectorit!(embed, "embed");
selectorit!(fieldset, "fieldset");
selectorit!(figcaption, "figcaption");
selectorit!(figure, "figure");
selectorit!(footer, "footer");
selectorit!(form, "form");
selectorit!(h1, "h1");
selectorit!(h2, "h2");
selectorit!(h3, "h3");
selectorit!(h4, "h4");
selectorit!(h5, "h5");
selectorit!(h6, "h6");
selectorit!(head, "head");
selectorit!(header, "header");
selectorit!(hr, "hr");
selectorit!(html, "html");
selectorit!(i, "i");
selectorit!(iframe, "iframe");
selectorit!(img, "img");
selectorit!(input, "input");
selectorit!(ins, "ins");
selectorit!(kbd, "kbd");
selectorit!(label, "label");
selectorit!(legend, "legend");
selectorit!(li, "li");
selectorit!(link, "link");
selectorit!(main, "main");
selectorit!(map, "map");
selectorit!(mark, "mark");
selectorit!(menu, "menu");
selectorit!(meta, "meta");
selectorit!(meter, "meter");
selectorit!(nav, "nav");
selectorit!(noscript, "noscript");
selectorit!(object, "object");
selectorit!(ol, "ol");
selectorit!(optgroup, "optgroup");
selectorit!(option, "option");
selectorit!(output, "output");
selectorit!(p, "p");
selectorit!(picture, "picture");
selectorit!(pre, "pre");
selectorit!(progress, "progress");
selectorit!(q, "q");
selectorit!(rp, "rp");
selectorit!(rt, "rt");
selectorit!(ruby, "ruby");
selectorit!(s, "s");
selectorit!(samp, "samp");
selectorit!(script, "script");
selectorit!(section, "section");
selectorit!(select, "select");
selectorit!(small, "small");
selectorit!(source, "source");
selectorit!(span, "span");
selectorit!(strong, "strong");
selectorit!(style, "style");
selectorit!(sub, "sub");
selectorit!(summary, "summary");
selectorit!(sup, "sup");
selectorit!(table, "table");
selectorit!(tbody, "tbody");
selectorit!(td, "td");
selectorit!(template, "template");
selectorit!(textarea, "textarea");
selectorit!(tfoot, "tfoot");
selectorit!(th, "th");
selectorit!(thead, "thead");
selectorit!(time, "time");
selectorit!(title, "title");
selectorit!(tr, "tr");
selectorit!(track, "track");
selectorit!(u, "u");
selectorit!(ul, "ul");
selectorit!(var, "var");
selectorit!(video, "video");
selectorit!(wbr, "wbr");

pub trait CssProperty {
    fn is_prop(&self) {}
}

macro_rules! propit {
    ($ident:ident, $lit:literal) => {
        paste::paste! {
        #[derive(Clone)]
        #[allow(non_camel_case_types)]
        pub struct [<_$ident>];
        impl CssProperty for [<_$ident>] {}

        impl std::fmt::Display for [<_$ident>] {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                return write!(f, "{}", $lit);
            }
        }
        }
    };
}

propit!(_webkit_line_clamp, "-webkit-line-clamp");
propit!(_webkit_text_fill_color, "-webkit-text-fill-color");
propit!(_webkit_text_stroke, "-webkit-text-stroke");
propit!(_webkit_text_stroke_color, "-webkit_text_stroke_color");
propit!(_webkit_text_stroke_width, "-webkit-text-stroke-width");
propit!(accent_color, "accent-color");
propit!(align_content, "align-content");
propit!(align_items, "align-items");
propit!(align_self, "align-self");
propit!(all, "all");
propit!(animation, "animation");
propit!(animation_delay, "animation-delay");
propit!(animation_direction, "animation-direction");
propit!(animation_duration, "animation-duration");
propit!(animation_fill_mode, "animation-fill-mode");
propit!(animation_iteration_count, "animation-iteration-count");
propit!(animation_name, "animation-name");
propit!(animation_play_state, "animation-play-state");
propit!(animation_timing_function, "animation-timing-function");
propit!(appearance, "appearance");
propit!(aspect_ration, "aspect-ration");
propit!(backdrop_filter, "backdrop-filter");
propit!(backface_visibility, "backface-visibility");
propit!(background, "background");
propit!(background_attachment, "background-attachment");
propit!(background_blend_mode, "background-blend-mode");
propit!(background_clip, "background-clip");
propit!(background_color, "background-color");
propit!(background_image, "background-image");
propit!(background_origin, "background-origin");
propit!(background_position, "background-position");
propit!(background_position_x, "background-position-x");
propit!(background_position_y, "background-position-y");
propit!(background_repeat, "background-repeat");
propit!(background_size, "background-size");
propit!(block_size, "block-size");
propit!(border, "border");
propit!(border_block, "border-block");
propit!(border_block_color, "border-block-color");
propit!(border_block_end, "border-block-end");
propit!(border_block_end_color, "border-block-end-color");
propit!(border_block_end_style, "border-block-end-style");
propit!(border_block_end_width, "border-block-end-width");
propit!(border_block_start, "border-block-start");
propit!(border_block_start_color, "border-block-start-color");
propit!(border_block_start_style, "border-block-start-style");
propit!(border_block_start_width, "border-block-start-width");
propit!(border_block_style, "border-block-style");
propit!(border_block_width, "border-block-width");
propit!(border_bottom, "border-bottom");
propit!(border_bottom_color, "border-bottom-color");
propit!(border_bottom_left_radius, "border-bottom-left-radius");
propit!(border_bottom_right_radius, "border-bottom-right-radius");
propit!(border_bottom_style, "border-bottom-style");
propit!(border_bottom_width, "border-bottom-width");
propit!(border_collapse, "border-collapse");
propit!(border_color, "border-color");
propit!(border_end_end_radius, "border-end-end-radius");
propit!(border_end_start_radius, "border-end-start-radius");
propit!(border_image, "border-image");
propit!(border_image_outset, "border-image-outset");
propit!(border_image_repeat, "border-image-repeat");
propit!(border_image_slice, "border-image-slice");
propit!(border_image_source, "border-image-source");
propit!(border_image_width, "border-image-width");
propit!(border_inline, "border-inline");
propit!(border_inline_color, "border-inline-color");
propit!(border_inline_end, "border-inline-end");
propit!(border_inline_end_color, "border-inline-end-color");
propit!(border_inline_end_style, "border-inline-end-style");
propit!(border_inline_end_width, "border-inline-end-width");
propit!(border_inline_start, "border-inline-start");
propit!(border_inline_start_color, "border-inline-start-color");
propit!(border_inline_start_style, "border-inline-start-style");
propit!(border_inline_start_width, "border-inline-start-width");
propit!(border_inline_style, "border-inline-style");
propit!(border_inline_width, "border-inline-width");
propit!(border_left, "border-left");
propit!(border_left_color, "border-left-color");
propit!(border_left_style, "border-left-style");
propit!(border_left_width, "border-left-width");
propit!(border_radius, "border-radius");
propit!(border_right, "border-right");
propit!(border_right_color, "border-right-color");
propit!(border_right_style, "border-right-style");
propit!(border_right_wdith, "border-right-wdith");
propit!(border_spacing, "border-spacing");
propit!(border_start_end_radius, "border-start-end-radius");
propit!(border_start_start_radius, "border-start-start-radius");
propit!(border_style, "border-style");
propit!(border_top, "border-top");
propit!(border_top_color, "border-top-color");
propit!(border_top_left_radius, "border-top-left-radius");
propit!(border_top_right_radius, "border-top-right-radius");
propit!(border_top_style, "border-top-style");
propit!(border_top_width, "border-top-width");
propit!(border_width, "border-width");
propit!(bottom, "bottom");
propit!(break_after, "break-after");
propit!(break_before, "break-before");
propit!(break_inside, "break-inside");
propit!(caption_side, "caption-side");
propit!(caret_color, "caret-color");
propit!(clear, "clear");
propit!(clip_path, "clip-path");
propit!(color, "color");
propit!(color_scheme, "color-scheme");
propit!(column_count, "column-count");
propit!(column_fill, "column-fill");
propit!(column_gap, "column-gap");
propit!(column_rule, "column-rule");
propit!(column_rule_color, "column-rule-color");
propit!(column_rule_style, "column-rule-style");
propit!(column_rule_width, "column-rule-width");
propit!(column_span, "column-span");
propit!(column_width, "column-width");
propit!(columns, "columns");
propit!(container, "container");
propit!(container_name, "container-name");
propit!(container_type, "container-type");
propit!(counter_increment, "counter-increment");
propit!(counter_reset, "counter-reset");
propit!(counter_set, "counter-set");
propit!(content, "content");
propit!(content_visibility, "content-visibility");
propit!(cursor, "cursor");
propit!(direction, "direction");
propit!(display, "display");
propit!(empty_cells, "empty-cells");
propit!(filter, "filter");
propit!(flex, "flex");
propit!(flex_basis, "flex-basis");
propit!(flex_direction, "flex-direction");
propit!(flex_flow, "flex-flow");
propit!(flex_grow, "flex-grow");
propit!(flex_shrink, "flex-shrink");
propit!(flex_wrap, "flex-wrap");
propit!(float, "float");
propit!(font, "font");
propit!(font_family, "font-family");
propit!(font_feature_setting, "font-feature-setting");
propit!(font_kerning, "font-kerning");
propit!(font_language_override, "font-language-override");
propit!(font_optical_sizing, "font-optical-sizing");
propit!(font_palette, "font-palette");
propit!(font_size, "font-size");
propit!(font_size_adjust, "font-size-adjust");
propit!(font_stretch, "font-stretch");
propit!(font_style, "font-style");
propit!(font_synthesis, "font-synthesis");
propit!(font_variant, "font-variant");
propit!(font_variant_alternatives, "font-variant-alternatives");
propit!(font_variant_caps, "font-variant-caps");
propit!(font_variant_east_asian, "font-variant-east-asian");
propit!(font_variant_emoji, "font-variant-emoji");
propit!(font_variant_ligatures, "font-variant-ligatures");
propit!(font_variant_numeric, "font-variant-numeric");
propit!(font_variant_position, "font-variant-position");
propit!(font_variant_settings, "font-variant-settings");
propit!(font_weight, "font-weight");
propit!(forced_color_adjust, "forced-color-adjust");
propit!(gap, "gap");
propit!(grid, "grid");
propit!(grid_area, "grid-area");
propit!(grid_auto_columns, "grid-auto-columns");
propit!(grid_auto_flow, "grid-auto-flow");
propit!(grid_auto_rows, "grid-auto-rows");
propit!(grid_column, "grid-column");
propit!(grid_column_end, "grid-column-end");
propit!(grid_column_start, "grid-column-start");
propit!(grid_row, "grid-row");
propit!(grid_row_end, "grid-row-end");
propit!(grid_row_start, "grid-row-start");
propit!(grid_template, "grid-template");
propit!(grid_template_areas, "grid-template-areas");
propit!(grid_template_columns, "grid-template-columns");
propit!(grid_template_rows, "grid-template-rows");
propit!(hanging_punctuation, "hanging-punctuation");
propit!(hyphenate_character, "hyphenate-character");
propit!(hyphenate_limit_chars, "hyphenate-limit-chars");
propit!(hyphens, "hyphens");
propit!(inline_size, "inline-size");
propit!(image_orientation, "image-orientation");
propit!(image_rendering, "image-rendering");
propit!(isolation, "isolation");
propit!(inset, "inset");
propit!(inset_block, "inset-block");
propit!(inset_block_end, "inset-block-end");
propit!(inset_block_start, "inset-block-start");
propit!(inset_inline, "inset-inline");
propit!(inset_inline_end, "inset-inline-end");
propit!(inset_inline_start, "inset-inline-start");
propit!(justify_content, "justify-content");
propit!(justify_items, "justify-items");
propit!(justify_self, "justify-self");
propit!(left, "left");
propit!(letter_spacing, "letter-spacing");
propit!(line_break, "line-break");
propit!(line_height, "line-height");
propit!(list_style, "list-style");
propit!(list_style_image, "list-style-image");
propit!(list_style_position, "list-style-position");
propit!(list_style_type, "list-style-type");
propit!(margin, "margin");
propit!(margin_block, "margin-block");
propit!(margin_block_end, "margin-block-end");
propit!(margin_block_start, "margin-block-start");
propit!(margin_bottom, "margin-bottom");
propit!(margin_inline, "margin-inline");
propit!(margin_inline_end, "margin-inline-end");
propit!(margin_inline_start, "margin-inline-start");
propit!(margin_left, "margin-left");
propit!(margin_right, "margin-right");
propit!(margin_top, "margin-top");
propit!(mask, "mask");
propit!(mask_border, "mask-border");
propit!(mask_border_mode, "mask-border-mode");
propit!(mask_border_outset, "mask-border-outset");
propit!(mask_border_repeat, "mask-border-repeat");
propit!(mask_border_slice, "mask-border-slice");
propit!(mask_border_source, "mask-border-source");
propit!(mask_border_width, "mask-border-width");
propit!(mask_clip, "mask-clip");
propit!(mask_composite, "mask-composite");
propit!(mask_image, "mask-image");
propit!(mask_mode, "mask-mode");
propit!(mask_origin, "mask-origin");
propit!(mask_position, "mask-position");
propit!(mask_repeat, "mask-repeat");
propit!(mask_size, "mask-size");
propit!(mask_type, "mask-type");
propit!(math_style, "math-style");
propit!(max_block_size, "max-block-size");
propit!(max_height, "max-height");
propit!(max_inline_size, "max-inline-size");
propit!(max_width, "max-width");
propit!(min_block_size, "min-block-size");
propit!(min_height, "min-height");
propit!(min_inline_size, "min-inline-size");
propit!(min_width, "min-width");
propit!(offset, "offset");
propit!(offset_anchar, "offset-anchar");
propit!(offset_distance, "offset-distance");
propit!(offset_path, "offset-path");
propit!(offset_rotate, "offset-rotate");
propit!(mix_blend_mode, "mix-blend-mode");
propit!(object_fit, "object-fit");
propit!(object_position, "object-position");
propit!(opacity, "opacity");
propit!(order, "order");
propit!(orphans, "orphans");
propit!(outline, "outline");
propit!(outline_color, "outline-color");
propit!(outline_offset, "outline-offset");
propit!(outline_style, "outline-style");
propit!(outline_width, "outline-width");
propit!(overflow, "overflow");
propit!(overflow_anchor, "overflow-anchor");
propit!(overflow_block, "overflow-block");
propit!(overflow_clip_margin, "overflow-clip-margin");
propit!(overflow_inline, "overflow-inline");
propit!(overflow_wrap, "overflow-wrap");
propit!(overflow_x, "overflow-x");
propit!(overflow_y, "overflow-y");
propit!(overscoll_behavior, "overscoll-behavior");
propit!(overscoll_behavior_block, "overscoll-behavior-block");
propit!(overscoll_behavior_inline, "overscoll-behavior-inline");
propit!(overscoll_behavior_x, "overscoll-behavior-x");
propit!(overscoll_behavior_y, "overscoll-behavior-y");
propit!(padding, "padding");
propit!(padding_block, "padding-block");
propit!(padding_block_end, "padding-block-end");
propit!(padding_block_start, "padding-block-start");
propit!(padding_bottom, "padding-bottom");
propit!(padding_inline, "padding-inline");
propit!(padding_inline_end, "padding-inline-end");
propit!(padding_inline_start, "padding-inline-start");
propit!(padding_left, "padding-left");
propit!(padding_right, "padding-right");
propit!(padding_top, "padding-top");
propit!(page_break_after, "page-break-after");
propit!(page_break_before, "page-break-before");
propit!(page_break_inside, "page-break-inside");
propit!(paint_order, "paint-order");
propit!(perspective, "perspective");
propit!(perspective_origin, "perspective-origin");
propit!(place_content, "place-content");
propit!(place_items, "place-items");
propit!(place_self, "place-self");
propit!(pointer_events, "pointer-events");
propit!(position, "position");
propit!(print_color_adjust, "print-color-adjust");
propit!(quotes, "quotes");
propit!(resize, "resize");
propit!(right, "right");
propit!(rotate, "rotate");
propit!(row_gap, "row-gap");
propit!(ruby_position, "ruby-position");
propit!(scale, "scale");
propit!(scroll_behavior, "scroll-behavior");
propit!(scroll_margin, "scroll-margin");
propit!(scroll_margin_block, "scroll-margin-block");
propit!(scroll_margin_block_end, "scroll-margin-block-end");
propit!(scroll_margin_block_start, "scroll-margin-block-start");
propit!(scroll_margin_bottom, "scroll-margin-bottom");
propit!(scroll_margin_inline, "scroll-margin-inline");
propit!(scroll_margin_inline_end, "scroll-margin-inline-end");
propit!(scroll_margin_inline_start, "scroll-margin-inline-start");
propit!(scroll_margin_left, "scroll-margin-left");
propit!(scroll_margin_right, "scroll-margin-right");
propit!(scroll_margin_top, "scroll-margin-top");
propit!(scroll_padding, "scroll-padding");
propit!(scroll_padding_block, "scroll-padding-block");
propit!(scroll_padding_block_end, "scroll-padding-block-end");
propit!(scroll_padding_block_start, "scroll-padding-block-start");
propit!(scroll_padding_bottom, "scroll-padding-bottom");
propit!(scroll_padding_inline, "scroll-padding-inline");
propit!(scroll_padding_inline_end, "scroll-padding-inline-end");
propit!(scroll_padding_inline_start, "scroll-padding-inline-start");
propit!(scroll_padding_left, "scroll-padding-left");
propit!(scroll_padding_right, "scroll-padding-right");
propit!(scroll_padding_top, "scroll-padding-top");
propit!(scroll_snap_align, "scroll-snap-align");
propit!(scroll_snap_stop, "scroll-snap-stop");
propit!(scroll_snap_type, "scroll-snap-type");
propit!(scrollbar_color, "scrollbar-color");
propit!(scrollbar_gutter, "scrollbar-gutter");
propit!(scrollbar_width, "scrollbar-width");
propit!(shape_image_threshold, "shape-image-threshold");
propit!(shape_margin, "shape-margin");
propit!(shape_outside, "shape-outside");
propit!(tab_size, "tab-size");
propit!(table_layout, "table-layout");
propit!(text_align, "text-align");
propit!(text_align_last, "text-align-last");
propit!(text_combine_upright, "text-combine-upright");
propit!(text_decoration, "text-decoration");
propit!(text_decoration_color, "text-decoration-color");
propit!(text_decoration_line, "text-decoration-line");
propit!(text_decoration_skip_ink, "text-decoration-skip-ink");
propit!(text_decoration_style, "text-decoration-style");
propit!(text_decoration_thickness, "text-decoration-thickness");
propit!(text_emphasis, "text-emphasis");
propit!(text_emphasis_color, "text-emphasis-color");
propit!(text_emphasis_position, "text-emphasis-position");
propit!(text_emphasis_style, "text-emphasis-style");
propit!(text_indent, "text-indent");
propit!(text_justify, "text-justify");
propit!(text_orientation, "text-orientation");
propit!(text_overflow, "text-overflow");
propit!(text_rendering, "text-rendering");
propit!(text_shadow, "text-shadow");
propit!(text_transform, "text-transform");
propit!(text_underline_offset, "text-underline-offset");
propit!(text_underline_position, "text-underline-position");
propit!(top, "top");
propit!(touch_action, "touch-action");
propit!(transform, "transform");
propit!(transform_box, "transform-box");
propit!(transform_origin, "transform-origin");
propit!(transform_style, "transform-style");
propit!(transition, "transition");
propit!(transition_delay, "transition-delay");
propit!(transition_duration, "transition-duration");
propit!(transition_property, "transition-property");
propit!(transition_timing_function, "transition-timing-function");
propit!(translate, "translate");
propit!(unicode_bidi, "unicode-bidi");
propit!(user_select, "user-select");
propit!(vertical_align, "vertical-align");
propit!(visibility, "visibility");
propit!(white_space, "white-space");
propit!(widows, "widows");
propit!(width, "width");
propit!(will_change, "will-change");
propit!(word_break, "word-break");
propit!(word_spacing, "word-spacing");
propit!(writing_mode, "writing-mode");
propit!(z_index, "z-index");

macro_rules! pseudoclassit {
    ($ident:ident, $lit:literal) => {
        paste::paste! {
        #[derive(Clone)]
        #[allow(non_camel_case_types)]
        pub struct [<___$ident>];
        impl CssPseudoClass for [<___$ident>] {}
        impl CssSelector for [<___$ident>] {}

        impl std::fmt::Display for [<___$ident>] {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                return write!(f, "{}", $lit);
            }
        }
        }
    };
}

macro_rules! parenable {
    ($ident:ident) => {
        paste::paste! {
        impl CssIsParenable for [<___$ident>] {}
        }
    };
}

pseudoclassit!(active, "active");
pseudoclassit!(any_link, "any-link");
pseudoclassit!(autofill, "autofill");
pseudoclassit!(checked, "checked");
pseudoclassit!(current, "current");
pseudoclassit!(default, "default");
pseudoclassit!(defined, "defined");
pseudoclassit!(dir, "dir");
parenable!(dir);
pseudoclassit!(disabled, "disabled");
pseudoclassit!(empty, "empty");
pseudoclassit!(enabled, "enabled");
pseudoclassit!(first, "first");
pseudoclassit!(first_child, "first-child");
pseudoclassit!(first_of_type, "first-of-type");
pseudoclassit!(focus, "focus");
pseudoclassit!(focus_visible, "focus-visible");
pseudoclassit!(focus_within, "focus-within");
pseudoclassit!(fullscreen, "fullscreen");
pseudoclassit!(future, "future");
pseudoclassit!(has, "has");
parenable!(has);
pseudoclassit!(hover, "hover");
pseudoclassit!(in_range, "in-range");
pseudoclassit!(indeterminate, "indeterminate");
pseudoclassit!(invalid, "invalid");
pseudoclassit!(is, "is");
parenable!(is);
pseudoclassit!(lang, "lang");
parenable!(lang);
pseudoclassit!(last_child, "last-child");
parenable!(last_child);
pseudoclassit!(last_of_type, "last-of-type");
pseudoclassit!(left, "left");
pseudoclassit!(link, "link");
pseudoclassit!(local_link, "local-link");
pseudoclassit!(modal, "modal");
pseudoclassit!(not, "not");
parenable!(not);
pseudoclassit!(nth_child, "nth-child");
parenable!(nth_child);
pseudoclassit!(nth_col, "nth-col");
parenable!(nth_col);
pseudoclassit!(nth_last_child, "nth-last-child");
parenable!(nth_last_child);
pseudoclassit!(nth_last_col, "nth-last-col");
parenable!(nth_last_col);
pseudoclassit!(nth_last_of_type, "nth-last-of-type");
parenable!(nth_last_of_type);
pseudoclassit!(nth_of_type, "nth-of-type");
parenable!(nth_of_type);
pseudoclassit!(only_child, "only-child");
pseudoclassit!(only_of_type, "only-of-type");
pseudoclassit!(optional, "optional");
pseudoclassit!(out_of_range, "out-of-range");
pseudoclassit!(past, "past");
pseudoclassit!(paused, "paused");
pseudoclassit!(picture_in_picture, "picture-in-picture");
pseudoclassit!(placeholder_shown, "placeholder-shown");
pseudoclassit!(playing, "playing");
pseudoclassit!(read_only, "read-only");
pseudoclassit!(read_write, "read-write");
pseudoclassit!(required, "required");
pseudoclassit!(right, "right");
pseudoclassit!(root, "root");
pseudoclassit!(scope, "scope");
pseudoclassit!(target, "target");
pseudoclassit!(target_within, "target-within");
pseudoclassit!(valid, "valid");
pseudoclassit!(visited, "valid");
pseudoclassit!(where, "where");
parenable!(where);

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// let css = css!(
///     p > div {
///         background-color: "green",
///     }
///     p div {
///         float: "left"
///     }
///    )
///    .render(); assert_eq!( css,
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
    (.$class:ident $($inner:tt)*) => {{
        CssClass::is_class(&$class);
        format_args!(".{}{}", $class.clone(), combinator!($($inner)*))
    }};
    (#$id:ident $($inner:tt)*) => {{
        CssId::is_id(&$id);
        format_args!("#{}{}", $id.clone(), combinator!($($inner)*))
    }};
    ($tag:ident $($inner:tt)*) => {{
        let ident = paste::paste! { [< _ $tag _ >] };
        CssSelector::is_selector(&ident);
        format_args!("{}{}", ident.clone(), combinator!($($inner)*))
    }};
    (:$($inner:tt)*) => {
        pseudo_class!(:$($inner)*)
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
macro_rules! pseudo_class {
    (:$head:ident$(-$next:ident)+ ($lit:expr) $($rest:tt)+) => {
        {
            let ident = paste::paste!{[<___$head $(_$next)*>]};
            CssPseudoClass::is_pseudo_class(&ident);
            format_args!(":{}({}){}", ident.clone(), $lit, combinator!($($rest)*))
        }
    };
    (:$head:ident$(-$next:ident)+ $($rest:tt)+) => {
        {
            let ident = paste::paste!{[<___$head $(_$next)*>]};
            CssPseudoClass::is_pseudo_class(&ident);
            format_args!(":{}{}", ident.clone(), combinator!($($rest)*))
        }
    };
    (:$head:ident ($lit:expr) $($rest:tt)+) => {
        {
            let ident = paste::paste!{[<___$head >]};
            CssPseudoClass::is_pseudo_class(&ident);
            CssIsParenable::is_parenable(&ident);
            format_args!(":{}({}){}", ident.clone(), $lit, combinator!($($rest)*))
        }
    };
    (:$head:ident $($rest:tt)+) => {
        {
            let ident = paste::paste!{[<___$head >]};
            CssPseudoClass::is_pseudo_class(&ident);
            format_args!(":{}{}", ident.clone(), combinator!($($rest)*))
        }
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
    ($head:ident $($selector:tt)+) => {{
        let ident = paste::paste! { [< _ $head _ >] };
        CssSelector::is_selector(&ident);
        format_args!(" {}{}", ident.clone(), selector!($($selector)*))
    }};
    (_ $($selector:tt)+) => {
        format_args!(" {}", selector!($($selector)*))
    };
    ({$($selector:tt)*}) => {
        format_args!("{}", css_body!($($selector)*))
    };
    ({$($selector:tt)*} $($next:tt)+) => {
        format_args!("{}{}", css_body!($($selector)*), selector!($($next)*))
    };
    ([$($inner:tt)*] $($next:tt)+) => {
        format_args!("{}{}", attr_selector!([$($inner)*]), selector!($($next)*))
    };
    ($($rest:tt)*) => {
        format_args!("{}", selector!($($rest)*))
    };
}

#[macro_export]
macro_rules! attr_selector {
    () => {
        format_args!("{}","")
    };
    (,.$attr:ident$(-$next:ident)* = $val:expr $(,.$attrs:ident$(-$nexts:ident)* = $vals:expr)*) => {{
        let ident = paste::paste! { [<$attr $(_$next)*_>] };

        format_args!(" {}=\"{}\"{}", ident.clone(), $val, attr_selector!($(,.$attrs$(-$nexts)* = $vals)*))
    }
    };
    ([.$attr:ident$(-$next:ident)* = $val:expr $(,.$attrs:ident$(-$nexts:ident)* = $vals:expr)*]) => {{
        let ident = paste::paste! { [<$attr $(_$next)*_>] };
        format_args!("[{}=\"{}\"{}]", ident.clone(), $val, attr_selector!($(,.$attrs$(-$nexts)* = $vals)*))
    }};
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
        let ident = paste::paste!{[<_$head>]};
        CssProperty::is_prop(&ident);
        format_args!("{}: {};\n  ", ident.clone(), property_value!($val).clone())
    }};
    (: $val:literal) => {
        format_args!(": {};\n  ", property_value!($val))
    };
    (: $val:literal, $($next:tt)*) => {
        format_args!(": {};\n  {}", property_value!($val), property!($($next)*))
    };
    ($head:ident : $val:literal, $($next:tt)*) => {{
            let ident = paste::paste!{[<_$head>]};
            CssProperty::is_prop(&ident);

        format_args!("{}: {};\n  {}", ident.clone(), property_value!($val), property!($($next)*))
    }
    };
    ($head:ident$(-$next:ident)+: $($rest:tt)*) => {
        {
            let ident = paste::paste!{[<_$head $(_$next)*>]};
            CssProperty::is_prop(&ident);
            format_args!("{}{}", ident.clone(), property!(: $($rest)*))
        }
    };
    (-$head:ident$(-$next:ident)+: $($rest:tt)*) => {
        {
            let ident = paste::paste!{[<__$head $(_$next)*>]};
            CssProperty::is_prop(&ident);
            format_args!("{}{}", ident.clone(), property!(: $($rest)*))
        }
    };
}
