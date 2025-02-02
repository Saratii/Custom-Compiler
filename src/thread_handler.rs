use std::{collections::HashMap, sync::{Arc, Mutex, mpsc}, time::Duration};
use chrono::Local;
use crate::{interpreter::{interpret, Primitive, Type}, parse::parse, token_block::TokenBlock};

const PURPLE: &str = "\x1b[35m";
const RESET: &str = "\x1b[0m";

pub fn parallel(dag: HashMap<String, TokenBlock>, verbose: bool) {
    let master_variable_map: Arc<Mutex<HashMap<String, HashMap<String, (Primitive, Type)>>>> =
        Arc::new(Mutex::new(HashMap::new()));
    let mut in_deg: HashMap<String, usize> = HashMap::new();
    let mut children_map: HashMap<String, Vec<String>> = HashMap::new();
    for (id, block) in dag.iter() {
        in_deg.insert(id.clone(), block.requires.len());
        children_map.insert(id.clone(), Vec::new());
    }
    for (_id, block) in dag.iter() {
        for required_id in block.requires.keys() {
            children_map.entry(required_id.clone()).or_insert_with(Vec::new).push(block.id.clone());
        }
    }
    let in_degree: Arc<Mutex<HashMap<String, usize>>> = Arc::new(Mutex::new(in_deg));
    let children: Arc<Mutex<HashMap<String, Vec<String>>>> = Arc::new(Mutex::new(children_map));
    let dag = Arc::new(dag);
    let total_tasks = dag.len();
    let (tx, rx) = mpsc::channel();
    {
        let in_degree_lock = in_degree.lock().unwrap();
        for (id, &deg) in in_degree_lock.iter() {
            if deg == 0 {
                tx.send(id.clone()).unwrap();
            }
        }
    }
    let tasks_done: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));
    let global_start = Local::now();
    while *tasks_done.lock().unwrap() < total_tasks {
        match rx.recv_timeout(Duration::from_millis(100)) {
            Ok(task_id) => {
                let tx_inner = tx.clone();
                let master_var_map_clone = Arc::clone(&master_variable_map);
                let in_degree_clone = Arc::clone(&in_degree);
                let children_clone = Arc::clone(&children);
                let dag_clone = Arc::clone(&dag);
                let tasks_done_clone = Arc::clone(&tasks_done);
                std::thread::spawn(move || {
                    let start_time = Local::now();
                    if verbose {
                        println!("Block {} starting at {}", task_id, start_time.format("%H:%M:%S"));
                    }
                    let block = dag_clone.get(&task_id).unwrap().clone();
                    let mut statements = parse(&mut block.tokens.clone());
                    let mut inherited_variable_map = Vec::new();
                    for req_id in block.requires.keys() {
                        if block.requires[req_id].is_empty() {
                            continue;
                        }
                        if let Some(map) = master_var_map_clone.lock().unwrap().get(req_id) {
                            inherited_variable_map.push(map.clone());
                        }
                    }
                    let local_variable_map = interpret(&mut statements, inherited_variable_map);
                    {
                        let mut master = master_var_map_clone.lock().unwrap();
                        master.insert(task_id.clone(), local_variable_map);
                    }
                    let now = Local::now();
                    if verbose {
                        let elapsed_ms = now.signed_duration_since(start_time).num_microseconds().unwrap_or(0) as f64 / 1000.0;
                        if elapsed_ms > 1000.0 {
                            let elapsed_sec = elapsed_ms / 1000.0;
                            println!("Block {} finished at {} ({:.3}s)", task_id, now.format("%H:%M:%S"), elapsed_sec);
                        } else {
                            println!("Block {} finished at {} ({:.3}ms)", task_id, now.format("%H:%M:%S"), elapsed_ms);
                        }
                    }
                    {
                        let mut in_deg_lock = in_degree_clone.lock().unwrap();
                        let children_map = children_clone.lock().unwrap();
                        if let Some(child_ids) = children_map.get(&task_id) {
                            for child in child_ids {
                                if let Some(count) = in_deg_lock.get_mut(child) {
                                    *count -= 1;
                                    if *count == 0 {
                                        tx_inner.send(child.clone()).unwrap();
                                    }
                                }
                            }
                        }
                    }
                    {
                        let mut done = tasks_done_clone.lock().unwrap();
                        *done += 1;
                    }
                });
            }
            Err(_) => { }
        }
    }
    let global_end = Local::now();
    let elapsed_ms = global_end.signed_duration_since(global_start).num_microseconds().unwrap_or(0) as f64 / 1000.0;
    if verbose {
        if elapsed_ms > 1000.0 {
            let elapsed_sec = elapsed_ms / 1000.0;
            println!("{}Finished execution in {:.3}s{}", PURPLE, elapsed_sec, RESET);
        } else {
            println!("{}Finished execution in {:.3}ms{}", PURPLE, elapsed_ms, RESET);
        }
    }
}
