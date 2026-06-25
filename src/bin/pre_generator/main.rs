//! Pre-generate the grapheme break property table
//!
//! # Usage
//! ```
//! cargo run
//! ```
//!
//! # Notes
//! The underlying parser is lightweight (around 50 loc)
//! and should be easy to understand.
//!
//! The generated output is meant to be included in the `thoth` crate.
//!
//! # License
//! This code is released under the [MIT License](https://opensource.org/licenses/MIT).

use crate::parser::parse_break_property_table_alt;

mod parser;

fn main() {
    let parsed = parse_break_property_table_alt(include_str!("GraphemeBreakProperty.txt"));
    std::fs::write("grapheme_break_property.txt", parsed).unwrap();
}

// fn main() {
//     let parsed: XffValue =
//         parse_break_property_table(include_str!("GraphemeBreakProperty.txt")).into();
//     let current_dir = std::env::current_dir().unwrap();
//     let path = current_dir.join("grapheme_break_property.xff");
//     match write(&path, parsed) {
//         Ok(()) => println!("Done, written to {:?}", path),
//         Err(err) => println!("Error: {}", err),
//     }
// }
