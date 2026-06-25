//! Kind of a bodge. Storing the table like this reduces the fs from 97kb to 34kb
//! But it should be more performant? I hope
//!
//! Also, its in its own file to help rustc a bit, see `include!` docs
use crate::{error::ThothError, utilities::Value};
use nemesis::NemesisError;
use std::collections::BTreeMap;

use crate::error::ThothResult;

pub fn table() -> ThothResult<BTreeMap<u32, Value>> {
    parse_break_property_table(include_str!("grapheme_break_property.txt"))
}

fn parse_break_property_table(
    unicode17_break_property_table: &str,
) -> ThothResult<BTreeMap<u32, Value>> {
    let mut output: BTreeMap<u32, Value> = BTreeMap::new();
    for line in unicode17_break_property_table.lines() {
        if !line.starts_with('#') && !line.is_empty() {
            if let Some((bytes, value)) = line.split_once(';') {
                let value = if let Ok(value) = Value::try_from(value.trim().to_string()) {
                    value
                } else {
                    return Err(NemesisError::new(
                            "thoth::parse_break_property_table",
                            ThothError::TableParsing(format!("Unknown value: {value}")),
                        ).add_ctx("This is not a user error, something is wrong with the included pre generated table."));
                };
                if bytes.is_empty() {
                    return Err(NemesisError::new(
                            "thoth::parse_break_property_table",
                            ThothError::TableParsing("Empty bytes".to_string()),
                        ).add_ctx("This is not a user error, something is wrong with the included pre generated table."));
                }
                if bytes.contains("..") {
                    if let Some((begin, end)) = bytes.trim().split_once("..") {
                        for i in i32::from_str_radix(begin.trim(), 16).unwrap()
                            ..=i32::from_str_radix(end.trim(), 16).unwrap()
                        {
                            output.insert(i as u32, value.try_into().unwrap());
                        }
                    }
                } else {
                    output.insert(
                        i32::from_str_radix(bytes.trim(), 16).unwrap() as u32,
                        value.try_into().unwrap(),
                    );
                }
            }
        }
    }
    assert_eq!(output.len(), 18_101);
    Ok(output)
}
