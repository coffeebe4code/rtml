macro_rules! baseit {
    ($base:ident, $val:expr) => {
        #[allow(non_camel_case_types)]
        #[derive(Clone)]
        pub struct $base;
        impl std::fmt::Display for $base {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                return write!(f, "{}", $val);
            }
        }
    };
}

// TAGS
baseit! {ATag, "a"}
baseit! {AbbrTag, "abbr"}
baseit! {AddressTag, "address"}
baseit! {AreaTag, "area"}
baseit! {ArticleTag, "article"}
baseit! {AsideTag, "aside"}
baseit! {AudioTag, "audio"}
baseit! {BTag, "b"}
baseit! {BaseTag, "base"}
baseit! {BdiTag, "bdi"}
baseit! {BdoTag, "bdo"}
baseit! {BlockquoteTag, "blockquote"}
baseit! {BodyTag, "body"}
baseit! {BrTag, "br"}
baseit! {ButtonTag, "button"}
baseit! {CanvasTag, "canvas"}
baseit! {CaptionTag, "caption"}
baseit! {CiteTag, "cite"}
baseit! {CodeTag, "code"}
baseit! {ColTag, "col"}
baseit! {ColgroupTag, "colgroup"}
baseit! {DataTag, "data"}
baseit! {DatalistTag, "datalist"}
baseit! {DdTag, "dd"}
baseit! {DelTag, "del"}
baseit! {DetailsTag, "details"}
baseit! {DfnTag, "dfn"}
baseit! {DialogTag, "dialog"}
baseit! {DivTag, "div"}
baseit! {DlTag, "dl"}
baseit! {DtTag, "dt"}
baseit! {EmTag, "em"}
baseit! {EmbedTag, "embed"}
baseit! {FieldsetTag, "fieldset"}
baseit! {FigcaptionTag, "figcaption"}
baseit! {FigureTag, "figure"}
baseit! {FooterTag, "footer"}
baseit! {FormTag, "form"}
baseit! {H1Tag, "h1"}
baseit! {H2Tag, "h2"}
baseit! {H3Tag, "h3"}
baseit! {H4Tag, "h4"}
baseit! {H5Tag, "h5"}
baseit! {H6Tag, "h6"}
baseit! {HeadTag, "head"}
baseit! {HeaderTag, "header"}
baseit! {HrTag, "hr"}
baseit! {HtmlTag, "html"}
baseit! {ITag, "i"}
baseit! {IframeTag, "iframe"}
baseit! {ImgTag, "img"}
baseit! {InputTag, "input"}
baseit! {InsTag, "ins"}
baseit! {KbdTag, "kbd"}
baseit! {LabelTag, "label"}
baseit! {LegendTag, "legend"}
baseit! {LiTag, "li"}
baseit! {LinkTag, "link"}
baseit! {MainTag, "main"}
baseit! {MapTag, "map"}
baseit! {MarkTag, "mark"}
baseit! {MenuTag, "menu"}
baseit! {MetaTag, "meta"}
baseit! {MeterTag, "meter"}
baseit! {NavTag, "nav"}
baseit! {NoscriptTag, "noscript"}
baseit! {ObjectTag, "object"}
baseit! {OlTag, "ol"}
baseit! {OptgroupTag, "optgroup"}
baseit! {OptionTag, "option"}
baseit! {OutputTag, "output"}
baseit! {PTag, "p"}
baseit! {PictureTag, "picture"}
baseit! {PreTag, "pre"}
baseit! {ProgressTag, "progress"}
baseit! {QTag, "q"}
baseit! {RpTag, "rp"}
baseit! {RtTag, "rt"}
baseit! {RubyTag, "ruby"}
baseit! {STag, "s"}
baseit! {SampTag, "samp"}
baseit! {ScriptTag, "script"}
baseit! {SectionTag, "section"}
baseit! {SelectTag, "select"}
baseit! {SmallTag, "small"}
baseit! {SourceTag, "source"}
baseit! {SpanTag, "span"}
baseit! {StrongTag, "strong"}
baseit! {StyleTag, "style"}
baseit! {SubTag, "sub"}
baseit! {SummaryTag, "summary"}
baseit! {SupTag, "sup"}
baseit! {TableTag, "table"}
baseit! {TbodyTag, "tbody"}
baseit! {TdTag, "td"}
baseit! {TemplateTag, "template"}
baseit! {TextareaTag, "textarea"}
baseit! {TfootTag, "tfoot"}
baseit! {ThTag, "th"}
baseit! {TheadTag, "thead"}
baseit! {TimeTag, "time"}
baseit! {TitleTag, "title"}
baseit! {TrTag, "tr"}
baseit! {TrackTag, "track"}
baseit! {UTag, "u"}
baseit! {UlTag, "ul"}
baseit! {VarTag, "var"}
baseit! {VideoTag, "video"}
baseit! {WbrTag, "wbr"}

