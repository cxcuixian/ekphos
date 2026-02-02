/// Represents a single tabstop in a snippet
#[derive(Debug, Clone, PartialEq)]
pub struct TabStop {
    /// The tabstop index (1, 2, 3, ... 0 for final)
    pub index: usize,
    /// Default value if any (from ${1:default} syntax)
    pub default_value: Option<String>,
    /// Start position in the expanded text
    pub start_pos: usize,
    /// End position in the expanded text
    pub end_pos: usize,
}

/// An active snippet instance with tabstop navigation
#[derive(Debug, Clone)]
pub struct SnippetInstance {
    /// The expanded text with defaults filled in
    pub text: String,
    /// All tabstops sorted by index
    pub tabstops: Vec<TabStop>,
    /// Current tabstop index (position in tabstops vec)
    pub current_tabstop: usize,
    /// Base absolute offset where snippet was inserted
    pub base_offset: usize,
}

impl SnippetInstance {
    /// Create a new snippet instance from snippet body lines
    pub fn from_body(body: &[String]) -> Option<Self> {
        let (text, tabstops) = parse_snippet_body(body);
        if tabstops.is_empty() {
            // No tabstops, just insert text
            return None;
        }

        Some(Self {
            text,
            tabstops,
            current_tabstop: 0,
            base_offset: 0,
        })
    }

    /// Get the current tabstop
    pub fn current(&self) -> Option<&TabStop> {
        self.tabstops.get(self.current_tabstop)
    }

    /// Move to the next tabstop
    pub fn next(&mut self) -> Option<&TabStop> {
        if self.current_tabstop < self.tabstops.len() - 1 {
            self.current_tabstop += 1;
            self.tabstops.get(self.current_tabstop)
        } else {
            None
        }
    }

    /// Check if snippet is complete (past last tabstop)
    pub fn is_complete(&self) -> bool {
        self.current_tabstop >= self.tabstops.len() - 1
    }

    /// Get the range of the current tabstop for selection
    pub fn current_range(&self) -> Option<(usize, usize)> {
        self.current().map(|t| (t.start_pos, t.end_pos))
    }
}

/// Parse snippet body and extract tabstops
/// Returns (expanded_text, sorted_tabstops)
pub fn parse_snippet_body(body: &[String]) -> (String, Vec<TabStop>) {
    let mut tabstops: Vec<TabStop> = Vec::new();
    let mut result = String::new();
    let mut current_pos = 0;

    for (line_idx, line) in body.iter().enumerate() {
        if line_idx > 0 {
            result.push('\n');
            current_pos += 1;
        }

        let mut chars = line.chars().peekable();
        while let Some(ch) = chars.next() {
            if ch == '$' {
                // Check for ${...} or $N syntax
                if let Some(&next_ch) = chars.peek() {
                    if next_ch == '{' {
                        // ${...} format
                        chars.next(); // consume '{'
                        if let Some((index, default_val, consumed)) =
                            parse_braced_tabstop(&mut chars)
                        {
                            let start_pos = current_pos;
                            let text_to_insert = default_val.as_deref().unwrap_or("");
                            result.push_str(text_to_insert);
                            current_pos += text_to_insert.len();

                            tabstops.push(TabStop {
                                index,
                                default_value: default_val,
                                start_pos,
                                end_pos: current_pos,
                            });

                            continue;
                        }
                    } else if next_ch.is_ascii_digit() {
                        // $N format
                        chars.next(); // consume digit
                        let index = next_ch.to_digit(10).unwrap() as usize;

                        tabstops.push(TabStop {
                            index,
                            default_value: None,
                            start_pos: current_pos,
                            end_pos: current_pos, // Empty placeholder
                        });
                        continue;
                    }
                }
                // Not a valid tabstop, treat as literal $
                result.push(ch);
                current_pos += 1;
            } else {
                result.push(ch);
                current_pos += 1;
            }
        }
    }

    // Sort tabstops: first by index (1, 2, 3...), then $0 goes last
    tabstops.sort_by(|a, b| {
        if a.index == 0 && b.index == 0 {
            std::cmp::Ordering::Equal
        } else if a.index == 0 {
            std::cmp::Ordering::Greater
        } else if b.index == 0 {
            std::cmp::Ordering::Less
        } else {
            a.index.cmp(&b.index)
        }
    });

    (result, tabstops)
}

