use crate::*;

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
        tag_inner!(ATag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(ATag, $inner_left $(,$inner)*)
    };
}

#[test]
fn test_a() {
    assert_eq!(a! {}.render(), "<a></a>");
    assert_eq!(
        a! { if true { "Link Text"} else {"Link Bad" }}.render(),
        "<a>Link Text</a>"
    );
    assert_eq!(
        a! {"Link Text", a!["inner"], "more"}.render(),
        "<a>Link Text<a>inner</a>more</a>"
    );
    assert_eq!(a! {.href="link"}.render(), "<a href=\"link\"></a>");
    assert_eq!(
        a! {.href="link","Text", "Another"}.render(),
        "<a href=\"link\">TextAnother</a>"
    );
    assert_eq!(
        a! {.href="link","Text", a!{"Nested"}}.render(),
        "<a href=\"link\">Text<a>Nested</a></a>"
    );
    assert_eq!(
        a! {.href="link", .download="yes","Text", a!{"Nested"}}.render(),
        "<a href=\"link\" download=\"yes\">Text<a>Nested</a></a>"
    );
}

/// # Example
/// ```
/// # #[macro_use] extern crate rtml;
/// # fn main() {
/// use rtml::*;
///
/// assert_eq!(
///     abbr![.title="World Health Organization", "WHO"].render(),
///     "<abbr title=\"World Health Organization\">WHO</abbr>"
/// );
///
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

