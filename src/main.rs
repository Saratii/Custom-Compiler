use std::{collections::HashSet, env, fs};
use std::sync::Arc;
use compiler::Compiler;
use dag::{build_dag, print_dag};
use thread_handler::parallel;
use token_block::{extract_block_meta, split_blocks, TokenBlock};
use tokenizer::tokenize;

mod interpreter;
mod parse;
pub mod tokenizer;
mod compiler;
mod llvm_ir;
pub mod build_script;
pub mod dag;
pub mod token_block;
pub mod thread_handler;

const RED: &str = "\x1b[31m";
const RESET: &str = "\x1b[0m";

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("{}Error[1]: File Name Required{}", RED, RESET);
    }
    let file_name = &args[1];
    let very_verbose = args.len() > 2 && args[2] == "-vv";
    let verbose = args.len() > 2 && (args[2] == "-v" || very_verbose);
    let text = read_file(file_name);
    let arc_compiler = Arc::new(Compiler::new());
    let string_blocks = split_blocks(&text);
    let mut token_blocks = HashSet::new();
    for block in string_blocks {
        let (id, requires, content) = extract_block_meta(&block);
        if !token_blocks.insert(TokenBlock::new(id, requires, tokenize(&content))) {
            panic!("{}Error[2]: Duplicate Block ID: {}{}", RED, id, RESET);
        }
    }
    if very_verbose {
        for block in &token_blocks {
            println!("{:?}", block);
        }
    }
    let dag = build_dag(&token_blocks);
    if very_verbose {
        print_dag(&dag);
    }
    parallel(dag, Arc::clone(&arc_compiler), verbose);
    // run(&statements, compiler.variable_map);
}

//Reads the raw text of a file.
//Args: file_name: &str - the name of the file to read.
//Returns: String - the contents of the file.
//Exits: If there is an error reading the file.
fn read_file(file_name: &str) -> String {
    match fs::read_to_string(file_name) {
        Ok(contents) => {
            contents
        }
        Err(err) => {
            panic!("{}Error[4]: Error reading file {}: {}{}", RED, file_name, err, RESET);
        }
    }
}