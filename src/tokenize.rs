use std::{collections::VecDeque, fmt::Display};

use regex::Regex;

#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    Print,
    String(String),
    OpenParen,
    CloseParen,
    StartBlock,
    EndBlock,
    TypeI32,
    TypeString,
    TypeBool,
    VariableName(String),
    ConstantNumber(String),
    Boolean(bool),
    WhileLoop,
    MathOp(MathOp),
    EndLine,
    If,
    ForLoop,
    Comma,
    Ignore,
    Increment,
    Decrement,
    Else,
    Elif,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Print => write!(f, "Print"),
            Token::String(_) => write!(f, "String"),
            Token::OpenParen => write!(f, "OpenParen"),
            Token::CloseParen => write!(f, "CloseParen"),
            Token::StartBlock => write!(f, "StartBlock"),
            Token::EndBlock => write!(f, "EndBlock"),
            Token::TypeI32 => write!(f, "TypeI32"),
            Token::TypeString => write!(f, "TypeString"),
            Token::TypeBool => write!(f, "TypeBool"),
            Token::VariableName(_) => write!(f, "VariableName"),
            Token::ConstantNumber(_) => write!(f, "ConstantNumber"),
            Token::Boolean(_) => write!(f, "Boolean"),
            Token::WhileLoop => write!(f, "WhileLoop"),
            Token::MathOp(_) => write!(f, "MathOp"),
            Token::EndLine => write!(f, "EndLine"),
            Token::If => write!(f, "If"),
            Token::ForLoop => write!(f, "ForLoop"),
            Token::Comma => write!(f, "Comma"),
            Token::Ignore => write!(f, "Ignore"),
            Token::Increment => write!(f, "IncrementUp"),
            Token::Decrement => write!(f, "IncrementDown"),
            Token::Else => write!(f, "Else"),
            Token::Elif => write!(f, "Elif"),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum MathOp {
    Add,
    Multiply,
    Divide,
    Subtract,
    Modulus,
    Equals,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanOrEqualTo,
    GreaterThanOrEqualTo,
}

static KEYWORDS: &'static [(&str, Token)] = &[
    ("print", Token::Print),
    ("(", Token::OpenParen),
    (")", Token::CloseParen),
    ("}", Token::EndBlock),
    ("{", Token::StartBlock),
    ("String ", Token::TypeString),
    ("i32 ", Token::TypeI32),
    ("Bool ", Token::TypeBool),
    (" + ", Token::MathOp(MathOp::Add)),
    (" - ", Token::MathOp(MathOp::Subtract)),
    (" * ", Token::MathOp(MathOp::Multiply)),
    (" / ", Token::MathOp(MathOp::Divide)),
    (" % ", Token::MathOp(MathOp::Modulus)),
    (" > ", Token::MathOp(MathOp::GreaterThan)),
    (" < ", Token::MathOp(MathOp::LessThan)),
    (" >= ", Token::MathOp(MathOp::GreaterThanOrEqualTo)),
    (" <= ", Token::MathOp(MathOp::LessThanOrEqualTo)),
    (", ", Token::Comma),
    (" == ", Token::MathOp(MathOp::Equals)),
    (" != ", Token::MathOp(MathOp::NotEqual)),
    ("True", Token::Boolean(true)),
    ("False", Token::Boolean(false)),
    (";", Token::EndLine),
    ("while ", Token::WhileLoop),
    ("while", Token::WhileLoop),
    ("for", Token::ForLoop),
    ("if", Token::If),
    (" = ", Token::Ignore),
    ("++", Token::Increment),
    ("--", Token::Decrement),
    ("else", Token::Else),
    ("elif", Token::Elif),
];

