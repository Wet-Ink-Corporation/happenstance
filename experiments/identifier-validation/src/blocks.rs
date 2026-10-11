//! The Unicode blocks of the right-to-left scripts, for criterion 2: no
//! codepoint the validator refuses may lie in one.
//!
//! Ranges are block boundaries from the Unicode Character Database's
//! `Blocks.txt`, written out by hand because no table crate is a dependency.

use core::ops::RangeInclusive;

/// One named Unicode block.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block {
    /// The block's name in `Blocks.txt`.
    pub name: &'static str,
    /// Its codepoints, assigned or not.
    pub range: RangeInclusive<char>,
}

const fn block(name: &'static str, range: RangeInclusive<char>) -> Block {
    Block { name, range }
}

/// Every block of the Hebrew, Arabic, Syriac, Thaana, N'Ko, Samaritan and
/// Mandaic scripts, and the RTL presentation forms.
pub const RTL_BLOCKS: [Block; 17] = [
    block("Hebrew", '\u{0590}'..='\u{05FF}'),
    block("Arabic", '\u{0600}'..='\u{06FF}'),
    block("Syriac", '\u{0700}'..='\u{074F}'),
    block("Arabic Supplement", '\u{0750}'..='\u{077F}'),
    block("Thaana", '\u{0780}'..='\u{07BF}'),
    block("NKo", '\u{07C0}'..='\u{07FF}'),
    block("Samaritan", '\u{0800}'..='\u{083F}'),
    block("Mandaic", '\u{0840}'..='\u{085F}'),
    block("Syriac Supplement", '\u{0860}'..='\u{086F}'),
    block("Arabic Extended-B", '\u{0870}'..='\u{089F}'),
    block("Arabic Extended-A", '\u{08A0}'..='\u{08FF}'),
    block(
        "Alphabetic Presentation Forms (Hebrew)",
        '\u{FB1D}'..='\u{FB4F}',
    ),
    block("Arabic Presentation Forms-A", '\u{FB50}'..='\u{FDFF}'),
    block("Arabic Presentation Forms-B", '\u{FE70}'..='\u{FEFF}'),
    block("Rumi Numeral Symbols", '\u{10E60}'..='\u{10E7F}'),
    block("Arabic Extended-C", '\u{10EC0}'..='\u{10EFF}'),
    block(
        "Arabic Mathematical Alphabetic Symbols",
        '\u{1EE00}'..='\u{1EEFF}',
    ),
];
