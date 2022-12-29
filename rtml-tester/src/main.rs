fn main() {
    use rtml::*;
    // Use the macros to generate some HTML
    let html = html! {
        .lang = "en",
            head!{
                title!{
                    .inner = "Title of the document"
                }
            },
            body!{
                    h1!{
                        .inner = "This is a heading"
                    },
                    p!{
                        .inner = "This is a paragraph"
                    }
            }
    };

    println!("{}", html);
}
