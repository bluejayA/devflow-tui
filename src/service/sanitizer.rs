/// Strip ANSI escape sequences from a string.
///
/// Handles:
/// - CSI sequences: ESC [ ... final_byte
/// - OSC sequences: ESC ] ... ST
/// - Simple escapes: ESC followed by single char
pub fn strip_ansi(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '\x1b' {
            // ESC found — determine sequence type
            match chars.peek() {
                Some('[') => {
                    // CSI sequence: ESC [ (params) final_byte
                    // Final byte range per ECMA-48: 0x40..=0x7E
                    chars.next(); // consume '['
                    for c in chars.by_ref() {
                        if ('\x40'..='\x7E').contains(&c) {
                            break;
                        }
                    }
                }
                Some(']') => {
                    // OSC sequence: ESC ] ... (ST = ESC \ or BEL)
                    chars.next(); // consume ']'
                    while let Some(c) = chars.next() {
                        if c == '\x07' {
                            break; // BEL terminator
                        }
                        if c == '\x1b' {
                            if chars.peek() == Some(&'\\') {
                                chars.next(); // consume '\'
                            }
                            break;
                        }
                    }
                }
                Some(_) => {
                    // Simple escape: skip the next char
                    chars.next();
                }
                None => {
                    // Trailing ESC at end of string — skip
                }
            }
        } else {
            result.push(ch);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_ansi_escape_csi() {
        let input = "\x1b[31mError\x1b[0m: something failed";
        assert_eq!(strip_ansi(input), "Error: something failed");
    }

    #[test]
    fn test_strip_ansi_escape_bold() {
        let input = "\x1b[1m\x1b[36mBold Cyan\x1b[0m";
        assert_eq!(strip_ansi(input), "Bold Cyan");
    }

    #[test]
    fn test_no_ansi_passthrough() {
        let input = "plain text with no escapes";
        assert_eq!(strip_ansi(input), input);
    }

    #[test]
    fn test_mixed_content() {
        let input = "[14:20] \x1b[32m✓\x1b[0m workspace-detection → B (Approve)";
        assert_eq!(
            strip_ansi(input),
            "[14:20] ✓ workspace-detection → B (Approve)"
        );
    }

    #[test]
    fn test_strip_osc_sequence() {
        let input = "\x1b]0;window title\x07normal text";
        assert_eq!(strip_ansi(input), "normal text");
    }

    #[test]
    fn test_strip_trailing_esc() {
        let input = "text\x1b";
        assert_eq!(strip_ansi(input), "text");
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(strip_ansi(""), "");
    }
}
