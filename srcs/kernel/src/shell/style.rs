use alloc::{format, string::String, vec::Vec};

use crate::Colors;

pub struct StyledString {
    pub string: String,
    pub fore_color: Colors,
    pub back_color: Colors,
}

// parse string like "Hello, world! \033[style;fore;backm" into a vector of StyledString
pub fn parse_style_in_str(msg: &str) -> Result<Vec<StyledString>, ()> {
    let mut result = Vec::new();
    let mut current_string = String::new();
    let mut fore_color = Colors::White;
    let mut back_color = Colors::Black;

    let mut chars = msg.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\x1B' {
            if chars.next() == Some('[') {
                // Push the current string with the current colors
                if !current_string.is_empty() {
                    result.push(StyledString {
                        string: current_string.clone(),
                        fore_color,
                        back_color,
                    });
                    current_string.clear();
                }

                // Parse the style sequence
                let mut style_code = String::new();
                while let Some(&next_char) = chars.peek() {
                    if next_char == 'm' {
                        chars.next(); // consume 'm'
                        break;
                    } else {
                        style_code.push(next_char);
                        chars.next();
                    }
                }

                let codes: Vec<&str> = style_code.split(';').collect();
                if codes[0] == "0" {
                    fore_color = Colors::White;
                    back_color = Colors::Black;
                } else if codes.len() == 3 {
                    fore_color = Colors::from_u32(codes[1].parse::<u32>().map_err(|_| ())?);
                    back_color = Colors::from_u32(codes[2].parse::<u32>().map_err(|_| ())?);
                }
            }
        } else {
            current_string.push(c);
        }
    }

    if !current_string.is_empty() {
        result.push(StyledString {
            string: current_string,
            fore_color,
            back_color,
        });
    }

    Ok(result)
}

pub fn style(fore: Colors, back: Colors) -> String {
    format!("\x1B[1;{};{}m", fore as u32, back as u32)
}