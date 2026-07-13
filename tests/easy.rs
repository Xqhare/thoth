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
fn first_bug_found_in_wild() {
    let text = "m̀";
    compare_test(text);
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
    compare_test(&text);
}

/// Compare graphemes with the unicode segmentation crate
fn compare_test(text: &str) {
    let graphemes = Thoth::grapheme_segmentation(text).unwrap();
    let unicode = text.graphemes(true).collect::<Vec<_>>();
    assert_eq!(graphemes, unicode);
}

// Emojis with variation selectors; Fails as emoji are not officially supported
#[should_panic]
#[test]
fn advanced_emoji() {
    let emoji = "🖖🏿";
    compare_test(emoji);
    // Starts failing here
    let emoji = "🤸🏾‍♂️";
    compare_test(emoji);
    let emoji = "🕵🏿‍♂️";
    compare_test(emoji);
    let emoji = "👨🏾‍🚀";
    compare_test(emoji);
    let emoji = "🐦‍⬛";
    compare_test(emoji);
    let emoji = "👩🏻‍🦱";
    compare_test(emoji);
}

/// Verify that emojis with variation selectors are correctly segmented (should be 1 grapheme)
#[test]
fn no_emoji() {
    let emoji = "❤️";
    compare_test(emoji);
}

/// Verify that single emoji characters do not cause subtraction underflow and segment correctly
#[test]
fn no_emoji2() {
    let emoji = "🦀";
    compare_test(emoji);
}

/// Verify that other emoji characters do not cause subtraction underflow and segment correctly
#[test]
fn no_emoji3() {
    let emoji = "✅";
    compare_test(emoji);
}
