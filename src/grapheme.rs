use std::collections::BTreeMap;

use crate::value::Value;

/// Grapheme segmentation of a UTF-8 stream using Unicode 17.0.0 boundary rules.
pub fn grapheme_segmentation_utf8_stream(
    input: Vec<(u32, &[u8])>,
    table: &BTreeMap<u32, Value>,
) -> Vec<String> {
    if input.is_empty() {
        return Vec::new();
    }
    let mut output = Vec::new();
    let mut current_grapheme = String::new();
    current_grapheme.push_str(std::str::from_utf8(input[0].1).unwrap());

    for i in 1..input.len() {
        if insert_break(input[i - 1], input[i], table) {
            output.push(current_grapheme.clone());
            current_grapheme.clear();
        }
        current_grapheme.push_str(std::str::from_utf8(input[i].1).unwrap());
    }
    output.push(current_grapheme);
    output
}

fn insert_break(current: (u32, &[u8]), next: (u32, &[u8]), table: &BTreeMap<u32, Value>) -> bool {
    let current_val = table.get(&current.0);
    let next_val = table.get(&next.0);

    // GB3: CR x LF
    if current_val == Some(&Value::Cr) && next_val == Some(&Value::Lf) {
        return false;
    }
    // GB4: (Control | CR | LF) ÷
    if current_val == Some(&Value::Cr)
        || current_val == Some(&Value::Lf)
        || current_val == Some(&Value::Control)
    {
        return true;
    }
    // GB5: ÷ (Control | CR | LF)
    if next_val == Some(&Value::Cr)
        || next_val == Some(&Value::Lf)
        || next_val == Some(&Value::Control)
    {
        return true;
    }
    // GB6: L x (L | V | LV | LVT)
    if current_val == Some(&Value::L)
        && (next_val == Some(&Value::L)
            || next_val == Some(&Value::V)
            || next_val == Some(&Value::LV)
            || next_val == Some(&Value::LVT))
    {
        return false;
    }
    // GB7: (LV | V) x (V | T)
    if (current_val == Some(&Value::LV) || current_val == Some(&Value::V))
        && (next_val == Some(&Value::V) || next_val == Some(&Value::T))
    {
        return false;
    }
    // GB8: (LVT | T) x T
    if (current_val == Some(&Value::LVT) || current_val == Some(&Value::T))
        && next_val == Some(&Value::T)
    {
        return false;
    }
    // GB9: x (Extend | ZWJ)
    if next_val == Some(&Value::Extend) || next_val == Some(&Value::ZWJ) {
        return false;
    }
    // GB9a: x SpacingMark
    if next_val == Some(&Value::SpaceingMark) {
        return false;
    }
    // GB9b: Prepend x
    if current_val == Some(&Value::Prepend) {
        return false;
    }
    // GB999: Otherwise, break.
    true
}