pub fn parse_to_tokens(raw: &str) -> VecDeque<Token> {
    let remove_comments_regex = Regex::new(r"(?:\/\/(.*)|\/\*((?:.|[\r\n])*?)\*\/)").unwrap();
    let remove_tabs = Regex::new(r"\n\s+").unwrap();
    let removed_comments = remove_comments_regex.replace_all(raw, "").to_string();
    let mut inputs = remove_tabs.replace_all(&removed_comments.as_str(), "\n").to_string();
    inputs = inputs.replace("\r", "").replace("\n", "");
    let number_regex = Regex::new(r"^(\d+)").unwrap();
    let name_regex = Regex::new(r"^([a-zA-Z_][a-zA-Z0-9_]*)").unwrap();

    let mut tokens = VecDeque::new();
    'outer: while &inputs.len() > &0 {
        for (keyword, token) in KEYWORDS {
            if inputs.starts_with(keyword) {
                tokens.push_back(token.clone());
                inputs = inputs[keyword.len()..].to_string();
                continue 'outer;
            }
        }
        if inputs.starts_with("\"") {
            inputs = inputs[1..].to_string();
            let mut string = "".to_string();
            while !inputs.starts_with("\"") {
                string = string + &inputs.chars().nth(0).unwrap().to_string();
                inputs = inputs[1..].to_string();
            }
            tokens.push_back(Token::String(string));
            inputs = inputs[1..].to_string();
        } else if number_regex.is_match(&inputs) {
            let constant_number = number_regex
                .captures(&inputs)
                .unwrap()
                .get(0)
                .unwrap()
                .as_str();
            tokens.push_back(Token::ConstantNumber(constant_number.to_string()));
            inputs = inputs[constant_number.len()..].to_string();
        } else if inputs.starts_with("true") {
            panic!("you typed: true, did you mean True?");
        } else if inputs.starts_with("false") {
            panic!("you typed: false, did you mean False?");
        } else if name_regex.is_match(&inputs) {
            let variable_name = name_regex
                .captures(&inputs)
                .unwrap()
                .get(0)
                .unwrap()
                .as_str();
            tokens.push_back(Token::VariableName(variable_name.to_string()));
            inputs = inputs[variable_name.len()..].to_string();
        } else {
            if inputs.len() != 0 {
                println!("Oopsie Woopsie: Code contains something that doesnt parse or hidden characters: {}", &inputs[0..])
            }
            break;
        }
    }
    tokens.retain(|token| *token != Token::Ignore);
    tokens
}

#[cfg(test)]
mod test {

    use crate::tokenize::MathOp;

    use super::{parse_to_tokens, Token};

