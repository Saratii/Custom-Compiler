use std::collections::VecDeque;
use std::fs::File;

use crate::compiler::Compiler;
use crate::parse::Statement;

impl Compiler{
    pub fn to_llvm_x86(&self, statements: &VecDeque<Statement>){
        let mut asm_file = File::create("src/main.ll").expect("creation failed");
    }
}
//clang -o main.exe main.ll     
//./main.exe