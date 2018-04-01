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
    assert_eq!(
        html! {
            span : Test::new(&32);
        }.into_string()
            .unwrap(),
        "<span><span>32</span></span>"
    );
    assert_eq!(
        html! {
            span : Test2::new(&32);
        }.into_string()
            .unwrap(),
        "<span><span>32</span></span>"
    );
}

mod submodule {
    template! {
       pub Test3(num: &u64) {
          div : num
       }
    }
}

#[test]
fn test_template_in_module() {
    assert_eq!(
        html! {p : submodule::Test3::new(&42)}
            .into_string()
            .unwrap(),
        "<p><div>42</div></p>"
    );
}
