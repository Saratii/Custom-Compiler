use compiler::Compiler;


mod eval;
mod parse;
mod tokenize;
mod compiler;
mod assembler;

fn main() {
    let code = include_str!("sarateese.txt");
    let mut compiler = Compiler::new();
    let mut tokens = compiler.tokenize(code);
    let statements = compiler.parse(&mut tokens);
    compiler.evaluate(&statements);
    // compiler.to_llvm_x86(&statements);
}