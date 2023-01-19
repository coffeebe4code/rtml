# rtml

__**description**__

(r)st macros for h(tml) expansion => rtml

__**usage**__

```rust
fn main() {
    use rtml::*;
    // Use the macros to generate some HTML
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
    };

    println!("{}", html.render());
}
```

the output html will be.
```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <title>Page Title</title>
  </head>
<body>

<div>
  text
  <h1>This is a Heading</h1>
  <p>This is a paragraph.</p>
</div>

</body>
</html> 
```

When there are attributes, they go first, before adding any inner nested html.

__**note**__

Although very easy to use, it is not complete, no global attributes or global events are on any of the html tags yet. So even very simple things like style are not yet included on any of the tags

I have much larger plans for creating component systems where you can do server side, or spa like behavior, you will be able to write rust methods which will generate wasm or javascript, etc.
