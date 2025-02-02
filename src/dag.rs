use std::collections::{HashMap, HashSet};
use crate::token_block::TokenBlock;

const GREEN: &str = "\x1b[32m";
const RESET: &str = "\x1b[0m";
const RED: &str = "\x1b[31m";

pub fn build_dag(token_blocks: &HashSet<TokenBlock>) -> HashMap<usize, TokenBlock> {
    let valid_ids: HashSet<usize> = token_blocks.iter().map(|block| block.id).collect();
    let mut dag: HashMap<usize, TokenBlock> = HashMap::new();
    for block in token_blocks {
        for &required_id in &block.requires {
            if !valid_ids.contains(&required_id) {
                panic!("{}Error[3]: Block {} requires block {} which is not defined!{}", RED, block.id, required_id, RESET);
            }
        }
        dag.insert(block.id, block.clone());
    }
    dag
}

fn build_children_map(dag: &HashMap<usize, TokenBlock>) -> HashMap<usize, Vec<usize>> {
    let mut children_map: HashMap<usize, Vec<usize>> = HashMap::new();
    for &id in dag.keys() {
        children_map.insert(id, Vec::new());
    }
    for (&id, block) in dag.iter() {
        for &required_id in &block.requires {
            if let Some(vec) = children_map.get_mut(&required_id) {
                vec.push(id);
            }
        }
    }
    children_map
}

pub fn print_dag(dag: &HashMap<usize, TokenBlock>) {
    println!("{}DAG:{} ", GREEN, RESET);
    let children_map = build_children_map(dag);
    let mut child_ids = HashSet::new();
    for children in children_map.values() {
        for &child in children {
            child_ids.insert(child);
        }
    }
    let mut roots: Vec<usize> = dag.keys().filter(|&&id| !child_ids.contains(&id)).cloned().collect();
    roots.sort();
    if roots.is_empty() {
        roots = dag.keys().cloned().collect();
        roots.sort();
    }
    let mut printed = HashSet::new();
    for (i, root) in roots.iter().enumerate() {
        let is_last = i == roots.len() - 1;
        print_tree(*root, &children_map, dag, "", true, is_last, &mut printed);
    }
}

fn print_tree(
    node: usize,
    children_map: &HashMap<usize, Vec<usize>>,
    dag: &HashMap<usize, TokenBlock>,
    prefix: &str,
    is_root: bool,
    is_last: bool,
    printed: &mut HashSet<usize>,
) {
    if is_root {
        println!("{}Block {}{}", GREEN, node, RESET);
    } else {
        let connector = if is_last { "└── " } else { "├── " };
        println!("{}{}Block {}{}", prefix, connector, node, RESET);
    }
    if printed.contains(&node) {
        return;
    }
    printed.insert(node);
    if let Some(children) = children_map.get(&node) {
        let mut children = children.clone();
        children.sort();
        let count = children.len();
        for (i, &child) in children.iter().enumerate() {
            let child_is_last = i == count - 1;
            let new_prefix = if is_root {
                "".to_string()
            } else {
                format!("{}{}", prefix, if is_last { "    " } else { "│   " })
            };
            print_tree(child, children_map, dag, &new_prefix, false, child_is_last, printed);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{HashSet, VecDeque};

    use crate::{dag::{build_dag, print_dag}, token_block::TokenBlock};

    #[test]
    fn test_build_and_print_dag() {
        let block1 = TokenBlock { id: 1, requires: vec![], tokens: VecDeque::new() };
        let block2 = TokenBlock { id: 2, requires: vec![], tokens: VecDeque::new() };
        let block3 = TokenBlock { id: 3, requires: vec![], tokens: VecDeque::new() };
        let block4 = TokenBlock { id: 4, requires: vec![1], tokens: VecDeque::new() };
        let token_blocks: HashSet<TokenBlock> = vec![block1, block2, block3, block4].into_iter().collect();
        let dag = build_dag(&token_blocks);
        assert_eq!(dag.len(), 4);
        assert!(dag.contains_key(&1));
        assert!(dag.contains_key(&2));
        assert!(dag.contains_key(&3));
        assert!(dag.contains_key(&4));
        print_dag(&dag);
    }
}