// Global Attributes
baseit! {accesskey, "accesskey"}
baseit! {class, "class"}
baseit! {contenteditable, "contenteditable"}
baseit! {dir, "dir"}
baseit! {draggable, "draggable"}
baseit! {hidden, "hidden"}
baseit! {id, "id"}
baseit! {lang, "lang"}
baseit! {spellcheck, "spellcheck"}
baseit! {style, "style"}
baseit! {tabindex, "tabindex"}
baseit! {title, "title"}
baseit! {translate, "translate"}

// Special
baseit! {_type, "type"}
baseit! {_loop, "loop"}
baseit! {_for, "for"}
baseit! {http_equiv, "http-equiv"}
baseit! {accept_charset, "accept-charset"}
baseit! {_as, "as"}
baseit! {_async, "async"}
baseit! {_kind, "kind"}

// Event
// Window
baseit! {onafterprint, "onafterprint"}
baseit! {onbeforeprint, "onbeforeprint"}
baseit! {onbeforeunload, "onbeforeunload"}
baseit! {onerror, "onerror"}
baseit! {onhashchange, "onhashchange"}
baseit! {onload, "onload"}
baseit! {onmessage, "onmessage"}
baseit! {onoffline, "onoffline"}
baseit! {ononline, "ononline"}
baseit! {onpagehide, "onpagehide"}
baseit! {onpageshow, "onpageshow"}
baseit! {onpopstate, "onpopstate"}
baseit! {onresize, "onresize"}
baseit! {onunload, "onunload"}

// Form
baseit! {onblur, "onblur"}
baseit! {onchange, "onchange"}
baseit! {oncontextmenu, "oncontextmenu"}
baseit! {onfocus, "onfocus"}
baseit! {oninput, "oninput"}
baseit! {onreset, "onreset"}
baseit! {onsearch, "onsearch"}
baseit! {onselect, "onselect"}
baseit! {onsubmit, "onsubmit"}

// Keyboard
baseit! {onkeydown, "onkeydown"}
baseit! {onkeypress, "onkeypress"}
baseit! {onkeyup, "onkeyup"}

// Mouse
baseit! {onclick, "onclick"}
baseit! {ondblclick, "ondblclick"}
baseit! {onmousedown, "onmousedown"}
baseit! {onmousemove, "onmousemove"}
baseit! {onmouseout, "onmouseout"}
baseit! {onmouseover, "onmouseover"}
baseit! {onmouseup, "onmouseup"}
baseit! {onwheel, "onwheel"}

// Drag
baseit! {ondrag, "ondrag"}
baseit! {ondragend, "ondragend"}
baseit! {ondragenter, "ondragenter"}
baseit! {ondragleave, "ondragleave"}
baseit! {ondragover, "ondragover"}
baseit! {ondragstart, "ondragstart"}
baseit! {ondrop, "ondrop"}
baseit! {onscroll, "onscroll"}

// Clipboard
baseit! {oncopy, "oncopy"}
baseit! {oncut, "oncut"}
baseit! {onpaste, "onpaste"}

// Media
baseit! {onabort, "onabort"}
baseit! {oncanplay, "oncanplay"}
baseit! {oncanplaythrough, "oncanplaythrough"}
baseit! {oncuechange, "oncuechange"}
baseit! {ondurationchange, "ondurationchange"}
baseit! {onemptied, "onemptied"}
baseit! {onended, "onended"}
baseit! {onloadeddata, "onloadeddata"}
baseit! {onloadedmetadata, "onloadedmetadata"}
baseit! {onloadstart, "onloadstart"}
baseit! {onpause, "onpause"}
baseit! {onplay, "onplay"}
baseit! {onplaying, "onplaying"}
baseit! {onprogress, "onprogress"}
baseit! {onratechange, "onratechange"}
baseit! {onseeked, "onseeked"}
baseit! {onseeking, "onseeking"}
baseit! {onstalled, "onstalled"}
baseit! {onsuspend, "onsuspend"}
baseit! {ontimeupdate, "ontimeupdate"}
baseit! {onvolumechange, "onvolumechange"}
baseit! {onwaiting, "onwaiting"}

// Misc
baseit! {ontoggle, "ontoggle"}

