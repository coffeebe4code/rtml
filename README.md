# rtml

__**description**__

(r)ust macros for h(tml) expansion => rtml

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

`div!{ .style="background-color: red;" }`

call `render()` when you want to build the html.

__**Getting Started**__

Install the latest version of the rtml crate into your project

use either specific tags you plan to use or reference all tags by adding this `use` statement.

```rust
  use rtml::p;  
  // or 
  use rtml::*;
```

__**Attributes**__

According to the html5 spec, there are 3 different kinds of attributes, `Global`, `Event`, and `Specific`.
Every tag can implement any of the attributes from the `Global` or `Event` list.
`Specific` attributes go to a specific tag. Some of the `Specific` tags can be on multiple tags. What makes this even more confusing, is the possible values an attribute can be for `Specific` can change between the tag.

`Event` attributes are not implemented yet.
One other goal of this project is to add extreme type safety on the possible attributes that can go on a tag and their specific value.

There are some special ones in `rtml` as their attribute name is a keyword in rust itself, or an invalid token stream. It would be possible to allow them thanks to rusts macro expansion, but for simplicity, these special tags are represented as follows.

| html5 | rtml |
|:------|:-----|
| type | type_ |
| for | for_ |
| loop | loop_ |
| kind | kind_ |
| async | async_ |
| http-equiv | http_equiv |
| accept-charset | accept_charset |

__**Components**__

not implemented yet
