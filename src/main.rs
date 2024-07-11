use build_script::run;
use compiler::Compiler;

mod interpreter;
mod parse;
mod tokenize;
mod compiler;
mod llvm_ir;
pub mod build_script;

fn main() {
    let code = include_str!("sarateese.txt");
    let mut compiler = Compiler::new();
    let mut tokens = compiler.tokenize(code);
    let statements = compiler.parse(&mut tokens);
    println!("statements: {:?}", statements);
    println!("Interpreted output:");
    compiler.interpret(&statements);
    run(&statements, compiler.variable_map);
}