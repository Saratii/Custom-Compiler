use std::{collections::VecDeque, fmt::Display};

use regex::Regex;

#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    String(String),
    OpenParen,
    CloseParen,
    StartBlock,
    EndBlock,
    Identifier,
    ConstantNumber(String),
    Boolean(bool),
    WhileLoop,
    MathOp(MathOp),
    EndLine,
    If,
    ForLoop,
    Comma,
    Increment,
    Decrement,
    Else,
    Elif,
    FunctionCall(String),
    Ignore,
    OpenBracket,
    CloseBracket,
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
    And,
    Or,
    Not,
}

pub fn parse_to_tokens(raw: &str) -> VecDeque<Token> {
    let number_regex = Regex::new(r"^(\d+)").unwrap();
    let name_regex = Regex::new(r"^([a-zA-Z_][a-zA-Z0-9_]*)").unwrap();
    let remove_comments_regex = Regex::new(r"(?:\/\/(.*)|\/\*((?:.|[\r\n])*?)\*\/)").unwrap();
    let keywords = Vec::from([
        ("print(", Token::FunctionCall("print()".to_string())),
        ("(", Token::OpenParen),
        (")", Token::CloseParen),
        ("}", Token::EndBlock),
        ("{", Token::StartBlock),
        ("i32(", Token::FunctionCall("i32()".to_string())),
        ("i64(", Token::FunctionCall("i64()".to_string())),
        ("f32(", Token::FunctionCall("f32()".to_string())),
        ("f64(", Token::FunctionCall("f64()".to_string())),
        ("++", Token::Increment),
        ("--", Token::Decrement),
        ("+", Token::MathOp(MathOp::Add)),
        ("-", Token::MathOp(MathOp::Subtract)),
        ("*", Token::MathOp(MathOp::Multiply)),
        ("/", Token::MathOp(MathOp::Divide)),
        ("%", Token::MathOp(MathOp::Modulus)),
        (">", Token::MathOp(MathOp::GreaterThan)),
        ("<", Token::MathOp(MathOp::LessThan)),
        (">=", Token::MathOp(MathOp::GreaterThanOrEqualTo)),
        ("<=", Token::MathOp(MathOp::LessThanOrEqualTo)),
        ("&&", Token::MathOp(MathOp::And)),
        ("||", Token::MathOp(MathOp::Or)),
        (",", Token::Comma),
        ("==", Token::MathOp(MathOp::Equals)),
        ("!=", Token::MathOp(MathOp::NotEqual)),
        ("!", Token::MathOp(MathOp::Not)),
        ("true", Token::Boolean(true)),
        ("false", Token::Boolean(false)),
        (";", Token::EndLine),
        ("while", Token::WhileLoop),
        ("for", Token::ForLoop),
        ("if", Token::If),
        ("else", Token::Else),
        ("elif", Token::Elif),
        ("=", Token::Ignore),
        ("[", Token::OpenBracket),
        ("]", Token::CloseBracket),
    ]);
    let mut inputs = remove_comments_regex.replace_all(raw, "").to_string();
    inputs = remove_spaces(&inputs);
    let mut tokens = VecDeque::new();
    'outer: while &inputs.len() > &0 {
        for (keyword, token) in &keywords {
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
                panic!("Oopsie Woopsie: Code contains something that doesnt parse or hidden characters: {}", &inputs[0..])
            }
            break;
        }
    }
    tokens.retain(|token| *token != Token::Ignore);
    tokens
}

pub fn remove_spaces(raw: &str) -> String {
    let mut eat = true;
    let chars: Vec<_> = raw.chars().collect();
    let mut result = String::new();
    for i in 0..chars.len() {
        if chars[i] != ' ' && chars[i] != '\t' && chars[i] != '\r' && chars[i] != '\n' {
            if (chars[i] == '"' || chars[i] == '\'') && chars[i - 1] != '\\' {
                eat = !eat;
            }
            result.push(chars[i]);
        } else {
            if !eat {
                result.push(chars[i])
            }
        }
    }
    return result;
}
#[cfg(test)]
mod test {

    use crate::tokenize::MathOp;

    use super::{parse_to_tokens, Token};

