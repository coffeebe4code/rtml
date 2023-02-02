use crate::*;
use std::fmt;
use std::fmt::Display;

pub trait Tag: Display + 'static {}

pub trait TagValue: ToString {
    fn render(&self) -> String {
        return self.to_string();
    }
}

macro_rules! tagit {
    ($tag:ident, $val:expr, $trait:ident $(,$attr:ident)*) => {
        #[derive(Clone)]
        pub struct $tag;
        impl fmt::Display for $tag {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                return write!(f, "{}", $val);
            }
        }
        #[allow(unused_variables)]
        impl $tag {
            pub fn type_check(&self, attr: &dyn $trait){}
        }
        #[warn(unused_variables)]
        pub trait $trait {}
        impl Tag for $tag {}

        $(
            paste::paste! {impl $trait for [< $attr _ >] {}
            })*
    };
}

tagit! {ATag, "a", ACompat, download, href, hreflang, media, ping, referrerpolicy, rel, target }
tagit! {AbbrTag, "abbr", AbbrCompat}
tagit! {AddressTag, "address", AddressCompat}
tagit! {AreaTag, "area", AreaCompat, alt, coords, download, href, hreflang, media, rel, shape, target }
tagit! {ArticleTag, "article", ArticleCompat}
tagit! {AsideTag, "aside", AsideCompat}
tagit! {AudioTag, "audio", AudioCompat, autoplay, controls, loop, muted, preload, src }
tagit! {BTag, "b", BCompat}
tagit! {BaseTag, "base", BaseCompat, href, target }
tagit! {BdiTag, "bdi", BdiCompat}
tagit! {BdoTag, "bdo", BdoCompat}
tagit! {BlockquoteTag, "blockquote", BlockquoteCompat, cite }
tagit! {BodyTag, "body", BodyCompat}
tagit! {BrTag, "br", BrCompat}
tagit! {ButtonTag, "button", ButtonCompat, autofocus, disabled, form, formaction, formenctype, formmethod, formnovalidate, formtarget, name, type, value }
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
tagit! {EmbedTag, "embed", EmbedCompat, height, src, type, width }
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
tagit! {InputTag, "input", InputCompat, accept, alt, autocomplete, autofocus, checked, disabled, form, formaction, formenctype, formmethod, formnovalidate, formtarget, height, list, max, maxlength, min, minlength, multiple, name, pattern, placeholder, readonly, required, size, src, step, type, value, width }
tagit! {InsTag, "ins", InsCompat, cite, datetime }
tagit! {KbdTag, "kbd", KbdCompat}
tagit! {LabelTag, "label", LabelCompat, form }
tagit! {LegendTag, "legend", LegendCompat}
tagit! {LiTag, "li", LiCompat, value }
tagit! {LinkTag, "link", LinkCompat, as, crossorigin, href, hreflang, media, rel, sizes, type }
tagit! {MainTag, "main", MainCompat}
tagit! {MapTag, "map", MapCompat, name }
tagit! {MarkTag, "mark", MarkCompat}
tagit! {MenuTag, "menu", MenuCompat, type }
tagit! {MetaTag, "meta", MetaCompat, charset, content, http_equiv, name }
tagit! {MeterTag, "meter", MeterCompat, high, low, max, min, optimum, value }
tagit! {NavTag, "nav", NavCompat}
tagit! {NoscriptTag, "noscript", NoscriptCompat}
tagit! {ObjectTag, "object", ObjectCompat, data, form, height, name, type, width }
tagit! {OlTag, "ol", OlCompat, reversed, start }
tagit! {OptgroupTag, "optgroup", OptgroupCompat, disabled, label }
tagit! {OptionTag, "option", OptionCompat, disabled, label, selected, value }
tagit! {OutputTag, "output", OutputCompat, for, form, name }
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
tagit! {ScriptTag, "script", ScriptCompat, async, crossorigin, defer, integrity, nomodule, src, type }
tagit! {SectionTag, "section", SectionCompat}
tagit! {SelectTag, "select", SelectCompat, autocomplete, autofocus, disabled, form, multiple, name, required, size }
tagit! {SmallTag, "small", SmallCompat}
tagit! {SourceTag, "source", SourceCompat, media, sizes, src, srcset, type }
tagit! {SpanTag, "span", SpanCompat}
tagit! {StrongTag, "strong", StrongCompat}
tagit! {StyleTag, "style", StyleCompat, media, nonce, type }
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
tagit! {TrackTag, "track", TrackCompat, default, kind, label, src, srclang }
tagit! {UTag, "u", UCompat}
tagit! {ULTag, "ul", ULCompat}
tagit! {VarTag, "var", VarCompat}
tagit! {VideoTag, "video", VideoCompat, autoplay, controls, crossorigin, height, loop, muted, playsinline, poster, preload, src, width }
tagit! {WbrTag, "wbr", WbrCompat}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     a![.href="https://www.example.com", "Link text"].render(),
///     "<a href=\"https://www.example.com\">Link text</a>"
/// );
///
/// assert_eq!(
///     a![.href="/path/to/page", "Click here"].render(),
///     "<a href=\"/path/to/page\">Click here</a>"
/// );
/// assert_eq!(
///     a![.href="mailto:user@example.com", "Send email"].render(),
///     "<a href=\"mailto:user@example.com\">Send email</a>"
/// );
///
/// # }
/// ```
#[macro_export]
macro_rules! a {
    () => {tag_inner!(ATag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
            parse_full_tag!(ATag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
        };
    ( $inner_left:expr $(,$inner:expr)*) => { parse_full_tag!(ATag, $inner_left $(,$inner)*)
        };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     abbr![.title="Hypertext Markup Language", "HTML"].render(),
///     "<abbr title=\"Hypertext Markup Language\">HTML</abbr>"
/// );
///
/// assert_eq!(
///     abbr![.title="Scalable Vector Graphics", "SVG"].render(),
///     "<abbr title=\"Scalable Vector Graphics\">SVG</abbr>"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! abbr {
    () => {tag_inner!(AbbrTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(AbbrTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(AbbrTag, $inner_left $(,$inner)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     address!["P. Sherman", br![], "42 Wallaby way", br![], "Sydney, Australia"].render(),
///     "<address>P. Sherman<br>42 Wallaby way<br>Sydney, Australia</address>"
/// );
///
/// # }
/// ```
#[macro_export]
macro_rules! address {
    () => {tag_inner!(AddressTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(AddressTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(AddressTag, $inner_left $(,$inner)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     area![.shape="rect", .coords="0,0,100,100", .href="example.com"].render(),
///     "<area shape=\"rect\" coords=\"0,0,100,100\" href=\"example.com\">"
/// );
///
/// # }
/// ```
#[macro_export]
macro_rules! area {
    () => {tag_no_inner!(AreaTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)*) => {
        tag_no_inner!(AreaTag ,.$attr_left = $value_left $(,.$attr = $value)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     article!["This is an article."].render(),
///     "<article>This is an article.</article>"
/// );
///
/// assert_eq!(
///     article!["This is an article with a ", b!["bold"], " word."].render(),
///     "<article>This is an article with a <b>bold</b> word.</article>"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! article {
    () => {tag_inner!(ArticleTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(ArticleTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(ArticleTag, $inner_left $(,$inner)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     aside!["This is an aside."].render(),
///     "<aside>This is an aside.</aside>"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! aside {
    () => {tag_inner!(AsideTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(AsideTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(AsideTag, $inner_left $(,$inner)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     audio![.src="example.mp3"].render(),
///     "<audio src=\"example.mp3\">"
/// );
///
/// assert_eq!(
///     audio![.src="example.mp3", .controls = "true"].render(),
///     "<audio src=\"example.mp3\" controls=\"true\">"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! audio {
    () => {tag_no_inner!(AudioTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)*) => {
        tag_no_inner!(AudioTag ,.$attr_left = $value_left $(,.$attr = $value)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     b!["This is bold text"].render(),
///     "<b>This is bold text</b>"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! b {
    () => {tag_inner!(BTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(BTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(BTag, $inner_left $(,$inner)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     base![.href="https://example.com"].render(),
///     "<base href=\"https://example.com\">"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! base {
    () => {tag_no_inner!(BaseTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)*) => {
        tag_no_inner!(BaseTag ,.$attr_left = $value_left $(,.$attr = $value)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     bdi!["This text will be displayed in a different direction"].render(),
///     "<bdi>This text will be displayed in a different direction</bdi>"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! bdi {
    () => {tag_inner!(BdiTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(BdiTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(BdiTag, $inner_left $(,$inner)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     bdo![.dir="rtl", "This text will be displayed in a right-to-left direction"].render(),
///     "<bdo dir=\"rtl\">This text will be displayed in a right-to-left direction</bdo>"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! bdo {
    () => {tag_inner!(BdoTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(BdoTag,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(BdoTag, $inner_left $(,$inner)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     blockquote!["This is a blockquote."].render(),
///     "<blockquote>This is a blockquote.</blockquote>"
/// );
///
/// assert_eq!(
///     blockquote![.cite="https://example.com","This is a blockquote with citation"].render(),
///     "<blockquote cite=\"https://example.com\">This is a blockquote with citation</blockquote>"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! blockquote {
    () => {tag_inner!(BlockquoteTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(BlockquoteTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(BlockquoteTag, $inner_left $(,$inner)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     body!["This is the body"].render(),
///     "<body>This is the body</body>"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! body {
    () => {tag_inner!(BodyTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(BodyTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(BodyTag, $inner_left $(,$inner)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     br![].render(),
///     "<br>"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! br {
    () => {
        format_args!("<{}>", BrTag)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     button!["Click me"].render(),
///     "<button>Click me</button>"
/// );
///
/// # }
/// ```

#[macro_export]
macro_rules! button {
    () => {tag_inner!(ButtonTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(ButtonTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(ButtonTag, $inner_left $(,$inner)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     canvas![].render(),
///     "<canvas></canvas>"
/// );
///
/// assert_eq!(
///     canvas![.width="300", .height="150"].render(),
///     "<canvas width=\"300\" height=\"150\"></canvas>"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! canvas {
    () => {tag_inner!(CanvasTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(CanvasTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(CanvasTag, $inner_left $(,$inner)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     caption!["This is a caption"].render(),
///     "<caption>This is a caption</caption>"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! caption {
    () => {tag_inner!(CaptionTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(CaptionTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(CaptionTag, $inner_left $(,$inner)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     cite!["This is a citation"].render(),
///     "<cite>This is a citation</cite>"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! cite {
    () => {tag_inner!(CiteTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(CiteTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(CiteTag, $inner_left $(,$inner)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     code![ "h1![\"This is awesome!\"]" ].render(),
///     "<code>h1![\"This is awesome!\"]</code>"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! code {
    () => {tag_inner!(CodeTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(CodeTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(CodeTag, $inner_left $(,$inner)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     col![.span="2", .style="background-color:red"].render(),
///     "<col span=\"2\" style=\"background-color:red\">"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! col {
    () => {tag_no_inner!(ColTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)*) => {
        tag_no_inner!(ColTag ,.$attr_left = $value_left $(,.$attr = $value)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     colgroup![col![], col![]].render(),
///     "<colgroup><col><col></colgroup>"
/// );
///
/// # }
/// ```
#[macro_export]
macro_rules! colgroup {
    () => {tag_inner!(ColgroupTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(ColgroupTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(ColgroupTag, $inner_left $(,$inner)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     data!["123"].render(),
///     "<data>123</data>"
/// );
///
/// assert_eq!(
///     data![.value = "456", "Data Value"].render(),
///     "<data value=\"456\">Data Value</data>"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! data {
    () => {tag_inner!(DataTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(DataTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(DataTag, $inner_left $(,$inner)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     datalist![option!["option1"], option!["option2"], option!["option3"]].render(),
///     "<datalist><option>option1</option><option>option2</option><option>option3</option></datalist>"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! datalist {
    () => {tag_inner!(DatalistTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(DatalistTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(DatalistTag, $inner_left $(,$inner)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     dd!["This is a definition"].render(),
///     "<dd>This is a definition</dd>"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! dd {
    () => {tag_inner!(DdTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(DdTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(DdTag, $inner_left $(,$inner)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     del!["This text is deleted"].render(),
///     "<del>This text is deleted</del>"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! del {
    () => {tag_inner!(DelTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(DelTag,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(DelTag, $inner_left $(,$inner)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     details![summary!["Details"], "This is the details content"].render(),
///     "<details><summary>Details</summary>This is the details content</details>"
/// );
///
/// # }
#[macro_export]
macro_rules! details {
    () => {tag_inner!(DetailsTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(DetailsTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(DetailsTag, $inner_left $(,$inner)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     dialog!["This is a dialog"].render(),
///     "<dialog>This is a dialog</dialog>"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! dialog {
    () => {tag_inner!(DialogTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(DialogTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(DialogTag, $inner_left $(,$inner)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     div!{ "This is a div\"s inner text!" }.render(),
///     "<div>This is a div\"s inner text!</div>"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! div {
    () => {tag_inner!(DivTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(DivTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(DivTag, $inner_left $(,$inner)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     dl![dt!["Term 1"], dd!["Definition 1"], dt!["Term 2"], dd!["Definition 2"]].render(),
///     "<dl><dt>Term 1</dt><dd>Definition 1</dd><dt>Term 2</dt><dd>Definition 2</dd></dl>"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! dl {
    () => {tag_inner!(DlTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(DlTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(DlTag, $inner_left $(,$inner)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     dl![dt!["Term 1"], dd!["Definition 1"], dt!["Term 2"], dd!["Definition 2"]].render(),
///     "<dl><dt>Term 1</dt><dd>Definition 1</dd><dt>Term 2</dt><dd>Definition 2</dd></dl>"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! dt {
    () => {tag_inner!(DtTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(DtTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(DtTag, $inner_left $(,$inner)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     em!["This is emphasized text"].render(),
///     "<em>This is emphasized text</em>"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! em {
    () => {tag_inner!(EmTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(EmTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(EmTag, $inner_left $(,$inner)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     embed![.src="example.swf"].render(),
///     "<embed src=\"example.swf\">"
/// );
///
/// assert_eq!(
///     embed![.src="example.swf", .type="application/x-shockwave-flash", .width="400", .height="300"].render(),
///     "<embed src=\"example.swf\" type=\"application/x-shockwave-flash\" width=\"400\" height=\"300\">"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! embed {
    () => {tag_no_inner!(EmbedTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)*) => {
        tag_no_inner!(EmbedTag ,.$attr_left = $value_left $(,.$attr = $value)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     fieldset![legend!["This is a fieldset"], "This is the fieldset content"].render(),
///     "<fieldset><legend>This is a fieldset</legend>This is the fieldset content</fieldset>"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! fieldset {
    () => {tag_inner!(FieldsetTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(FieldsetTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(FieldsetTag, $inner_left $(,$inner)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     figcaption!["This is a caption"].render(),
///     "<figcaption>This is a caption</figcaption>"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! figcaption {
    () => {tag_inner!(FigcaptionTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(FigcaptionTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(FigcaptionTag, $inner_left $(,$inner)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     figure![img![.src="example.jpg"], figcaption!["This is a caption"]].render(),
///     "<figure><img src=\"example.jpg\"><figcaption>This is a caption</figcaption></figure>"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! figure {
    () => {tag_inner!(FigureTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(FigureTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(FigureTag, $inner_left $(,$inner)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     footer!["This is a footer"].render(),
///     "<footer>This is a footer</footer>"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! footer {
    () => {tag_inner!(FooterTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(FooterTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(FooterTag, $inner_left $(,$inner)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     form![.action="https://www.example.com", .method="post", "This is a form"].render(),
///     "<form action=\"https://www.example.com\" method=\"post\">This is a form</form>"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! form {
    () => {tag_inner!(FormTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(FormTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(FormTag, $inner_left $(,$inner)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     h1!["This is a heading"].render(),
///     "<h1>This is a heading</h1>"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! h1 {
    () => {tag_inner!(H1Tag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(H1Tag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(H1Tag, $inner_left $(,$inner)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     h2!["This is a heading"].render(),
///     "<h2>This is a heading</h2>"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! h2 {
    () => {tag_inner!(H2Tag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(H2Tag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(H2Tag, $inner_left $(,$inner)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     h3!["This is a heading"].render(),
///     "<h3>This is a heading</h3>"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! h3 {
    () => {tag_inner!(H3Tag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(H3Tag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(H3Tag, $inner_left $(,$inner)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     h4!["This is a heading"].render(),
///     "<h4>This is a heading</h4>"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! h4 {
    () => {tag_inner!(H4Tag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(H4Tag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(H4Tag, $inner_left $(,$inner)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     h5!["This is a heading"].render(),
///     "<h5>This is a heading</h5>"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! h5 {
    () => {tag_inner!(H5Tag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(H5Tag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(H5Tag, $inner_left $(,$inner)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     h6!["This is a heading"].render(),
///     "<h6>This is a heading</h6>"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! h6 {
    () => {tag_inner!(H6Tag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(H6Tag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(H6Tag, $inner_left $(,$inner)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     head![title!["This is a title"], link![.rel="stylesheet", .href="example.css"]].render(),
///     "<head><title>This is a title</title><link rel=\"stylesheet\" href=\"example.css\"></head>"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! head {
    () => {tag_inner!(HeadTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(HeadTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(HeadTag, $inner_left $(,$inner)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     header!["This is a header"].render(),
///     "<header>This is a header</header>"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! header {
    () => {tag_inner!(HeaderTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(HeaderTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(HeaderTag, $inner_left $(,$inner)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     hr![].render(),
///     "<hr>"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! hr {
    () => {tag_no_inner!(HrTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)*) => {
        tag_no_inner!(HrTag ,.$attr_left = $value_left $(,.$attr = $value)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// let html = html! {
///  .lang = "en",
///      head!{
///          title!{
///              "Title of the document"
///          }
///      },
///      body!{
///              div!{
///                  "text",
///                  h1!{
///                      "This is a heading"
///                  },
///                  p!{
///                      "This is a paragraph"
///                  }
///              }
///      }
/// }.render();
///
/// println!("{}", html);
///
/// assert_eq!(
///     html![head![title!["This is a title"]], body!["This is the body"]].render(),
///     "<html><head><title>This is a title</title></head><body>This is the body</body></html>"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! html {
    () => {tag_inner!(HtmlTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(HtmlTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(HtmlTag, $inner_left $(,$inner)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     i!["This is in italic"].render(),
///     "<i>This is in italic</i>"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! i {
    () => {tag_inner!(ITag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(ITag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(ITag, $inner_left $(,$inner)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     iframe![.src="example.html"].render(),
///     "<iframe src=\"example.html\"></iframe>"
/// );
///
/// assert_eq!(
///     iframe![.src="example.html", .width="400", .height="300"].render(),
///     "<iframe src=\"example.html\" width=\"400\" height=\"300\"></iframe>"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! iframe {
    () => {tag_inner!(IframeTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(IframeTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(IframeTag, $inner_left $(,$inner)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     img![.src="example.jpg"].render(),
///     "<img src=\"example.jpg\">"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! img {
    () => {tag_no_inner!(ImgTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)*) => {
        tag_no_inner!(ImgTag ,.$attr_left = $value_left $(,.$attr = $value)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     input![.type="text", .name="name"].render(),
///     "<input type=\"text\" name=\"name\">"
/// );
///
/// assert_eq!(
///     input![.type="submit", .value="submit"].render(),
///     "<input type=\"submit\" value=\"submit\">"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! input {
    () => {tag_no_inner!(InputTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)*) => {
        tag_no_inner!(InputTag ,.$attr_left = $value_left $(,.$attr = $value)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     ins![.datetime="2022-01-01", "This is inserted text"].render(),
///     "<ins datetime=\"2022-01-01\">This is inserted text</ins>"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! ins {
    () => {tag_inner!(InsTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(InsTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(InsTag, $inner_left $(,$inner)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     kbd!["This is keyboard input"].render(),
///     "<kbd>This is keyboard input</kbd>"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! kbd {
    () => {tag_inner!(KbdTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(KbdTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(KbdTag, $inner_left $(,$inner)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     label!["Name:"].render(),
///     "<label>Name:</label>"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! label {
    () => {tag_inner!(LabelTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(LabelTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(LabelTag, $inner_left $(,$inner)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     fieldset![legend!["This is a fieldset"], "This is the fieldset content"].render(),
///     "<fieldset><legend>This is a fieldset</legend>This is the fieldset content</fieldset>"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! legend {
    () => {tag_inner!(LegendTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(LegendTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(LegendTag, $inner_left $(,$inner)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     li!["Item 1"].render(),
///     "<li>Item 1</li>"
/// );
///
/// assert_eq!(
///     li![.value=5, "Item 5"].render(),
///     "<li value=\"5\">Item 5</li>"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! li {
    () => {tag_inner!(LiTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(LiTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(LiTag, $inner_left $(,$inner)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     link![.rel="stylesheet", .href="example.css"].render(),
///     "<link rel=\"stylesheet\" href=\"example.css\">"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! link {
    () => {tag_no_inner!(LinkTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)*) => {
        tag_no_inner!(LinkTag ,.$attr_left = $value_left $(,.$attr = $value)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     main!["This is the main content"].render(),
///     "<main>This is the main content</main>"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! main {
    () => {tag_inner!(MainTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(MainTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(MainTag, $inner_left $(,$inner)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     map![.name="example", area![.shape="rect", .coords="0,0,100,100", .href="example.html"]].render(),
///     "<map name=\"example\"><area shape=\"rect\" coords=\"0,0,100,100\" href=\"example.html\"></map>"
/// );
/// # }
#[macro_export]
macro_rules! map {
    () => {tag_inner!(MapTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(MapTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(MapTag, $inner_left $(,$inner)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     mark!["This is marked text"].render(),
///     "<mark>This is marked text</mark>"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! mark {
    () => {tag_inner!(MarkTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(MarkTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(MarkTag, $inner_left $(,$inner)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     menu![li!["Item 1"], li!["Item 2"]].render(),
///     "<menu><li>Item 1</li><li>Item 2</li></menu>"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! menu {
    () => {tag_inner!(MenuTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(MenuTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(MenuTag, $inner_left $(,$inner)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     meta![.name="viewport", .content="width=device-width, initial-scale=1"].render(),
///     "<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! meta {
    () => {tag_no_inner!(MetaTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)*) => {
        tag_no_inner!(MetaTag ,.$attr_left = $value_left $(,.$attr = $value)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     meter![.value=2, .min=0, .max=5, "2 out of 5"].render(),
///     "<meter value=\"2\" min=\"0\" max=\"5\">2 out of 5</meter>"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! meter {
    () => {tag_inner!(MeterTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(MeterTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(MeterTag, $inner_left $(,$inner)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     nav![a![.href="home.html", "Home"], a![.href="about.html", "About"], a![.href="contact.html", "Contact"]].render(),
///     "<nav><a href=\"home.html\">Home</a><a href=\"about.html\">About</a><a href=\"contact.html\">Contact</a></nav>"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! nav {
    () => {tag_inner!(NavTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(NavTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(NavTag, $inner_left $(,$inner)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     noscript!["JavaScript is disabled."].render(),
///     "<noscript>JavaScript is disabled.</noscript>"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! noscript {
    () => {tag_inner!(NoscriptTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(NoscriptTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(NoscriptTag, $inner_left $(,$inner)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     ol![li!["Item 1"], li!["Item 2"]].render(),
///     "<ol><li>Item 1</li><li>Item 2</li></ol>"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! ol {
    () => {tag_inner!(OlTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(OlTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(OlTag, $inner_left $(,$inner)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     optgroup![.label="Fruits", option!["Apple"], option!["Banana"], option!["Cherry"]].render(),
///     "<optgroup label=\"Fruits\"><option>Apple</option><option>Banana</option><option>Cherry</option></optgroup>"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! optgroup {
    () => {tag_inner!(OptgroupTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(OptgroupTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(OptgroupTag, $inner_left $(,$inner)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     option!["Apple"].render(),
///     "<option>Apple</option>"
/// );
///
/// assert_eq!(
///     option![.value="a", "Apple"].render(),
///     "<option value=\"a\">Apple</option>"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! option {
    () => {tag_inner!(OptionTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(OptionTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(OptionTag, $inner_left $(,$inner)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     output!["10"].render(),
///     "<output>10</output>"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! output {
    () => {tag_inner!(OutputTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(OutputTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(OutputTag, $inner_left $(,$inner)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     p!["This is a paragraph"].render(),
///     "<p>This is a paragraph</p>"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! p {
    () => {tag_inner!(PTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(PTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(PTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! picture {
    () => {tag_inner!(PictureTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(PictureTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(PictureTag, $inner_left $(,$inner)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     pre!["    var x = 5;\n    var y = 10;\n    var z = x + y;\n"].render(),
///     "<pre>    var x = 5;\n    var y = 10;\n    var z = x + y;\n</pre>"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! pre {
    () => {tag_inner!(PreTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(PreTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(PreTag, $inner_left $(,$inner)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     progress![.value=50, .max=100, "50%"].render(),
///     "<progress value=\"50\" max=\"100\">50%</progress>"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! progress {
    () => {tag_inner!(ProgressTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(ProgressTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(ProgressTag, $inner_left $(,$inner)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     q!["This is a short quote."].render(),
///     "<q>This is a short quote.</q>"
/// );
///
/// assert_eq!(
///     q![.cite="https://www.example.com", "This is a short quote."].render(),
///     "<q cite=\"https://www.example.com\">This is a short quote.</q>"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! q {
    () => {tag_inner!(QTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(QTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(QTag, $inner_left $(,$inner)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     rp!["」"].render(),
///     "<rp>」</rp>"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! rp {
    () => {tag_inner!(RpTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(RpTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(RpTag, $inner_left $(,$inner)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     ruby![ "漢", rt!["ㄏㄢˋ"] ].render(),
///     "<ruby>漢<rt>ㄏㄢˋ</rt></ruby>"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! rt {
    () => {tag_inner!(RtTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(RtTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(RtTag, $inner_left $(,$inner)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     ruby![ "漢", rt!["ㄏㄢˋ"] ].render(),
///     "<ruby>漢<rt>ㄏㄢˋ</rt></ruby>"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! ruby {
    () => {tag_inner!(RubyTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(RubyTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(RubyTag, $inner_left $(,$inner)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     s!["This text is no longer accurate"].render(),
///     "<s>This text is no longer accurate</s>"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! s {
    () => {tag_inner!(STag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(STag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(STag, $inner_left $(,$inner)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     samp!["Sample output from a computer program"].render(),
///     "<samp>Sample output from a computer program</samp>"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! samp {
    () => {tag_inner!(SampTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(SampTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(SampTag, $inner_left $(,$inner)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     script![.src="script.js", .type="text/javascript"].render(),
///     "<script src=\"script.js\" type=\"text/javascript\"></script>"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! script {
    () => {tag_inner!(ScriptTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(ScriptTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(ScriptTag, $inner_left $(,$inner)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     section![h1!["Section Title"], "Section content."].render(),
///     "<section><h1>Section Title</h1>Section content.</section>"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! section {
    () => {tag_inner!(SectionTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(SectionTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(SectionTag, $inner_left $(,$inner)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     select![
///         option!["Option 1"],
///         option!["Option 2"],
///         option!["Option 3"]
///     ].render(),
///     "<select><option>Option 1</option><option>Option 2</option><option>Option 3</option></select>"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! select {
    () => {tag_inner!(SelectTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(SelectTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(SelectTag, $inner_left $(,$inner)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     small!["This text is small"].render(),
///     "<small>This text is small</small>"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! small {
    () => {tag_inner!(SmallTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(SmallTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(SmallTag, $inner_left $(,$inner)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     source![.src="audio.ogg", .type="audio/ogg"].render(),
///     "<source src=\"audio.ogg\" type=\"audio/ogg\">"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! source {
    () => {tag_no_inner!(SourceTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)*) => {
        tag_no_inner!(SourceTag ,.$attr_left = $value_left $(,.$attr = $value)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     span![.class="highlight", "This text is highlighted"].render(),
///     "<span class=\"highlight\">This text is highlighted</span>"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! span {
    () => {tag_inner!(SpanTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(SpanTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(SpanTag, $inner_left $(,$inner)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     strong!["This text is important"].render(),
///     "<strong>This text is important</strong>"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! strong {
    () => {tag_inner!(StrongTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(StrongTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(StrongTag, $inner_left $(,$inner)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     style![.type="text/css", "body {background-color: black;} h1 {color: white;}"].render(),
///     "<style type=\"text/css\">body {background-color: black;} h1 {color: white;}</style>"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! style {
    () => {tag_inner!(StyleTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(StyleTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(StyleTag, $inner_left $(,$inner)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     sub!["This text is a subscript"].render(),
///     "<sub>This text is a subscript</sub>"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! sub {
    () => {tag_inner!(SubTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(SubTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(SubTag, $inner_left $(,$inner)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     summary!["This is a summary"].render(),
///     "<summary>This is a summary</summary>"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! summary {
    () => {tag_inner!(SummaryTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(SummaryTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(SummaryTag, $inner_left $(,$inner)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     sup!["This text is a superscript"].render(),
///     "<sup>This text is a superscript</sup>"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! sup {
    () => {tag_inner!(SupTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(SupTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(SupTag, $inner_left $(,$inner)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     table![
///         tr![
///             td!["Cell 1,1"],
///             td!["Cell 1,2"]
///         ],
///         tr![
///             td!["Cell 2,1"],
///             td!["Cell 2,2"]
///         ]
///     ].render(),
///     "<table><tr><td>Cell 1,1</td><td>Cell 1,2</td></tr><tr><td>Cell 2,1</td><td>Cell 2,2</td></tr></table>"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! table {
    () => {tag_inner!(TableTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(TableTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(TableTag, $inner_left $(,$inner)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     tbody![
///         tr![
///             td!["Cell 1,1"],
///             td!["Cell 1,2"]
///         ],
///         tr![
///             td!["Cell 2,1"],
///             td!["Cell 2,2"]
///         ]
///     ].render(),
///     "<tbody><tr><td>Cell 1,1</td><td>Cell 1,2</td></tr><tr><td>Cell 2,1</td><td>Cell 2,2</td></tr></tbody>"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! tbody {
    () => {tag_inner!(TbodyTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(TbodyTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(TbodyTag, $inner_left $(,$inner)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     td!["Cell 1,1"].render(),
///     "<td>Cell 1,1</td>"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! td {
    () => {tag_inner!(TdTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(TdTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(TdTag, $inner_left $(,$inner)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     template![
///         h1!["Title"],
///         p!["Description"]
///     ].render(),
///     "<template><h1>Title</h1><p>Description</p></template>"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! template {
    () => {tag_inner!(TemplateTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(TemplateTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(TemplateTag, $inner_left $(,$inner)*)
    };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     textarea![.name="message", "Default text"].render(),
///     "<textarea name=\"message\">Default text</textarea>"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! textarea {
    () => {tag_inner!(TextareaTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(TextareaTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(TextareaTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! tfoot {
    () => {tag_inner!(TfootTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(TfootTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(TfootTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! th {
    () => {tag_inner!(ThTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(ThTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(ThTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! thead {
    () => {tag_inner!(TheadTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(TheadTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(TheadTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! time {
    () => {tag_inner!(TimeTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(TimeTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(TimeTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! title {
    () => {tag_inner!(TitleTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(TitleTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(TitleTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! tr {
    () => {tag_inner!(TrTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(TrTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(TrTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! track {
    () => {tag_inner!(TrackTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(TrackTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(TrackTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! u {
    () => {tag_inner!(UTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(UTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(UTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! ul {
    () => {tag_inner!(UlTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(UlTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(UlTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! var {
    () => {tag_inner!(VarTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(VarTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(VarTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! video {
    () => {tag_inner!(VideoTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(VideoTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(VideoTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! wbr {
    () => {
        format_args!("<{}>", WbrTag)
    };
}

#[macro_export]
macro_rules! tag_inner {
    (,$inner_left:expr) => {
        format_args!("{}", $inner_left)
    };
    (,$inner_left:expr $(,$inner:expr)+) => {
        format_args!("{}{}", $inner_left, tag_inner!($(,$inner)*))
    };
    ($tag:ident,) => {
        format_args!("<{}></{}>", $tag, $tag)
    };
    ($tag:ident) => {
        format_args!("<{}></{}>", $tag, $tag)
    };
    ($tag:ident $(,$inner:expr)+) => {
        format_args!("<{}>{}</{}>", $tag, tag_inner!($(,$inner)*), $tag)
    };
    ($tag:ident $(,.$attr:ident = $value:expr)*) => {
        format_args!("<{}{}></{}>", $tag, attr_inner!($tag $(,.$attr = $value)*), $tag)
    };
    ($tag:ident $(,.$attr:ident = $value:expr)* $(,$inner:expr)*) => {
        format_args!("<{}{}>{}</{}>", $tag, attr_inner!($tag $(,.$attr = $value)*), tag_inner!($(,$inner)*), $tag)
    };
}

#[macro_export]
macro_rules! tag_no_inner {
    ($tag:ident,) => {
        format_args!("<{}>", $tag)
    };
    ($tag:ident) => {
        format_args!("<{}>", $tag)
    };
    ($tag:ident $(,.$attr:ident = $value:expr)*) => {
        format_args!("<{}{}>", $tag, attr_inner!($tag $(,.$attr = $value)*))
    };
}

#[macro_export]
macro_rules! attr_inner {
    () => { format_args!("{}", "") };
    ($tag:ident) => { format_args!("{}", "") };
    ($tag:ident, .$attr:ident = $value:expr) => {
        {
            let ident = paste::paste! { [<$attr _>] };
            $tag.type_check(&ident);
            format_args!(" {}=\"{}\"", ident.clone(), $value)
        }
    };
    (,.$attr:ident = $value:expr $(,.$right_attr:ident = $right_expr:expr)*) => {{
        let ident = paste::paste! { [<$attr _>] };
        format_args!(" {}=\"{}\"{}", ident.clone(), $value, attr_inner!($(,.$right_attr = $right_expr)*))
    }};
    ($tag:ident, .$attr:ident = $value:expr $(,.$right_attr:ident = $right_expr:expr)*) => {
        {
            let ident = paste::paste! { [<$attr _>] };
            $tag.type_check(&ident);
            format_args!(" {}=\"{}\"{}", ident.clone(), $value, attr_inner!($(,.$right_attr = $right_expr)*))
        }
    };
}

#[macro_export]
macro_rules! parse_full_tag {
    ($tag:ident) => {
        format_args!("<{}></{}>", $tag, $tag)
    };
    ($tag:ident $(,$inner:expr)+) => {
        format_args!("<{}>{}</{}>", $tag, parse_inner!($($inner)*), $tag)
    };
    ($tag:ident $(,.$attr:ident = $value:expr)+) => {
        format_args!("<{}{}></{}>", $tag, parse_attr!($tag $(,.$attr = $value)*), $tag)
    };
    ($tag:ident $(,.$attr:ident = $value:expr)* $(,$inner:expr)*) => {
        format_args!("<{}{}>{}</{}>", $tag, parse_attr!($tag $(,.$attr = $value)*), parse_inner!($($inner)*), $tag)
    };
}

#[macro_export]
macro_rules! parse_inner {
    ($left:expr $(,$inner:expr)*) => {
        format_args!("{}{}", $left, parse_inner!($($inner)*))
    };
    ($left:expr) => {
        format_args!("{}", $left)
    };
    () => { format_args!("{}", "")
    };
}

#[macro_export]
macro_rules! parse_tag {
    ($tag:ident) => {
        format_args!("<{}>", $tag)
    };
    ($tag:ident $(,.$attr:ident = $value:expr)*) => {
        format_args!("<{}{}>", $tag, parse_attr!($tag $(,.$attr = $value)*))
    };
}

#[macro_export]
macro_rules! parse_attr {
    ($tag:ident) => {
        format_args!("{}", "")
    };
    ($tag:ident, .$attr:ident = $value:expr $(,.$right_attr:ident = $right_expr:expr)*) => {{
        let ident = paste::paste! { [<$attr _>] };
        format_args!(" {}{}{}", ident.clone(), parse_attr_val!($value), parse_attr!($tag $(,.$right_attr = $right_expr)*))
    }};
}

#[macro_export]
macro_rules! parse_attr_val {
    () => {
        format_args!("{}", "")
    };
    ($val:expr) => {
        format_args!("=\"{}\"", "")
    };
}
