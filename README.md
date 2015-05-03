# Horrorshow

An xml (mostly just the html subset) templating library.

## Example:

```rust
xml! {
    html {
        head {
            title { : "Hello world!" }
        }
        body {
            h1(id="heading") { : "Hello!" }
            p { : "Let's count to 10!" }
            ol(id="count") {
                @ for i in 0..10 {
                    append_xml! {
                        li {
                            #{"{}", i+1 }
                        }
                    }
                }
            }
            br; br;
            p {
                : "Easy!"
            }
        }
    }
}
```

Becomes

```html
<html>
  <head>
    <title>Hello world!</title>
  </head>
  <body>
    <h1 id="heading">Hello!</h1>
    <p>Let's count to 10!</p>
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
    <br /><br />
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

In rust code embedded inside of a template, you can invoke `append!("format_str", args...)` or
`append_xml! { xml_template... }` to append to the template at the current position. That's how
the for loop works in the example above.

## Notes:

1. This library does no escaping, sanitization. You have to do that yourself!
2. There are bugs.
