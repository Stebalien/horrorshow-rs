#![feature(test)]
extern crate test;

#[macro_use]
extern crate horrorshow;

use horrorshow::{Template, Raw};

#[bench]
fn bench_short_fmt(b: &mut test::Bencher) {
    b.iter(|| {
        let count = test::black_box(10);
        let mut s = String::new();
        let template = html! {
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
                    // Insert raw text (unescaped)
                    p : Raw("Let's <i>count</i>!");
                    ol(id="count") {
                        // run some inline code...
                        @ for i in 0..count {
                            li : format_args!("{}", i+1)
                        }
                    }
                    // You need semi-colons for tags without children.
                    br; br;
                    p : "Easy!"
                }
            }
        };
        template.write_to_fmt(&mut s).unwrap()
    });
}

#[bench]
fn bench_short(b: &mut test::Bencher) {
    b.iter(|| {
        let count = test::black_box(10);
        (html! {
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
                    // Insert raw text (unescaped)
                    p : Raw("Let's <i>count</i>!");
                    ol(id="count") {
                        // run some inline code...
                        @ for i in 0..count {
                            li : format_args!("{}", i+1)
                        }
                    }
                    // You need semi-colons for tags without children.
                    br; br;
                    p : "Easy!"
                }
            }
        }).into_string().unwrap()
    });
}

#[bench]
fn bench_long(b: &mut test::Bencher) {
    b.iter(|| {
        let count = test::black_box(100);
        (html! {
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
                    // Insert raw text (unescaped)
                    p : Raw("Let's <i>count</i>!");
                    ol(id="count") {
                        // run some inline code...
                        @ for i in 0..count {
                            li : format_args!("{}", i+1)
                        }
                    }
                    // You need semi-colons for tags without children.
                    br; br;
                    p : "Easy!"
                }
            }
        }).into_string().unwrap()
    });
}