// Specific Attributes
// a
baseit! {href, "href"}
baseit! {src, "src"}
baseit! {download, "download"}
baseit! {media, "media"}
baseit! {ping, "ping"}
baseit! {referrerpolicy, "referrerpolicy"}
baseit! {rel, "rel"}
baseit! {hreflang, "hreflang"}
baseit! {target, "target"}
// area
baseit! {alt, "alt"}
baseit! {coords, "coords"}
baseit! {shape, "shape"}
// audio
baseit! {autoplay, "autoplay"}
baseit! {controls, "controls"}
baseit! {muted, "muted"}
baseit! {preload, "preload"}
// DUPLICATE
//baseit! {cite, "cite"}
// button
baseit! {autofocus, "autofocus"}
baseit! {disabled, "disabled"}
// DUPLICATE
// baseit! {form, "form"}
baseit! {formaction, "formaction"}
baseit! {formenctype, "formenctype"}
baseit! {formmethod, "formmethod"}
baseit! {formnovalidate, "formnovalidate"}
baseit! {formtarget, "formtarget"}
baseit! {name, "name"}
baseit! {value, "value"}
// canvas
baseit! {height, "height"}
baseit! {width, "width"}
// col
baseit! {span, "span"}
// del
baseit! {datetime, "datetime"}
// details
baseit! {open, "open"}
// form
baseit! {action, "action"}
baseit! {autocomplete, "autocomplete"}
baseit! {enctype, "enctype"}
baseit! {method, "method"}
baseit! {novalidate, "novalidate"}
// html
baseit! {xmlns, "xmlns"}
// iframe
baseit! {allow, "allow"}
baseit! {allowfullscreen, "allowfullscreen"}
baseit! {allowpaymentrequest, "allowpaymentrequest"}
baseit! {loading, "loading"}
baseit! {sandbox, "sandbox"}
baseit! {srcdoc, "srcdoc"}
// img
baseit! {crossorigin, "crossorigin"}
baseit! {decoding, "decoding"}
baseit! {ismap, "ismap"}
baseit! {longdesc, "longdesc"}
baseit! {srcset, "srcset"}
baseit! {sizes, "sizes"}
baseit! {usemap, "usemap"}
// input
baseit! {accept, "accept"}
baseit! {checked, "checked"}
baseit! {dirname, "dirname"}
baseit! {list, "list"}
baseit! {max, "max"}
baseit! {maxlength, "maxlength"}
baseit! {min, "min"}
baseit! {minlength, "minlength"}
baseit! {multiple, "multiple"}
baseit! {pattern, "pattern"}
baseit! {placeholder, "placeholder"}
baseit! {readonly, "readonly"}
baseit! {required, "required"}
baseit! {step, "step"}
// meta
baseit! {content, "content"}
baseit! {charset, "charset"}
// meter
baseit! {high, "high"}
baseit! {low, "low"}
baseit! {optimum, "optimum"}
// object
// DUPLICATE
//baseit! {data, "data"}
// ol
baseit! {reversed, "reversed"}
baseit! {start, "start"}
// optgroup
// DUPLICATE
//baseit! {label, "label"}
// option
baseit! {selected, "selected"}
// script
baseit! {defer, "defer"}
baseit! {integrity, "integrity"}
baseit! {nomodule, "nomodule"}
// select
baseit! {size, "size"}
// style
baseit! {nonce, "nonce"}
// table
baseit! {colspan, "colspan"}
// textarea
baseit! {cols, "cols"}
baseit! {rows, "rows"}
// td
baseit! {rowspan, "rowspan"}
// textarea
baseit! {wrap, "wrap"}
// track
baseit! {default, "default"}
baseit! {srclang, "srclang"}
// th
baseit! {headers, "headers"}
baseit! {scope, "scope"}
// video
baseit! {playsinline, "playsinline"}
baseit! {poster, "poster"}

