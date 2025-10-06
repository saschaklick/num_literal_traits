// Copyright 2025 Sascha Klick
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use num_traits::Num;
use crate::NumLiteralTrait;

fn identify_literal<'a>(text: &'a str) -> (&'a str, u32) {
    let text = text.trim();    
    return
        if text.to_lowercase().starts_with("0b") { (&text[2..], 2) } else            
        if text.to_lowercase().starts_with("0x") { (&text[2..], 16) } else            
        if text.len() > 1 && text.to_lowercase().starts_with("0") { (&text[1..], 8) } else
        { (text, 10) };
}

impl<T> NumLiteralTrait<T> for T where T: Num {
    fn parse_literal(text: &str) -> Result<T, T::FromStrRadixErr> {
        if text.len() == 3 && text.starts_with("'") && text.ends_with("'") {
            let chr = text.as_bytes().iter().nth(1).unwrap();            
            return T::from_str_radix(chr.to_string().as_str(), 10);
        } else {
            let (num_part, radix) = identify_literal(text);        
            return T::from_str_radix(&num_part.replace("_", ""), radix);
        }
    }
    
    fn parse_literal_fallback(text: &str, fallback: T) -> T {
        if text.len() == 3 && text.starts_with("'") && text.ends_with("'") {
            let chr = text.as_bytes().iter().nth(1).unwrap();            
            return T::from_str_radix(chr.to_string().as_str(), 10).unwrap_or(fallback);
        } else {
            let (num_part, radix) = identify_literal(text);        
            return T::from_str_radix(&num_part.replace("_", ""), radix).unwrap_or(fallback); 
        }
    }
}