    #[test]
    fn hello_world() {
        let actual = parse_to_tokens("print(\"hello world\");");
        let expected = vec![
            Token::FunctionCall("print()".to_string()),
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
        let actual = parse_to_tokens("Bool feefoo = false;");
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
        let actual = parse_to_tokens("Bool eee = true;\nprint(eee);");
        let expected = vec![
            Token::TypeBool,
            Token::VariableName("eee".to_string()),
            Token::Boolean(true),
            Token::EndLine,
            Token::FunctionCall("print()".to_string()),
            Token::VariableName("eee".to_string()),
            Token::CloseParen,
            Token::EndLine,
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn print_string_variable() {
        let actual = parse_to_tokens("String ee = \"shoul?\";\nprint(ee);");
        let expected = vec![
            Token::TypeString,
            Token::VariableName("ee".to_string()),
            Token::String("shoul?".to_string()),
            Token::EndLine,
            Token::FunctionCall("print()".to_string()),
            Token::VariableName("ee".to_string()),
            Token::CloseParen,
            Token::EndLine,
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn while_true() {
        let actual = parse_to_tokens("while (true){\nprint(69);\n}");
        let expected = vec![
            Token::WhileLoop,
            Token::OpenParen,
            Token::Boolean(true),
            Token::CloseParen,
            Token::StartBlock,
            Token::FunctionCall("print()".to_string()),
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
while (true){
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
            Token::FunctionCall("print()".to_string()),
            Token::VariableName("e".to_string()),
            Token::CloseParen,
            Token::EndLine,
            Token::EndBlock,
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn change_variable() {
        let actual = parse_to_tokens("i32 i = 0;\ni = 1;\nString e = \"hello\";\ne = \"bye\";\nBool yes = true;\nyes = false;");
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
            Token::FunctionCall("print()".to_string()),
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
            Token::FunctionCall("print()".to_string()),
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
            Token::FunctionCall("print()".to_string()),
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
            Token::FunctionCall("print()".to_string()),
            Token::VariableName("i".to_string()),
            Token::CloseParen,
            Token::EndLine,
            Token::EndBlock,
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn double_if() {
        let actual = parse_to_tokens("if(true){if(false){print(\"a\");}}");
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
            Token::FunctionCall("print()".to_string()),
            Token::String("a".to_string()),
            Token::CloseParen,
            Token::EndLine,
            Token::EndBlock,
            Token::EndBlock,
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn basic_comment() {
        let actual = parse_to_tokens("i32 i = 10;\n//i32 e = 9;\ni32 g = 8;");
        let expected = vec![
            Token::TypeI32,
            Token::VariableName("i".to_string()),
            Token::ConstantNumber("10".to_string()),
            Token::EndLine,
            Token::TypeI32,
            Token::VariableName("g".to_string()),
            Token::ConstantNumber("8".to_string()),
            Token::EndLine,
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn multi_line_comment() {
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
    fn else_elif_test() {
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
            Token::FunctionCall("print()".to_string()),
            Token::String("e".to_string()),
            Token::CloseParen,
            Token::EndLine,
            Token::EndBlock,
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn i32_i64_f32_f64() {
        let actual = parse_to_tokens("i32 i = 31;i64 e = 63;f32 f = 32; f64 g = 64;");
        let expected = vec![
            Token::TypeI32,
            Token::VariableName("i".to_string()),
            Token::ConstantNumber("31".to_string()),
            Token::EndLine,
            Token::TypeI64,
            Token::VariableName("e".to_string()),
            Token::ConstantNumber("63".to_string()),
            Token::EndLine,
            Token::TypeF32,
            Token::VariableName("f".to_string()),
            Token::ConstantNumber("32".to_string()),
            Token::EndLine,
            Token::TypeF64,
            Token::VariableName("g".to_string()),
            Token::ConstantNumber("64".to_string()),
            Token::EndLine,
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn one_dim_array() {
        let actual = parse_to_tokens("i32[] a = [];i64[]b=[100, 200];");
        let expected = vec![
            Token::TypeI32Array,
            Token::VariableName("a".to_string()),
            Token::OpenBracket,
            Token::CloseBracket,
            Token::EndLine,
            Token::TypeI64Array,
            Token::VariableName("b".to_string()),
            Token::OpenBracket,
            Token::ConstantNumber("100".to_string()),
            Token::Comma,
            Token::ConstantNumber("200".to_string()),
            Token::CloseBracket,
            Token::EndLine,
        ];
        assert_eq!(actual, expected);
    }
}
