pub mod parser;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

/// A single snippet definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snippet {
    pub prefix: String,
    pub body: Vec<String>,
    #[serde(default)]
    pub description: String,
}

/// VSCode snippet file format (keyed by snippet name)
#[derive(Debug, Deserialize)]
pub struct SnippetFile(HashMap<String, SnippetEntry>);

/// Individual snippet entry in VSCode format
#[derive(Debug, Deserialize)]
struct SnippetEntry {
    prefix: String,
    #[serde(deserialize_with = "deserialize_body")]
    body: Vec<String>,
    #[serde(default)]
    description: String,
}

/// Deserialize body which can be String or Vec<String>
fn deserialize_body<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::{self, Visitor};
    use std::fmt;

    struct BodyVisitor;

    impl<'de> Visitor<'de> for BodyVisitor {
        type Value = Vec<String>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string or array of strings")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(value.lines().map(|s| s.to_string()).collect())
        }

        fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(value.lines().map(|s| s.to_string()).collect())
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: de::SeqAccess<'de>,
        {
            let mut vec = Vec::new();
            while let Some(element) = seq.next_element::<String>()? {
                vec.push(element);
            }
            Ok(vec)
        }
    }

    deserializer.deserialize_any(BodyVisitor)
}

/// Collection of loaded snippets
#[derive(Debug, Clone, Default)]
pub struct SnippetCollection {
    snippets: Vec<Snippet>,
}

impl SnippetCollection {
    /// Load snippets from the default VSCode path
    pub fn load_default() -> Self {
        let path = Self::default_path();
        Self::load_from_path(&path)
    }

    /// Get the default ekphos snippets path
    pub fn default_path() -> PathBuf {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".config")
            .join("ekphos")
            .join("snippets")
            .join("markdown.json")
    }

    /// Load snippets from a specific path
    pub fn load_from_path(path: &PathBuf) -> Self {
        if !path.exists() {
            return Self::default();
        }

        match fs::read_to_string(path) {
            Ok(content) => Self::load_from_str(&content),
            Err(e) => {
                eprintln!("Warning: Failed to read snippets file: {}", e);
                Self::default()
            }
        }
    }

    /// Load snippets from a JSON string
    pub fn load_from_str(content: &str) -> Self {
        let content_clean = strip_jsonc(content);
        match serde_json::from_str::<SnippetFile>(&content_clean) {
            Ok(file) => {
                let snippets: Vec<Snippet> = file
                    .0
                    .into_iter()
                    .map(|(name, entry)| Snippet {
                        prefix: entry.prefix,
                        body: entry.body,
                        description: if entry.description.is_empty() {
                            name
                        } else {
                            entry.description
                        },
                    })
                    .collect();
                Self { snippets }
            }
            Err(e) => {
                eprintln!("Warning: Failed to parse snippets file: {}", e);
                // Try to debug by printing a snippet of the cleaned content
                let start = content_clean.len().min(500);
                eprintln!("Cleaned content preview: {}", &content_clean[..start]);
                Self::default()
            }
        }
    }

    /// Find a snippet by its prefix
    pub fn find_by_prefix(&self, prefix: &str) -> Option<&Snippet> {
        self.snippets.iter().find(|s| s.prefix == prefix)
    }

    /// Get all snippets
    pub fn all(&self) -> &[Snippet] {
        &self.snippets
    }

    /// Check if any snippets are loaded
    pub fn is_empty(&self) -> bool {
        self.snippets.is_empty()
    }
}

