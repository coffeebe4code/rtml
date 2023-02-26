use crate::*;

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

pub trait CssPseudoElement: CssSelector {
    fn is_pseudo_element(&self) {}
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
    ($ident:ident) => {
        impl CssElement for $ident {}
        impl CssSelector for $ident {}
    };
}

selectorit!(a);
selectorit!(abbr);
selectorit!(address);
selectorit!(area);
selectorit!(article);
selectorit!(aside);
selectorit!(audio);
selectorit!(b);
selectorit!(base);
selectorit!(bdi);
selectorit!(bdo);
selectorit!(blockquote);
selectorit!(body);
selectorit!(br);
selectorit!(button);
selectorit!(canvas);
selectorit!(caption);
selectorit!(cite);
selectorit!(code);
selectorit!(col);
selectorit!(colgroup);
selectorit!(data);
selectorit!(datalist);
selectorit!(dd);
selectorit!(del);
selectorit!(details);
selectorit!(dfn);
selectorit!(dialog);
selectorit!(div);
selectorit!(dl);
selectorit!(dt);
selectorit!(em);
selectorit!(embed);
selectorit!(fieldset);
selectorit!(figcaption);
selectorit!(figure);
selectorit!(footer);
selectorit!(form);
selectorit!(h1);
selectorit!(h2);
selectorit!(h3);
selectorit!(h4);
selectorit!(h5);
selectorit!(h6);
selectorit!(head);
selectorit!(header);
selectorit!(hr);
selectorit!(html);
selectorit!(i);
selectorit!(iframe);
selectorit!(img);
selectorit!(input);
selectorit!(ins);
selectorit!(kbd);
selectorit!(label);
selectorit!(legend);
selectorit!(li);
selectorit!(link);
selectorit!(main);
selectorit!(map);
selectorit!(mark);
selectorit!(menu);
selectorit!(meta);
selectorit!(meter);
selectorit!(nav);
selectorit!(noscript);
selectorit!(object);
selectorit!(ol);
selectorit!(optgroup);
selectorit!(option);
selectorit!(output);
selectorit!(p);
selectorit!(picture);
selectorit!(pre);
selectorit!(progress);
selectorit!(q);
selectorit!(rp);
selectorit!(rt);
selectorit!(ruby);
selectorit!(s);
selectorit!(samp);
selectorit!(script);
selectorit!(section);
selectorit!(select);
selectorit!(small);
selectorit!(source);
selectorit!(span);
selectorit!(strong);
selectorit!(style);
selectorit!(sub);
selectorit!(summary);
selectorit!(sup);
selectorit!(table);
selectorit!(tbody);
selectorit!(td);
selectorit!(template);
selectorit!(textarea);
selectorit!(tfoot);
selectorit!(th);
selectorit!(thead);
selectorit!(time);
selectorit!(title);
selectorit!(tr);
selectorit!(track);
selectorit!(u);
selectorit!(ul);
selectorit!(var);
selectorit!(video);
selectorit!(wbr);

pub trait CssProperty {
    fn is_prop(&self) {}
}

macro_rules! propit {
    ($ident:ident) => {
        impl CssProperty for $ident {}
    };
}

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

macro_rules! pseudoclassit {
    ($ident:ident) => {
        impl CssPseudoClass for $ident {}
        impl CssSelector for $ident {}
    };
}

macro_rules! parenable {
    ($ident:ident) => {
        impl CssIsParenable for $ident {}
    };
}