// SELECTORS
baseit!(a, "a");
baseit!(abbr, "abbr");
baseit!(address, "address");
baseit!(area, "area");
baseit!(article, "article");
baseit!(aside, "aside");
baseit!(audio, "audio");
baseit!(b, "b");
baseit!(base, "base");
baseit!(bdi, "bdi");
baseit!(bdo, "bdo");
baseit!(blockquote, "blockquote");
baseit!(body, "body");
baseit!(br, "br");
baseit!(button, "button");
baseit!(canvas, "canvas");
baseit!(caption, "caption");
baseit!(cite, "cite");
baseit!(code, "code");
baseit!(col, "col");
baseit!(colgroup, "colgroup");
baseit!(data, "data");
baseit!(datalist, "datalist");
baseit!(dd, "dd");
baseit!(del, "del");
baseit!(details, "details");
baseit!(dfn, "dfn");
baseit!(dialog, "dialog");
baseit!(div, "div");
baseit!(dl, "dl");
baseit!(dt, "dt");
baseit!(em, "em");
baseit!(embed, "embed");
baseit!(fieldset, "fieldset");
baseit!(figcaption, "figcaption");
baseit!(figure, "figure");
baseit!(footer, "footer");
baseit!(form, "form");
baseit!(h1, "h1");
baseit!(h2, "h2");
baseit!(h3, "h3");
baseit!(h4, "h4");
baseit!(h5, "h5");
baseit!(h6, "h6");
baseit!(head, "head");
baseit!(header, "header");
baseit!(hr, "hr");
baseit!(html, "html");
baseit!(i, "i");
baseit!(iframe, "iframe");
baseit!(img, "img");
baseit!(input, "input");
baseit!(ins, "ins");
baseit!(kbd, "kbd");
baseit!(label, "label");
baseit!(legend, "legend");
baseit!(li, "li");
baseit!(link, "link");
baseit!(main, "main");
baseit!(map, "map");
baseit!(mark, "mark");
baseit!(menu, "menu");
baseit!(meta, "meta");
baseit!(meter, "meter");
baseit!(nav, "nav");
baseit!(noscript, "noscript");
baseit!(object, "object");
baseit!(ol, "ol");
baseit!(optgroup, "optgroup");
baseit!(option, "option");
baseit!(output, "output");
baseit!(p, "p");
baseit!(picture, "picture");
baseit!(pre, "pre");
baseit!(progress, "progress");
baseit!(q, "q");
baseit!(rp, "rp");
baseit!(rt, "rt");
baseit!(ruby, "ruby");
baseit!(s, "s");
baseit!(samp, "samp");
baseit!(script, "script");
baseit!(section, "section");
baseit!(select, "select");
baseit!(small, "small");
baseit!(source, "source");
// DUPLICATE
//baseit!(span, "span");
baseit!(strong, "strong");
// DUPLICATE
//baseit!(style, "style");
baseit!(sub, "sub");
baseit!(summary, "summary");
baseit!(sup, "sup");
baseit!(table, "table");
baseit!(tbody, "tbody");
baseit!(td, "td");
baseit!(template, "template");
baseit!(textarea, "textarea");
baseit!(tfoot, "tfoot");
baseit!(th, "th");
baseit!(thead, "thead");
baseit!(time, "time");
// DUPLICATE
//baseit!(title, "title");
baseit!(tr, "tr");
baseit!(track, "track");
baseit!(u, "u");
baseit!(ul, "ul");
baseit!(var, "var");
baseit!(video, "video");
baseit!(wbr, "wbr");

