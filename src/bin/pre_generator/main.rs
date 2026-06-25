//! Pre-generate the grapheme break property table
//!
//! # Usage
//! ```
//! cargo run
//! ```
//!
//! # Notes
//! The generated output is meant to be included in the `thoth` crate (`thoth/src/table/grapheme_break_property.txt`).
//!
//! # License
//! This code is released under the [MIT License](https://opensource.org/licenses/MIT).

use std::fs::write;

use crate::parser::parse_break_property_table;

mod parser;

fn main() {
    let parsed = parse_break_property_table(include_str!("GraphemeBreakProperty.txt"));
    write("grapheme_break_property.txt", parsed).unwrap();
}
