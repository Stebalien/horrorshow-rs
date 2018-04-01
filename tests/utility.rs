#[macro_use]
extern crate horrorshow;

use horrorshow::Template;

#[test]
fn test_default_labels() {
    let i = 10;
    assert_eq!(labels!("active").into_string().unwrap(), "active");
    assert_eq!(labels!(1).into_string().unwrap(), "1");
    assert_eq!(labels!("active" => true).into_string().unwrap(), "active");
    assert_eq!(labels!("active" => false).into_string().unwrap(), "");
    assert_eq!(
        labels!("button", "active" => true, "bold", format_args!("nth-{}", i))
            .into_string()
            .unwrap(),
        "button active bold nth-10"
    );
    assert_eq!(
        labels!("button", "active" => false, "bold", format_args!("nth-{}", i))
            .into_string()
            .unwrap(),
        "button bold nth-10"
    );
    assert_eq!(
        labels!("button", "bold", "active" => true, format_args!("nth-{}", i))
            .into_string()
            .unwrap(),
        "button bold active nth-10"
    );
    assert_eq!(
        labels!("button", "bold", "active" => false, format_args!("nth-{}", i))
            .into_string()
            .unwrap(),
        "button bold nth-10"
    );

    // usage

    let example = html!{
        div(class = labels!("active" => true, "button-style")) {
            : "foo"
        }

        div(class = labels!("active" => false, "button-style")) {
            : "bar"
        }
    }.into_string()
        .unwrap();

    assert_eq!(
        example,
        "<div class=\"active button-style\">foo</div><div class=\"button-style\">bar</div>"
    );
}

#[test]
fn test_labels_sep_by() {
    let i = 10;
    assert_eq!(
        labels_sep_by!(";"; "active").into_string().unwrap(),
        "active"
    );
    assert_eq!(labels_sep_by!(";"; 1).into_string().unwrap(), "1");
    assert_eq!(
        labels_sep_by!(";"; "active" => true).into_string().unwrap(),
        "active"
    );
    assert_eq!(
        labels_sep_by!(";"; "active" => false)
            .into_string()
            .unwrap(),
        ""
    );
    assert_eq!(
        labels_sep_by!(";"; "button", "active" => true, "bold", format_args!("nth-{}", i))
            .into_string()
            .unwrap(),
        "button;active;bold;nth-10"
    );
    assert_eq!(
        labels_sep_by!(";"; "button", "active" => false, "bold", format_args!("nth-{}", i))
            .into_string()
            .unwrap(),
        "button;bold;nth-10"
    );
    assert_eq!(
        labels_sep_by!(";"; "button", "bold", "active" => true, format_args!("nth-{}", i))
            .into_string()
            .unwrap(),
        "button;bold;active;nth-10"
    );
    assert_eq!(
        labels_sep_by!(";"; "button", "bold", "active" => false, format_args!("nth-{}", i))
            .into_string()
            .unwrap(),
        "button;bold;nth-10"
    );

    // usage

    let example = html!{
        div(style = labels_sep_by!(";"; "color: #000" => true, "font-weight: bold")) {
            : "foo"
        }

        div(style = labels_sep_by!(";"; "color: #000" => false, "font-weight: bold")) {
            : "bar"
        }
    }.into_string()
        .unwrap();

    assert_eq!(example,
        "<div style=\"color: #000;font-weight: bold\">foo</div><div style=\"font-weight: bold\">bar</div>");
}
