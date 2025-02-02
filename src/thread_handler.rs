use std::collections::HashMap;
use std::sync::Arc;
use chrono::Local;
use crate::{compiler::Compiler, token_block::TokenBlock};

const PURPLE: &str = "\x1b[35m";
const RESET: &str = "\x1b[0m";

pub fn parallel(dag: HashMap<usize, TokenBlock>, compiler: Arc<Compiler>, verbose: bool) {
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
    let global_start = Local::now();
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
                let mut local_compiler = (*compiler_clone).clone();
                let mut statements = local_compiler.parse(&mut block.tokens);
                local_compiler.interpret(&mut statements);
                let now = Local::now();
                if verbose {
                    let elapsed_ms = now.signed_duration_since(start_time).num_microseconds().unwrap_or(0) as f64 / 1000.0;
                    if elapsed_ms > 1000.0 {
                        let elapsed_sec = elapsed_ms / 1000.0;
                        println!("Block {} finished at {} ({:.3}s)", id_copy, now.format("%H:%M:%S"), elapsed_sec);
                    } else {
                        println!("Block {} finished at {} ({:.3}ms)", id_copy, now.format("%H:%M:%S"), elapsed_ms);
                    }
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
    if verbose {
        let global_end = Local::now();
        let elapsed_ms = global_end.signed_duration_since(global_start).num_microseconds().unwrap_or(0) as f64 / 1000.0;
        if elapsed_ms > 1000.0 {
            let elapsed_sec = elapsed_ms / 1000.0;
            println!("{}Finished execution in {:.3}s{}", PURPLE, elapsed_sec, RESET);
        } else {
            println!("{}Finished execution in {:.3}ms{}", PURPLE, elapsed_ms, RESET);
        }
    }
}
