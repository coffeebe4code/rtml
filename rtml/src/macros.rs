use crate::*;

///// # Example
///// ```
///// # #[macro_use] extern crate rtml;
///// # fn main() {
///// use rtml::*;
/////
///// assert_eq!(
/////     a![.href="https://www.example.com", "Link text"].render(),
/////     "<a href=\"https://www.example.com\">Link text</a>"
///// );
/////
///// assert_eq!(
/////     a![.href="/path/to/page", "Click here"].render(),
/////     "<a href=\"/path/to/page\">Click here</a>"
///// );
///// assert_eq!(
/////     a![.href="mailto:user@example.com", "Send email"].render(),
/////     "<a href=\"mailto:user@example.com\">Send email</a>"
///// );
/////
///// # }
///// ```
//#[macro_export]
//macro_rules! a {
//    () => { tag_inner!(ATag) };
//    ( $(.$attr:ident = $value:expr,)+) => {
//        tag_inner!(ATag, $(.$attr = $value,)*)
//    };
//    ( $(.$attr:ident = $value:expr,)+ $($inner:expr,)+) => {
//        tag_inner!(ATag, $(.$attr = $value,)* tag_inner!($($inner)*))
//    };
//    ( $($inner:expr)+) => {
//        tag_inner!(ATag, $($inner)*)
//    };
//}

//#[test]
//fn test_a() {
//    assert_eq!(a! {}.render(), "<a></a>");
//    assert_eq!(a! {"Link Text"}.render(), "<a>Link Text</a>");
//    assert_eq!(a! {.href="link",}.render(), "<a href=\"link\"></a>");
//    assert_eq!(
//        a! {.href="link","Text",}.render(),
//        "<a href=\"link\">Text</a>"
//    );
//    assert_eq!(
//        a! {.href="link","Text", a!{"Nested"},}.render(),
//        "<a href=\"link\">Text<a>Nested</a></a>"
//    );
//}

#[macro_export]
macro_rules! abbr {
    ($($inner:tt)* ) => {
        format_args!("<{}>{}</{}>", AbbrTag, $($inner)*, AbbrTag)
    };
    ( .title = $val:expr, $($inner:tt)* ) => {
        format_args!("<{} {}{}\">{}</{}>", AbbrTag, TitleAttr, String::from($val), $($inner)*, AbbrTag)
    };
}

#[macro_export]
macro_rules! address {
    ( $($inner:tt)* ) => {
        concat!("<address>", $($inner)*,"</address>")
    };
}

#[macro_export]
macro_rules! area {
    ( .alt = $val:expr, .coords = $coords:expr, .href = $href:expr, .shape = $shape:expr ) => {
        concat!(
            "<area alt='",
            $val,
            "' coords='",
            $coords,
            "' href='",
            $href,
            "' shape='",
            $shape,
            "'>"
        )
    };
}

#[macro_export]
macro_rules! article {
    ( $($inner:tt)* ) => {
        concat!("<article>", $($inner)*,"</article>")
    };
}

#[macro_export]
macro_rules! aside {
    ( $($inner:tt)* ) => {
        concat!("<aside>", $($inner)*,"</aside>")
    };
}

#[macro_export]
macro_rules! audio {
    ( .src = $src:expr, $($inner:tt)* ) => {
        concat!("<audio src='", $src, "'>", $($inner)*,"</audio>")
    };
}

#[macro_export]
macro_rules! b {
    ( $($inner:tt)* ) => {
        concat!("<b>", $($inner)*, "</b>")
    };
}

#[macro_export]
macro_rules! base {
    ( .href = $val:expr ) => {
        concat!("<base href='", $val, "'>")
    };
}

#[macro_export]
macro_rules! bdi {
    ( .dir = $val:expr, $($inner:tt)* ) => {
        concat!("<bdi dir='", $val, "'>", $($inner)*, "</bdi>")
    };
}

