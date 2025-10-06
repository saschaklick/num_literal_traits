// Copyright 2025 Sascha Klick
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Parsing common formats numeric literal formats
//!
//! ## Compatibility
//!
//! The `num-literal-traits` crate is tested for rustc 1.88 and greater.
use num_traits::Num;

/// The trait adds String parsing functions to types already implementing
/// the num_traits::Num trait.
pub trait NumLiteralTrait<T: Num>: Num {
    
    /// Determine the literal type, then convert to a number value or
    /// return an error.
    /// 
    /// # Arguments
    /// - `text`: Textual representation of a number.    
    /// # Returns
    /// - Numerical result or error.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use num_traits::Num;
    /// use num_literal_traits::NumLiteralTrait;
    ///
    /// let result = u32::parse_literal("0xCAFE");
    /// assert_eq!(result, Ok(0xcafe));
    /// 
    /// let result = u32::parse_literal("0b1000_0001_1111_1010");
    /// assert_eq!(result, Ok(33274));
    ///
    /// let result = u32::parse_literal("CAFE");
    /// assert!(result.is_err());
    /// 
    /// let result = u32::parse_literal("'A'");
    /// assert_eq!(result, Ok(65));
    /// ```
    ///
    /// # Supported formats
    ///
    /// Most integer literal formats found in C and C++ are supported:
    /// Binary      : `0b100010`, `0B0`, `0b10101101`
    /// Octal       : `0123`, `00`, `04763523`
    /// Decimal     : `123`, `0`, `7635223`    
    /// Hexadecimal : `0xCAFE`, `0x0`, `0xa1fb484`
    /// Char        : `'A'`, `'!'`
    ///
    /// Additionally, the numeric parts can contain underscores `_` to
    /// which get removed before converting.    
    fn parse_literal(text: &str) -> Result<T, T::FromStrRadixErr>;
    
    /// Determine the literal type, then convert to a number value or
    /// return the provided fallback if the parsing fails.
    ///
    /// # Arguments
    /// - `text`: Textual representation of a number.    
    /// - `fallback`: Fallback return value if the parsing fails.
    /// # Returns
    /// - Numerical result, either the parsed value or the fallback.
    ///  
    /// # Examples
    ///
    /// ```rust
    /// use num_traits::Num;
    /// use num_literal_traits::NumLiteralTrait;
    ///
    /// let result = u32::parse_literal_fallback("0xCAFE", 0xfabc);
    /// assert_eq!(result, 0xcafe);
    ///
    /// let result = u32::parse_literal_fallback("CAFE", 0xfabc);
    /// assert_eq!(result, 0xfabc);
    ///     
    /// let result = u32::parse_literal_fallback("'全'", 0xfabc);
    /// assert_eq!(result, 0xfabc);
    ///    
    /// ```
    fn parse_literal_fallback(text: &str, fallback: T) -> T;
}

mod parse_literal;

#[cfg(test)]
mod tests {    
    use super::*;
    
    #[test]

    fn null_works() {
        let result = u32::parse_literal("0");
        assert_eq!(result, Ok(0));    
    }
    fn binary_works() {
        let result = u32::parse_literal("0B1000111011000010");
        assert_eq!(result, Ok(36546));    
    }
    
    #[test]
    fn octal_works() {
        let result = u32::parse_literal("0723642");
        assert_eq!(result, Ok(239522));    
    }

    #[test]
    fn decimal_works() {
        let result = u32::parse_literal("9823642");
        assert_eq!(result, Ok(9823642));    
    }

    #[test]
    fn hexadecimal_works() {
        let result = u32::parse_literal("0xA82c6fE");
        assert_eq!(result, Ok(0xa82c6fe));    
    }

    #[test]
    fn fallback_works() {
        let result = u32::parse_literal_fallback("random text", 0xCAFE);
        assert_eq!(result, 0xcafe);  
    }

    #[test]
    fn underscores_works() {
        let result = u32::parse_literal("0b1101_0011_1000_0111");
        assert_eq!(result, Ok(54151));  
    }

    #[test]
    fn random_text_fails() {
        let res = u32::parse_literal("t322545");
        assert!(res.is_err());
    }

    #[test]
    fn empty_char_fails() {
        let res = u32::parse_literal("''");
        assert!(res.is_err());
    }

    #[test]
    fn multiple_chars_fails() {
        let res = u32::parse_literal("'ABC'");
        assert!(res.is_err());
    }

    #[test]
    fn nonascii_chars_fails() {
        let res = u32::parse_literal("'全'");
        assert!(res.is_err());
    }

}