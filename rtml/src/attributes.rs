use crate::*;
use std::fmt;
use std::fmt::Display;

pub trait GlobalAttribute {}
pub trait EventAttribute {}

pub trait Attribute: Display + 'static {}

pub trait AttributeValue: ToString {
    fn render(&self) -> String {
        self.to_string()
    }
}

macro_rules! globalattributeit {
    ($attr:ident, $val:expr) => {
        paste::paste! {
        #[allow(non_camel_case_types)]
        #[derive(Clone)]
        pub struct [<$attr _>];
        impl fmt::Display for [<$attr _>] {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                return write!(f, "{}", $val);
            }
        }
        impl ACompat for [< $attr _>] {}
        impl AbbrCompat for [< $attr _>] {}
        impl AddressCompat for [< $attr _>] {}
        impl AreaCompat for [< $attr _>] {}
        impl ArticleCompat for [< $attr _>] {}
        impl AsideCompat for [< $attr _>] {}
        impl AudioCompat for [< $attr _>] {}
        impl BCompat for [< $attr _>] {}
        impl BaseCompat for [< $attr _>] {}
        impl BdiCompat for [< $attr _>] {}
        impl BdoCompat for [< $attr _>] {}
        impl BlockquoteCompat for [< $attr _>] {}
        impl BodyCompat for [< $attr _>] {}
        impl BrCompat for [< $attr _>] {}
        impl ButtonCompat for [< $attr _>] {}
        impl CanvasCompat for [< $attr _>] {}
        impl CaptionCompat for [< $attr _>] {}
        impl CiteCompat for [< $attr _>] {}
        impl CodeCompat for [< $attr _>] {}
        impl ColCompat for [< $attr _>] {}
        impl ColgroupCompat for [< $attr _>] {}
        impl DataCompat for [< $attr _>] {}
        impl DatalistCompat for [< $attr _>] {}
        impl DdCompat for [< $attr _>] {}
        impl DelCompat for [< $attr _>] {}
        impl DetailsCompat for [< $attr _>] {}
        impl DfnCompat for [< $attr _>] {}
        impl DialogCompat for [< $attr _>] {}
        impl DivCompat for [< $attr _>] {}
        impl DlCompat for [< $attr _>] {}
        impl DtCompat for [< $attr _>] {}
        impl EmCompat for [< $attr _>] {}
        impl EmbedCompat for [< $attr _>] {}
        impl FieldsetCompat for [< $attr _>] {}
        impl FigcaptionCompat for [< $attr _>] {}
        impl FigureCompat for [< $attr _>] {}
        impl FooterCompat for [< $attr _>] {}
        impl FormCompat for [< $attr _>] {}
        impl H1Compat for [< $attr _>] {}
        impl H2Compat for [< $attr _>] {}
        impl H3Compat for [< $attr _>] {}
        impl H4Compat for [< $attr _>] {}
        impl H5Compat for [< $attr _>] {}
        impl H6Compat for [< $attr _>] {}
        impl HeadCompat for [< $attr _>] {}
        impl HeaderCompat for [< $attr _>] {}
        impl HrCompat for [< $attr _>] {}
        impl HtmlCompat for [< $attr _>] {}
        impl ICompat for [< $attr _>] {}
        impl IframeCompat for [< $attr _>] {}
        impl ImgCompat for [< $attr _>] {}
        impl InputCompat for [< $attr _ >] {}
        impl InsCompat for [< $attr _ >] {}
        impl KbdCompat for [< $attr _ >] {}
        impl LabelCompat for [< $attr _ >] {}
        impl LegendCompat for [< $attr _ >] {}
        impl LiCompat for [< $attr _ >] {}
        impl LinkCompat for [< $attr _ >] {}
        impl MainCompat for [< $attr _ >] {}
        impl MapCompat for [< $attr _ >] {}
        impl MarkCompat for [< $attr _ >] {}
        impl MetaCompat for [< $attr _ >] {}
        impl MeterCompat for [< $attr _ >] {}
        impl NavCompat for [< $attr _ >] {}
        impl NoscriptCompat for [< $attr _ >] {}
        impl ObjectCompat for [< $attr _ >] {}
        impl OlCompat for [< $attr _ >] {}
        impl OptgroupCompat for [< $attr _ >] {}
        impl OptionCompat for [< $attr _ >] {}
        impl OutputCompat for [< $attr _ >] {}
        impl PCompat for [< $attr _ >] {}
        impl PictureCompat for [< $attr _ >] {}
        impl PreCompat for [< $attr _ >] {}
        impl ProgressCompat for [< $attr _ >] {}
        impl QCompat for [< $attr _ >] {}
        impl RpCompat for [< $attr _ >] {}
        impl RtCompat for [< $attr _ >] {}
        impl RubyCompat for [< $attr _ >] {}
        impl SCompat for [< $attr _ >] {}
        impl ScriptCompat for [< $attr _ >] {}
        impl SectionCompat for [< $attr _ >] {}
        impl SelectCompat for [< $attr _ >] {}
        impl SmallCompat for [< $attr _ >] {}
        impl SourceCompat for [< $attr _ >] {}
        impl SpanCompat for [< $attr _ >] {}
        impl StrongCompat for [< $attr _ >] {}
        impl StyleCompat for [< $attr _ >] {}
        impl SubCompat for [< $attr _ >] {}
        impl SummaryCompat for [< $attr _ >] {}
        impl SupCompat for [< $attr _ >] {}
        impl TableCompat for [< $attr _ >] {}
        impl TbodyCompat for [< $attr _ >] {}
        impl TdCompat for [< $attr _ >] {}
        impl TemplateCompat for [< $attr _ >] {}
        impl TextareaCompat for [< $attr _ >] {}
        impl TfootCompat for [< $attr _ >] {}
        impl ThCompat for [< $attr _ >] {}
        impl TheadCompat for [< $attr _ >] {}
        impl TimeCompat for [< $attr _ >] {}
        impl TitleCompat for [< $attr _ >] {}
        impl TrCompat for [< $attr _ >] {}
        impl TrackCompat for [< $attr _ >] {}
        impl UCompat for [< $attr _ >] {}
        impl VarCompat for [< $attr _ >] {}
        impl VideoCompat for [< $attr _ >] {}
        impl WbrCompat for [< $attr _ >] {}

        impl GlobalAttribute for [< $attr _ >] {}
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
