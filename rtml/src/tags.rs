use crate::*;

pub trait Tag: std::fmt::Display + 'static {}

pub trait TagValue: ToString {
    fn render(&self) -> String {
        return self.to_string();
    }
}

macro_rules! tagit {
    ($tag:ident, $trait:ident $(,$attr:ident)*) => {
        #[allow(unused_variables)]
        impl $tag {
            pub fn type_check(&self, attr: &dyn $trait){}
        }
        #[warn(unused_variables)]
        pub trait $trait {}
        impl Tag for $tag {}

        $(impl $trait for $attr {})*
    };
}

tagit! {ATag, ACompat, download, href, hreflang, media, ping, referrerpolicy, rel, target }
tagit! {AbbrTag, AbbrCompat}
tagit! {AddressTag, AddressCompat}
tagit! {AreaTag, AreaCompat, alt, coords, download, href, hreflang, media, rel, shape, target }
tagit! {ArticleTag, ArticleCompat}
tagit! {AsideTag, AsideCompat}
tagit! {AudioTag, AudioCompat, autoplay, controls, _loop, muted, preload, src }
tagit! {BTag, BCompat}
tagit! {BaseTag, BaseCompat, href, target }
tagit! {BdiTag, BdiCompat}
tagit! {BdoTag, BdoCompat}
tagit! {BlockquoteTag, BlockquoteCompat, cite }
tagit! {BodyTag, BodyCompat}
tagit! {BrTag, BrCompat}
tagit! {ButtonTag, ButtonCompat, autofocus, disabled, form, formaction, formenctype, formmethod, formnovalidate, formtarget, name, _type, value }
tagit! {CanvasTag, CanvasCompat, height, width }
tagit! {CaptionTag, CaptionCompat}
tagit! {CiteTag, CiteCompat}
tagit! {CodeTag, CodeCompat}
tagit! {ColTag, ColCompat, span }
tagit! {ColgroupTag, ColgroupCompat, span }
tagit! {DataTag, DataCompat, value }
tagit! {DatalistTag, DatalistCompat}
tagit! {DdTag, DdCompat}
tagit! {DelTag, DelCompat, cite, datetime }
tagit! {DetailsTag, DetailsCompat, open }
tagit! {DfnTag, DfnCompat}
tagit! {DialogTag, DialogCompat, open }
tagit! {DivTag, DivCompat}
tagit! {DlTag, DlCompat}
tagit! {DtTag, DtCompat}
tagit! {EmTag, EmCompat}
tagit! {EmbedTag, EmbedCompat, height, src, _type, width }
tagit! {FieldsetTag, FieldsetCompat, disabled, form, name }
tagit! {FigcaptionTag, FigcaptionCompat}
tagit! {FigureTag, FigureCompat}
tagit! {FooterTag, FooterCompat}
tagit! {FormTag, FormCompat, accept_charset, action, autocomplete, enctype, method, name, novalidate, target }
tagit! {H1Tag, H1Compat}
tagit! {H2Tag, H2Compat}
tagit! {H3Tag, H3Compat}
tagit! {H4Tag, H4Compat}
tagit! {H5Tag, H5Compat}
tagit! {H6Tag, H6Compat}
tagit! {HeadTag, HeadCompat}
tagit! {HeaderTag, HeaderCompat}
tagit! {HrTag, HrCompat}
tagit! {HtmlTag, HtmlCompat}
tagit! {ITag, ICompat}
tagit! {IframeTag, IframeCompat, allow, allowfullscreen, height, name, referrerpolicy, sandbox, src, srcdoc, width }
tagit! {ImgTag, ImgCompat, alt, crossorigin, decoding, height, referrerpolicy, sizes, src, srcset, usemap, width }
tagit! {InputTag, InputCompat, accept, alt, autocomplete, autofocus, checked, disabled, form, formaction, formenctype, formmethod, formnovalidate, formtarget, height, list, max, maxlength, min, minlength, multiple, name, pattern, placeholder, readonly, required, size, src, step, _type, value, width }
tagit! {InsTag, InsCompat, cite, datetime }
tagit! {KbdTag, KbdCompat}
tagit! {LabelTag, LabelCompat, form }
tagit! {LegendTag, LegendCompat}
tagit! {LiTag, LiCompat, value }
tagit! {LinkTag, LinkCompat, _as, crossorigin, href, hreflang, media, rel, sizes, _type }
tagit! {MainTag, MainCompat}
tagit! {MapTag, MapCompat, name }
tagit! {MarkTag, MarkCompat}
tagit! {MenuTag, MenuCompat, _type }
tagit! {MetaTag, MetaCompat, charset, content, http_equiv, name }
tagit! {MeterTag, MeterCompat, high, low, max, min, optimum, value }
tagit! {NavTag, NavCompat}
tagit! {NoscriptTag, NoscriptCompat}
tagit! {ObjectTag, ObjectCompat, data, form, height, name, _type, width }
tagit! {OlTag, OlCompat, reversed, start }
tagit! {OptgroupTag, OptgroupCompat, disabled, label }
tagit! {OptionTag, OptionCompat, disabled, label, selected, value }
tagit! {OutputTag, OutputCompat, _for, form, name }
tagit! {PTag, PCompat}
tagit! {PictureTag, PictureCompat}
tagit! {PreTag, PreCompat}
tagit! {ProgressTag, ProgressCompat, max, value }
tagit! {QTag, QCompat, cite }
tagit! {RpTag, RpCompat}
tagit! {RtTag, RtCompat}
tagit! {RubyTag, RubyCompat}
tagit! {STag, SCompat}
tagit! {SampTag, SampCompat}
tagit! {ScriptTag, ScriptCompat, _async, crossorigin, defer, integrity, nomodule, src, _type }
tagit! {SectionTag, SectionCompat}
tagit! {SelectTag, SelectCompat, autocomplete, autofocus, disabled, form, multiple, name, required, size }
tagit! {SmallTag, SmallCompat}
tagit! {SourceTag, SourceCompat, muted, media, sizes, src, srcset, _type }
tagit! {SpanTag, SpanCompat}
tagit! {StrongTag, StrongCompat}
tagit! {StyleTag, StyleCompat, media, nonce, _type }
tagit! {SubTag, SubCompat}
tagit! {SummaryTag, SummaryCompat}
tagit! {SupTag, SupCompat}
tagit! {TableTag, TableCompat}
tagit! {TbodyTag, TbodyCompat}
tagit! {TdTag, TdCompat, colspan, headers, rowspan, scope }
tagit! {TemplateTag, TemplateCompat}
tagit! {TextareaTag, TextareaCompat, autocomplete, autofocus, cols, disabled, form, maxlength, minlength, name, placeholder, readonly, required, rows, wrap }
tagit! {TfootTag, TfootCompat}
tagit! {ThTag, ThCompat, colspan, headers, rowspan, scope }
tagit! {TheadTag, TheadCompat}
tagit! {TimeTag, TimeCompat, datetime }
tagit! {TitleTag, TitleCompat}
tagit! {TrTag, TrCompat}
tagit! {TrackTag, TrackCompat, default, _kind, label, src, srclang }
tagit! {UTag, UCompat}
tagit! {UlTag, UlCompat}
tagit! {VarTag, VarCompat}
tagit! {VideoTag, VideoCompat, autoplay, controls, crossorigin, height, _loop, muted, playsinline, poster, preload, src, width }
tagit! {WbrTag, WbrCompat}

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
    ($($all:tt)*) => {parse_double_tag!(ATag, $($all)*) };
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
    ($($all:tt)*) => {parse_double_tag!(AbbrTag, $($all)*) };
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
    ($($all:tt)*) => {parse_double_tag!(AddressTag, $($all)*) };
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
    ($($all:tt)*) => {parse_single_tag!(AreaTag, $($all)*) };
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
    ($($all:tt)*) => {parse_double_tag!(ArticleTag, $($all)*) };
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
    ($($all:tt)*) => {parse_double_tag!(AsideTag, $($all)*) };
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
    ($($all:tt)*) => {parse_single_tag!(AudioTag, $($all)*) };
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
    ($($all:tt)*) => {parse_double_tag!(BTag, $($all)*) };
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
    ($($all:tt)*) => {parse_single_tag!(BaseTag, $($all)*) };
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
    ($($all:tt)*) => {parse_double_tag!(BdiTag, $($all)*) };
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
    ($($all:tt)*) => {parse_double_tag!(BdoTag, $($all)*) };
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
    ($($all:tt)*) => {parse_double_tag!(BlockquoteTag, $($all)*) };
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
    ($($all:tt)*) => {parse_double_tag!(BodyTag, $($all)*) };
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
    ($($all:tt)*) => {parse_single_tag!(BrTag, $($all)*) };
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
    ($($all:tt)*) => {parse_double_tag!(ButtonTag, $($all)*) };
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
    ($($all:tt)*) => {parse_double_tag!(CanvasTag, $($all)*) };
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
    ($($all:tt)*) => {parse_double_tag!(CaptionTag, $($all)*) };
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
    ($($all:tt)*) => {parse_double_tag!(CiteTag, $($all)*) };
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
    ($($all:tt)*) => {parse_double_tag!(CodeTag, $($all)*) };
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
    ($($all:tt)*) => {parse_single_tag!(ColTag, $($all)*) };
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
    ($($all:tt)*) => {parse_double_tag!(ColgroupTag, $($all)*) };
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
    ($($all:tt)*) => {parse_double_tag!(DataTag, $($all)*) };
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
    ($($all:tt)*) => {parse_double_tag!(DatalistTag, $($all)*) };
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
    ($($all:tt)*) => {parse_double_tag!(DdTag, $($all)*) };
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
    ($($all:tt)*) => {parse_double_tag!(DelTag, $($all)*) };
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
    ($($all:tt)*) => {parse_double_tag!(DetailsTag, $($all)*) };
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
    ($($all:tt)*) => {parse_double_tag!(DialogTag, $($all)*) };
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
    ($($all:tt)*) => {parse_double_tag!(DivTag, $($all)*) };
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
    ($($all:tt)*) => {parse_double_tag!(DlTag, $($all)*) };
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
    ($($all:tt)*) => {parse_double_tag!(DtTag, $($all)*) };
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
    ($($all:tt)*) => {parse_double_tag!(EmTag, $($all)*) };
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
///     embed![.src="example.swf", ._type="application/x-shockwave-flash", .width="400", .height="300"].render(),
///     "<embed src=\"example.swf\" type=\"application/x-shockwave-flash\" width=\"400\" height=\"300\">"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! embed {
    ($($all:tt)*) => {parse_single_tag!(EmbedTag, $($all)*) };
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
    ($($all:tt)*) => {parse_double_tag!(FieldsetTag, $($all)*) };
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
    ($($all:tt)*) => {parse_double_tag!(FigcaptionTag, $($all)*) };
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
    ($($all:tt)*) => {parse_double_tag!(FigureTag, $($all)*) };
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
    ($($all:tt)*) => {parse_double_tag!(FooterTag, $($all)*) };
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
    ($($all:tt)*) => {parse_double_tag!(FormTag, $($all)*) };
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
    ($($all:tt)*) => {parse_double_tag!(H1Tag, $($all)*) };
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
    ($($all:tt)*) => {parse_double_tag!(H2Tag, $($all)*) };
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
    ($($all:tt)*) => {parse_double_tag!(H3Tag, $($all)*) };
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
    ($($all:tt)*) => {parse_double_tag!(H4Tag, $($all)*) };
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
    ($($all:tt)*) => {parse_double_tag!(H5Tag, $($all)*) };
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
    ($($all:tt)*) => {parse_double_tag!(H6Tag, $($all)*) };
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
    ($($all:tt)*) => {parse_double_tag!(HeadTag, $($all)*) };
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
    ($($all:tt)*) => {parse_double_tag!(HeaderTag, $($all)*) };
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
    ($($all:tt)*) => {parse_single_tag!(HrTag, $($all)*) };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// let result = html! {
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
/// println!("{}", result);
///
/// assert_eq!(
///     html![head![title!["This is a title"]], body!["This is the body"]].render(),
///     "<html><head><title>This is a title</title></head><body>This is the body</body></html>"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! html {
    ($($all:tt)*) => {parse_double_tag!(HtmlTag, $($all)*) };
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
    ($($all:tt)*) => {parse_double_tag!(ITag, $($all)*) };
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
    ($($all:tt)*) => {parse_double_tag!(IframeTag, $($all)*) };
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
    ($($all:tt)*) => {parse_single_tag!(ImgTag, $($all)*) };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     input![._type="text", .name="name"].render(),
