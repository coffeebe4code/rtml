pub mod attributes;
pub mod css;
pub mod render;
pub mod tags;

pub use attributes::*;
pub use css::*;
pub use render::*;
pub use tags::*;

#[cfg(test)]
pub mod tests {
    use super::*;

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
    #[test]
    fn test_tag_no_inner() {
        assert_eq!(tag_no_inner!(WbrTag).render(), "<wbr>");
        assert_eq!(
            tag_no_inner!(AreaTag, .shape = "rect").render(),
            "<area shape=\"rect\">"
        );
        assert_eq!(
            tag_no_inner!(SourceTag, .src = "link", .muted="true").render(),
            "<source src=\"link\" muted=\"true\">"
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
    #[test]
    fn test_attr_inner() {
        assert_eq!(attr_inner!(ATag,.href = "link").render(), " href=\"link\"");
        assert_eq!(
            attr_inner!(ATag, .href = "link", .download = "yes please", .hreflang="en").render(),
            " href=\"link\" download=\"yes please\" hreflang=\"en\""
        );
        assert_eq!(attr_inner!().render(), "");
        assert_eq!(
            attr_inner!(FieldsetTag, .name = "yes").render(),
            " name=\"yes\""
        );
    }

    #[test]
    fn test_html() {
        let html = html! {
         .lang = "en",
             head!{
                 title!{
                     "Title of the document"
                 }
             },
             body!{
                     div!{
                         "text",
                         h1!{
                             "This is a heading"
                         },
                         p!{
                             "This is a paragraph"
                         }
                     }
             }
        }
        .render();

        println!("{}", html);
    }

    #[test]
    fn test_value() {
        assert_eq!(property_value!("red").render(), "red");
    }

    #[test]
    fn test_property() {
        assert_eq!(
            property!(background-color: "red",).render(),
            "background-color: red;\n  "
        );
        assert_eq!(
            property!(border-top-width: "20px",).render(),
            "border-top-width: 20px;\n  "
        );
        assert_eq!(
            property!(-webkit-line-clamp: "yes",).render(),
            "-webkit-line-clamp: yes;\n  "
        );
        assert_eq!(
            property!(background-color: "red").render(),
            "background-color: red;\n  "
        );
        assert_eq!(
            property!(background-color: "red", float: "left",).render(),
            "background-color: red;\n  float: left;\n  "
        );
        assert_eq!(
            property!(align-self: "stretch", float: "left",).render(),
            "align-self: stretch;\n  float: left;\n  "
        );
    }

    #[test]
    fn test_css_body() {
        assert_eq!(
            css_body!(background-color: "red",).render(),
            " {\n  background-color: red;\n  }\n"
        );
    }

    #[test]
    fn test_selector_and_class() {
        class!(my_class);
        assert_eq!(selector!(.my_class {}).render(), ".my_class {\n  }\n");
        assert_eq!(
            selector!(.my_class {background-color: "red"}).render(),
            ".my_class {\n  background-color: red;\n  }\n"
        );
    }

    #[test]
    fn test_css() {
        let css = css!(
            p > div {
                background-color: "green",
            }
            p div {
                float: "left"

            }
        )
        .render();

        assert_eq!(
            css,
            "p > div {\n  background-color: green;\n  }\np div {\n  float: left;\n  }\n"
        );
    }
}
