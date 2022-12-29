#[macro_export]
macro_rules! a {
    ( .href = $val:expr, .inner = $text:expr ) => {
        concat!("<a href='", $val, "'>", $text, "</a>")
    };
}

#[macro_export]
macro_rules! abbr {
    ( .title = $val:expr, .inner = $text:expr ) => {
        concat!("<abbr title='", $val, "'>", $text, "</abbr>")
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
    ( .inner = $text:expr ) => {
        concat!("<b>", $text, "</b>")
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
    ( .dir = $val:expr, .inner = $text:expr ) => {
        concat!("<bdi dir='", $val, "'>", $text, "</bdi>")
    };
}

#[macro_export]
macro_rules! bdo {
    ( .dir = $val:expr, .inner = $text:expr ) => {
        concat!("<bdo dir='", $val, "'>", $text, "</bdo>")
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
    ( .inner = $text:expr ) => {
        concat!("<button>", $text, "</button>")
    };
    ( .form = $form:expr, .inner = $text:expr ) => {
        concat!("<button form='", $form, "'>", $text, "</button>")
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
    ( .inner = $text:expr ) => {
        concat!("<cite>", $text, "</cite>")
    };
}

#[macro_export]
macro_rules! code {
    ( .inner = $text:expr ) => {
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
    ( .value = $val:expr, .inner = $text:expr ) => {
        concat!("<data value='", $val, "'>", $text, "</data>")
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
    ( .inner = $text:expr ) => {
        concat!("<dfn>", $text, "</dfn>")
    };
}

#[macro_export]
macro_rules! dialog {
    ( $($inner:tt)* ) => {
        concat!("<dialog>", $($inner)*,"</dialog>")
    };
}

#[macro_export]
macro_rules! div {
    ( $($inner:tt)* ) => {
        concat!("<div>", $($inner)*,"</div>")
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
    ( .inner = $text:expr ) => {
        concat!("<em>", $text, "</em>")
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
    ( .inner = $text:expr ) => {
        concat!("<h1>", $text, "</h1>")
    };
}

#[macro_export]
macro_rules! h2 {
    ( .inner = $text:expr ) => {
        concat!("<h1>", $text, "</h1>")
    };
}

#[macro_export]
macro_rules! h3 {
    ( .inner = $text:expr ) => {
        concat!("<h3>", $text, "</h3>")
    };
}

#[macro_export]
macro_rules! h4 {
    ( .inner = $text:expr ) => {
        concat!("<h4>", $text, "</h4>")
    };
}

#[macro_export]
macro_rules! h5 {
    ( .inner = $text:expr ) => {
        concat!("<h5>", $text, "</h5>")
    };
}

#[macro_export]
macro_rules! h6 {
    ( .inner = $text:expr ) => {
        concat!("<h6>", $text, "</h6>")
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
    ( .lang = $lang:expr, $($inner:tt)* ) => {
        concat!("<!DOCTYPE html><html lang='", $lang, "'>", $($inner)*,"</html>")
    };
}

#[macro_export]
macro_rules! i {
    ( .inner = $text:expr ) => {
        concat!("<i>", $text, "</i>")
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
    ( .inner = $text:expr ) => {
        concat!("<kbd>", $text, "</kbd>")
    };
}

#[macro_export]
macro_rules! label {
    ( .for = $val:expr, .inner = $text:expr ) => {
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
    ( .inner = $text:expr ) => {
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
    ( .inner = $text:expr ) => {
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
macro_rules! object {
    ( .data = $data:expr, .type = $typ:expr ) => {
        concat!("<object data='", $data, "' type='", $typ, "'></object>")
    };
    ( .data = $data:expr, .type = $typ:expr, .width = $width:expr, .height = $height:expr ) => {
        concat!("<object data='", $data, "' type='", $typ, "' width='", $width, "' height='", $height, "'></object>")
    };
    ( .data = $data:expr, .type = $typ:expr, $($inner:tt)* ) => {
        concat!("<object data='", $data, "' type='", $typ, "'>", $($inner)*,"</object>")
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
    ( .inner = $text:expr ) => {
        concat!("<option>", $text, "</option>")
    };
    ( .value = $value:expr, .inner = $text:expr ) => {
        concat!("<option value='", $value, "'>", $text, "</option>")
    };
}

#[macro_export]
macro_rules! output {
    ( .for = $val:expr, .inner = $text:expr ) => {
        concat!("<output for='", $val, "'>", $text, "</output>")
    };
    ( .name = $name:expr, .inner = $text:expr ) => {
        concat!("<output name='", $name, "'>", $text, "</output>")
    };
}

#[macro_export]
macro_rules! p {
    ( .inner = $text:expr ) => {
        concat!("<p>", $text, "</p>")
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
    ( .cite = $cite:expr, .inner = $text:expr ) => {
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
    ( .inner = $text:expr ) => {
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
    ( .inner = $text:expr ) => {
        concat!("<s>", $text, "</s>")
    };
}

#[macro_export]
macro_rules! samp {
    ( .inner = $text:expr ) => {
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
    ( .inner = $text:expr ) => {
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
    ( .inner = $text:expr ) => {
        concat!("<strong>", $text, "</strong>")
    };
}

#[macro_export]
macro_rules! style {
    ( .type = $typ:expr, .inner = $text:expr ) => {
        concat!("<style type='", $typ, "'>", $text, "</style>")
    };
}

#[macro_export]
macro_rules! sub {
    ( .inner = $text:expr ) => {
        concat!("<sub>", $text, "</sub>")
    };
}

#[macro_export]
macro_rules! summary {
    ( .inner = $text:expr ) => {
        concat!("<summary>", $text, "</summary>")
    };
}

#[macro_export]
macro_rules! sup {
    ( .inner = $text:expr ) => {
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
    ( .inner = $text:expr ) => {
        concat!("<td>", $text, "</td>")
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
    ( .name = $name:expr, .inner = $text:expr ) => {
        concat!("<textarea name='", $name, "'>", $text, "</textarea>")
    };
    ( .name = $name:expr, .cols = $cols:expr, .rows = $rows:expr, .inner = $text:expr ) => {
        concat!(
            "<textarea name='",
            $name,
            "' cols='",
            $cols,
            "' rows='",
            $rows,
            "'>",
            $text,
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
    ( .inner = $text:expr ) => {
        concat!("<th>", $text, "</th>")
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
    ( .datetime = $datetime:expr, .inner = $text:expr ) => {
        concat!("<time datetime='", $datetime, "'>", $text, "</time>")
    };
}

#[macro_export]
macro_rules! title {
    ( .inner = $text:expr ) => {
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
    ( .inner = $text:expr ) => {
        concat!("<u>", $text, "</u>")
    };
}

#[macro_export]
macro_rules! ul {
    ( $($inner:tt)* ) => {
        concat!("<ul>", $($inner)*,"</ul>")
    };
}

#[macro_export]
macro_rules! var {
    ( .inner = $text:expr ) => {
        concat!("<var>", $text, "</var>")
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
