# rtml

__**description**__

(r)st macros for h(tml) expansion => rtml

__**usage**__

```rust
fn main() {
    // Use the macros to generate some HTML
    let html = html!{
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
```

__**note**__
Although very easy to use, it is not complete, no global attributes or global events are on any of the html tags yet. So even very simple things like style are not yet included on any of the tags

I have much larger plans for creating component systems where you can do server side, or spa like behavior, you can write rust methods which will generate wasm or javascript, etc.
