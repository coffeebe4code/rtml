use crate::*;
use std::fmt;
use std::fmt::Display;

pub trait GlobalAttribute {}
pub trait EventAttribute {}

pub trait Attribute: Display + 'static {}

#[macro_export]
macro_rules! allattrs {
    ($attr:ident) => {
        impl ACompat for $attr {}
        impl AbbrCompat for $attr {}
        impl AddressCompat for $attr {}
        impl AreaCompat for $attr {}
        impl ArticleCompat for $attr {}
        impl AsideCompat for $attr {}
        impl AudioCompat for $attr {}
        impl BCompat for $attr {}
        impl BaseCompat for $attr {}
        impl BdiCompat for $attr {}
        impl BdoCompat for $attr {}
        impl BlockquoteCompat for $attr {}
        impl BodyCompat for $attr {}
        impl BrCompat for $attr {}
        impl ButtonCompat for $attr {}
        impl CanvasCompat for $attr {}
        impl CaptionCompat for $attr {}
        impl CiteCompat for $attr {}
        impl CodeCompat for $attr {}
        impl ColCompat for $attr {}
        impl ColgroupCompat for $attr {}
        impl DataCompat for $attr {}
        impl DatalistCompat for $attr {}
        impl DdCompat for $attr {}
        impl DelCompat for $attr {}
        impl DetailsCompat for $attr {}
        impl DfnCompat for $attr {}
        impl DialogCompat for $attr {}
        impl DivCompat for $attr {}
        impl DlCompat for $attr {}
        impl DtCompat for $attr {}
        impl EmCompat for $attr {}
        impl EmbedCompat for $attr {}
        impl FieldsetCompat for $attr {}
        impl FigcaptionCompat for $attr {}
        impl FigureCompat for $attr {}
        impl FooterCompat for $attr {}
        impl FormCompat for $attr {}
        impl H1Compat for $attr {}
        impl H2Compat for $attr {}
        impl H3Compat for $attr {}
        impl H4Compat for $attr {}
        impl H5Compat for $attr {}
        impl H6Compat for $attr {}
        impl HeadCompat for $attr {}
        impl HeaderCompat for $attr {}
        impl HrCompat for $attr {}
        impl HtmlCompat for $attr {}
        impl ICompat for $attr {}
        impl IframeCompat for $attr {}
        impl ImgCompat for $attr {}
        impl InputCompat for $attr {}
        impl InsCompat for $attr {}
        impl KbdCompat for $attr {}
        impl LabelCompat for $attr {}
        impl LegendCompat for $attr {}
        impl LiCompat for $attr {}
        impl LinkCompat for $attr {}
        impl MainCompat for $attr {}
        impl MapCompat for $attr {}
        impl MarkCompat for $attr {}
        impl MetaCompat for $attr {}
        impl MeterCompat for $attr {}
        impl NavCompat for $attr {}
        impl NoscriptCompat for $attr {}
        impl ObjectCompat for $attr {}
        impl OlCompat for $attr {}
        impl OptgroupCompat for $attr {}
        impl OptionCompat for $attr {}
        impl OutputCompat for $attr {}
        impl PCompat for $attr {}
        impl PictureCompat for $attr {}
        impl PreCompat for $attr {}
        impl ProgressCompat for $attr {}
        impl QCompat for $attr {}
        impl RpCompat for $attr {}
        impl RtCompat for $attr {}
        impl RubyCompat for $attr {}
        impl SCompat for $attr {}
        impl ScriptCompat for $attr {}
        impl SectionCompat for $attr {}
        impl SelectCompat for $attr {}
        impl SmallCompat for $attr {}
        impl SourceCompat for $attr {}
        impl SpanCompat for $attr {}
        impl StrongCompat for $attr {}
        impl StyleCompat for $attr {}
        impl SubCompat for $attr {}
        impl SummaryCompat for $attr {}
        impl SupCompat for $attr {}
        impl TableCompat for $attr {}
        impl TbodyCompat for $attr {}
        impl TdCompat for $attr {}
        impl TemplateCompat for $attr {}
        impl TextareaCompat for $attr {}
        impl TfootCompat for $attr {}
        impl ThCompat for $attr {}
        impl TheadCompat for $attr {}
        impl TimeCompat for $attr {}
        impl TitleCompat for $attr {}
        impl TrCompat for $attr {}
        impl TrackCompat for $attr {}
        impl UCompat for $attr {}
        impl VarCompat for $attr {}
        impl VideoCompat for $attr {}
        impl WbrCompat for $attr {}

        impl GlobalAttribute for $attr {}
    };
}

#[macro_export]
macro_rules! globalattributeit {
    ($attr:ident, $val:expr) => {
        paste::paste! {
        #[allow(non_camel_case_types)]
        #[derive(Clone)]
        pub struct [<$attr _>];
        impl std::fmt::Display for [<$attr _>] {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                return write!(f, "{}", $val);
            }
        }
        allattrs!([< $attr _>]);
        }
    };
}