pseudoclassit!(active);
pseudoclassit!(any_link);
pseudoclassit!(autofill);
pseudoclassit!(checked);
pseudoclassit!(current);
pseudoclassit!(default);
pseudoclassit!(defined);
pseudoclassit!(dir);
parenable!(dir);
pseudoclassit!(disabled);
pseudoclassit!(empty);
pseudoclassit!(enabled);
pseudoclassit!(first);
pseudoclassit!(first_child);
pseudoclassit!(first_of_type);
pseudoclassit!(focus);
pseudoclassit!(focus_visible);
pseudoclassit!(focus_within);
pseudoclassit!(fullscreen);
pseudoclassit!(future);
pseudoclassit!(has);
parenable!(has);
pseudoclassit!(hover);
pseudoclassit!(in_range);
pseudoclassit!(indeterminate);
pseudoclassit!(invalid);
pseudoclassit!(is);
parenable!(is);
pseudoclassit!(lang);
parenable!(lang);
pseudoclassit!(last_child);
parenable!(last_child);
pseudoclassit!(last_of_type);
pseudoclassit!(left);
// DUPLICATE selector
// pseudoclassit!(link);
pseudoclassit!(local_link);
pseudoclassit!(modal);
pseudoclassit!(not);
parenable!(not);
pseudoclassit!(nth_child);
parenable!(nth_child);
pseudoclassit!(nth_col);
parenable!(nth_col);
pseudoclassit!(nth_last_child);
parenable!(nth_last_child);
pseudoclassit!(nth_last_col);
parenable!(nth_last_col);
pseudoclassit!(nth_last_of_type);
parenable!(nth_last_of_type);
pseudoclassit!(nth_of_type);
parenable!(nth_of_type);
pseudoclassit!(only_child);
pseudoclassit!(only_of_type);
pseudoclassit!(optional);
pseudoclassit!(out_of_range);
pseudoclassit!(past);
pseudoclassit!(paused);
pseudoclassit!(picture_in_picture);
pseudoclassit!(placeholder_shown);
pseudoclassit!(playing);
pseudoclassit!(read_only);
pseudoclassit!(read_write);
pseudoclassit!(required);
pseudoclassit!(right);
pseudoclassit!(root);
pseudoclassit!(scope);
pseudoclassit!(target);
pseudoclassit!(target_within);
pseudoclassit!(valid);
pseudoclassit!(visited);
pseudoclassit!(_where);
parenable!(_where);

macro_rules! pseudoelementit {
    ($ident:ident) => {
        impl CssPseudoElement for $ident {}
        impl CssSelector for $ident {}
    };
}

pseudoelementit!(after);
pseudoelementit!(backdrop);
pseudoelementit!(before);
pseudoelementit!(cue);
pseudoelementit!(cue_region);
pseudoelementit!(file_selector_button);
pseudoelementit!(first_letter);
pseudoelementit!(first_line);
pseudoelementit!(marker);
pseudoelementit!(part);
parenable!(part);
pseudoelementit!(placeholder);
pseudoelementit!(selection);
pseudoelementit!(slotted);
parenable!(slotted);

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
        selector!($($next)*)
    };
}

#[macro_export]
macro_rules! selector {
    (.$class:ident $($inner:tt)*) => {{
        CssClass::is_class(&$class);
        render_fn!(".{}{}", $class, combinator!($($inner)*))
    }};
    (#$id:ident $($inner:tt)*) => {{
        CssId::is_id(&$id);
        render_fn!("#{}{}", $id, combinator!($($inner)*))
    }};
    ($tag:ident $($inner:tt)*) => {{
        let ident = $tag;
        CssSelector::is_selector(&ident);
        render_fn!("{}{}", ident, combinator!($($inner)*))
    }};
    (:$($inner:tt)*) => {
        pseudo_class!($($inner)*)
    };
    (::$($inner:tt)*) => {
        pseudo_element!($($inner)*)
    };
    (* $($inner:tt)*) => {
        render_fn!("*{}", combinator!($($inner)*))
    };
    ({ $($inner:tt)* }) => {
        css_body!($($inner)*)
    };
    ({ $($inner:tt)* } $($next:tt)+) => {
        render_fn!("{}{}", css_body!($($inner)*), selector!($($next)*))
    };
}

