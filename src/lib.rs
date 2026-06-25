#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![warn(clippy::pedantic)]
#![warn(clippy::all)]
#![warn(clippy::restriction)]
#![expect(
    clippy::missing_docs_in_private_items,
    clippy::print_stdout,
    clippy::implicit_return,
    clippy::single_call_fn,
    clippy::str_to_string,
    clippy::question_mark_used,
    clippy::indexing_slicing,
    clippy::pattern_type_mismatch,
    clippy::arbitrary_source_item_ordering,
    clippy::doc_paragraphs_missing_punctuation,
    clippy::exhaustive_enums,
    clippy::min_ident_chars,
    clippy::missing_trait_methods,
    clippy::impl_trait_in_params,
    clippy::as_conversions,
    clippy::cast_lossless,
    clippy::shadow_reuse,
    clippy::blanket_clippy_restriction_lints,
    clippy::doc_include_without_cfg,
    reason = "Ignored warnings"
)]

use std::collections::BTreeMap;

use crate::{
    error::ThothResult, grapheme::grapheme_segmentation_utf8_stream, table::table,
    utf_stream::new_utf_stream, value::Value,
};

mod error;
mod grapheme;
mod table;
mod utf_stream;
mod value;

/// Internal utilities that need to be publicly exposed - Not needed by users
pub mod utilities {
    pub use crate::value::Value;
}

/// The core grapheme segmentation algorithm
pub struct Thoth {
    pub(crate) table: BTreeMap<u32, Value>,
}

/// Trait for grapheme segmentation.
///
/// Does not support emoji.
///
/// # Notes
///
/// Only supports UTF-8, unicode version 17.
///
/// Ignores rules `GB9c` through to `GB13` of the `Grapheme Cluster Boundary Rules`.
/// This is done for simplicity.
///
/// However, this also means that emoji (including country flags), along with 'not breaking within certain combinations with Indic_Conjunct_Break' of extended clusters are *not* supported.
///
/// This means that `Thoth` is not a true unicode grapheme segmentation algorithm.
/// It also means that `Thoth` supports neither extended or legacy grapheme clusters.
///
/// `Thoth` is also limited to a single thread and uses a double pass parsing algorithm.
impl Thoth {
    /// Grapheme segmentation of a string into a vector of strings
    ///
    /// # Arguments
    /// * `text` - The string to segment
    ///
    /// # Returns
    /// A vector of strings. Each string is a single grapheme cluster
    ///
    /// # Notes
    /// Emoji are *not* supported
    ///
    /// This function is a wrapper for `Thoth::grapheme_segmentation_u8`.
    /// If you read the text from a file, or have otherwise access to a `&[u8]`, then you should use `Thoth::grapheme_segmentation_u8` directly for better performance.
    pub fn grapheme_segmentation(text: &str) -> ThothResult<Vec<String>> {
        // Don't think this is the most performant, but its simple
        Self::grapheme_segmentation_u8(text.as_bytes())
    }
    /// Grapheme segmentation of a byte array into a vector of strings
    ///
    /// # Arguments
    /// * `text` - The byte array to segment
    ///
    /// # Returns
    /// A vector of strings. Each string is a single grapheme cluster
    ///
    /// # Notes
    /// Emoji are *not* supported
    pub fn grapheme_segmentation_u8(input: &[u8]) -> ThothResult<Vec<String>> {
        let thoth = Self::new()?;
        // TODO: Move from a double pass to a single pass
        let stream = new_utf_stream(input)?;
        Ok(grapheme_segmentation_utf8_stream(stream, &thoth.table))
    }
    fn new() -> ThothResult<Thoth> {
        let table: BTreeMap<u32, Value> = table()?;
        Ok(Thoth { table })
    }
}
