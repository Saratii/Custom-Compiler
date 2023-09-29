use compiler::Compiler;


mod eval;
mod parse;
mod tokenize;
mod compiler;

fn main() {
    let code = include_str!("sarateese.txt");
    let mut compiler = Compiler::new();
    let mut tokens = compiler.tokenize(code);
    let statements = compiler.parse(&mut tokens);
    compiler.evaluate(statements)
    // println!("tokens: {:?}", tokens);
    // let lines = parse::parse_tokens(&mut tokens);
    // println!("lines: {:?}", lines);

    // evaluate(lines);
}