#[macro_export]
macro_rules! pseudo_class {
    ($head:ident $($rest:tt)+) => {
        {
            let ident = $head ;
            CssPseudoClass::is_pseudo_class(&ident);
            render_fn!(":{}{}", ident, combinator!($($rest)*))
        }
    };
    ($head:ident ($lit:expr) $($rest:tt)+) => {
        {
            let ident = $head;
            CssPseudoClass::is_pseudo_class(&ident);
            render_fn!(":{}({}){}", ident, $lit, combinator!($($rest)*))
        }
    };
}

#[macro_export]
macro_rules! pseudo_element {
    ($head:ident $($rest:tt)+) => {
        {
            let ident = $head;
            CssPseudoElement::is_pseudo_element(&ident);
            render_fn!("::{}{}", ident, combinator!($($rest)*))
        }

    };
    ($head:ident ($lit:expr) $($rest:tt)+) => {
        {
            let ident = $head;
            CssPseudoElement::is_pseudo_element(&ident);
            render_fn!("::{}({}){}", ident, $lit, combinator!($($rest)*))
        }
    };
}

#[macro_export]
macro_rules! combinator {
    (~ $($selector:tt)+) => {
        render_fn!(" ~ {}", selector!($($selector)*))
    };
    (+ $($selector:tt)+) => {
        render_fn!(" + {}", selector!($($selector)*))
    };
    (, $($selector:tt)+) => {
        render_fn!(",\n{}", selector!($($selector)*))
    };
    (> $($selector:tt)+) => {
        render_fn!(" > {}", selector!($($selector)*))
    };
    ($head:ident $($selector:tt)+) => {{
        let ident = $head;
        CssSelector::is_selector(&ident);
        render_fn!(" {}{}", ident, selector!($($selector)*))
    }};
    (_ $($selector:tt)+) => {
        render_fn!(" {}", selector!($($selector)*))
    };
    ({$($selector:tt)*}) => {
        css_body!($($selector)*)
    };
    ({$($selector:tt)*} $($next:tt)+) => {
        render_fn!("{}{}", css_body!($($selector)*), selector!($($next)*))
    };
    ([$($inner:tt)*] $($next:tt)+) => {
        render_fn!("{}{}", attr_selector!([$($inner)*]), selector!($($next)*))
    };
    ($($rest:tt)*) => {
        selector!($($rest)*)
    };
}

#[macro_export]
macro_rules! attr_selector {
    () => {
        ""
    };
    (,.$attr:ident = $val:expr $(,.$attrs:ident$(-$nexts:ident)* = $vals:expr)*) => {{
        let ident = $attr;

        render_fn!(" {}=\"{}\"{}", ident, $val, attr_selector!($(,.$attrs = $vals)*))
    }
    };
    ([.$attr:ident = $val:expr $(,.$attrs:ident = $vals:expr)*]) => {{
        let ident = $attr;
        render_fn!("[{}=\"{}\"{}]", ident,$val, attr_selector!($(,.$attrs = $vals)*))
    }};
}

#[macro_export]
macro_rules! css_body {
    ($($inner:tt)*) => {
        render_fn!(" {{\n  {}}}\n", property!($($inner)*))
    };
}

#[macro_export]
macro_rules! property_value {
    ($val:literal) => {
        $val
    };
}

#[macro_export]
macro_rules! property {
    () => { "" };
    ($head:ident : $val:literal) => {{
        let ident = $head;
        CssProperty::is_prop(&ident);
        render_fn!("{}: {};\n  ", ident, property_value!($val))
    }};
    (: $val:literal) => {
        render_fn!(": {};\n  ", property_value!($val))
    };
    (: $val:literal, $($next:tt)*) => {
        render_fn!(": {};\n  {}", property_value!($val), property!($($next)*))
    };
    ($head:ident : $val:literal, $($next:tt)*) => {{
            let ident = $head;
            CssProperty::is_prop(&ident);

        render_fn!("{}: {};\n  {}", ident, property_value!($val), property!($($next)*))
    }
    };
    ($head:ident : $($rest:tt)*) => {
        {
            let ident = $head;
            CssProperty::is_prop(&ident);
            render_fn!("{}{}", ident, property!(: $($rest)*))
        }
    };
}