#[macro_export]
macro_rules! bdo {
    ( .dir = $val:expr, $($inner:tt)* ) => {
        concat!("<bdo dir='", $val, "'>", $($inner)*, "</bdo>")
    };
}

#[macro_export]
macro_rules! blockquote {
    ( .cite = $val:expr, $($inner:tt)* ) => {
        concat!("<blockquote cite='", $val, "'>", $($inner)*,"</blockquote>")
    };
}

#[macro_export]
macro_rules! body {
    ( $($inner:tt)* ) => {
        concat!("<body>", $($inner)*,"</body>")
	};
}

#[macro_export]
macro_rules! button {
    ( $($inner:tt)* ) => {
        concat!("<button>", $($inner)*, "</button>")
    };
    ( .form = $form:expr, $($inner:tt)*) => {
        concat!("<button form='", $form, "'>", $($inner)*, "</button>")
    };
}

#[macro_export]
macro_rules! canvas {
    ( .height = $height:expr, .width = $width:expr ) => {
        concat!(
            "<canvas height='",
            $height,
            "' width='",
            $width,
            "'></canvas>"
        )
    };
}

#[macro_export]
macro_rules! caption {
    ( $($inner:tt)* ) => {
        concat!("<caption>", $($inner)*,"</caption>")
    };
}

#[macro_export]
macro_rules! cite {
    ( $text:expr ) => {
        concat!("<cite>", $text, "</cite>")
    };
}

#[macro_export]
macro_rules! code {
    ( $text:expr ) => {
        concat!("<code>", $text, "</code>")
    };
}

#[macro_export]
macro_rules! col {
    ( .span = $val:expr ) => {
        concat!("<col span='", $val, "'>")
    };
}

#[macro_export]
macro_rules! colgroup {
    ( .span = $val:expr, $($inner:tt)* ) => {
        concat!("<colgroup span='", $val, "'>", $($inner)*,"</colgroup>")
    };
}

#[macro_export]
macro_rules! data {
    ( .value = $val:expr,  $($inner:tt)* ) => {
        concat!("<data value='", $val, "'>", $($inner)*, "</data>")
    };
}

#[macro_export]
macro_rules! datalist {
    ( $($inner:tt)* ) => {
        concat!("<datalist>", $($inner)*,"</datalist>")
    };
}

#[macro_export]
macro_rules! dd {
    ( $($inner:tt)* ) => {
        concat!("<dd>", $($inner)*,"</dd>")
    };
}

#[macro_export]
macro_rules! del {
    ( .cite = $cite:expr, .datetime = $datetime:expr, $($inner:tt)* ) => {
        concat!("<del cite='", $cite, "' datetime='", $datetime, "'>", $($inner)*,"</del>")
    };
}

#[macro_export]
macro_rules! details {
    ( $($inner:tt)* ) => {
        concat!("<details>", $($inner)*,"</details>")
    };
}

#[macro_export]
macro_rules! dfn {
    ( $text:expr ) => {
        concat!("<dfn>", $text, "</dfn>")
    };
}