/// Strip JSONC comments (// and /* ... */) and trailing commas
fn strip_jsonc(input: &str) -> String {
    // First pass: strip comments
    let mut no_comments = String::with_capacity(input.len());
    let mut chars = input.chars().peekable();
    let mut in_string = false;
    let mut escaped = false;

    while let Some(c) = chars.next() {
        if in_string {
            no_comments.push(c);
            if escaped {
                escaped = false;
            } else if c == '\\' {
                escaped = true;
            } else if c == '"' {
                in_string = false;
            }
        } else {
            if c == '/' {
                if let Some(&next) = chars.peek() {
                    if next == '/' {
                        chars.next();
                        while let Some(c) = chars.next() {
                            if c == '\n' {
                                no_comments.push('\n');
                                break;
                            }
                        }
                        continue;
                    } else if next == '*' {
                        chars.next();
                        while let Some(c) = chars.next() {
                            if c == '*' {
                                if let Some(&next_next) = chars.peek() {
                                    if next_next == '/' {
                                        chars.next();
                                        break;
                                    }
                                }
                            }
                        }
                        continue;
                    }
                }
            }

            if c == '"' {
                in_string = true;
            }
            no_comments.push(c);
        }
    }

    // Second pass: strip trailing commas
    let chars: Vec<char> = no_comments.chars().collect();
    let mut commas_to_delete = std::collections::HashSet::new();
    let mut last_comma_idx = None;
    let mut idx = 0;

    in_string = false;
    escaped = false;

    while idx < chars.len() {
        let c = chars[idx];
        if in_string {
            if escaped {
                escaped = false;
            } else if c == '\\' {
                escaped = true;
            } else if c == '"' {
                in_string = false;
            }
        } else {
            if c == '"' {
                in_string = true;
                last_comma_idx = None;
            } else if c == ',' {
                last_comma_idx = Some(idx);
            } else if c == '}' || c == ']' {
                if let Some(comma_idx) = last_comma_idx {
                    let mut is_trailing = true;
                    for k in (comma_idx + 1)..idx {
                        if !chars[k].is_whitespace() {
                            is_trailing = false;
                            break;
                        }
                    }
                    if is_trailing {
                        commas_to_delete.insert(comma_idx);
                    }
                }
                last_comma_idx = None;
            } else if !c.is_whitespace() {
                last_comma_idx = None;
            }
        }
        idx += 1;
    }

    let mut output = String::with_capacity(no_comments.len());
    for (i, c) in chars.iter().enumerate() {
        if !commas_to_delete.contains(&i) {
            output.push(*c);
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_jsonc() {
        let json = r#"{
            // comment
            "key": "val//ue", /* block */
            "key2": "val/*ue",
            "list": [
                1,
                2,
            ],
        }"#;
        let clean = strip_jsonc(json);
        // Verify no comments
        assert!(!clean.contains("// comment"));
        assert!(!clean.contains("/* block */"));
        // Verify strings preserved
        assert!(clean.contains(r#""key": "val//ue""#));
        assert!(clean.contains(r#""key2": "val/*ue""#));

        // Verify parseable
        let parsed: serde_json::Value = serde_json::from_str(&clean).unwrap();
        assert_eq!(parsed["list"][0], 1);
        assert_eq!(parsed["list"][1], 2);
    }

    #[test]
    fn test_load_from_str() {
        let json = r##"{
            "Heading 1": {
                "prefix": "h1",
                "body": ["# $1", ""],
                "description": "Insert a heading 1"
            },
            "Link": {
                "prefix": "link",
                "body": "[$1]($2)",
                "description": "Insert a markdown link"
            }
        }"##;

        let collection = SnippetCollection::load_from_str(json);
        assert_eq!(collection.snippets.len(), 2);

        let h1 = collection.find_by_prefix("h1").unwrap();
        assert_eq!(h1.body, vec!["# $1", ""]);
        assert_eq!(h1.description, "Insert a heading 1");

        let link = collection.find_by_prefix("link").unwrap();
        assert_eq!(link.body, vec!["[$1]($2)"]);
    }

    #[test]
    fn test_find_by_prefix() {
        let json = r#"{"Test": {"prefix": "test", "body": "hello"}}"#;
        let collection = SnippetCollection::load_from_str(json);

        assert!(collection.find_by_prefix("test").is_some());
        assert!(collection.find_by_prefix("nonexistent").is_none());
    }

    #[test]
    fn test_empty_file() {
        let collection = SnippetCollection::load_from_str("{}");
        assert!(collection.is_empty());
    }

    #[test]
    fn test_invalid_json() {
        let collection = SnippetCollection::load_from_str("not valid json");
        assert!(collection.is_empty());
    }
}
