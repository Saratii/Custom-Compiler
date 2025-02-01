use std::{collections::{HashMap, HashSet}, sync::Mutex};
use std::sync::Arc;
use chrono::Local;
use crate::{compiler::Compiler, token_block::TokenBlock};

pub fn build_dag(token_blocks: &HashSet<TokenBlock>) -> HashMap<usize, Vec<&TokenBlock>> {
    let mut dag: HashMap<usize, Vec<&TokenBlock>> = HashMap::new();
    let block_map: HashMap<usize, &TokenBlock> = token_blocks.iter().map(|b| (b.id, b)).collect();
    for block in token_blocks {
        dag.entry(block.id).or_insert_with(Vec::new);
    }
    for block in token_blocks {
        for &required_id in &block.requires {
            if !block_map.contains_key(&required_id) {
                eprintln!(
                    "Error: Block {} requires block {} which is not defined!",
                    block.id, required_id
                );
                panic!("Missing dependency: block {} not defined", required_id);
            }
            dag.entry(required_id).or_insert_with(Vec::new).push(block);
        }
    }
    dag
}

pub fn parallel(dag: HashMap<usize, TokenBlock>, compiler: Arc<Mutex<Compiler>>, verbose: bool) {
    let mut in_degree: HashMap<usize, usize> = HashMap::new();
    for (&id, block) in dag.iter() {
        in_degree.insert(id, block.requires.len());
    }
    let mut children_map: HashMap<usize, Vec<usize>> = HashMap::new();
    for (&id, block) in dag.iter() {
        for &req in &block.requires {
            children_map.entry(req).or_insert_with(Vec::new).push(id);
        }
    }
    while !in_degree.is_empty() {
        let ready: Vec<usize> = in_degree
            .iter()
            .filter(|(_, &deg)| deg == 0)
            .map(|(&id, _)| id)
            .collect();
        if ready.is_empty() {
            eprintln!("Cycle detected or unsatisfiable dependencies!");
            break;
        }
        let mut handles = Vec::new();
        for id in ready.iter() {
            let id_copy = *id;
            let mut block = dag.get(&id_copy).unwrap().clone();
            let compiler_clone = Arc::clone(&compiler);
            handles.push(std::thread::spawn(move || {
                let start_time = Local::now();
                if verbose {
                    println!("Block {} starting at {}", id_copy, start_time.format("%H:%M:%S"));
                }
                let mut compiler = compiler_clone.lock().unwrap();
                let mut statements = compiler.parse(&mut block.tokens);
                compiler.interpret(&mut statements);
                if verbose {
                    let now = Local::now();
                    println!("Block {} finished at {} ({:?}ms)", id_copy, now.format("%H:%M:%S"), now.signed_duration_since(start_time).num_microseconds().unwrap_or(0) as f64 / 1000.0);
                }
            }));
        }
        for handle in handles {
            handle.join().unwrap();
        }
        for id in ready {
            in_degree.remove(&id);
            if let Some(children) = children_map.get(&id) {
                for &child in children {
                    if let Some(count) = in_degree.get_mut(&child) {
                        *count -= 1;
                    }
                }
            }
        }
    }
}
