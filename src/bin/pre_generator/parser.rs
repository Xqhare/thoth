use std::collections::BTreeMap;

use thoth::utilities::Value;

pub fn parse_break_property_table(unicode17_break_property_table: &str) -> String {
    let mut output = String::new();
    for line in unicode17_break_property_table.lines() {
        if !line.starts_with('#') && !line.is_empty() {
            if let Some((start, _)) = line.split_once('#') {
                output.push_str(start);
                output.push('\n');
            }
        }
    }
    output
}
