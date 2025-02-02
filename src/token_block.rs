use regex::Regex;
use std::{collections::{HashMap, VecDeque}, hash::{Hash, Hasher}};
use crate::tokenizer::Token;

const RESET: &str = "\x1b[0m";
const RED: &str = "\x1b[31m";

#[derive(Debug, Clone)]
pub struct TokenBlock {
    pub requires: HashMap<String, Vec<String>>,
    pub id: String,
    pub tokens: VecDeque<Token>
}

impl TokenBlock {
    pub fn new(id: String, requires: HashMap<String, Vec<String>>, tokens: VecDeque<Token>) -> Self {
        TokenBlock { requires, id, tokens }
    }
}

impl PartialEq for TokenBlock {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for TokenBlock {}

impl Hash for TokenBlock {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

pub fn split_blocks(text: &str) -> Vec<String> {
    let re = Regex::new(r"(block\s+[A-Za-z_][A-Za-z0-9_]*\s*(?:requires\s*\[.*?\])?\s*\{)").unwrap();
    let mut results = Vec::new();
    let mut last_end = 0;
    for cap in re.find_iter(text) {
        let start = cap.start();
        if start > last_end {
            results.push(text[last_end..start].to_string());
        }
        last_end = start;
    }
    if last_end < text.len() {
        results.push(text[last_end..].to_string());
    }
    results
}

pub fn extract_block_meta(block_text: &str) -> (String, HashMap<String, Vec<String>>, String) {
    let header_re = Regex::new(r"(?s)^(block\s+([A-Za-z_][A-Za-z0-9_]*)\s*(?:requires\s*\[(.*?)\])?\s*\{)").unwrap();
    let header_caps = header_re.captures(block_text).expect(&format!("{}Error[7]: Invalid block format{}", RED, RESET));
    let block_id = header_caps.get(2).unwrap().as_str().to_string();
    let req_str = header_caps.get(3).map(|m| m.as_str()).unwrap_or("");
    let mut requirements = HashMap::new();
    if !req_str.trim().is_empty() {
        let req_item_re = Regex::new(r"([A-Za-z_][A-Za-z0-9_]*)(?:\s*\[\s*(.*?)\s*\])?").unwrap();
        for cap in req_item_re.captures_iter(req_str) {
            let req_id = cap.get(1).unwrap().as_str().to_string();
            let var_list_str = cap.get(2).map(|m| m.as_str()).unwrap_or("");
            let var_names: Vec<String> = if var_list_str.trim().is_empty() {
                Vec::new()
            } else {
                var_list_str.split(',')
                    .map(|s| s.trim().to_string())
                    .collect()
            };
            requirements.insert(req_id, var_names);
        }
    }
    let start_index = header_caps.get(0).unwrap().end();
    let mut depth = 1;
    let mut end_index = start_index;
    for (i, c) in block_text[start_index..].char_indices() {
        if c == '{' {
            depth += 1;
        } else if c == '}' {
            depth -= 1;
            if depth == 0 {
                end_index = start_index + i;
                break;
            }
        }
    }
    if depth != 0 {
        panic!("Unmatched braces in block");
    }
    let content = block_text[start_index..end_index].trim().to_string();
    (block_id, requirements, content)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_split_blocks_with_labels() {
        let input = "block abc{stuff}block def requires[ghi[j, k]]{morestuff}";
        let expected = vec![
            "block abc{stuff}",
            "block def requires[ghi[j, k]]{morestuff}",
        ];
        let result = split_blocks(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_extract_block_meta_no_requires() {
        let input = "block def {\n    some content here\n}";
        let expected_id = "def".to_string();
        let expected_requirements: HashMap<String, Vec<String>> = HashMap::new();
        let expected_content = "some content here";
        let (block_id, requirements, content) = extract_block_meta(input);
        assert_eq!(block_id, expected_id);
        assert_eq!(requirements, expected_requirements);
        assert_eq!(content, expected_content);
    }

    #[test]
    fn test_extract_block_meta_shorthand_requires() {
        let input = "block xyz requires[abc]{}";
        let expected_id = "xyz".to_string();
        let mut expected_requirements = HashMap::new();
        expected_requirements.insert("abc".to_string(), Vec::new());
        let expected_content = "";
        let (block_id, requirements, content) = extract_block_meta(input);
        assert_eq!(block_id, expected_id);
        assert_eq!(requirements, expected_requirements);
        assert_eq!(content, expected_content);
    }

    #[test]
    fn test_extract_block_meta_with_vars() {
        let input = "block xyz requires[abc[a, b, c], def[d]] {\n    some content here\n}";
        let expected_id = "xyz".to_string();
        let mut expected_requirements = HashMap::new();
        expected_requirements.insert("abc".to_string(), vec!["a".to_string(), "b".to_string(), "c".to_string()]);
        expected_requirements.insert("def".to_string(), vec!["d".to_string()]);
        let expected_content = "some content here";
        let (block_id, requirements, content) = extract_block_meta(input);
        assert_eq!(block_id, expected_id);
        assert_eq!(requirements, expected_requirements);
        assert_eq!(content, expected_content);
    }
}