///     "<input type=\"text\" name=\"name\">"
/// );
///
/// assert_eq!(
///     input![._type="submit", .value="submit"].render(),
///     "<input type=\"submit\" value=\"submit\">"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! input {
    ($($all:tt)*) => {parse_single_tag!(InputTag, $($all)*) };
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
    ($($all:tt)*) => {parse_double_tag!(InsTag, $($all)*) };
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
    ($($all:tt)*) => {parse_double_tag!(KbdTag, $($all)*) };
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
    ($($all:tt)*) => {parse_double_tag!(LabelTag, $($all)*) };
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
    ($($all:tt)*) => {parse_double_tag!(LegendTag, $($all)*) };
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
    ($($all:tt)*) => {parse_double_tag!(LiTag, $($all)*) };
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
    ($($all:tt)*) => {parse_single_tag!(LinkTag, $($all)*) };
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
    ($($all:tt)*) => {parse_double_tag!(MainTag, $($all)*) };
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
    ($($all:tt)*) => {parse_double_tag!(MapTag, $($all)*) };
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
    ($($all:tt)*) => {parse_double_tag!(MarkTag, $($all)*) };
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
    ($($all:tt)*) => {parse_double_tag!(MenuTag, $($all)*) };
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
    ($($all:tt)*) => {parse_single_tag!(MetaTag, $($all)*) };
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
    ($($all:tt)*) => {parse_double_tag!(MeterTag, $($all)*) };
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
    ($($all:tt)*) => {parse_double_tag!(NavTag, $($all)*) };
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
    ($($all:tt)*) => {parse_double_tag!(NoscriptTag, $($all)*) };
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
    ($($all:tt)*) => {parse_double_tag!(OlTag, $($all)*) };
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
    ($($all:tt)*) => {parse_double_tag!(OptgroupTag, $($all)*) };
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
    ($($all:tt)*) => {parse_double_tag!(OptionTag, $($all)*) };
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
    ($($all:tt)*) => {parse_double_tag!(OutputTag, $($all)*) };
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
    ($($all:tt)*) => {parse_double_tag!(PTag, $($all)*) };
}

