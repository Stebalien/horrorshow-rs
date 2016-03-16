#![feature(plugin, test)]
#![plugin(maud_macros)]

extern crate test;
extern crate maud;

use maud::PreEscaped;

#[bench]
fn bench_short(b: &mut test::Bencher) {
    b.iter(|| {
        let mut s = String::new();
        html!(s, {
            html {
                head {
                    title { "Hello world!" }
                }
                body {
                    h1 id="heading" {
                        "Hello! This is <html />"
                    }
                    p {
                        ^PreEscaped("Let's <i>count</i> to 10!")
                    }
                    ol id="count" {
                        @for i in 0..10 {
                            li {
                                // format some text
                                ^(i+1)
                            }
                        }
                    }
                    br /
                    br /
                    p {
                        "Easy!"
                    }
                }
            }
        }).unwrap();
        test::black_box(s);
    });
}



#[bench]
fn bench_long(b: &mut test::Bencher) {
    let count = test::black_box(100);
    b.iter(|| {
        let mut s = String::new();
        html!(s, {
            html {
                head {
                    title { "Hello world!" }
                }
                body {
                    h1 id="heading" {
                        "Hello! This is <html />"
                    }
                    p {
                        ^PreEscaped("Let's <i>count</i> to 100!")
                    }
                    ol id="count" {
                        @for i in 0..count {
                            li {
                                // format some text
                                ^(i+1)
                            }
                        }
                    }
                    br /
                    br /
                    p {
                        "Easy!"
                    }
                }
            }
        }).unwrap();
        test::black_box(s);
    });
}