#[macro_export]
macro_rules! make_data {
    ($attr:ident $(-$additional:ident)*) => {
        paste::paste! {
        #[allow(non_camel_case_types)]
        #[derive(Clone)]
        pub struct [<data _ $attr $(_$additional)*_>];
        impl std::fmt::Display for [<data _ $attr $(_$additional)*_>] {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                return write!(f, "{}", str::replace(stringify!([<data _ $attr $(_$additional)*>]), "_", "-"));
            }
        }
        allattrs!([< data _ $attr $(_$additional)*_>]);
        }
    };
}

macro_rules! attributeit {
    ($attr:ident, $val:expr) => {
        paste::paste! {
        #[allow(non_camel_case_types)]
        #[derive(Clone)]
        pub struct [< $attr _ >];
        impl fmt::Display for [<$attr _>] {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                return write!(f, "{}", $val);
            }
        }
        impl Attribute for [<$attr _>] {}
        }
    };
}

// Global Attributes
globalattributeit! {accesskey, "accesskey"}
globalattributeit! {class, "class"}
globalattributeit! {contenteditable, "contenteditable"}
globalattributeit! {dir, "dir"}
globalattributeit! {draggable, "draggable"}
globalattributeit! {hidden, "hidden"}
globalattributeit! {id, "id"}
globalattributeit! {lang, "lang"}
globalattributeit! {spellcheck, "spellcheck"}
globalattributeit! {style, "style"}
globalattributeit! {tabindex, "tabindex"}
globalattributeit! {title, "title"}
globalattributeit! {translate, "translate"}

// Special
attributeit! {type, "type"}
attributeit! {loop, "loop"}
attributeit! {for, "for"}
attributeit! {http_equiv, "http-equiv"}
attributeit! {accept_charset, "accept-charset"}
attributeit! {as, "as"}
attributeit! {async, "async"}
attributeit! {kind, "kind"}

// Event
// Window
attributeit! {onafterprint, "onafterprint"}
attributeit! {onbeforeprint, "onbeforeprint"}
attributeit! {onbeforeunload, "onbeforeunload"}
attributeit! {onerror, "onerror"}
attributeit! {onhashchange, "onhashchange"}
attributeit! {onload, "onload"}
attributeit! {onmessage, "onmessage"}
attributeit! {onoffline, "onoffline"}
attributeit! {ononline, "ononline"}
attributeit! {onpagehide, "onpagehide"}
attributeit! {onpageshow, "onpageshow"}
attributeit! {onpopstate, "onpopstate"}
attributeit! {onresize, "onresize"}
attributeit! {onunload, "onunload"}

// Form
attributeit! {onblur, "onblur"}
attributeit! {onchange, "onchange"}
attributeit! {oncontextmenu, "oncontextmenu"}
attributeit! {onfocus, "onfocus"}
attributeit! {oninput, "oninput"}
attributeit! {onreset, "onreset"}
attributeit! {onsearch, "onsearch"}
attributeit! {onselect, "onselect"}
attributeit! {onsubmit, "onsubmit"}

// Keyboard
attributeit! {onkeydown, "onkeydown"}
attributeit! {onkeypress, "onkeypress"}
attributeit! {onkeyup, "onkeyup"}

// Mouse
attributeit! {onclick, "onclick"}
attributeit! {ondblclick, "ondblclick"}
attributeit! {onmousedown, "onmousedown"}
attributeit! {onmousemove, "onmousemove"}
attributeit! {onmouseout, "onmouseout"}
attributeit! {onmouseover, "onmouseover"}
attributeit! {onmouseup, "onmouseup"}
attributeit! {onwheel, "onwheel"}

// Drag
attributeit! {ondrag, "ondrag"}
attributeit! {ondragend, "ondragend"}
attributeit! {ondragenter, "ondragenter"}
attributeit! {ondragleave, "ondragleave"}
attributeit! {ondragover, "ondragover"}
attributeit! {ondragstart, "ondragstart"}
attributeit! {ondrop, "ondrop"}
attributeit! {onscroll, "onscroll"}

// Clipboard
attributeit! {oncopy, "oncopy"}
attributeit! {oncut, "oncut"}
attributeit! {onpaste, "onpaste"}