/// Parse ${...} tabstop syntax
/// Returns (index, default_value, chars_consumed)
fn parse_braced_tabstop(
    chars: &mut std::iter::Peekable<std::str::Chars>,
) -> Option<(usize, Option<String>, usize)> {
    let mut num_str = String::new();
    let mut consumed = 0;

    // Parse number
    while let Some(&ch) = chars.peek() {
        if ch.is_ascii_digit() {
            num_str.push(ch);
            chars.next();
            consumed += 1;
        } else {
            break;
        }
    }

    if num_str.is_empty() {
        return None;
    }

    let index = num_str.parse::<usize>().ok()?;

    // Check for :default syntax
    if let Some(&':') = chars.peek() {
        chars.next();
        consumed += 1;

        let mut default_val = String::new();
        let mut brace_depth = 1;

        while let Some(ch) = chars.next() {
            consumed += 1;
            if ch == '{' {
                brace_depth += 1;
                default_val.push(ch);
            } else if ch == '}' {
                brace_depth -= 1;
                if brace_depth == 0 {
                    return Some((index, Some(default_val), consumed));
                } else {
                    default_val.push(ch);
                }
            } else {
                default_val.push(ch);
            }
        }
        // Unclosed brace
        None
    } else if let Some(&'}') = chars.peek() {
        chars.next();
        consumed += 1;
        Some((index, None, consumed))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_tabstop() {
        let body = vec!["Hello $1".to_string()];
        let (text, tabstops) = parse_snippet_body(&body);

        assert_eq!(text, "Hello ");
        assert_eq!(tabstops.len(), 1);
        assert_eq!(tabstops[0].index, 1);
        assert_eq!(tabstops[0].start_pos, 6);
        assert_eq!(tabstops[0].end_pos, 6);
    }

    #[test]
    fn test_tabstop_with_default() {
        let body = vec!["${1:world}".to_string()];
        let (text, tabstops) = parse_snippet_body(&body);

        assert_eq!(text, "world");
        assert_eq!(tabstops.len(), 1);
        assert_eq!(tabstops[0].index, 1);
        assert_eq!(tabstops[0].default_value, Some("world".to_string()));
        assert_eq!(tabstops[0].start_pos, 0);
        assert_eq!(tabstops[0].end_pos, 5);
    }

    #[test]
    fn test_multiple_tabstops() {
        let body = vec!["${1:first} ${2:second}".to_string()];
        let (text, tabstops) = parse_snippet_body(&body);

        assert_eq!(text, "first second");
        assert_eq!(tabstops.len(), 2);
        assert_eq!(tabstops[0].index, 1);
        assert_eq!(tabstops[1].index, 2);
    }

    #[test]
    fn test_final_tabstop() {
        let body = vec!["$1 $0".to_string()];
        let (text, tabstops) = parse_snippet_body(&body);

        assert_eq!(text, " ");
        assert_eq!(tabstops.len(), 2);
        assert_eq!(tabstops[0].index, 1); // $1 comes first
        assert_eq!(tabstops[1].index, 0); // $0 comes last
    }

    #[test]
    fn test_snippet_instance_navigation() {
        let body = vec!["${1:first} ${2:second}".to_string()];
        let mut instance = SnippetInstance::from_body(&body).unwrap();

        assert_eq!(instance.current().unwrap().index, 1);
        instance.next();
        assert_eq!(instance.current().unwrap().index, 2);
        assert!(instance.next().is_none());
        assert!(instance.is_complete());
    }

    #[test]
    fn test_literal_dollar() {
        let body = vec!["Price: $$100".to_string()];
        let (text, tabstops) = parse_snippet_body(&body);

        assert_eq!(text, "Price: $100");
        assert!(tabstops.is_empty());
    }

    #[test]
    fn test_multiline_body() {
        let body = vec![
            "# ${1:Title}".to_string(),
            "".to_string(),
            "${2:Content}".to_string(),
        ];
        let (text, tabstops) = parse_snippet_body(&body);

        assert_eq!(text, "# Title\n\nContent");
        assert_eq!(tabstops.len(), 2);
    }
}
