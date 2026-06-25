/// A value from the `grapheme_break_property.txt` table
///
/// For internal use, see `Value::try_from`
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Value {
    /// Carriage Return
    Cr,
    /// Line Feed
    Lf,
    /// Control
    Control,
    /// Extend
    Extend,
    /// Zero Width Joiner
    ZWJ,
    /// Regional Indicator (unused, needed for country flag emojis)
    RegionalIndicator,
    /// Prepend
    Prepend,
    /// SpacingMark
    SpaceingMark,
    /// Hangul
    L,
    /// Hangul
    V,
    /// Hangul
    T,
    /// Hangul
    LV,
    /// Hangul
    LVT,
}

impl Into<String> for Value {
    fn into(self) -> String {
        match self {
            Value::Cr => "CR".to_string(),
            Value::Lf => "LF".to_string(),
            Value::Control => "Control".to_string(),
            Value::Extend => "Extend".to_string(),
            Value::ZWJ => "ZWJ".to_string(),
            Value::RegionalIndicator => "Regional_Indicator".to_string(),
            Value::Prepend => "Prepend".to_string(),
            Value::SpaceingMark => "SpacingMark".to_string(),
            Value::L => "L".to_string(),
            Value::V => "V".to_string(),
            Value::T => "T".to_string(),
            Value::LV => "LV".to_string(),
            Value::LVT => "LVT".to_string(),
        }
    }
}

impl TryFrom<String> for Value {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "CR" => Ok(Value::Cr),
            "LF" => Ok(Value::Lf),
            "Control" => Ok(Value::Control),
            "Extend" => Ok(Value::Extend),
            "ZWJ" => Ok(Value::ZWJ),
            "Regional_Indicator" => Ok(Value::RegionalIndicator),
            "Prepend" => Ok(Value::Prepend),
            "SpacingMark" => Ok(Value::SpaceingMark),
            "L" => Ok(Value::L),
            "V" => Ok(Value::V),
            "T" => Ok(Value::T),
            "LV" => Ok(Value::LV),
            "LVT" => Ok(Value::LVT),
            _ => Err(()),
        }
    }
}
