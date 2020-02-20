#[macro_use]
extern crate horrorshow;

use horrorshow::Template;

#[test]
fn test_utf8() {
    let data = "м, о";
    // Test fmt::Write
    assert_eq!(format!("{}", html! {: data}), data);
    // Test String
    #[cfg(feature = "alloc")]
    assert_eq!(html! {: data}.into_string().unwrap(), data);

    // Test io::Write (if we have std)
    #[cfg(feature = "std")]
    {
        let mut v = Vec::new();
        (html! {: data}).write_to_io(&mut v).unwrap();
        assert_eq!(String::from_utf8(v).unwrap(), data);
    }
}
