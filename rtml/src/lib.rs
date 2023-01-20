pub mod attributes;
pub mod macros;
pub mod tags;

use attributes::*;
use std::fmt;
use tags::Tag;

pub trait Render: ToString {
    fn render(&self) -> String {
        self.to_string()
    }
}

impl Render for &'static str {}

impl Render for fmt::Arguments<'_> {
    fn render(&self) -> String {
        format!("{}", self)
    }
}

macro_rules! globalattributeit {
    ($attr:ident, $val:expr) => {
        #[allow(non_camel_case_types)]
        pub struct $attr;
        impl fmt::Display for $attr {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                return write!(f, "{}", $val);
            }
        }
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

macro_rules! attributeit {
    ($attr:ident, $val:expr) => {
        #[allow(non_camel_case_types)]
        pub struct $attr;
        impl fmt::Display for $attr {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                return write!(f, "{}", $val);
            }
        }
        impl Attribute for $attr {}
    };
}

macro_rules! tagit {
    ($tag:ident, $val:expr, $trait:ident $(,$attr:ident)*) => {
        pub struct $tag;
        impl fmt::Display for $tag {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                return write!(f, "{}", $val);
            }
        }
        #[allow(unused_variables)]
        impl $tag {
            pub fn type_check(self, attr: &dyn $trait){}
        }
        #[warn(unused_variables)]
        pub trait $trait {}
        impl Tag for $tag {}
        $(impl $trait for $attr {})*
    };
}

// need a special dataAttr as its attribute name is dynamic
#[allow(non_camel_case_types)]
struct data(String);
impl fmt::Display for data {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{}-{}=\"", "data", self.0);
    }
}
impl Attribute for data {}

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
attributeit! {type_, "type"}
attributeit! {loop_, "loop"}
attributeit! {for_, "for"}
attributeit! {http_equiv, "http-equiv"}
attributeit! {accept_charset, "accept-charset"}
attributeit! {as_, "as"}
attributeit! {async_, "async"}
attributeit! {kind_, "kind"}

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

