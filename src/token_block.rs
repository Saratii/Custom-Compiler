use regex::Regex;
use std::{collections::VecDeque, hash::{Hash, Hasher}};

use crate::tokenizer::Token;

#[derive(Debug, Clone)]
pub struct TokenBlock {
    pub requires: Vec<usize>,
    pub id: usize,
    pub tokens: VecDeque<Token>
}

impl TokenBlock {
    pub fn new(id: usize, requires: Vec<usize>, tokens: VecDeque<Token>) -> Self {
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

//Splits given text into "blocks"
//Args: text: &str - the text to split, expected to be without white space.
//Returns: Vec<String> - a list of blocks as strings
pub fn split_blocks(text: &str) -> Vec<String> {
    let re = Regex::new(r"(requires\s*\[.*?\]\s*)?(block\s*\d+\s*\{)").unwrap();
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

pub fn extract_block_meta(block_text: &str) -> (usize, Vec<usize>, String) {
    let header_re = Regex::new(r"(?s)^(?:requires\[(.*?)\]\s*)?block\s*(\d+)\s*\{").unwrap();
    let header_caps = header_re.captures(block_text).expect("Invalid block format");
    let req_str = header_caps.get(1).map(|m| m.as_str()).unwrap_or("");
    let requirements: Vec<usize> = if req_str.is_empty() {
        Vec::new()
    } else {
        req_str.split(',')
            .filter_map(|s| s.trim().parse::<usize>().ok())
            .collect()
    };
    let block_id: usize = header_caps.get(2).unwrap().as_str().parse().expect("Invalid block id");
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

    #[test]
    fn test_split_blocks_with_labels() {
        let input = "block 1{stuff}block 23{stuffgoeshere}requires[stuff]block 42{morestuff}";
        let expected = vec![
            "block 1{stuff}",
            "block 23{stuffgoeshere}",
            "requires[stuff]block 42{morestuff}",
        ];
        let result = split_blocks(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_extract_block_meta() {
        let input = "requires[1,2,3]\nblock 42 {\n    some content here\n}";
        let expected_id = 42;
        let expected_requirements = vec![1, 2, 3];
        let expected_content = "some content here";
        let (block_id, requirements, content) = extract_block_meta(input);
        assert_eq!(block_id, expected_id);
        assert_eq!(requirements, expected_requirements);
        assert_eq!(content, expected_content);
    }

    #[test]
    fn test_extract_block_meta_no_requires() {
        let input = "block 42 {\ncontent without requires\n}";
        let expected_id = 42;
        let expected_requirements = vec![];
        let expected_content = "content without requires";
        let (block_id, requirements, content) = extract_block_meta(input);
        assert_eq!(block_id, expected_id);
        assert_eq!(requirements, expected_requirements);
        assert_eq!(content, expected_content);
    }

    #[test]
    fn test_1() {
        let input = "requires[1]\nblock 2{}";
        let expected_id = 2;
        let expected_requirements = vec![1];
        let expected_content = "";
        let (block_id, requirements, content) = extract_block_meta(input);
        assert_eq!(block_id, expected_id);
        assert_eq!(requirements, expected_requirements);
        assert_eq!(content, expected_content);
        let block = TokenBlock::new(block_id, requirements, VecDeque::new());
        println!("{:?}", block);
        println!("1");
        println!("2");
        
    }
}