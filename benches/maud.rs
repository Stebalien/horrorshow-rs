#![feature(plugin, test)]
#![plugin(maud_macros)]

extern crate test;
extern crate maud;

use maud::PreEscaped;

fn count(count: u32) -> String {
    (html!{
        html {
            head {
                title { "Hello world!" }
            }
            body {
                h1 id="heading" {
                    "Hello! This is <html />"
                }
                p {
                    (PreEscaped("Let's <i>count</i>!"))
                }
                ol id="count" {
                    @for i in 0..count {
                        li {
                            // format some text
                            (i+1)
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
    }).into_string()
}

#[bench]
fn bench_short(b: &mut test::Bencher) {
    b.iter(|| {
        count(test::black_box(10))
    });
}



#[bench]
fn bench_long(b: &mut test::Bencher) {
    b.iter(|| {
        count(test::black_box(100))
    });
}
