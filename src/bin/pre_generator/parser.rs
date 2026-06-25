use std::collections::BTreeMap;

use thoth::utilities::Value;
#[allow(dead_code)]
pub fn parse_break_property_table(
    unicode17_break_property_table: &str,
) -> BTreeMap<String, String> {
    let mut output = BTreeMap::new();
    for line in unicode17_break_property_table.lines() {
        if !line.starts_with('#') && !line.is_empty() {
            if let Some((start, _)) = line.split_once('#') {
                if let Some((bytes, value)) = start.split_once(';') {
                    let value = if let Ok(value) = Value::try_from(value.trim().to_string()) {
                        value
                    } else {
                        panic!("Unknown value: {}", value);
                    };
                    if bytes.is_empty() {
                        panic!("Empty bytes");
                    }
                    if bytes.contains("..") {
                        if let Some((begin, end)) = bytes.trim().split_once("..") {
                            for i in i32::from_str_radix(begin.trim(), 16).unwrap()
                                ..=i32::from_str_radix(end.trim(), 16).unwrap()
                            {
                                output.insert(i.to_string(), value.try_into().unwrap());
                            }
                        }
                    } else {
                        output.insert(
                            i32::from_str_radix(bytes.trim(), 16).unwrap().to_string(),
                            value.try_into().unwrap(),
                        );
                    }
                }
            }
        }
    }
    assert_eq!(output.len(), 18_101);
    output
}

pub fn parse_break_property_table_alt(unicode17_break_property_table: &str) -> String {
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
