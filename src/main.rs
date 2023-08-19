use crate::eval::evaluate;

mod eval;
mod parse;
mod tokenize;

fn main() {
    let code = include_str!("sarateese.txt");
    let tokens = tokenize::parse_to_tokens(code);
    // println!("tokens: {:?}", tokens);
    let lines = parse::parse(tokens);
    evaluate(lines);
}
