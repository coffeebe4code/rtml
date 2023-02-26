pub mod attributes;
pub mod basic_structs;
pub mod css;
pub mod render;
pub mod tags;

pub use attributes::*;
pub use basic_structs::*;
pub use css::*;
pub use render::*;
pub use tags::*;

extern crate paste;

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_ul() {
        assert_eq!(
            ul![li!["Item 1"], li!["Item 2"], li!["Item 3"]].render(),
            "<ul><li>Item 1</li><li>Item 2</li><li>Item 3</li></ul>"
        );
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

    #[test]
    fn test_tag_inner() {
        assert_eq!(parse_double_tag!(ATag).render(), "<a></a>");
        assert_eq!(parse_double_tag!(ATag, "inner").render(), "<a>inner</a>");
        assert_eq!(
            parse_double_tag!(ATag, .href = "link", .download= "file.html").render(),
            "<a href=\"link\" download=\"file.html\"></a>"
        );
        assert_eq!(
            parse_double_tag!(ATag, .href = "link", .download= "file.html", "Cool Link").render(),
            "<a href=\"link\" download=\"file.html\">Cool Link</a>"
        );
        assert_eq!(
            parse_double_tag!(ATag, .href = "link", .download= "file.html", "Cool Link", parse_double_tag!(ATag))
                .render(),
            "<a href=\"link\" download=\"file.html\">Cool Link<a></a></a>"
        );
    }
    #[test]
    fn test_tag_no_inner() {
        assert_eq!(parse_single_tag!(WbrTag).render(), "<wbr>");
        assert_eq!(
            parse_single_tag!(AreaTag, .shape = "rect").render(),
            "<area shape=\"rect\">"
        );
        assert_eq!(
            parse_single_tag!(SourceTag, .src = "link", .muted="true").render(),
            "<source src=\"link\" muted=\"true\">"
        );
    }
    #[test]
    fn test_attr_inner() {
        assert_eq!(parse_attr!(ATag,.href = "link").render(), " href=\"link\"");
        assert_eq!(parse_attr!(ATag,.href = "link").render(), " href=\"link\"");
        assert_eq!(
            parse_attr!(ATag, .href = "link", .download = "yes please", .hreflang="en").render(),
            " href=\"link\" download=\"yes please\" hreflang=\"en\""
        );
        assert_eq!(parse_attr!().render(), "");
        assert_eq!(
            parse_attr!(MetaTag, .http_equiv="value").render(),
            " http-equiv=\"value\""
        );
        assert_eq!(
            parse_attr!(FormTag, .accept_charset="value").render(),
            " accept-charset=\"value\""
        );
        assert_eq!(
            parse_attr!(FieldsetTag, .name = "yes").render(),
            " name=\"yes\""
        );
        make_data!(cy_optional);
        make_data!(cy);
        assert_eq!(
            parse_attr!(FormTag, .data_cy_optional = "link").render(),
            " data-cy-optional=\"link\""
        );
        assert_eq!(
            parse_attr!(FormTag, .data_cy = "link").render(),
            " data-cy=\"link\""
        );
    }

    #[test]
    fn test_ids() {
        make_id!(my_id);
        let css = css!(
        #my_id {
            color: "red"
        })
        .render();
        assert_eq!(css, "#my_id {\n  color: red;\n  }\n");
    }

    // the purpose of this test is to determine if we can forgo using .render() immediately.
    #[test]
    fn test_html() {
        let result = html! {
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
            property!(background_color: "red",).render(),
            "background-color: red;\n  "
        );
        assert_eq!(
            property!(border_top_width: "20px",).render(),
            "border-top-width: 20px;\n  "
        );
        assert_eq!(
            property!(_webkit_line_clamp: "yes",).render(),
            "-webkit-line-clamp: yes;\n  "
        );
        assert_eq!(
            property!(background_color: "red").render(),
            "background-color: red;\n  "
        );
        assert_eq!(
            property!(background_color: "red", float: "left",).render(),
            "background-color: red;\n  float: left;\n  "
        );
        assert_eq!(
            property!(align_self: "stretch", float: "left",).render(),
            "align-self: stretch;\n  float: left;\n  "
        );
    }

    #[test]
    fn test_css_body() {
        assert_eq!(
            css_body!(background_color: "red",).render(),
            " {\n  background-color: red;\n  }\n"
        );
    }

    #[test]
    fn test_selector_and_class() {
        make_class!(my_class);
        assert_eq!(selector!(.my_class {}).render(), ".my_class {\n  }\n");
        assert_eq!(
            selector!(.my_class {background_color: "red"}).render(),
            ".my_class {\n  background-color: red;\n  }\n"
        );
    }

    #[test]
    fn test_universal() {
        assert_eq!(selector!(*{}).render(), "* {\n  }\n");
        assert_eq!(
            selector!(time + * { clear: "left" }).render(),
            "time + * {\n  clear: left;\n  }\n"
        );
    }

    #[test]
    fn test_pseudo_class() {
        assert_eq!(pseudo_class!(active {}).render(), ":active {\n  }\n");
        assert_eq!(pseudo_class!(dir("rtl") {}).render(), ":dir(rtl) {\n  }\n");
        assert_eq!(css!(p:dir("rtl") {}).render(), "p:dir(rtl) {\n  }\n");
        assert_eq!(css!(div:hover p {}).render(), "div:hover p {\n  }\n");
    }

    #[test]
    fn test_pseudo_elements() {
        assert_eq!(pseudo_element!(after {}).render(), "::after {\n  }\n");
        assert_eq!(css!(p::after {}).render(), "p::after {\n  }\n");
    }

    #[test]
    fn test_attr_selectors() {
        make_class!(my_class);
        make_data!(test);
        assert_eq!(attr_selector!([.href = "#"]).render(), "[href=\"#\"]");
        assert_eq!(
            attr_selector!([.href="#", .class=my_class]).render(),
            "[href=\"#\" class=\"my_class\"]"
        );
        assert_eq!(
            attr_selector!([.data_test="#", .class=my_class]).render(),
            "[data-test=\"#\" class=\"my_class\"]"
        );
        assert_eq!(css!(a[.href = "#"] {}).render(), "a[href=\"#\"] {\n  }\n");
    }

    #[test]
    fn test_css() {
        let css = css!(
            p > div {
                background_color: "green",
            }
            p div {
                float: "left"
            }
        );

        assert_eq!(
            css.render(),
            "p > div {\n  background-color: green;\n  }\np div {\n  float: left;\n  }\n"
        );
    }

    #[test]
    fn test_css_render() {
        let property_value = property_value!("left");
        assert_eq!(property_value.render(), "left");
        let property = property!(float: "left");
        assert_eq!(property.render(), "float: left;\n  ");
    }

    #[test]
    fn test_iter() {
        let val = ["one", "two", "three"];
        let html = ul!(
            .class="new",
            iter_fold!(val, |x| li!{ x })
        );

        assert_eq!(
            html.render(),
            "<ul class=\"new\"><li>one</li><li>two</li><li>three</li></ul>"
        );
    }
}
