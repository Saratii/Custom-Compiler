use crate::eval::evaluate;

mod eval;
mod lex;
mod tokenize;

fn main() {
    let code = include_str!("sarateese.txt");
    let tokens = tokenize::parse_to_tokens(code);
    let lines = lex::parse(tokens);
    evaluate(lines);
}
