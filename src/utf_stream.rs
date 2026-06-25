use nemesis::NemesisError;

use crate::error::{ThothError, ThothResult};

/// Returns a stream of valid utf8 code points
pub fn new_utf_stream(input: &[u8]) -> ThothResult<Vec<(u32, &[u8])>> {
    let mut index = 0;
    let mut output = Vec::new();

    while index < input.len() {
        let b0 = input[index];

        let (len, initial_mask) = if b0 <= 0x7F {
            (1, 0b0111_1111) // 0xxxxxxx (7 bits)
        } else if b0 >= 0b1111_0000 {
            (4, 0b0000_0111) // 11110xxx (3 bits)
        } else if b0 >= 0b1110_0000 {
            (3, 0b0000_1111) // 1110xxxx (4 bits)
        } else if b0 >= 0b1100_0000 {
            (2, 0b0001_1111) // 110xxxxx (5 bits)
        } else {
            // Catches 10xxxxxx (continuation byte without a valid start byte)
            return Err(NemesisError::new(
                "thoth::new_utf_stream",
                ThothError::InvalidUtf8(format!(
                    "Invalid start byte {:#04X} at index {}",
                    b0, index
                )),
            ));
        };

        if index + len > input.len() {
            return Err(NemesisError::new(
                "thoth::new_utf_stream",
                ThothError::InvalidUtf8("Unexpected end of input stream".to_string()),
            ));
        }

        let mut code_point = (b0 & initial_mask) as u32;

        // Process any continuation bytes
        for i in 1..len {
            let b = input[index + i];

            // Continuation bytes MUST start with 10xxxxxx
            if b & 0b1100_0000 != 0b1000_0000 {
                return Err(NemesisError::new(
                    "thoth::new_utf_stream",
                    ThothError::InvalidUtf8(format!(
                        "Invalid continuation byte {:#04X} at index {}",
                        b,
                        index + i
                    )),
                ));
            }

            // Shift accumulator by 6 and OR in the 6 payload bits
            code_point = (code_point << 6) | ((b & 0b0011_1111) as u32);
        }

        output.push((code_point, &input[index..index + len]));

        index += len;
    }

    Ok(output)
}
