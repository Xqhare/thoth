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
        "{}{}{}{}{}{}",
        include_str!("../README.md"),
        include_str!("../LICENSE"),
        include_str!("../CONTRIBUTING.md"),
        include_str!("../Cargo.toml"),
        include_str!("../Cargo.lock"),
        include_str!("../.gitignore")
    );
    let thot = Thoth::grapheme_segmentation(&text).unwrap();
    let unicode = text.graphemes(true).collect::<Vec<_>>();
    assert_eq!(thot, unicode);
}

/// Doesn't panic, but returns a vector of length 3 instead of the correct 1
#[test]
fn no_emoji() {
    let emoji = "❤️";
    let graphemes = Thoth::grapheme_segmentation(emoji).unwrap();
    assert_ne!(graphemes.len(), 1);
    assert_eq!(graphemes.len(), 3);
}

/// Panics with `attempt to subtract with overflow`
#[test]
#[should_panic]
fn no_emoji2() {
    let emoji = "🦀";
    let graphemes = Thoth::grapheme_segmentation(emoji).unwrap();
    assert_ne!(graphemes.len(), 1);
}

/// Panics with `attempt to subtract with overflow`
#[test]
#[should_panic]
fn no_emoji3() {
    let emoji = "✅";
    let graphemes = Thoth::grapheme_segmentation(emoji).unwrap();
    assert_ne!(graphemes.len(), 1);
}