#[macro_export]
macro_rules! area {
    () => {tag_inner!(AreaTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(AreaTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(AreaTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! article {
    () => {tag_inner!(ArticleTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(ArticleTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(ArticleTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! aside {
    () => {tag_inner!(AsideTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(AsideTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(AsideTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! audio {
    () => {tag_inner!(AudioTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(AudioTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(AudioTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! b {
    () => {tag_inner!(BTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(BTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(BTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! base {
    () => {tag_inner!(BaseTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(BaseTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(BaseTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! bdi {
    () => {tag_inner!(BdiTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(BdiTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(BdiTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! bdo {
    () => {tag_inner!(BdoTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(BdoTag,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(BdoTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! blockquote {
    () => {tag_inner!(BlockquoteTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(BlockquoteTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(BlockquoteTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! body {
    () => {tag_inner!(BodyTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(BodyTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(BodyTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! br {
    () => {
        format_args!("<{}>", BrTag)
    };
}

#[macro_export]
macro_rules! button {
    () => {tag_inner!(ButtonTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(ButtonTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(ButtonTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! canvas {
    () => {tag_inner!(CanvasTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(CanvasTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(CanvasTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! caption {
    () => {tag_inner!(CaptionTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(CaptionTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(CaptionTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! cite {
    () => {tag_inner!(CiteTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(CiteTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(CiteTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! code {
    () => {tag_inner!(CodeTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(CodeTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(CodeTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! col {
    () => {tag_inner!(ColTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(ColTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(ColTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! colgroup {
    () => {tag_inner!(ColgroupTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(ColgroupTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(ColgroupTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! data {
    () => {tag_inner!(DataTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(DataTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(DataTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! datalist {
    () => {tag_inner!(DatalistTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(DatalistTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(DatalistTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! dd {
    () => {tag_inner!(DdTagTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(DdTagTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(DdTagTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! del {
    () => {tag_inner!(DelTagTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(DelTagTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(DelTagTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! details {
    () => {tag_inner!(DetailsTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(DetailsTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(DetailsTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! dfn {
    () => {tag_inner!(DfnTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(DfnTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(DfnTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! dialog {
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
///     div!{ "This is a div's inner text!" }.render(),
///     "<div>This is a div's inner text!</div>"
/// );
///
/// //assert_eq!(
/// //    div!{ .if true, "This is a div's conditional inner text!" }.render(),
/// //    "<div>This is a div's conditional inner text!</div>"
/// //);
/// //assert_eq!(
/// //    div!{ .if false, "This isn't going to render" }.render(),
/// //    ""
/// //);
///
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

#[macro_export]
macro_rules! dl {
    () => {tag_inner!(ColgroupTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(ColgroupTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(ColgroupTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! dt {
    () => {tag_inner!(ColgroupTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(ColgroupTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(ColgroupTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! em {
    () => {tag_inner!(ColgroupTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(ColgroupTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(ColgroupTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! embed {
    () => {tag_inner!(ColgroupTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(ColgroupTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(ColgroupTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! fieldset {
    () => {tag_inner!(ColgroupTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(ColgroupTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(ColgroupTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! figcaption {
    () => {tag_inner!(ColgroupTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(ColgroupTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(ColgroupTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! figure {
    () => {tag_inner!(ColgroupTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(ColgroupTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(ColgroupTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! footer {
    () => {tag_inner!(ColgroupTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(ColgroupTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(ColgroupTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! form {
    () => {tag_inner!(ColgroupTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(ColgroupTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(ColgroupTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! h1 {
    () => {tag_inner!(ColgroupTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(ColgroupTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(ColgroupTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! h2 {
    () => {tag_inner!(ColgroupTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(ColgroupTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(ColgroupTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! h3 {
    () => {tag_inner!(ColgroupTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(ColgroupTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(ColgroupTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! h4 {
    () => {tag_inner!(ColgroupTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(ColgroupTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(ColgroupTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! h5 {
    () => {tag_inner!(ColgroupTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(ColgroupTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(ColgroupTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! h6 {
    () => {tag_inner!(ColgroupTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(ColgroupTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(ColgroupTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! head {
    () => {tag_inner!(ColgroupTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(ColgroupTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(ColgroupTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! header {
    () => {tag_inner!(ColgroupTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(ColgroupTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(ColgroupTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! hgroup {
    () => {tag_inner!(ColgroupTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(ColgroupTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(ColgroupTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! hr {
    () => {tag_inner!(ColgroupTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(ColgroupTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(ColgroupTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! html {
    () => {tag_inner!(ColgroupTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(ColgroupTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(ColgroupTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! i {
    () => {tag_inner!(ColgroupTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(ColgroupTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(ColgroupTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! iframe {
    () => {tag_inner!(ColgroupTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(ColgroupTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(ColgroupTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! img {
    () => {tag_inner!(ColgroupTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(ColgroupTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(ColgroupTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! input {
    () => {tag_inner!(ColgroupTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(ColgroupTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(ColgroupTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! ins {
    () => {tag_inner!(ColgroupTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(ColgroupTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(ColgroupTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! kbd {
    () => {tag_inner!(ColgroupTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(ColgroupTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(ColgroupTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! label {
    () => {tag_inner!(ColgroupTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(ColgroupTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(ColgroupTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! legend {
    () => {tag_inner!(ColgroupTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(ColgroupTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(ColgroupTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! li {
    () => {tag_inner!(ColgroupTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(ColgroupTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(ColgroupTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! main {
    () => {tag_inner!(ColgroupTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(ColgroupTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(ColgroupTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! map {
    () => {tag_inner!(ColgroupTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(ColgroupTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(ColgroupTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! mark {
    () => {tag_inner!(ColgroupTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(ColgroupTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(ColgroupTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! menu {
    () => {tag_inner!(ColgroupTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(ColgroupTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(ColgroupTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! menuitem {
    () => {tag_inner!(ColgroupTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(ColgroupTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(ColgroupTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! meta {
    () => {tag_inner!(ColgroupTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(ColgroupTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(ColgroupTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! meter {
    () => {tag_inner!(ColgroupTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(ColgroupTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(ColgroupTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! nav {
    () => {tag_inner!(ColgroupTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(ColgroupTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(ColgroupTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! noscript {
    () => {tag_inner!(ColgroupTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(ColgroupTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(ColgroupTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! ol {
    () => {tag_inner!(ColgroupTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(ColgroupTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(ColgroupTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! optgroup {
    () => {tag_inner!(ColgroupTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(ColgroupTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(ColgroupTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! option {
    () => {tag_inner!(ColgroupTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(ColgroupTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(ColgroupTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! output {
    () => {tag_inner!(ColgroupTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(ColgroupTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(ColgroupTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! p {
    () => {tag_inner!(ColgroupTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(ColgroupTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(ColgroupTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! param {
    () => {tag_inner!(ColgroupTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(ColgroupTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(ColgroupTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! picture {
    () => {tag_inner!(ColgroupTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(ColgroupTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(ColgroupTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! pre {
    () => {tag_inner!(ColgroupTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(ColgroupTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(ColgroupTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! progress {
    () => {tag_inner!(ColgroupTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(ColgroupTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(ColgroupTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! q {
    () => {tag_inner!(ColgroupTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(ColgroupTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(ColgroupTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! rp {
    () => {tag_inner!(ColgroupTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(ColgroupTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(ColgroupTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! rt {
    () => {tag_inner!(ColgroupTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(ColgroupTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(ColgroupTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! ruby {
    () => {tag_inner!(ColgroupTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(ColgroupTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(ColgroupTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! s {
    () => {tag_inner!(ColgroupTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(ColgroupTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(ColgroupTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! samp {
    () => {tag_inner!(ColgroupTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(ColgroupTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(ColgroupTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! script {
    () => {tag_inner!(ColgroupTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(ColgroupTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(ColgroupTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! section {
    () => {tag_inner!(ColgroupTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(ColgroupTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(ColgroupTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! select {
    () => {tag_inner!(ColgroupTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(ColgroupTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(ColgroupTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! small {
    () => {tag_inner!(ColgroupTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(ColgroupTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(ColgroupTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! source {
    () => {tag_inner!(ColgroupTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(ColgroupTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(ColgroupTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! span {
    () => {tag_inner!(ColgroupTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(ColgroupTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(ColgroupTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! strong {
    () => {tag_inner!(ColgroupTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(ColgroupTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(ColgroupTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! style {
    () => {tag_inner!(ColgroupTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(ColgroupTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(ColgroupTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! sub {
    () => {tag_inner!(ColgroupTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(ColgroupTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(ColgroupTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! summary {
    () => {tag_inner!(ColgroupTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(ColgroupTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(ColgroupTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! sup {
    () => {tag_inner!(ColgroupTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(ColgroupTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(ColgroupTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! table {
    () => {tag_inner!(ColgroupTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(ColgroupTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(ColgroupTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! tbody {
    () => {tag_inner!(ColgroupTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(ColgroupTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(ColgroupTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! td {
    () => {tag_inner!(ColgroupTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(ColgroupTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(ColgroupTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! template {
    () => {tag_inner!(ColgroupTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(ColgroupTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(ColgroupTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! textarea {
    () => {tag_inner!(ColgroupTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(ColgroupTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(ColgroupTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! tfoot {
    () => {tag_inner!(ColgroupTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(ColgroupTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(ColgroupTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! th {
    () => {tag_inner!(ColgroupTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(ColgroupTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(ColgroupTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! thead {
    () => {tag_inner!(ColgroupTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(ColgroupTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(ColgroupTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! time {
    () => {tag_inner!(ColgroupTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(ColgroupTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(ColgroupTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! title {
    () => {tag_inner!(ColgroupTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(ColgroupTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(ColgroupTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! tr {
    () => {tag_inner!(ColgroupTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(ColgroupTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(ColgroupTag, $inner_left $(,$inner)*)
    };
}

#[macro_export]
macro_rules! track {
    () => {tag_inner!(ColgroupTag) };
    ( .$attr_left:ident = $value_left:expr $(,.$attr:ident = $value:expr)* $(,$inner:expr)* ) => {
        tag_inner!(ColgroupTag ,.$attr_left = $value_left $(,.$attr = $value)* $(,$inner)*)
    };
    ( $inner_left:expr $(,$inner:expr)*) => { tag_inner!(ColgroupTag, $inner_left $(,$inner)*)
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
    ($tag:ident) => {
        format_args!("<{}></{}>", $tag, $tag)
    };
    ($tag:ident $(,$inner:expr)+) => {
        format_args!("<{}>{}</{}>", $tag, tag_inner!($(,$inner)*), $tag)
    };
    ($tag:ident $(,.$attr:ident = $value:expr)*) => {
        format_args!("<{}{}></{}>", $tag, attr_inner!($(,.$attr = $value)*), $tag)
    };
    ($tag:ident $(,.$attr:ident = $value:expr)* $(,$inner:expr)*) => {
        format_args!("<{}{}>{}</{}>", $tag, attr_inner!($(,.$attr = $value)*), tag_inner!($(,$inner)*), $tag)
    };
}

#[test]
fn test_tag_inner() {
    assert_eq!(tag_inner!(ATag).render(), "<a></a>");
    assert_eq!(tag_inner!(ATag, "inner").render(), "<a>inner</a>");
    assert_eq!(
        tag_inner!(ATag, .href = "link", .download= "file.html").render(),
        "<a href=\"link\" download=\"file.html\"></a>"
    );
    assert_eq!(
        tag_inner!(ATag, .href = "link", .download= "file.html", "Cool Link").render(),
        "<a href=\"link\" download=\"file.html\">Cool Link</a>"
    );
    assert_eq!(
        tag_inner!(ATag, .href = "link", .download= "file.html", "Cool Link", tag_inner!(ATag))
            .render(),
        "<a href=\"link\" download=\"file.html\">Cool Link<a></a></a>"
    );
}

#[macro_export]
macro_rules! attr_inner {
    () => { format_args!("{}", "") };
    (.$attr:ident = $value:expr) => {
        format_args!(" {}=\"{}\"", $attr, $value)
    };
    (,.$attr:ident = $value:expr $(,.$right_attr:ident = $right_expr:expr)*) => {
        format_args!(" {}=\"{}\"{}", $attr, $value, attr_inner!($(,.$right_attr = $right_expr)*))
    };
    (.$attr:ident = $value:expr $(,.$right_attr:ident = $right_expr:expr)*) => {
        format_args!(" {}=\"{}\"{}", $attr, $value, attr_inner!($(,.$right_attr = $right_expr)*))
    };
}

#[test]
fn test_attr_inner() {
    assert_eq!(attr_inner!(.href = "link").render(), " href=\"link\"");
    assert_eq!(
        attr_inner!(.href = "link", .download = "yes please", .hreflang="en").render(),
        " href=\"link\" download=\"yes please\" hreflang=\"en\""
    );
    assert_eq!(attr_inner!().render(), "");
}