tagit! {ATag, "a", ACompat, download, href, hreflang, media, ping, referrerpolicy, rel, target }
tagit! {AbbrTag, "abbr", AbbrCompat}
tagit! {AddressTag, "address", AddressCompat}
tagit! {AreaTag, "area", AreaCompat, alt, coords, download, href, hreflang, media, rel, shape, target }
tagit! {ArticleTag, "article", ArticleCompat}
tagit! {AsideTag, "aside", AsideCompat}
tagit! {AudioTag, "audio", AudioCompat, autoplay, controls, loop_, muted, preload, src }
tagit! {BTag, "b", BCompat}
tagit! {BaseTag, "base", BaseCompat, href, target }
tagit! {BdiTag, "bdi", BdiCompat}
tagit! {BdoTag, "bdo", BdoCompat}
tagit! {BlockquoteTag, "blockquote", BlockquoteCompat, cite }
tagit! {BodyTag, "body", BodyCompat}
tagit! {BrTag, "br", BrCompat}
tagit! {ButtonTag, "button", ButtonCompat, autofocus, disabled, form, formaction, formenctype, formmethod, formnovalidate, formtarget, name, type_, value }
tagit! {CanvasTag, "canvas", CanvasCompat, height, width }
tagit! {CaptionTag, "caption", CaptionCompat}
tagit! {CiteTag, "cite", CiteCompat}
tagit! {CodeTag, "code", CodeCompat}
tagit! {ColTag, "col", ColCompat, span }
tagit! {ColgroupTag, "colgroup", ColgroupCompat, span }
tagit! {DataTag, "data", DataCompat, value }
tagit! {DatalistTag, "datalist", DatalistCompat}
tagit! {DdTag, "dd", DdCompat}
tagit! {DelTag, "del", DelCompat, cite, datetime }
tagit! {DetailsTag, "details", DetailsCompat, open }
tagit! {DfnTag, "dfn", DfnCompat}
tagit! {DialogTag, "dialog", DialogCompat, open }
tagit! {DivTag, "div", DivCompat}
tagit! {DlTag, "dl", DlCompat}
tagit! {DtTag, "dt", DtCompat}
tagit! {EmTag, "em", EmCompat}
tagit! {EmbedTag, "embed", EmbedCompat, height, src, type_, width }
tagit! {FieldsetTag, "fieldset", FieldsetCompat, disabled, form, name }
tagit! {FigcaptionTag, "figcaption", FigcaptionCompat}
tagit! {FigureTag, "figure", FigureCompat}
tagit! {FooterTag, "footer", FooterCompat}
tagit! {FormTag, "form", FormCompat, accept_charset, action, autocomplete, enctype, method, name, novalidate, target }
tagit! {H1Tag, "h1", H1Compat}
tagit! {H2Tag, "h2", H2Compat}
tagit! {H3Tag, "h3", H3Compat}
tagit! {H4Tag, "h4", H4Compat}
tagit! {H5Tag, "h5", H5Compat}
tagit! {H6Tag, "h6", H6Compat}
tagit! {HeadTag, "head", HeadCompat}
tagit! {HeaderTag, "header", HeaderCompat}
tagit! {HrTag, "hr", HrCompat}
tagit! {HtmlTag, "html", HtmlCompat}
tagit! {ITag, "i", ICompat}
tagit! {IframeTag, "iframe", IframeCompat, allow, allowfullscreen, height, name, referrerpolicy, sandbox, src, srcdoc, width }
tagit! {ImgTag, "img", ImgCompat, alt, crossorigin, decoding, height, referrerpolicy, sizes, src, srcset, usemap, width }
tagit! {InputTag, "input", InputCompat, accept, alt, autocomplete, autofocus, checked, disabled, form, formaction, formenctype, formmethod, formnovalidate, formtarget, height, list, max, maxlength, min, minlength, multiple, name, pattern, placeholder, readonly, required, size, src, step, type_, value, width }
tagit! {InsTag, "ins", InsCompat, cite, datetime }
tagit! {KbdTag, "kbd", KbdCompat}
tagit! {LabelTag, "label", LabelCompat, form }
tagit! {LegendTag, "legend", LegendCompat}
tagit! {LiTag, "li", LiCompat, value }
tagit! {LinkTag, "link", LinkCompat, as_, crossorigin, href, hreflang, media, rel, sizes, type_ }
tagit! {MainTag, "main", MainCompat}
tagit! {MapTag, "map", MapCompat, name }
tagit! {MarkTag, "mark", MarkCompat}
tagit! {MenuTag, "menu", MenuCompat, type_ }
tagit! {MetaTag, "meta", MetaCompat, charset, content, http_equiv, name }
tagit! {MeterTag, "meter", MeterCompat, high, low, max, min, optimum, value }
tagit! {NavTag, "nav", NavCompat}
tagit! {NoscriptTag, "noscript", NoscriptCompat}
tagit! {ObjectTag, "object", ObjectCompat, data, form, height, name, type_, width }
tagit! {OlTag, "ol", OlCompat, reversed, start }
tagit! {OptgroupTag, "optgroup", OptgroupCompat, disabled, label }
tagit! {OptionTag, "option", OptionCompat, disabled, label, selected, value }
tagit! {OutputTag, "output", OutputCompat, for_, form, name }
tagit! {PTag, "p", PCompat}
tagit! {PictureTag, "picture", PictureCompat}
tagit! {PreTag, "pre", PreCompat}
tagit! {ProgressTag, "progress", ProgressCompat, max, value }
tagit! {QTag, "q", QCompat, cite }
tagit! {RpTag, "rp", RpCompat}
tagit! {RtTag, "rt", RtCompat}
tagit! {RubyTag, "ruby", RubyCompat}
tagit! {STag, "s", SCompat}
tagit! {SampTag, "samp", SampCompat}
tagit! {ScriptTag, "script", ScriptCompat, async_, crossorigin, defer, integrity, nomodule, src, type_ }
tagit! {SectionTag, "section", SectionCompat}
tagit! {SelectTag, "select", SelectCompat, autocomplete, autofocus, disabled, form, multiple, name, required, size }
tagit! {SmallTag, "small", SmallCompat}
tagit! {SourceTag, "source", SourceCompat, media, sizes, src, srcset, type_ }
tagit! {SpanTag, "span", SpanCompat}
tagit! {StrongTag, "strong", StrongCompat}
tagit! {StyleTag, "style", StyleCompat, media, nonce, type_ }
tagit! {SubTag, "sub", SubCompat}
tagit! {SummaryTag, "summary", SummaryCompat}
tagit! {SupTag, "sup", SupCompat}
tagit! {TableTag, "table", TableCompat}
tagit! {TbodyTag, "tbody", TbodyCompat}
tagit! {TdTag, "td", TdCompat, colspan, headers, rowspan, scope }
tagit! {TemplateTag, "template", TemplateCompat}
tagit! {TextareaTag, "textarea", TextareaCompat, autocomplete, autofocus, cols, disabled, form, maxlength, minlength, name, placeholder, readonly, required, rows, wrap }
tagit! {TfootTag, "tfoot", TfootCompat}
tagit! {ThTag, "th", ThCompat, colspan, headers, rowspan, scope }
tagit! {TheadTag, "thead", TheadCompat}
tagit! {TimeTag, "time", TimeCompat, datetime }
tagit! {TitleTag, "title", TitleCompat}
tagit! {TrTag, "tr", TrCompat}
tagit! {TrackTag, "track", TrackCompat, default, kind_, label, src, srclang }
tagit! {UTag, "u", UCompat}
tagit! {ULTag, "ul", ULCompat}
tagit! {VarTag, "var", VarCompat}
tagit! {VideoTag, "video", VideoCompat, autoplay, controls, crossorigin, height, loop_, muted, playsinline, poster, preload, src, width }
tagit! {WbrTag, "wbr", WbrCompat}