#[macro_export]
macro_rules! dialog {
    ( $($inner:tt)* ) => {
        concat!("<dialog>", $($inner)*,"</dialog>")
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
    ($text:expr) => {
        $text.to_format()
    };
    //($head_text$($inner:tt)*) => {
    //    tag_inner!(DivTag, "", $($inner)*)
    //};
    ( $(.$attr:ident = $val:expr)*, $($inner:tt)*) => {
        tag_inner!(DivTag,attr_inner!($($attr,$val)*), $($inner)*)
    };
}

#[macro_export]
macro_rules! dl {
    ( $($inner:tt)* ) => {
        concat!("<dl>", $($inner)*,"</dl>")
    };
}

#[macro_export]
macro_rules! dt {
    ( $($inner:tt)* ) => {
        concat!("<dt>", $($inner)*,"</dt>")
    };
}

#[macro_export]
macro_rules! em {
    ( $($inner:tt)* ) => {
        concat!("<em>", $($inner)*, "</em>")
    };
}

#[macro_export]
macro_rules! embed {
    ( .src = $src:expr, .type = $typ:expr ) => {
        concat!("<embed src='", $src, "' type='", $typ, "'>")
    };
}

#[macro_export]
macro_rules! fieldset {
    ( $($inner:tt)* ) => {
        concat!("<fieldset>", $($inner)*,"</fieldset>")
    };
}

#[macro_export]
macro_rules! figcaption {
    ( $($inner:tt)* ) => {
        concat!("<figcaption>", $($inner)*,"</figcaption>")
    };
}

#[macro_export]
macro_rules! figure {
    ( $($inner:tt)* ) => {
        concat!("<figure>", $($inner)*,"</figure>")
    };
}

#[macro_export]
macro_rules! footer {
    ( $($inner:tt)* ) => {
        concat!("<footer>", $($inner)*,"</footer>")
    };
}

#[macro_export]
macro_rules! form {
    ( .action = $val:expr, .method = $method:expr, $($inner:tt)* ) => {
        concat!("<form action='", $val, "' method='", $method, "'>", $($inner)*,"</form>")
    };
}

#[macro_export]
macro_rules! h1 {
    ( $($inner:tt)* ) => {
        concat!("<h1>", $($inner)*, "</h1>")
    };
}

#[macro_export]
macro_rules! h2 {
    ( $($inner:tt)* ) => {
        concat!("<h2>", $($inner)*, "</h2>")
    };
}

#[macro_export]
macro_rules! h3 {
    ( $($inner:tt)* ) => {
        concat!("<h3>", $($inner)*, "</h3>")
    };
}

#[macro_export]
macro_rules! h4 {
    ( $($inner:tt)* ) => {
        concat!("<h4>", $($inner)*, "</h4>")
    };
}

#[macro_export]
macro_rules! h5 {
    ( $($inner:tt)* ) => {
        concat!("<h5>", $($inner)*, "</h5>")
    };
}

#[macro_export]
macro_rules! h6 {
    ( $($inner:tt)* ) => {
        concat!("<h6>", $($inner)*, "</h6>")
    };
}

#[macro_export]
macro_rules! head {
    ( $($inner:tt)* ) => {
        concat!("<head>", $($inner)*,"</head>")
    };
}

#[macro_export]
macro_rules! header {
    ( $($inner:tt)* ) => {
        concat!("<header>", $($inner)*,"</header>")
    };
}

#[macro_export]
macro_rules! hgroup {
    ( $($inner:tt)* ) => {
        concat!("<hgroup>", $($inner)*,"</hgroup>")
    };
}

#[macro_export]
macro_rules! hr {
    () => {
        "<hr>"
    };
}

#[macro_export]
macro_rules! html {
    ( $($inner:tt)* ) => {
        format_args!("<!DOCTYPE html><html>{}</html>", $($inner)*).render()
    };
    ( .lang = $lang:expr, $($inner:tt)* ) => {
        format_args!("<!DOCTYPE html><html lang=\"{}\">{}</html>", $lang, $($inner)*).render()
    };
}

#[macro_export]
macro_rules! i {
    ( $($inner:tt)* ) => {
        concat!("<i>", $($inner)*, "</i>")
    };
}

#[macro_export]
macro_rules! iframe {
    ( .src = $src:expr, .height = $height:expr, .width = $width:expr ) => {
        concat!(
            "<iframe src='",
            $src,
            "' height='",
            $height,
            "' width='",
            $width,
            "'></iframe>"
        )
    };
}

#[macro_export]
macro_rules! img {
    ( .alt = $alt:expr, .src = $src:expr ) => {
        concat!("<img alt='", $alt, "' src='", $src, "'>")
    };
}

#[macro_export]
macro_rules! input {
    ( .type = $typ:expr ) => {
        concat!("<input type='", $typ, "'>")
    };
    ( .type = $typ:expr, .name = $name:expr ) => {
        concat!("<input type='", $typ, "' name='", $name, "'>")
    };
    ( .type = $typ:expr, .value = $value:expr ) => {
        concat!("<input type='", $typ, "' value='", $value, "'>")
    };
    ( .type = $typ:expr, .name = $name:expr, .value = $value:expr ) => {
        concat!(
            "<input type='",
            $typ,
            "' name='",
            $name,
            "' value='",
            $value,
            "'>"
        )
    };
}

#[macro_export]
macro_rules! ins {
    ( .cite = $cite:expr, .datetime = $datetime:expr, $($inner:tt)* ) => {
        concat!("<ins cite='", $cite, "' datetime='", $datetime, "'>", $($inner)*,"</ins>")
    };
}

#[macro_export]
macro_rules! kbd {
    ( $text:expr ) => {
        concat!("<kbd>", $text, "</kbd>")
    };
}

#[macro_export]
macro_rules! label {
    ( .for = $val:expr, $text:expr ) => {
        concat!("<label for='", $val, "'>", $text, "</label>")
    };
}

#[macro_export]
macro_rules! legend {
    ( $($inner:tt)* ) => {
        concat!("<legend>", $($inner)*,"</legend>")
    };
}

#[macro_export]
macro_rules! li {
    ( $($inner:tt)* ) => {
        concat!("<li>", $($inner)*,"</li>")
    };
}

#[macro_export]
macro_rules! main {
    ( $($inner:tt)* ) => {
        concat!("<main>", $($inner)*,"</main>")
    };
}

#[macro_export]
macro_rules! map {
    ( .name = $name:expr, $($inner:tt)* ) => {
        concat!("<map name='", $name, "'>", $($inner)*,"</map>")
    };
}

#[macro_export]
macro_rules! mark {
    ( $text:expr ) => {
        concat!("<mark>", $text, "</mark>")
    };
}

#[macro_export]
macro_rules! menu {
    ( $($inner:tt)* ) => {
        concat!("<menu>", $($inner)*,"</menu>")
    };
}

#[macro_export]
macro_rules! menuitem {
    ( $text:expr ) => {
        concat!("<menuitem>", $text, "</menuitem>")
    };
    ( .type = $typ:expr, .label = $label:expr ) => {
        concat!("<menuitem type='", $typ, "' label='", $label, "'>")
    };
}

#[macro_export]
macro_rules! meta {
    ( .name = $name:expr, .content = $content:expr ) => {
        concat!("<meta name='", $name, "' content='", $content, "'>")
    };
    ( .charset = $charset:expr ) => {
        concat!("<meta charset='", $charset, "'>")
    };
}

#[macro_export]
macro_rules! meter {
    ( .value = $val:expr, .min = $min:expr, .max = $max:expr ) => {
        concat!(
            "<meter value='",
            $val,
            "' min='",
            $min,
            "' max='",
            $max,
            "'></meter>"
        )
    };
    ( .value = $val:expr, .min = $min:expr, .max = $max:expr, .low = $low:expr, .high = $high:expr ) => {
        concat!(
            "<meter value='",
            $val,
            "' min='",
            $min,
            "' max='",
            $max,
            "' low='",
            $low,
            "' high='",
            $high,
            "'></meter>"
        )
    };
    ( .value = $val:expr, .min = $min:expr, .max = $max:expr, .low = $low:expr, .high = $high:expr, .optimum = $optimum:expr ) => {
        concat!(
            "<meter value='",
            $val,
            "' min='",
            $min,
            "' max='",
            $max,
            "' low='",
            $low,
            "' high='",
            $high,
            "' optimum='",
            $optimum,
            "'></meter>"
        )
    };
}

#[macro_export]
macro_rules! nav {
    ( $($inner:tt)* ) => {
        concat!("<nav>", $($inner)*,"</nav>")
    };
}

#[macro_export]
macro_rules! noscript {
    ( $($inner:tt)* ) => {
        concat!("<noscript>", $($inner)*,"</noscript>")
    };
}

#[macro_export]
macro_rules! ol {
    ( $($inner:tt)* ) => {
        concat!("<ol>", $($inner)*,"</ol>")
    };
}

#[macro_export]
macro_rules! optgroup {
    ( .label = $label:expr, $($inner:tt)* ) => {
        concat!("<optgroup label='", $label, "'>", $($inner)*,"</optgroup>")
    };
}

#[macro_export]
macro_rules! option {
    ( $text:expr ) => {
        concat!("<option>", $text, "</option>")
    };
    ( .value = $value:expr, $text:expr ) => {
        concat!("<option value='", $value, "'>", $text, "</option>")
    };
}

#[macro_export]
macro_rules! output {
    ( .for = $val:expr, $text:expr ) => {
        concat!("<output for='", $val, "'>", $text, "</output>")
    };
    ( .name = $name:expr, $text:expr ) => {
        concat!("<output name='", $name, "'>", $text, "</output>")
    };
}

#[macro_export]
macro_rules! p {
    ( $($inner:tt)* ) => {
        concat!("<p>", $($inner)*, "</p>")
    };
}

#[macro_export]
macro_rules! param {
    ( .name = $name:expr, .value = $value:expr ) => {
        concat!("<param name='", $name, "' value='", $value, "'>")
    };
}

#[macro_export]
macro_rules! picture {
    ( $($inner:tt)* ) => {
        concat!("<picture>", $($inner)*,"</picture>")
    };
}

#[macro_export]
macro_rules! pre {
    ( $($inner:tt)* ) => {
        concat!("<pre>", $($inner)*,"</pre>")
    };
}

#[macro_export]
macro_rules! progress {
    ( .value = $val:expr, .max = $max:expr ) => {
        concat!("<progress value='", $val, "' max='", $max, "'></progress>")
    };
}

#[macro_export]
macro_rules! q {
    ( .cite = $cite:expr, $text:expr ) => {
        concat!("<q cite='", $cite, "'>", $text, "</q>")
    };
}

#[macro_export]
macro_rules! rp {
    ( $($inner:tt)* ) => {
        concat!("<rp>", $($inner)*,"</rp>")
    };
}

#[macro_export]
macro_rules! rt {
    ( $text:expr ) => {
        concat!("<rt>", $text, "</rt>")
    };
}

#[macro_export]
macro_rules! ruby {
    ( $($inner:tt)* ) => {
        concat!("<ruby>", $($inner)*,"</ruby>")
    };
}

#[macro_export]
macro_rules! s {
    ( $text:expr ) => {
        concat!("<s>", $text, "</s>")
    };
}

#[macro_export]
macro_rules! samp {
    ( $text:expr ) => {
        concat!("<samp>", $text, "</samp>")
    };
}

#[macro_export]
macro_rules! script {
    ( .src = $src:expr ) => {
        concat!("<script src='", $src, "'></script>")
    };
}

#[macro_export]
macro_rules! section {
    ( $($inner:tt)* ) => {
        concat!("<section>", $($inner)*,"</section>")
    };
}

#[macro_export]
macro_rules! select {
    ( .name = $name:expr, $($inner:tt)* ) => {
        concat!("<select name='", $name, "'>", $($inner)*,"</select>")
    };
    ( .name = $name:expr, .required = true, $($inner:tt)* ) => {
        concat!("<select name='", $name, "' required>", $($inner)*,"</select>")
    };
}

#[macro_export]
macro_rules! small {
    ( $text:expr ) => {
        concat!("<small>", $text, "</small>")
    };
}

#[macro_export]
macro_rules! source {
    ( .src = $src:expr, .type = $typ:expr ) => {
        concat!("<source src='", $src, "' type='", $typ, "'>")
    };
    ( .srcset = $srcset:expr, .type = $typ:expr ) => {
        concat!("<source srcset='", $srcset, "' type='", $typ, "'>")
    };
    ( .src = $src:expr, .srcset = $srcset:expr, .type = $typ:expr ) => {
        concat!(
            "<source src='",
            $src,
            "' srcset='",
            $srcset,
            "' type='",
            $typ,
            "'>"
        )
    };
}

#[macro_export]
macro_rules! span {
    ( $($inner:tt)* ) => {
        concat!("<span>", $($inner)*,"</span>")
    };
}

#[macro_export]
macro_rules! strong {
    ( $text:expr ) => {
        concat!("<strong>", $text, "</strong>")
    };
}

#[macro_export]
macro_rules! style {
    ( .type = $typ:expr, $text:expr ) => {
        concat!("<style type='", $typ, "'>", $text, "</style>")
    };
}

#[macro_export]
macro_rules! sub {
    ( $text:expr ) => {
        concat!("<sub>", $text, "</sub>")
    };
}

#[macro_export]
macro_rules! summary {
    ( $($inner:tt)* ) => {
        concat!("<summary>", $($inner)*, "</summary>")
    };
}

#[macro_export]
macro_rules! sup {
    ( $text:expr ) => {
        concat!("<sup>", $text, "</sup>")
    };
}

#[macro_export]
macro_rules! table {
    ( $($inner:tt)* ) => {
        concat!("<table>", $($inner)*,"</table>")
    };
}

#[macro_export]
macro_rules! tbody {
    ( $($inner:tt)* ) => {
        concat!("<tbody>", $($inner)*,"</tbody>")
    };
}

#[macro_export]
macro_rules! td {
    ( $($inner:tt)* ) => {
        concat!("<td>", $($inner)*, "</td>")
    };
}

#[macro_export]
macro_rules! template {
    ( $($inner:tt)* ) => {
        concat!("<template>", $($inner)*,"</template>")
    };
}

#[macro_export]
macro_rules! textarea {
    ( .name = $name:expr, $($inner:tt)* ) => {
        concat!("<textarea name='", $name, "'>", $($inner)*, "</textarea>")
    };
    ( .name = $name:expr, .cols = $cols:expr, .rows = $rows:expr, $($inner:tt)* ) => {
        concat!(
            "<textarea name='",
            $name,
            "' cols='",
            $cols,
            "' rows='",
            $rows,
            "'>",
            $($inner)*,
            "</textarea>"
        )
    };
}

#[macro_export]
macro_rules! tfoot {
    ( $($inner:tt)* ) => {
        concat!("<tfoot>", $($inner)*,"</tfoot>")
    };
}

#[macro_export]
macro_rules! th {
    ( $($inner:tt)* ) => {
        concat!("<th>", $($inner)*, "</th>")
    };
}

#[macro_export]
macro_rules! thead {
    ( $($inner:tt)* ) => {
        concat!("<thead>", $($inner)*,"</thead>")
    };
}

#[macro_export]
macro_rules! time {
    ( .datetime = $datetime:expr, $text:expr ) => {
        concat!("<time datetime='", $datetime, "'>", $text, "</time>")
    };
}

#[macro_export]
macro_rules! title {
    ( $text:expr ) => {
        concat!("<title>", $text, "</title>")
    };
}

#[macro_export]
macro_rules! tr {
    ( $($inner:tt)* ) => {
        concat!("<tr>", $($inner)*,"</tr>")
    };
}

#[macro_export]
macro_rules! track {
    ( .src = $src:expr, .kind = $kind:expr ) => {
        concat!("<track src='", $src, "' kind='", $kind, "'>")
    };
    ( .src = $src:expr, .kind = $kind:expr, .srclang = $srclang:expr ) => {
        concat!(
            "<track src='",
            $src,
            "' kind='",
            $kind,
            "' srclang='",
            $srclang,
            "'>"
        )
    };
    ( .src = $src:expr, .kind = $kind:expr, .srclang = $srclang:expr, .label = $label:expr ) => {
        concat!(
            "<track src='",
            $src,
            "' kind='",
            $kind,
            "' srclang='",
            $srclang,
            "' label='",
            $label,
            "'>"
        )
    };
}

#[macro_export]
macro_rules! u {
    ( $($inner:tt)* ) => {
        concat!("<u>", $($inner)*, "</u>")
    };
}

#[macro_export]
macro_rules! ul {
    ( .iter $closure:expr, $($inner:tt)* ) => {
        format_args!("<ul>{}{}</ul>", $closure, $($inner)*).to_string()
    };
    ( .if $cond:expr, $($inner:tt)* ) => {
        if $cond {
            concat!("<ul>", $($inner)*,"</ul>")
        }
        else { "" }
    };
    ( $($inner:tt)* ) => {
            format!("<ul>{}</ul>", $($inner)*)
    };
}

#[macro_export]
macro_rules! var {
    ( $($inner:tt)* ) => {
        concat!("<var>", $($inner)*, "</var>")
    };
}

#[macro_export]
macro_rules! video {
    ( .src = $src:expr ) => {
        concat!("<video src='", $src, "'></video>")
    };
    ( .src = $src:expr, .poster = $poster:expr ) => {
        concat!("<video src='", $src, "' poster='", $poster, "'></video>")
    };
    ( .src = $src:expr, $($inner:tt)* ) => {
        concat!("<video src='", $src, "'>", $($inner)*,"</video>")
    };
    ( .src = $src:expr, .poster = $poster:expr, $($inner:tt)* ) => {
        concat!("<video src='", $src, "' poster='", $poster, "'>", $($inner)*,"</video>")
    };
}

#[macro_export]
macro_rules! wbr {
    () => {
        "<wbr>"
    };
}

#[macro_export]
macro_rules! tag_inner {
    (,$inner:expr) => {
        format_args!("{}", $inner)
    };
    (,$inner:expr $(,inner:expr)+) => {
        format_args!("{}{}", $inner, tag_inner!($(,$inner)*))
    };
    ($tag:ident) => {
        format_args!("<{}></{}>", $tag, $tag)
    };
    ($tag:ident $(,$inner:expr)+) => {
        format_args!("<{}>{}</{}>", $tag, $($inner)*, $tag)
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
        tag_inner!(ATag, .href = "link", .download= "file.html", "Cool Link", tag_inner!(ATag))
            .render(),
        "<a href=\"link\" download=\"file.html\">Cool Link</a>"
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

#[test]
fn test_conditional() {
    assert_eq!(ul![ .if true, "List Item 1"], "<ul>List Item 1</ul>");
    assert_eq!(ul![ .if false, "List Item 1"], "");
}

#[test]
fn test_nested() {
    //assert_eq!(
    //    div! { "This is a div's inner text!", div!{"Nested div"} }.render(),
    //    "<div>This is a div's inner text!<div>Nested div</div></div>"
    //);
}

//#[test]
//fn test_multi_attributes() {
//    assert_eq!(
//        a![.href="/google", p!["nested p tag"]].render(),
//        "<a href=\"/google\"><p>nested p tag</p></a>"
//    );
//}
//
//#[test]
//fn test_multi_tags() {
//    assert_eq!(
//        a![.href="/google", p!["nested p tag"]].render(),
//        "<a href=\"/google\"><p>nested p tag</p></a>"
//    );
//}
//
//#[test]
//fn test_a() {
//    assert_eq!(
//        a![.href="/google", "cool link"].render(),
//        "<a href=\"/google\">cool link</a>"
//    );
//}
//
//#[test]
//fn test_iter() {}
