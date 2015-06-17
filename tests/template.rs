#[macro_use]
extern crate horrorshow;

use horrorshow::Template;

template! {
    Test(num: &u64) {
        span : num
    }
    pub Test2(num: &u64) {
        span : num
    }
}

#[test]
fn test_template() {
    assert_eq!(html! {
        span : Test::new(&32);
    }.into_string().unwrap(), "<span><span>32</span></span>");
    assert_eq!(html! {
        span : Test2::new(&32);
    }.into_string().unwrap(), "<span><span>32</span></span>");
}

