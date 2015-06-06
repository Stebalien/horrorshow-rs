# Horrorshow

[![Build Status](https://travis-ci.org/Stebalien/horrorshow-rs.svg?branch=master)](https://travis-ci.org/Stebalien/horrorshow-rs)

A macro-based html templating library (1.0 compatible).

Documentation: https://stebalien.github.io/horrorshow-rs/horrorshow/

## Example:

```rust
# #[macro_use] extern crate horrorshow;
# fn main() {
use horrorshow::prelude::*;
let actual = html! {
    html {
        head {
            title { : "Hello world!" }
        }
        body {
            // attributes
            h1(id="heading") {
                // Insert escaped text
                : "Hello! This is <html />"
            }
            p {
                // Insert raw text (unescaped)
                : raw!("Let's <i>count</i> to 10!")
            }
            ol(id="count") {
                // run some inline code...
                |tmpl| for i in 0..10 {
                    &mut *tmpl << html! {
                        li {
                            // format some text
                            : format_args!("{}", i+1)
                        }
                    };
                }
            }
            // You need semi-colons for tags without children.
            br; br;
            p {
                : "Easy!"
            }
        }
    }
}.into_string().unwrap();

let expected = "\
<html>\
  <head>\
    <title>Hello world!</title>\
  </head>\
  <body>\
    <h1 id=\"heading\">Hello! This is &lt;html /&gt;</h1>\
    <p>Let's <i>count</i> to 10!</p>\
    <ol id=\"count\">\
      <li>1</li>\
      <li>2</li>\
      <li>3</li>\
      <li>4</li>\
      <li>5</li>\
      <li>6</li>\
      <li>7</li>\
      <li>8</li>\
      <li>9</li>\
      <li>10</li>\
    </ol>\
    <br /><br />\
    <p>Easy!</p>\
  </body>\
</html>";
assert_eq!(expected, actual);

# }
```