baseit!(_webkit_line_clamp, "-webkit-line-clamp");
baseit!(_webkit_text_fill_color, "-webkit-text-fill-color");
baseit!(_webkit_text_stroke, "-webkit-text-stroke");
baseit!(_webkit_text_stroke_color, "-webkit_text_stroke_color");
baseit!(_webkit_text_stroke_width, "-webkit-text-stroke-width");
baseit!(accent_color, "accent-color");
baseit!(align_content, "align-content");
baseit!(align_items, "align-items");
baseit!(align_self, "align-self");
baseit!(all, "all");
baseit!(animation, "animation");
baseit!(animation_delay, "animation-delay");
baseit!(animation_direction, "animation-direction");
baseit!(animation_duration, "animation-duration");
baseit!(animation_fill_mode, "animation-fill-mode");
baseit!(animation_iteration_count, "animation-iteration-count");
baseit!(animation_name, "animation-name");
baseit!(animation_play_state, "animation-play-state");
baseit!(animation_timing_function, "animation-timing-function");
baseit!(appearance, "appearance");
baseit!(aspect_ration, "aspect-ration");
baseit!(backdrop_filter, "backdrop-filter");
baseit!(backface_visibility, "backface-visibility");
baseit!(background, "background");
baseit!(background_attachment, "background-attachment");
baseit!(background_blend_mode, "background-blend-mode");
baseit!(background_clip, "background-clip");
baseit!(background_color, "background-color");
baseit!(background_image, "background-image");
baseit!(background_origin, "background-origin");
baseit!(background_position, "background-position");
baseit!(background_position_x, "background-position-x");
baseit!(background_position_y, "background-position-y");
baseit!(background_repeat, "background-repeat");
baseit!(background_size, "background-size");
baseit!(block_size, "block-size");
baseit!(border, "border");
baseit!(border_block, "border-block");
baseit!(border_block_color, "border-block-color");
baseit!(border_block_end, "border-block-end");
baseit!(border_block_end_color, "border-block-end-color");
baseit!(border_block_end_style, "border-block-end-style");
baseit!(border_block_end_width, "border-block-end-width");
baseit!(border_block_start, "border-block-start");
baseit!(border_block_start_color, "border-block-start-color");
baseit!(border_block_start_style, "border-block-start-style");
baseit!(border_block_start_width, "border-block-start-width");
baseit!(border_block_style, "border-block-style");
baseit!(border_block_width, "border-block-width");
baseit!(border_bottom, "border-bottom");
baseit!(border_bottom_color, "border-bottom-color");
baseit!(border_bottom_left_radius, "border-bottom-left-radius");
baseit!(border_bottom_right_radius, "border-bottom-right-radius");
baseit!(border_bottom_style, "border-bottom-style");
baseit!(border_bottom_width, "border-bottom-width");
baseit!(border_collapse, "border-collapse");
baseit!(border_color, "border-color");
baseit!(border_end_end_radius, "border-end-end-radius");
baseit!(border_end_start_radius, "border-end-start-radius");
baseit!(border_image, "border-image");
baseit!(border_image_outset, "border-image-outset");
baseit!(border_image_repeat, "border-image-repeat");
baseit!(border_image_slice, "border-image-slice");
baseit!(border_image_source, "border-image-source");
baseit!(border_image_width, "border-image-width");
baseit!(border_inline, "border-inline");
baseit!(border_inline_color, "border-inline-color");
baseit!(border_inline_end, "border-inline-end");
baseit!(border_inline_end_color, "border-inline-end-color");
baseit!(border_inline_end_style, "border-inline-end-style");
baseit!(border_inline_end_width, "border-inline-end-width");
baseit!(border_inline_start, "border-inline-start");
baseit!(border_inline_start_color, "border-inline-start-color");
baseit!(border_inline_start_style, "border-inline-start-style");
baseit!(border_inline_start_width, "border-inline-start-width");
baseit!(border_inline_style, "border-inline-style");
baseit!(border_inline_width, "border-inline-width");
baseit!(border_left, "border-left");
baseit!(border_left_color, "border-left-color");
baseit!(border_left_style, "border-left-style");
baseit!(border_left_width, "border-left-width");
baseit!(border_radius, "border-radius");
baseit!(border_right, "border-right");
baseit!(border_right_color, "border-right-color");
baseit!(border_right_style, "border-right-style");
baseit!(border_right_wdith, "border-right-wdith");
baseit!(border_spacing, "border-spacing");
baseit!(border_start_end_radius, "border-start-end-radius");
baseit!(border_start_start_radius, "border-start-start-radius");
baseit!(border_style, "border-style");
baseit!(border_top, "border-top");
baseit!(border_top_color, "border-top-color");
baseit!(border_top_left_radius, "border-top-left-radius");
baseit!(border_top_right_radius, "border-top-right-radius");
baseit!(border_top_style, "border-top-style");
baseit!(border_top_width, "border-top-width");
baseit!(border_width, "border-width");
baseit!(bottom, "bottom");
baseit!(break_after, "break-after");
baseit!(break_before, "break-before");
baseit!(break_inside, "break-inside");
baseit!(caption_side, "caption-side");
baseit!(caret_color, "caret-color");
baseit!(clear, "clear");
baseit!(clip_path, "clip-path");
baseit!(color, "color");
baseit!(color_scheme, "color-scheme");
baseit!(column_count, "column-count");
baseit!(column_fill, "column-fill");
baseit!(column_gap, "column-gap");
baseit!(column_rule, "column-rule");
baseit!(column_rule_color, "column-rule-color");
baseit!(column_rule_style, "column-rule-style");
baseit!(column_rule_width, "column-rule-width");
baseit!(column_span, "column-span");
baseit!(column_width, "column-width");
baseit!(columns, "columns");
baseit!(container, "container");
baseit!(container_name, "container-name");
baseit!(container_type, "container-type");
baseit!(counter_increment, "counter-increment");
baseit!(counter_reset, "counter-reset");
baseit!(counter_set, "counter-set");
// DUPLICATE
// baseit!(content, "content");
baseit!(content_visibility, "content-visibility");
baseit!(cursor, "cursor");
baseit!(direction, "direction");
baseit!(display, "display");
baseit!(empty_cells, "empty-cells");
baseit!(filter, "filter");
baseit!(flex, "flex");
baseit!(flex_basis, "flex-basis");
baseit!(flex_direction, "flex-direction");
baseit!(flex_flow, "flex-flow");
baseit!(flex_grow, "flex-grow");
baseit!(flex_shrink, "flex-shrink");
baseit!(flex_wrap, "flex-wrap");
baseit!(float, "float");
baseit!(font, "font");
baseit!(font_family, "font-family");
baseit!(font_feature_setting, "font-feature-setting");
baseit!(font_kerning, "font-kerning");
baseit!(font_language_override, "font-language-override");
baseit!(font_optical_sizing, "font-optical-sizing");
baseit!(font_palette, "font-palette");
baseit!(font_size, "font-size");
baseit!(font_size_adjust, "font-size-adjust");
baseit!(font_stretch, "font-stretch");
baseit!(font_style, "font-style");
baseit!(font_synthesis, "font-synthesis");
baseit!(font_variant, "font-variant");
baseit!(font_variant_alternatives, "font-variant-alternatives");
baseit!(font_variant_caps, "font-variant-caps");
baseit!(font_variant_east_asian, "font-variant-east-asian");
baseit!(font_variant_emoji, "font-variant-emoji");
baseit!(font_variant_ligatures, "font-variant-ligatures");
baseit!(font_variant_numeric, "font-variant-numeric");
baseit!(font_variant_position, "font-variant-position");
baseit!(font_variant_settings, "font-variant-settings");
baseit!(font_weight, "font-weight");
baseit!(forced_color_adjust, "forced-color-adjust");
baseit!(gap, "gap");
baseit!(grid, "grid");
baseit!(grid_area, "grid-area");
baseit!(grid_auto_columns, "grid-auto-columns");
baseit!(grid_auto_flow, "grid-auto-flow");
baseit!(grid_auto_rows, "grid-auto-rows");
baseit!(grid_column, "grid-column");
baseit!(grid_column_end, "grid-column-end");
baseit!(grid_column_start, "grid-column-start");
baseit!(grid_row, "grid-row");
baseit!(grid_row_end, "grid-row-end");
baseit!(grid_row_start, "grid-row-start");
baseit!(grid_template, "grid-template");
baseit!(grid_template_areas, "grid-template-areas");
baseit!(grid_template_columns, "grid-template-columns");
baseit!(grid_template_rows, "grid-template-rows");
baseit!(hanging_punctuation, "hanging-punctuation");
baseit!(hyphenate_character, "hyphenate-character");
baseit!(hyphenate_limit_chars, "hyphenate-limit-chars");
baseit!(hyphens, "hyphens");
baseit!(inline_size, "inline-size");
baseit!(image_orientation, "image-orientation");
baseit!(image_rendering, "image-rendering");
baseit!(isolation, "isolation");
baseit!(inset, "inset");
baseit!(inset_block, "inset-block");
baseit!(inset_block_end, "inset-block-end");
baseit!(inset_block_start, "inset-block-start");
baseit!(inset_inline, "inset-inline");
baseit!(inset_inline_end, "inset-inline-end");
baseit!(inset_inline_start, "inset-inline-start");
baseit!(justify_content, "justify-content");
baseit!(justify_items, "justify-items");
baseit!(justify_self, "justify-self");
baseit!(left, "left");
baseit!(letter_spacing, "letter-spacing");
baseit!(line_break, "line-break");
baseit!(line_height, "line-height");
baseit!(list_style, "list-style");
baseit!(list_style_image, "list-style-image");
baseit!(list_style_position, "list-style-position");
baseit!(list_style_type, "list-style-type");
baseit!(margin, "margin");
baseit!(margin_block, "margin-block");
baseit!(margin_block_end, "margin-block-end");
baseit!(margin_block_start, "margin-block-start");
baseit!(margin_bottom, "margin-bottom");
baseit!(margin_inline, "margin-inline");
baseit!(margin_inline_end, "margin-inline-end");
baseit!(margin_inline_start, "margin-inline-start");
baseit!(margin_left, "margin-left");
baseit!(margin_right, "margin-right");
baseit!(margin_top, "margin-top");
baseit!(mask, "mask");
baseit!(mask_border, "mask-border");
baseit!(mask_border_mode, "mask-border-mode");
baseit!(mask_border_outset, "mask-border-outset");
baseit!(mask_border_repeat, "mask-border-repeat");
baseit!(mask_border_slice, "mask-border-slice");
baseit!(mask_border_source, "mask-border-source");
baseit!(mask_border_width, "mask-border-width");
baseit!(mask_clip, "mask-clip");
baseit!(mask_composite, "mask-composite");
baseit!(mask_image, "mask-image");
baseit!(mask_mode, "mask-mode");
baseit!(mask_origin, "mask-origin");
baseit!(mask_position, "mask-position");
baseit!(mask_repeat, "mask-repeat");
baseit!(mask_size, "mask-size");
baseit!(mask_type, "mask-type");
baseit!(math_style, "math-style");
baseit!(max_block_size, "max-block-size");
baseit!(max_height, "max-height");
baseit!(max_inline_size, "max-inline-size");
baseit!(max_width, "max-width");
baseit!(min_block_size, "min-block-size");
baseit!(min_height, "min-height");
baseit!(min_inline_size, "min-inline-size");
baseit!(min_width, "min-width");
baseit!(offset, "offset");
baseit!(offset_anchar, "offset-anchar");
baseit!(offset_distance, "offset-distance");
baseit!(offset_path, "offset-path");
baseit!(offset_rotate, "offset-rotate");
baseit!(mix_blend_mode, "mix-blend-mode");
baseit!(object_fit, "object-fit");
baseit!(object_position, "object-position");
baseit!(opacity, "opacity");
baseit!(order, "order");
baseit!(orphans, "orphans");
baseit!(outline, "outline");
baseit!(outline_color, "outline-color");
baseit!(outline_offset, "outline-offset");
baseit!(outline_style, "outline-style");
baseit!(outline_width, "outline-width");
baseit!(overflow, "overflow");
baseit!(overflow_anchor, "overflow-anchor");
baseit!(overflow_block, "overflow-block");
baseit!(overflow_clip_margin, "overflow-clip-margin");
baseit!(overflow_inline, "overflow-inline");
baseit!(overflow_wrap, "overflow-wrap");
baseit!(overflow_x, "overflow-x");
baseit!(overflow_y, "overflow-y");
baseit!(overscoll_behavior, "overscoll-behavior");
baseit!(overscoll_behavior_block, "overscoll-behavior-block");
baseit!(overscoll_behavior_inline, "overscoll-behavior-inline");
baseit!(overscoll_behavior_x, "overscoll-behavior-x");
baseit!(overscoll_behavior_y, "overscoll-behavior-y");
baseit!(padding, "padding");
baseit!(padding_block, "padding-block");
baseit!(padding_block_end, "padding-block-end");
baseit!(padding_block_start, "padding-block-start");
baseit!(padding_bottom, "padding-bottom");
baseit!(padding_inline, "padding-inline");
baseit!(padding_inline_end, "padding-inline-end");
baseit!(padding_inline_start, "padding-inline-start");
baseit!(padding_left, "padding-left");
baseit!(padding_right, "padding-right");
baseit!(padding_top, "padding-top");
baseit!(page_break_after, "page-break-after");
baseit!(page_break_before, "page-break-before");
baseit!(page_break_inside, "page-break-inside");
baseit!(paint_order, "paint-order");
baseit!(perspective, "perspective");
baseit!(perspective_origin, "perspective-origin");
baseit!(place_content, "place-content");
baseit!(place_items, "place-items");
baseit!(place_self, "place-self");
baseit!(pointer_events, "pointer-events");
baseit!(position, "position");
baseit!(print_color_adjust, "print-color-adjust");
baseit!(quotes, "quotes");
baseit!(resize, "resize");
baseit!(right, "right");
baseit!(rotate, "rotate");
baseit!(row_gap, "row-gap");
baseit!(ruby_position, "ruby-position");
baseit!(scale, "scale");
baseit!(scroll_behavior, "scroll-behavior");
baseit!(scroll_margin, "scroll-margin");
baseit!(scroll_margin_block, "scroll-margin-block");
baseit!(scroll_margin_block_end, "scroll-margin-block-end");
baseit!(scroll_margin_block_start, "scroll-margin-block-start");
baseit!(scroll_margin_bottom, "scroll-margin-bottom");
baseit!(scroll_margin_inline, "scroll-margin-inline");
baseit!(scroll_margin_inline_end, "scroll-margin-inline-end");
baseit!(scroll_margin_inline_start, "scroll-margin-inline-start");
baseit!(scroll_margin_left, "scroll-margin-left");
baseit!(scroll_margin_right, "scroll-margin-right");
baseit!(scroll_margin_top, "scroll-margin-top");
baseit!(scroll_padding, "scroll-padding");
baseit!(scroll_padding_block, "scroll-padding-block");
baseit!(scroll_padding_block_end, "scroll-padding-block-end");
baseit!(scroll_padding_block_start, "scroll-padding-block-start");
baseit!(scroll_padding_bottom, "scroll-padding-bottom");
baseit!(scroll_padding_inline, "scroll-padding-inline");
baseit!(scroll_padding_inline_end, "scroll-padding-inline-end");
baseit!(scroll_padding_inline_start, "scroll-padding-inline-start");
baseit!(scroll_padding_left, "scroll-padding-left");
baseit!(scroll_padding_right, "scroll-padding-right");
baseit!(scroll_padding_top, "scroll-padding-top");
baseit!(scroll_snap_align, "scroll-snap-align");
baseit!(scroll_snap_stop, "scroll-snap-stop");
baseit!(scroll_snap_type, "scroll-snap-type");
baseit!(scrollbar_color, "scrollbar-color");
baseit!(scrollbar_gutter, "scrollbar-gutter");
baseit!(scrollbar_width, "scrollbar-width");
baseit!(shape_image_threshold, "shape-image-threshold");
baseit!(shape_margin, "shape-margin");
baseit!(shape_outside, "shape-outside");
baseit!(tab_size, "tab-size");
baseit!(table_layout, "table-layout");
baseit!(text_align, "text-align");
baseit!(text_align_last, "text-align-last");
baseit!(text_combine_upright, "text-combine-upright");
baseit!(text_decoration, "text-decoration");
baseit!(text_decoration_color, "text-decoration-color");
baseit!(text_decoration_line, "text-decoration-line");
baseit!(text_decoration_skip_ink, "text-decoration-skip-ink");
baseit!(text_decoration_style, "text-decoration-style");
baseit!(text_decoration_thickness, "text-decoration-thickness");
baseit!(text_emphasis, "text-emphasis");
baseit!(text_emphasis_color, "text-emphasis-color");
baseit!(text_emphasis_position, "text-emphasis-position");
baseit!(text_emphasis_style, "text-emphasis-style");
baseit!(text_indent, "text-indent");
baseit!(text_justify, "text-justify");
baseit!(text_orientation, "text-orientation");
baseit!(text_overflow, "text-overflow");
baseit!(text_rendering, "text-rendering");
baseit!(text_shadow, "text-shadow");
baseit!(text_transform, "text-transform");
baseit!(text_underline_offset, "text-underline-offset");
baseit!(text_underline_position, "text-underline-position");
baseit!(top, "top");
baseit!(touch_action, "touch-action");
baseit!(transform, "transform");
baseit!(transform_box, "transform-box");
baseit!(transform_origin, "transform-origin");
baseit!(transform_style, "transform-style");
baseit!(transition, "transition");
baseit!(transition_delay, "transition-delay");
baseit!(transition_duration, "transition-duration");
baseit!(transition_property, "transition-property");
baseit!(transition_timing_function, "transition-timing-function");
// DUPLICATE
// baseit!(translate, "translate");
baseit!(unicode_bidi, "unicode-bidi");
baseit!(user_select, "user-select");
baseit!(vertical_align, "vertical-align");
baseit!(visibility, "visibility");
baseit!(white_space, "white-space");
baseit!(widows, "widows");
// DUPLICATE
// baseit!(width, "width");
baseit!(will_change, "will-change");
baseit!(word_break, "word-break");
baseit!(word_spacing, "word-spacing");
baseit!(writing_mode, "writing-mode");
baseit!(z_index, "z-index");