    #[test]
    fn test_1() {
        let actual = parse_to_tokens("print(\"hello world\");");
        let expected = vec![
            Token::Print,
            Token::OpenParen,
            Token::String("hello world".to_string()),
            Token::CloseParen,
            Token::EndLine,
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn integer_variable_test() {
        let actual = parse_to_tokens("i32 eeebo = 69;");
        let expected = vec![
            Token::TypeI32,
            Token::VariableName("eeebo".to_string()),
            Token::ConstantNumber("69".to_string()),
            Token::EndLine,
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn string_variable_test() {
        let actual = parse_to_tokens("String beebo = \"inhale carbon monoxide\";");
        let expected = vec![
            Token::TypeString,
            Token::VariableName("beebo".to_string()),
            Token::String("inhale carbon monoxide".to_string()),
            Token::EndLine,
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn bool_variable_test() {
        let actual = parse_to_tokens("Bool feefoo = False;");
        let expected = vec![
            Token::TypeBool,
            Token::VariableName("feefoo".to_string()),
            Token::Boolean(false),
            Token::EndLine,
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn i32_variable_test() {
        let actual = parse_to_tokens("i32 furfu = 420;");
        let expected = vec![
            Token::TypeI32,
            Token::VariableName("furfu".to_string()),
            Token::ConstantNumber("420".to_string()),
            Token::EndLine,
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn print_variable_test() {
        let actual = parse_to_tokens("Bool eee = True;\nprint(eee);");
        let expected = vec![
            Token::TypeBool,
            Token::VariableName("eee".to_string()),
            Token::Boolean(true),
            Token::EndLine,
            Token::Print,
            Token::OpenParen,
            Token::VariableName("eee".to_string()),
            Token::CloseParen,
            Token::EndLine,
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn print_string_variable() {
        let actual = parse_to_tokens("String ee = \"should I kill myself?\";\nprint(ee);");
        let expected = vec![
            Token::TypeString,
            Token::VariableName("ee".to_string()),
            Token::String("should I kill myself?".to_string()),
            Token::EndLine,
            Token::Print,
            Token::OpenParen,
            Token::VariableName("ee".to_string()),
            Token::CloseParen,
            Token::EndLine,
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn while_true() {
        let actual = parse_to_tokens("while (True){\nprint(69);\n}");
        let expected = vec![
            Token::WhileLoop,
            Token::OpenParen,
            Token::Boolean(true),
            Token::CloseParen,
            Token::StartBlock,
            Token::Print,
            Token::OpenParen,
            Token::ConstantNumber("69".to_string()),
            Token::CloseParen,
            Token::EndLine,
            Token::EndBlock,
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn format_tab() {
        let actual = parse_to_tokens(
            "
i32 e = 69;
while (True){
    print(e);
}",
        );
        let expected = vec![
            Token::TypeI32,
            Token::VariableName("e".to_string()),
            Token::ConstantNumber("69".to_string()),
            Token::EndLine,
            Token::WhileLoop,
            Token::OpenParen,
            Token::Boolean(true),
            Token::CloseParen,
            Token::StartBlock,
            Token::Print,
            Token::OpenParen,
            Token::VariableName("e".to_string()),
            Token::CloseParen,
            Token::EndLine,
            Token::EndBlock,
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn change_variable() {
        let actual = parse_to_tokens("i32 i = 0;\ni = 1;\nString e = \"hello\";\ne = \"bye\";\nBool yes = True;\nyes = False;");
        let expected = vec![
            Token::TypeI32,
            Token::VariableName("i".to_string()),
            Token::ConstantNumber("0".to_string()),
            Token::EndLine,
            Token::VariableName("i".to_string()),
            Token::ConstantNumber("1".to_string()),
            Token::EndLine,
            Token::TypeString,
            Token::VariableName("e".to_string()),
            Token::String("hello".to_string()),
            Token::EndLine,
            Token::VariableName("e".to_string()),
            Token::String("bye".to_string()),
            Token::EndLine,
            Token::TypeBool,
            Token::VariableName("yes".to_string()),
            Token::Boolean(true),
            Token::EndLine,
            Token::VariableName("yes".to_string()),
            Token::Boolean(false),
            Token::EndLine,
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn simple_math() {
        let actual =
            parse_to_tokens("i32 e = 4 + 3;\ni32 ee = 4 - 3;\ni32 eee = 8 / 2;\ni32 eeee = 8 * 2;");
        let expected = vec![
            Token::TypeI32,
            Token::VariableName("e".to_string()),
            Token::ConstantNumber("4".to_string()),
            Token::MathOp(MathOp::Add),
            Token::ConstantNumber("3".to_string()),
            Token::EndLine,
            Token::TypeI32,
            Token::VariableName("ee".to_string()),
            Token::ConstantNumber("4".to_string()),
            Token::MathOp(MathOp::Subtract),
            Token::ConstantNumber("3".to_string()),
            Token::EndLine,
            Token::TypeI32,
            Token::VariableName("eee".to_string()),
            Token::ConstantNumber("8".to_string()),
            Token::MathOp(MathOp::Divide),
            Token::ConstantNumber("2".to_string()),
            Token::EndLine,
            Token::TypeI32,
            Token::VariableName("eeee".to_string()),
            Token::ConstantNumber("8".to_string()),
            Token::MathOp(MathOp::Multiply),
            Token::ConstantNumber("2".to_string()),
            Token::EndLine,
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn multi_term_simple_math() {
        let actual = parse_to_tokens("i32 foo = 3 + 5 / 4 * 68;");
        let expected = vec![
            Token::TypeI32,
            Token::VariableName("foo".to_string()),
            Token::ConstantNumber("3".to_string()),
            Token::MathOp(MathOp::Add),
            Token::ConstantNumber("5".to_string()),
            Token::MathOp(MathOp::Divide),
            Token::ConstantNumber("4".to_string()),
            Token::MathOp(MathOp::Multiply),
            Token::ConstantNumber("68".to_string()),
            Token::EndLine,
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn print_equation() {
        let actual = parse_to_tokens("print(1 + 69);");
        let expected = vec![
            Token::Print,
            Token::OpenParen,
            Token::ConstantNumber("1".to_string()),
            Token::MathOp(MathOp::Add),
            Token::ConstantNumber("69".to_string()),
            Token::CloseParen,
            Token::EndLine,
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn variable_adding() {
        let actual = parse_to_tokens("i32 e = 1;i32 ee = 2;print(e + ee);");
        let expected = vec![
            Token::TypeI32,
            Token::VariableName("e".to_string()),
            Token::ConstantNumber("1".to_string()),
            Token::EndLine,
            Token::TypeI32,
            Token::VariableName("ee".to_string()),
            Token::ConstantNumber("2".to_string()),
            Token::EndLine,
            Token::Print,
            Token::OpenParen,
            Token::VariableName("e".to_string()),
            Token::MathOp(MathOp::Add),
            Token::VariableName("ee".to_string()),
            Token::CloseParen,
            Token::EndLine,
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn basic_if() {
        let actual = parse_to_tokens("i32 e = 69;if(e == 69){print(e);}");
        let expected = vec![
            Token::TypeI32,
            Token::VariableName("e".to_string()),
            Token::ConstantNumber("69".to_string()),
            Token::EndLine,
            Token::If,
            Token::OpenParen,
            Token::VariableName("e".to_string()),
            Token::MathOp(MathOp::Equals),
            Token::ConstantNumber("69".to_string()),
            Token::CloseParen,
            Token::StartBlock,
            Token::Print,
            Token::OpenParen,
            Token::VariableName("e".to_string()),
            Token::CloseParen,
            Token::EndLine,
            Token::EndBlock,
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn for_loop() {
        let actual = parse_to_tokens("for(i32 i = 0, i < 10, i++){\nprint(i);\n}");
        let expected = vec![
            Token::ForLoop,
            Token::OpenParen,
            Token::TypeI32,
            Token::VariableName("i".to_string()),
            Token::ConstantNumber("0".to_string()),
            Token::Comma,
            Token::VariableName("i".to_string()),
            Token::MathOp(MathOp::LessThan),
            Token::ConstantNumber("10".to_string()),
            Token::Comma,
            Token::VariableName("i".to_string()),
            Token::Increment,
            Token::CloseParen,
            Token::StartBlock,
            Token::Print,
            Token::OpenParen,
            Token::VariableName("i".to_string()),
            Token::CloseParen,
            Token::EndLine,
            Token::EndBlock,
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn double_if() {
        let actual = parse_to_tokens("if(True){if(False){print(\"a\");}}");
        let expected = vec![
            Token::If,
            Token::OpenParen,
            Token::Boolean(true),
            Token::CloseParen,
            Token::StartBlock,
            Token::If,
            Token::OpenParen,
            Token::Boolean(false),
            Token::CloseParen,
            Token::StartBlock,
            Token::Print,
            Token::OpenParen,
            Token::String("a".to_string()),
            Token::CloseParen,
            Token::EndLine,
            Token::EndBlock,
            Token::EndBlock,
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn basic_comment(){
        let actual = parse_to_tokens("i32 i = 10;\n//i32 e = 9;\ni32 g = 8;");
        let expected = vec![
            Token::TypeI32,
            Token::VariableName("i".to_string()),
            Token::ConstantNumber("10".to_string()),
            Token::EndLine,
            Token::TypeI32,
            Token::VariableName("g".to_string()),
            Token::ConstantNumber("8".to_string()),
            Token::EndLine
            ];
            assert_eq!(actual, expected);
    }
    #[test]
    fn multi_line_comment(){
        let actual = parse_to_tokens("i32 i = 10;\n/*unga\nbunga\nwunga\n*/i32 e = 0;");
        let expected = vec![
            Token::TypeI32,
            Token::VariableName("i".to_string()),
            Token::ConstantNumber("10".to_string()),
            Token::EndLine,
            Token::TypeI32,
            Token::VariableName("e".to_string()),
            Token::ConstantNumber("0".to_string()),
            Token::EndLine,
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn else_elif_test(){
        let actual = parse_to_tokens("if(i == 6){}elif(i == 7){}else{print(\"e\");}");
        let expected = vec![
            Token::If,
            Token::OpenParen,
            Token::VariableName("i".to_string()),
            Token::MathOp(MathOp::Equals),
            Token::ConstantNumber("6".to_string()),
            Token::CloseParen,
            Token::StartBlock,
            Token::EndBlock,
            Token::Elif,
            Token::OpenParen,
            Token::VariableName("i".to_string()),
            Token::MathOp(MathOp::Equals),
            Token::ConstantNumber("7".to_string()),
            Token::CloseParen,
            Token::StartBlock,
            Token::EndBlock,
            Token::Else,
            Token::StartBlock,
            Token::Print,
            Token::OpenParen,
            Token::String("e".to_string()),
            Token::CloseParen,
            Token::EndLine,
            Token::EndBlock,
        ];
        assert_eq!(actual, expected);
    }
}
