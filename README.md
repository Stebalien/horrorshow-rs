# Horrorshow

[![Build Status](https://travis-ci.org/Stebalien/horrorshow-rs.svg?branch=master)](https://travis-ci.org/Stebalien/horrorshow-rs)

A macro-based xml templating library (1.0 compatible).

API: https://stebalien.github.io/horrorshow-rs/horrorshow/

## Example:

```rust
xml! {
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
                ! "Let's <i>count</i> to 10!"
            }
            ol(id="count") {
                // run some inline code...
                |mut tmpl| for i in 0..10 {
                    // append to the current template.
                    // store output because rust bug #25753
                    tmpl = tmpl << xml! {
                        li {
                            // format some text
                            #{"{}", i+1 }
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
}.render();
```

Becomes (whitespace added for clarity).

```html
<html>
  <head>
    <title>Hello world!</title>
  </head>
  <body>
    <h1 id="heading">Hello! This is &lt;html /&gt;</h1>
    <p>Let's <i>count</i> to 10!</p>
    <ol id="count">
      <li>1</li>
      <li>2</li>
      <li>3</li>
      <li>4</li>
      <li>5</li>
      <li>6</li>
      <li>7</li>
      <li>8</li>
      <li>9</li>
      <li>10</li>
    </ol>
    <br /> <br />
    <p>Easy!</p>
  </body>
</html>
```

## Usage

Inside an xml template, the following expressions are valid:

* `some_tag;` -- Insert a the tag `some_tag`.

* `some_tag(attr=rust_expresion,...);` -- Insert a the tag `some_tag` with the specified
   attributes. The attribute values will be evaluated as rust expressions at runtime.

* `some_tag { ... }` -- Insert a the tag `some_tag` and recursivly evaluate the `...`.

* `some_tag(...) { ... }` -- Same as above but with custom attributes.

* `: rust_expression`, `: { rust_code }` -- Evaluate the expression or block and insert result current position.

* `#{"format_str", rust_expressions... }` -- Format the arguments according to `format_str` and insert the
result at the current position.

* `@ rust_expression`, `@ { rust_code }` -- Evaluate the expression or block.

In rust code embedded inside of a template, you can append text with any of the following
macros:

* `append_fmt!("format_str", args...)` -- format, escape, and append arguments
* `append_raw!(text)` -- append text without escaping
* `append!(text)` -- escape and append text
* `append_xml! { xml_template... }` -- append an xml template.

## Disclaimer

This library is mostly untested and probably a security hazard.

