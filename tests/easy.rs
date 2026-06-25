use thoth::Thoth;
use unicode_segmentation::UnicodeSegmentation;
#[test]
fn hello_world() {
    assert_eq!(
        Thoth::grapheme_segmentation("Hello World!").unwrap(),
        vec!["H", "e", "l", "l", "o", " ", "W", "o", "r", "l", "d", "!"]
    );
}

#[test]
fn compare() {
    let text = format!(
        "{}{}{}{}{}",
        include_str!("../README.md"),
        include_str!("../LICENSE"),
        include_str!("../CONTRIBUTING.md"),
        include_str!("../Cargo.toml"),
        include_str!("../Cargo.lock"),
    );
    let thot = Thoth::grapheme_segmentation(&text).unwrap();
    let unicode = text.graphemes(true).collect::<Vec<_>>();
    assert_eq!(thot, unicode);
}
