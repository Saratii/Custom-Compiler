use crate::eval::evaluate;

mod eval;
mod lex;
mod tokenize;

fn main() {
    // let remove_tabs = Regex::new(r"\n\s+").unwrap();
    // let test = "\ni32 e = 69\nwhile (True){\n    print(e)\n}";
    // let new = remove_tabs.replace_all(&test, "\n").to_string();
    // println!("{}", test);
    // println!("{}", new);
    let code = include_str!("sarateese.txt");
    let tokens = tokenize::parse_to_tokens(code);
    println!("{:?}", tokens);
    let lines = lex::lex(tokens);
    evaluate(lines);
}
