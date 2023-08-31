use crate::eval::evaluate;

mod eval;
mod parse;
mod tokenize;

fn main() {
    let code = include_str!("sarateese.txt");
    let mut tokens = tokenize::parse_to_tokens(code);
    // println!("tokens: {:?}", tokens);
    let lines = parse::parse_tokens(&mut tokens);
    // println!("lines: {:?}", lines);

    evaluate(lines);
}
