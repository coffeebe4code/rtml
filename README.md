# rtml

__**description**__

(r)ust macros for h(tml) expansion => rtml
but also (r)ust type safe (css) => rcss

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
    }.render();

    println!("{}", html);
}
```

the output html will be in non pretty form.

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

```rust
div!{ .style="background-color: red;" }
```

call `render()` when the html needs to be built.

__**Getting Started**__

Install the latest version of the rtml crate into your project

use either specific tags you plan to use or reference all tags by adding this `use` statement.

```rust
  use rtml::p;  
  // or 
  use rtml::*;
```

__**CSS**__

Rtml also supports type safe css.
```rust
fn main() {
    use rtml::*;
    // Use the macros to generate some CSS
    let css = css! {
      p {
        background: yellow;
      }
    }.render();

    println!("{}", css);
}
```

css has a limited sub implementation. Attribute selectors, Pseudo-classes, Pseudo-elements, At-rules, functions, and types are not yet implemented. They can be used, however, with string literals.

Css Properties, Selectors, and Combinators are implemented in full.


__**Attributes**__

There are some special attributes in `rtml` as their attribute name is a keyword in rust itself, or an invalid token stream. It would be possible to allow the rust keyword attributes thanks to rusts macro expansion, but for simplicity, these special tags are represented as follows.

| html5 | rtml |
|:------|:-----|
| type | type_ |
| for | for_ |
| loop | loop_ |
| kind | kind_ |
| async | async_ |
| a | a_ |
| http-equiv | http_equiv |
| accept-charset | accept_charset |

__**Type Safe Attributes**__

rtml allows an additional layer of type safety with tag use. For example,

```rust
a! { .href="/documents", My Documents }
```
```html
<a href="/documents">My Documents</a>
```
Is the correct attribute allowed on the `a` tag.
If you were not familiar, you might try to use the `src` attribute which is invalid. Most other website building projects would not prevent incompatible attribute use.

In rtml, there is a helpful error.

```rust
a! { .src="/documents", My Documents }
```

```html
<a src="/documents">My Documents</a>
```

```
the trait bound `src: ACompat` is not satisified
the following other types implement the trait `ACompat`:
accesskey
class
contenteditable
dir
download
draggable
hidden
href
and 13 others
``` 

Breaking this error down,

`the trait bound src: ACompat...`

- `src` : is the name of the attribute attempting to be used
- `ACompat` : is the trait that would need to be implemented, every tag has their own version. Such as `DivCompat` and `StyleCompat`
- `accesskey class contenteditable` : are a few of the attributes supported, Rust lists them in alphabetical order and therefore will not show all

__**Components**__

not implemented yet