baseit!(active, "active");
baseit!(any_link, "any-link");
baseit!(autofill, "autofill");
// DUPLICATE
//baseit!(checked, "checked");
baseit!(current, "current");
// DUPLICATE
// baseit!(default, "default");
baseit!(defined, "defined");
// DUPLICATE
// baseit!(dir, "dir");
// DUPLICATE
// baseit!(disabled, "disabled");
baseit!(empty, "empty");
baseit!(enabled, "enabled");
baseit!(first, "first");
baseit!(first_child, "first-child");
baseit!(first_of_type, "first-of-type");
baseit!(focus, "focus");
baseit!(focus_visible, "focus-visible");
baseit!(focus_within, "focus-within");
baseit!(fullscreen, "fullscreen");
baseit!(future, "future");
baseit!(has, "has");
baseit!(hover, "hover");
baseit!(in_range, "in-range");
baseit!(indeterminate, "indeterminate");
baseit!(invalid, "invalid");
baseit!(is, "is");
// DUPLICATE baseit!(lang, "lang");
baseit!(last_child, "last-child");
baseit!(last_of_type, "last-of-type");
// DUPLICATE
//baseit!(left, "left");
// DUPLICATE
//baseit!(link, "link");
baseit!(local_link, "local-link");
baseit!(modal, "modal");
baseit!(not, "not");
baseit!(nth_child, "nth-child");
baseit!(nth_col, "nth-col");
baseit!(nth_last_child, "nth-last-child");
baseit!(nth_last_col, "nth-last-col");
baseit!(nth_last_of_type, "nth-last-of-type");
baseit!(nth_of_type, "nth-of-type");
baseit!(only_child, "only-child");
baseit!(only_of_type, "only-of-type");
baseit!(optional, "optional");
baseit!(out_of_range, "out-of-range");
baseit!(past, "past");
baseit!(paused, "paused");
baseit!(picture_in_picture, "picture-in-picture");
baseit!(placeholder_shown, "placeholder-shown");
baseit!(playing, "playing");
baseit!(read_only, "read-only");
baseit!(read_write, "read-write");
// DUPLICATE
//baseit!(required, "required");
//baseit!(right, "right");
baseit!(root, "root");
// DUPLICATE
//baseit!(scope, "scope");
//baseit!(target, "target");
baseit!(target_within, "target-within");
baseit!(valid, "valid");
baseit!(visited, "valid");
baseit!(_where, "where");

baseit!(after, "after");
baseit!(backdrop, "backdrop");
baseit!(before, "before");
baseit!(cue, "cue");
baseit!(cue_region, "cue-region");
baseit!(file_selector_button, "file-selector-button");
baseit!(first_letter, "first-letter");
baseit!(first_line, "first-line");
baseit!(marker, "marker");
baseit!(part, "part");
// DUPLICATE
// baseit!(placeholder, "placeholder");
baseit!(selection, "selection");
baseit!(slotted, "slotted");
