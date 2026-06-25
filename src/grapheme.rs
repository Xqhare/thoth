use std::collections::BTreeMap;

use crate::value::Value;

pub fn grapheme_segmentation_utf8_stream(
    input: Vec<(u32, &[u8])>,
    table: &BTreeMap<u32, Value>,
) -> Vec<String> {
    let mut output = Vec::new();
    let mut buffer = String::new();
    let mut index = 0;

    while index < input.len() {
        if output.len() == 2 {
            // TODO: This is a hack
            if output[0] == String::new() {
                output.remove(0);
            }
        }
        if input.len() == index + 1 {
            if insert_break(input[index - 1], input[index], table) {
                output.push(buffer.clone());
                buffer.clear();
            }
            buffer.push_str(std::str::from_utf8(input[index].1).unwrap());
            output.push(buffer.clone());
            buffer.clear();
            break;
        } else if insert_break(input[index], input[index + 1], table) {
            output.push(buffer.clone());
            buffer.clear();
        }
        buffer.push_str(std::str::from_utf8(input[index].1).unwrap());
        index += 1;
    }
    if !buffer.is_empty() {
        output.push(buffer.clone());
    }
    output
}

fn insert_break(current: (u32, &[u8]), next: (u32, &[u8]), table: &BTreeMap<u32, Value>) -> bool {
    if gb3_to_5(current, next, table) {
        true
    } else if gb6_to_8(current, next, table) {
        true
    } else if gb9(next, table) {
        true
    } else if gb9a_b(current, next, table) {
        true
    } else if gb999() {
        true
    } else {
        unreachable!("Unreachable thanks to gb999");
    }
}

/// Do not break between a CR and LF. Otherwise, break before and after controls.
///
/// Returns `true` if a break should be inserted, `false` otherwise
fn gb3_to_5(current: (u32, &[u8]), next: (u32, &[u8]), table: &BTreeMap<u32, Value>) -> bool {
    if let Some(val) = table.get(&current.0) {
        if val == &Value::Cr {
            if let Some(nval) = table.get(&next.0) {
                if nval == &Value::Lf {
                    return false;
                }
            }
            return true;
        }
    }
    false
}

/// Do not break Hangul syllable or other conjoining sequences.
fn gb6_to_8(current: (u32, &[u8]), next: (u32, &[u8]), table: &BTreeMap<u32, Value>) -> bool {
    if gb6(current, next, table) {
        true
    } else if gb7(current, next, table) {
        true
    } else if gb8(current, next, table) {
        true
    } else {
        false
    }
}

/// Do not break before extending characters or ZWJ.
fn gb9(next: (u32, &[u8]), table: &BTreeMap<u32, Value>) -> bool {
    if let Some(val) = table.get(&next.0) {
        if val == &Value::ZWJ || val == &Value::Extend {
            return false;
        }
    }
    true
}

/// Do not break before SpacingMarks, or after Prepend characters.
fn gb9a_b(current: (u32, &[u8]), next: (u32, &[u8]), table: &BTreeMap<u32, Value>) -> bool {
    if let Some(val) = table.get(&next.0) {
        if val == &Value::SpaceingMark {
            return false;
        }
    }
    if let Some(val) = table.get(&current.0) {
        if val == &Value::Prepend {
            return false;
        }
    }
    true
}

/// Otherwise, break everywhere.
fn gb999() -> bool {
    true
}

fn gb8(current: (u32, &[u8]), next: (u32, &[u8]), table: &BTreeMap<u32, Value>) -> bool {
    if let Some(val) = table.get(&current.0) {
        if val == &Value::LVT || val == &Value::T {
            if let Some(nval) = table.get(&next.0) {
                if nval == &Value::T {
                    return false;
                }
            }
            return true;
        }
    }
    false
}

fn gb7(current: (u32, &[u8]), next: (u32, &[u8]), table: &BTreeMap<u32, Value>) -> bool {
    if let Some(val) = table.get(&current.0) {
        if val == &Value::LV || val == &Value::V {
            if let Some(nval) = table.get(&next.0) {
                if nval == &Value::L || nval == &Value::V {
                    return false;
                }
            }
            return true;
        }
    }
    false
}

fn gb6(current: (u32, &[u8]), next: (u32, &[u8]), table: &BTreeMap<u32, Value>) -> bool {
    if let Some(val) = table.get(&current.0) {
        if val == &Value::L {
            if let Some(nval) = table.get(&next.0) {
                if nval == &Value::L
                    || nval == &Value::V
                    || nval == &Value::LV
                    || nval == &Value::LVT
                {
                    return false;
                }
            }
            return true;
        }
    }
    false
}