// Media
attributeit! {onabort, "onabort"}
attributeit! {oncanplay, "oncanplay"}
attributeit! {oncanplaythrough, "oncanplaythrough"}
attributeit! {oncuechange, "oncuechange"}
attributeit! {ondurationchange, "ondurationchange"}
attributeit! {onemptied, "onemptied"}
attributeit! {onended, "onended"}
attributeit! {onloadeddata, "onloadeddata"}
attributeit! {onloadedmetadata, "onloadedmetadata"}
attributeit! {onloadstart, "onloadstart"}
attributeit! {onpause, "onpause"}
attributeit! {onplay, "onplay"}
attributeit! {onplaying, "onplaying"}
attributeit! {onprogress, "onprogress"}
attributeit! {onratechange, "onratechange"}
attributeit! {onseeked, "onseeked"}
attributeit! {onseeking, "onseeking"}
attributeit! {onstalled, "onstalled"}
attributeit! {onsuspend, "onsuspend"}
attributeit! {ontimeupdate, "ontimeupdate"}
attributeit! {onvolumechange, "onvolumechange"}
attributeit! {onwaiting, "onwaiting"}

// Misc
attributeit! {ontoggle, "ontoggle"}

// Specific Attributes
// a
attributeit! {href, "href"}
attributeit! {src, "src"}
attributeit! {download, "download"}
attributeit! {media, "media"}
attributeit! {ping, "ping"}
attributeit! {referrerpolicy, "referrerpolicy"}
attributeit! {rel, "rel"}
attributeit! {hreflang, "hreflang"}
attributeit! {target, "target"}
// area
attributeit! {alt, "alt"}
attributeit! {coords, "coords"}
attributeit! {shape, "shape"}
// audio
attributeit! {autoplay, "autoplay"}
attributeit! {controls, "controls"}
attributeit! {muted, "muted"}
attributeit! {preload, "preload"}
attributeit! {cite, "cite"}
// button
attributeit! {autofocus, "autofocus"}
attributeit! {disabled, "disabled"}
attributeit! {form, "form"}
attributeit! {formaction, "formaction"}
attributeit! {formenctype, "formenctype"}
attributeit! {formmethod, "formmethod"}
attributeit! {formnovalidate, "formnovalidate"}
attributeit! {formtarget, "formtarget"}
attributeit! {name, "name"}
attributeit! {value, "value"}
// canvas
attributeit! {height, "height"}
attributeit! {width, "width"}
// col
attributeit! {span, "span"}
// del
attributeit! {datetime, "datetime"}
// details
attributeit! {open, "open"}
// form
attributeit! {action, "action"}
attributeit! {autocomplete, "autocomplete"}
attributeit! {enctype, "enctype"}
attributeit! {method, "method"}
attributeit! {novalidate, "novalidate"}
// html
attributeit! {xmlns, "xmlns"}
// iframe
attributeit! {allow, "allow"}
attributeit! {allowfullscreen, "allowfullscreen"}
attributeit! {allowpaymentrequest, "allowpaymentrequest"}
attributeit! {loading, "loading"}
attributeit! {sandbox, "sandbox"}
attributeit! {srcdoc, "srcdoc"}
// img
attributeit! {crossorigin, "crossorigin"}
attributeit! {decoding, "decoding"}
attributeit! {ismap, "ismap"}
attributeit! {longdesc, "longdesc"}
attributeit! {srcset, "srcset"}
attributeit! {sizes, "sizes"}
attributeit! {usemap, "usemap"}
// input
attributeit! {accept, "accept"}
attributeit! {checked, "checked"}
attributeit! {dirname, "dirname"}
attributeit! {list, "list"}
attributeit! {max, "max"}
attributeit! {maxlength, "maxlength"}
attributeit! {min, "min"}
attributeit! {minlength, "minlength"}
attributeit! {multiple, "multiple"}
attributeit! {pattern, "pattern"}
attributeit! {placeholder, "placeholder"}
attributeit! {readonly, "readonly"}
attributeit! {required, "required"}
attributeit! {step, "step"}
// meta
attributeit! {content, "content"}
attributeit! {charset, "charset"}
// meter
attributeit! {high, "high"}
attributeit! {low, "low"}
attributeit! {optimum, "optimum"}
// object
attributeit! {data, "data"}
// ol
attributeit! {reversed, "reversed"}
attributeit! {start, "start"}
// optgroup
attributeit! {label, "label"}
// option
attributeit! {selected, "selected"}
// script
attributeit! {defer, "defer"}
attributeit! {integrity, "integrity"}
attributeit! {nomodule, "nomodule"}
// select
attributeit! {size, "size"}
// style
attributeit! {nonce, "nonce"}
// table
attributeit! {colspan, "colspan"}
// textarea
attributeit! {cols, "cols"}
attributeit! {rows, "rows"}
// td
attributeit! {rowspan, "rowspan"}
// textarea
attributeit! {wrap, "wrap"}
// track
attributeit! {default, "default"}
attributeit! {srclang, "srclang"}
// th
attributeit! {headers, "headers"}
attributeit! {scope, "scope"}
// video
attributeit! {playsinline, "playsinline"}
attributeit! {poster, "poster"}