#[macro_export]
macro_rules! picture {
    ($($all:tt)*) => {parse_double_tag!(PictureTag, $($all)*) };
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
    ($($all:tt)*) => {parse_double_tag!(PreTag, $($all)*) };
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
    ($($all:tt)*) => {parse_double_tag!(ProgressTag, $($all)*) };
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
    ($($all:tt)*) => {parse_double_tag!(QTag, $($all)*) };
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
    ($($all:tt)*) => {parse_double_tag!(RpTag, $($all)*) };
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
    ($($all:tt)*) => {parse_double_tag!(RtTag, $($all)*) };
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
    ($($all:tt)*) => {parse_double_tag!(RubyTag, $($all)*) };
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
    ($($all:tt)*) => {parse_double_tag!(STag, $($all)*) };
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
    ($($all:tt)*) => {parse_double_tag!(SampTag, $($all)*) };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     script![.src="script.js", ._type="text/javascript"].render(),
///     "<script src=\"script.js\" type=\"text/javascript\"></script>"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! script {
    ($($all:tt)*) => {parse_double_tag!(ScriptTag, $($all)*) };
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
    ($($all:tt)*) => {parse_double_tag!(SectionTag, $($all)*) };
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
    ($($all:tt)*) => {parse_double_tag!(SelectTag, $($all)*) };
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
    ($($all:tt)*) => {parse_double_tag!(SmallTag, $($all)*) };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     source![.src="audio.ogg", ._type="audio/ogg"].render(),
///     "<source src=\"audio.ogg\" type=\"audio/ogg\">"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! source {
    ($($all:tt)*) => {parse_single_tag!(SourceTag, $($all)*) };
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
    ($($all:tt)*) => {parse_double_tag!(SpanTag, $($all)*) };
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
    ($($all:tt)*) => {parse_double_tag!(StrongTag, $($all)*) };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     style![._type="text/css", "body {background-color: black;} h1 {color: white;}"].render(),
///     "<style type=\"text/css\">body {background-color: black;} h1 {color: white;}</style>"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! style {
    ($($all:tt)*) => {parse_double_tag!(StyleTag, $($all)*) };
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
    ($($all:tt)*) => {parse_double_tag!(SubTag, $($all)*) };
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
    ($($all:tt)*) => {parse_double_tag!(SummaryTag, $($all)*) };
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
    ($($all:tt)*) => {parse_double_tag!(SupTag, $($all)*) };
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
    ($($all:tt)*) => {parse_double_tag!(TableTag, $($all)*) };
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
    ($($all:tt)*) => {parse_double_tag!(TbodyTag, $($all)*) };
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
    ($($all:tt)*) => {parse_double_tag!(TdTag, $($all)*) };
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
    ($($all:tt)*) => {parse_double_tag!(TemplateTag, $($all)*) };
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
    ($($all:tt)*) => {parse_double_tag!(TextareaTag, $($all)*) };
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
///             td!["Cell 1,1"]
///         ],
///         tfoot![
///             tr![
///                 td!["Footer 1"]
///             ]
///         ]
///     ].render(),
///     "<table><tr><td>Cell 1,1</td></tr><tfoot><tr><td>Footer 1</td></tr></tfoot></table>"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! tfoot {
    ($($all:tt)*) => {parse_double_tag!(TfootTag, $($all)*) };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     table![
///         thead![
///             tr![
///                 th!["Header 1"]
///             ]
///         ],
///         tbody![
///             tr![
///                 td!["Cell 1,1"]
///             ]
///         ],
///         tfoot![
///             tr![
///                 td!["Footer 1"]
///             ]
///         ]
///     ].render(),
///     "<table><thead><tr><th>Header 1</th></tr></thead><tbody><tr><td>Cell 1,1</td></tr></tbody><tfoot><tr><td>Footer 1</td></tr></tfoot></table>"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! th {
    ($($all:tt)*) => {parse_double_tag!(ThTag, $($all)*) };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     table![
///         thead![
///             tr![
///                 th!["Header 1"]
///             ]
///         ],
///         tbody![
///             tr![
///                 td!["Cell 1,1"]
///             ]
///         ],
///         tfoot![
///             tr![
///                 td!["Footer 1"]
///             ]
///         ]
///     ].render(),
///     "<table><thead><tr><th>Header 1</th></tr></thead><tbody><tr><td>Cell 1,1</td></tr></tbody><tfoot><tr><td>Footer 1</td></tr></tfoot></table>"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! thead {
    ($($all:tt)*) => {parse_double_tag!(TheadTag, $($all)*) };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     time![
///         .datetime="2022-07-05T18:00:00Z",
///         "July 5th, 2022 6:00 PM"
///     ].render(),
///     "<time datetime=\"2022-07-05T18:00:00Z\">July 5th, 2022 6:00 PM</time>"
///     );
/// # }
/// ```
#[macro_export]
macro_rules! time {
    ($($all:tt)*) => {parse_double_tag!(TimeTag, $($all)*) };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     title![
///         "Page Title"
///     ].render(),
///     "<title>Page Title</title>"
///     );
/// # }
/// ```
#[macro_export]
macro_rules! title {
    ($($all:tt)*) => {parse_double_tag!(TitleTag, $($all)*) };
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
///             td!["Cell 1,1"]
///         ],
///         tfoot![
///             tr![
///                 td!["Footer 1"]
///             ]
///         ]
///     ].render(),
///     "<table><tr><td>Cell 1,1</td></tr><tfoot><tr><td>Footer 1</td></tr></tfoot></table>"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! tr {
    ($($all:tt)*) => {parse_double_tag!(TrTag, $($all)*) };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     track![
///         ._kind="captions",
///         .src="https://example.com/captions.vtt",
///         .srclang="en",
///         .label="English Captions"
///     ].render(),
///     "<track kind=\"captions\" src=\"https://example.com/captions.vtt\" srclang=\"en\" label=\"English Captions\"></track>"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! track {
    ($($all:tt)*) => {parse_double_tag!(TrackTag, $($all)*) };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     u![
///         "Underlined text"
///     ].render(),
///     "<u>Underlined text</u>"
///     );
/// # }
/// ```
#[macro_export]
macro_rules! u {
    ($($all:tt)*) => {parse_double_tag!(UTag, $($all)*) };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     ul![
///         li!["Item 1"],
///         li!["Item 2"],
///         li!["Item 3"]
///     ].render(),
///     "<ul><li>Item 1</li><li>Item 2</li><li>Item 3</li></ul>"
///     );
/// # }
/// ```
#[macro_export]
macro_rules! ul {
    ($($all:tt)*) => {parse_double_tag!(UlTag, $($all)*) };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     var![ "math" ].render(),
///     "<var>math</var>"
///     );
/// # }
/// ```
#[macro_export]
macro_rules! var {
    ($($all:tt)*) => {parse_double_tag!(VarTag, $($all)*) };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     video![
///         .src="https://example.com/video.mp4",
///         .width="720",
///         .height="480",
///         .controls="true"
///     ].render(),
///     "<video src=\"https://example.com/video.mp4\" width=\"720\" height=\"480\" controls=\"true\"></video>"
///     );
/// # }
/// ```
#[macro_export]
macro_rules! video {
    ($($all:tt)*) => {parse_double_tag!(VideoTag, $($all)*) };
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///    wbr![].render(),
///    "<wbr>"
///    );
/// # }
/// ```
#[macro_export]
macro_rules! wbr {
    ($($all:tt)*) => {parse_single_tag!(WbrTag, $($all)*) };
}

#[macro_export]
macro_rules! parse_double_tag {
    () => {
        ""
    };
    (,$inner_left:expr $(,$inner:expr)*) => {
        render_fn!("{}{}", $inner_left, parse_double_tag!($(,$inner)*))
    };
    ($tag:ident,) => {
        render_fn!("<{}></{}>", $tag, $tag)
    };
    ($tag:ident $(,$inner:expr)*) => {
        render_fn!("<{}>{}</{}>", $tag, parse_double_tag!($(,$inner)*), $tag)
    };
    ($tag:ident $(,.$attr:ident = $value:expr)* $(,$inner:expr)*) => {
        render_fn!("<{}{}>{}</{}>", $tag, parse_attr!($tag $(,.$attr = $value)*), parse_double_tag!($(,$inner)*), $tag)
    };
}

#[macro_export]
macro_rules! parse_single_tag {
    ($tag:ident,) => {
        render_fn!("<{}>", $tag)
    };
    ($tag:ident $(,.$attr:ident = $value:expr)*) => {
        render_fn!("<{}{}>", $tag, parse_attr!($tag $(,.$attr = $value)*))
    };
}

#[macro_export]
macro_rules! parse_attr {
    () => { "" };
    ($tag:ident) => {
        ""
    };
    ($tag:ident, .$attr:ident = $value:expr $(,.$right_attr:ident$(-$nexts:ident)* = $right_expr:expr)*) => {{
        let ident = $attr;
        $tag.type_check(&ident);
        render_fn!(" {}{}{}", ident, parse_val!($value), parse_attr!($tag $(,.$right_attr = $right_expr)*))
    }};
}

#[macro_export]
macro_rules! parse_val {
    () => {
        ""
    };
    ($val:expr) => {
        render_fn!("=\"{}\"", $val)
    };
}
