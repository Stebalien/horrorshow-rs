#![feature(test)]
extern crate test;

#[macro_use]
extern crate horrorshow;

#[bench]
fn bench(b: &mut test::Bencher) {
    b.iter(|| {
        test::black_box(xml! {
            html {
                head {
                    title { : "Hello world!" }
                }
                body {
                    // attributes
                    h1(id="heading") {
                        // Insert escaped text (actually, anything that defines Display)
                        : "Hello! This is <html />"
                    }
                    p {
                        // Insert raw text (unescaped)
                        ! "Let's <i>count</i> to 10!"
                    }
                    ol(id="count") {
                        // run some inline code...
                        @ for i in 0..10 {
                            // append to the current template.
                            append_html! {
                                li {
                                    // format some text
                                    #{"{}", i+1 }
                                }
                            }
                        }
                    }
                    // You need semi-colons for tags without children.
                    br; br;
                    p {
                        : "Easy!"
                    }
                }
            }
        });
    });
}
