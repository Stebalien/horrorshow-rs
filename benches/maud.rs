#![feature(plugin, test)]
#![plugin(maud_macros)]

extern crate test;
extern crate maud;

#[bench]
fn bench(b: &mut test::Bencher) {
    b.iter(|| {
        let mut s = String::new();
        let template = html! {
            html {
                head {
                    title { "Hello world!" }
                }
                body {
                    h1 id="heading" {
                        "Hello! This is <html />"
                    }
                    p {
                        $$"Let's <i>count</i> to 10!"
                    }
                    ol id="count" {
                        $ for i in 0..10 {
                            li {
                                // format some text
                                $(i+1)
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
        };
        template.render_fmt(&mut s).unwrap();
        test::black_box(s);
    });
}

