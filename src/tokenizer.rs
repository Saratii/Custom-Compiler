use std::{collections::VecDeque, iter::Peekable};

use regex::Regex;

#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    Identifier(String),
    String(String),
    OpenParen,
    CloseParen,
    OpenBlock,
    CloseBlock,
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
    Ignore,
    OpenBracket,
    CloseBracket,
    DefineFunction,
    Assign,
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

pub fn tokenize(chars: &str) -> VecDeque<Token> {
    let remove_comments_regex = Regex::new(r"(?:\/\/(.*)|\/\*((?:.|[\r\n])*?)\*\/)").unwrap();
    let binding = remove_comments_regex.replace_all(chars, "");
    let mut chars_without_comments = binding.chars().peekable();
    let mut tokens = VecDeque::new();
    loop {
        let token = scan_token(&mut chars_without_comments);
        match token {
            Some(token) => tokens.push_back(token.clone()),
            None => break,
        }
    }
    tokens.retain(|token| *token != Token::Ignore);
    return tokens;
}
fn scan_token(chars: &mut Peekable<std::str::Chars>) -> Option<Token> {
    loop {
        match chars.peek() {
            Some(&ch) => match ch {
                '0'..='9' => return Some(scan_numeric(chars)),
                'a'..='z' | 'A'..='Z' => return Some(scan_identity(chars)),
                ' ' | '\t' | '\n'| '\r' => {
                    chars.next();
                }
                '[' => {
                    chars.next();
                    return Some(Token::OpenBracket);
                }
                ']' => {
                    chars.next();
                    return Some(Token::CloseBracket);
                }
                '<' => {
                    chars.next();
                    match chars.peek() {
                        Some(chh) => match chh {
                            '=' => {
                                chars.next();
                                return Some(Token::MathOp(MathOp::LessThanOrEqualTo));
                            }
                            _ => return Some(Token::MathOp(MathOp::LessThan)),
                        },
                        None => return None,
                    }
                }
                '>' => {
                    chars.next();
                    match chars.peek() {
                        Some(chh) => match chh {
                            '=' => {
                                chars.next();
                                return Some(Token::MathOp(MathOp::GreaterThanOrEqualTo));
                            }
                            _ => return Some(Token::MathOp(MathOp::GreaterThan)),
                        },
                        None => return None,
                    }
                }
                '!' => {
                    chars.next();
                    match chars.peek() {
                        Some(chh) => match chh {
                            '=' => {
                                chars.next();
                                return Some(Token::MathOp(MathOp::NotEqual));
                            }
                            _ => return Some(Token::MathOp(MathOp::Not)),
                        },
                        None => return None,
                    }
                }
                '&' => {
                    chars.next();
                    match chars.peek() {
                        Some(chh) => match chh {
                            '&' => {
                                chars.next();
                                return Some(Token::MathOp(MathOp::And));
                            },
                            _ => return None
                        },
                        None => return None,
                    }
                }
                '|' => {
                    chars.next();
                    match chars.peek() {
                        Some(chh) => match chh {
                            '|' => {
                                chars.next();
                                return Some(Token::MathOp(MathOp::Or));
                            },
                            _ => return None
                        },
                        None => return None,
                    }
                }
                ',' => {
                    chars.next();
                    return Some(Token::Comma);
                }
                '"' => {
                    chars.next();
                    return Some(scan_string(chars));
                }
                '+' => {
                    chars.next();
                    match chars.peek() {
                        Some(chh) => match chh {
                            '+' => {
                                chars.next();
                                return Some(Token::Increment);
                            }
                            _ => return Some(Token::MathOp(MathOp::Add)),
                        },
                        None => return None,
                    }
                }
                '/' => {
                    chars.next();
                    return Some(Token::MathOp(MathOp::Divide));
                }
                '*' => {
                    chars.next();
                    return Some(Token::MathOp(MathOp::Multiply));
                }
                '-' => {
                    chars.next();
                    match chars.peek() {
                        Some(chh) => match chh {
                            '-' => {
                                chars.next();
                                return Some(Token::Decrement);
                            }
                            _ => return Some(Token::MathOp(MathOp::Subtract)),
                        },
                        None => return None,
                    }
                }
                '%' => {
                    chars.next();
                    return Some(Token::MathOp(MathOp::Modulus));
                }
                '(' => {
                    chars.next();
                    return Some(Token::OpenParen);
                }
                ')' => {
                    chars.next();
                    return Some(Token::CloseParen);
                }
                ';' => {
                    chars.next();
                    return Some(Token::EndLine);
                }
                '{' => {
                    chars.next();
                    return Some(Token::OpenBlock);
                }
                '}' => {
                    chars.next();
                    return Some(Token::CloseBlock);
                }
                '=' => {
                    chars.next();
                    match chars.peek() {
                        Some(chh) => match chh {
                            '=' => {
                                chars.next();
                                return Some(Token::MathOp(MathOp::Equals));
                            }
                            _ => return Some(Token::Assign),
                        },
                        None => return None,
                    }
                }
                _ => {
                    panic!("unexpected character: {}, error\n", ch)
                }
            },
            None => return None,
        }
    }
}
fn scan_identity(chars: &mut Peekable<std::str::Chars>) -> Token {
    let mut identifier = String::new();
    while let Some(&ch) = chars.peek() {
        match ch {
            'a'..='z' | 'A'..='Z' | '_' | '0'..='9' | '<' | '>' => {
                identifier.push(ch);
                chars.next();
            }
            _ => break,
        }
    }
    match scan_keywords(&identifier) {
        Some(token) => return token,
        None => return Token::Identifier(identifier),
    }
}
fn scan_keywords(ident: &String) -> Option<Token> {
    match ident.as_str() {
        "while" => return Some(Token::WhileLoop),
        "if" => return Some(Token::If),
        "elif" => return Some(Token::Elif),
        "else" => return Some(Token::Else),
        "false" => return Some(Token::Boolean(false)),
        "true" => return Some(Token::Boolean(true)),
        "fn" => return Some(Token::DefineFunction),
        "for" => return Some(Token::ForLoop),
        _ => return None,
    }
}
fn scan_numeric(chars: &mut Peekable<std::str::Chars>) -> Token {
    let mut number = String::new();
    while let Some(&ch) = chars.peek() {
        match ch {
            '_' | '0'..='9' => {
                number.push(ch);
                chars.next();
            }
            _ => break,
        }
    }
    return Token::ConstantNumber(number);
}
fn scan_string(chars: &mut Peekable<std::str::Chars>) -> Token {
    let mut string = String::new();
    while chars.peek() != Some(&'"') {
        string.push(chars.peek().unwrap().clone());
        chars.next();
    }
    chars.next();
    return Token::String(string);
}


#[cfg(test)]
mod test {

    use crate::tokenizer::{tokenize, MathOp};

    use super::Token;

    #[test]
    fn hello_world() {
        let actual = tokenize("print(\"hello world\");");
        let expected = vec![
            Token::Identifier("print".to_string()),
            Token::OpenParen,
            Token::String("hello world".to_string()),
            Token::CloseParen,
            Token::EndLine,
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn integer_variable_test() {
        let actual = tokenize("i32 eeebo = 6;");
        let expected = vec![
            Token::Identifier("i32".to_string()),
            Token::Identifier("eeebo".to_string()),
            Token::Assign,
            Token::ConstantNumber("6".to_string()),
            Token::EndLine,
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn string_variable_test() {
        let actual = tokenize("String beebo = \"carbon monoxide\";");
        let expected = vec![
            Token::Identifier("String".to_string()),
            Token::Identifier("beebo".to_string()),
            Token::Assign,
            Token::String("carbon monoxide".to_string()),
            Token::EndLine,
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn bool_variable_test() {
        let actual = tokenize("Bool feefoo = false;");
        let expected = vec![
            Token::Identifier("Bool".to_string()),
            Token::Identifier("feefoo".to_string()),
            Token::Assign,
            Token::Boolean(false),
            Token::EndLine,
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn i32_variable_test() {
        let actual = tokenize("i32 furfu = 42;");
        let expected = vec![
            Token::Identifier("i32".to_string()),
            Token::Identifier("furfu".to_string()),
            Token::Assign,
            Token::ConstantNumber("42".to_string()),
            Token::EndLine,
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn print_variable_test() {
        let actual = tokenize("Bool eee = true;\nprint(eee);");
        let expected = vec![
            Token::Identifier("Bool".to_string()),
            Token::Identifier("eee".to_string()),
            Token::Assign,
            Token::Boolean(true),
            Token::EndLine,
            Token::Identifier("print".to_string()),
            Token::OpenParen,
            Token::Identifier("eee".to_string()),
            Token::CloseParen,
            Token::EndLine,
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn print_string_variable() {
        let actual = tokenize("String ee = \"should?\";\nprint(ee);");
        let expected = vec![
            Token::Identifier("String".to_string()),
            Token::Identifier("ee".to_string()),
            Token::Assign,
            Token::String("should?".to_string()),
            Token::EndLine,
            Token::Identifier("print".to_string()),
            Token::OpenParen,
            Token::Identifier("ee".to_string()),
            Token::CloseParen,
            Token::EndLine,
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn while_true() {
        let actual = tokenize("while (true){\nprint(6);\n}");
        let expected = vec![
            Token::WhileLoop,
            Token::OpenParen,
            Token::Boolean(true),
            Token::CloseParen,
            Token::OpenBlock,
            Token::Identifier("print".to_string()),
            Token::OpenParen,
            Token::ConstantNumber("6".to_string()),
            Token::CloseParen,
            Token::EndLine,
            Token::CloseBlock,
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn format_tab() {
        let actual = tokenize(
            "
i32 e = 6;
while (true){
    print(e);
}",
        );
        let expected = vec![
            Token::Identifier("i32".to_string()),
            Token::Identifier("e".to_string()),
            Token::Assign,
            Token::ConstantNumber("6".to_string()),
            Token::EndLine,
            Token::WhileLoop,
            Token::OpenParen,
            Token::Boolean(true),
            Token::CloseParen,
            Token::OpenBlock,
            Token::Identifier("print".to_string()),
            Token::OpenParen,
            Token::Identifier("e".to_string()),
            Token::CloseParen,
            Token::EndLine,
            Token::CloseBlock,
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn change_variable() {
        let actual = tokenize("i32 i = 0;\ni = 1;\nString e = \"hello\";\ne = \"bye\";\nBool yes = true;\nyes = false;");
        let expected = vec![
            Token::Identifier("i32".to_string()),
            Token::Identifier("i".to_string()),
            Token::Assign,
            Token::ConstantNumber("0".to_string()),
            Token::EndLine,
            Token::Identifier("i".to_string()),
            Token::Assign,
            Token::ConstantNumber("1".to_string()),
            Token::EndLine,
            Token::Identifier("String".to_string()),
            Token::Identifier("e".to_string()),
            Token::Assign,
            Token::String("hello".to_string()),
            Token::EndLine,
            Token::Identifier("e".to_string()),
            Token::Assign,
            Token::String("bye".to_string()),
            Token::EndLine,
            Token::Identifier("Bool".to_string()),
            Token::Identifier("yes".to_string()),
            Token::Assign,
            Token::Boolean(true),
            Token::EndLine,
            Token::Identifier("yes".to_string()),
            Token::Assign,
            Token::Boolean(false),
            Token::EndLine,
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn simple_math() {
        let actual = tokenize("i32 e = 4 + 3;\ni32 ee = 4 - 3;\ni32 eee = 8 / 2;\ni32 eeee = 8 * 2;");
        let expected = vec![
            Token::Identifier("i32".to_string()),
            Token::Identifier("e".to_string()),
            Token::Assign,
            Token::ConstantNumber("4".to_string()),
            Token::MathOp(MathOp::Add),
            Token::ConstantNumber("3".to_string()),
            Token::EndLine,
            Token::Identifier("i32".to_string()),
            Token::Identifier("ee".to_string()),
            Token::Assign,
            Token::ConstantNumber("4".to_string()),
            Token::MathOp(MathOp::Subtract),
            Token::ConstantNumber("3".to_string()),
            Token::EndLine,
            Token::Identifier("i32".to_string()),
            Token::Identifier("eee".to_string()),
            Token::Assign,
            Token::ConstantNumber("8".to_string()),
            Token::MathOp(MathOp::Divide),
            Token::ConstantNumber("2".to_string()),
            Token::EndLine,
            Token::Identifier("i32".to_string()),
            Token::Identifier("eeee".to_string()),
            Token::Assign,
            Token::ConstantNumber("8".to_string()),
            Token::MathOp(MathOp::Multiply),
            Token::ConstantNumber("2".to_string()),
            Token::EndLine,
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn multi_term_simple_math() {
        let actual = tokenize("i32 foo = 3 + 5 / 4 * 68;");
        let expected = vec![
            Token::Identifier("i32".to_string()),
            Token::Identifier("foo".to_string()),
            Token::Assign,
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
        let actual = tokenize("print(1 + 6);");
        let expected = vec![
            Token::Identifier("print".to_string()),
            Token::OpenParen,
            Token::ConstantNumber("1".to_string()),
            Token::MathOp(MathOp::Add),
            Token::ConstantNumber("6".to_string()),
            Token::CloseParen,
            Token::EndLine,
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn variable_adding() {
        let actual = tokenize("i32 e = 1;i32 ee = 2;print(e + ee);");
        let expected = vec![
            Token::Identifier("i32".to_string()),
            Token::Identifier("e".to_string()),
            Token::Assign,
            Token::ConstantNumber("1".to_string()),
            Token::EndLine,
            Token::Identifier("i32".to_string()),
            Token::Identifier("ee".to_string()),
            Token::Assign,
            Token::ConstantNumber("2".to_string()),
            Token::EndLine,
            Token::Identifier("print".to_string()),
            Token::OpenParen,
            Token::Identifier("e".to_string()),
            Token::MathOp(MathOp::Add),
            Token::Identifier("ee".to_string()),
            Token::CloseParen,
            Token::EndLine,
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn basic_if() {
        let actual = tokenize("i32 e = 6;if(e == 6){print(e);}");
        let expected: Vec<Token> = vec![
            Token::Identifier("i32".to_string()),
            Token::Identifier("e".to_string()),
            Token::Assign,
            Token::ConstantNumber("6".to_string()),
            Token::EndLine,
            Token::If,
            Token::OpenParen,
            Token::Identifier("e".to_string()),
            Token::MathOp(MathOp::Equals),
            Token::ConstantNumber("6".to_string()),
            Token::CloseParen,
            Token::OpenBlock,
            Token::Identifier("print".to_string()),
            Token::OpenParen,
            Token::Identifier("e".to_string()),
            Token::CloseParen,
            Token::EndLine,
            Token::CloseBlock,
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn for_loop() {
        let actual = tokenize("for(i32 i = 0, i < 10, i++){\nprint(i);\n}");
        let expected = vec![
            Token::ForLoop,
            Token::OpenParen,
            Token::Identifier("i32".to_string()),
            Token::Identifier("i".to_string()),
            Token::Assign,
            Token::ConstantNumber("0".to_string()),
            Token::Comma,
            Token::Identifier("i".to_string()),
            Token::MathOp(MathOp::LessThan),
            Token::ConstantNumber("10".to_string()),
            Token::Comma,
            Token::Identifier("i".to_string()),
            Token::Increment,
            Token::CloseParen,
            Token::OpenBlock,
            Token::Identifier("print".to_string()),
            Token::OpenParen,
            Token::Identifier("i".to_string()),
            Token::CloseParen,
            Token::EndLine,
            Token::CloseBlock,
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn double_if() {
        let actual = tokenize("if(true){if(false){print(\"a\");}}");
        let expected = vec![
            Token::If,
            Token::OpenParen,
            Token::Boolean(true),
            Token::CloseParen,
            Token::OpenBlock,
            Token::If,
            Token::OpenParen,
            Token::Boolean(false),
            Token::CloseParen,
            Token::OpenBlock,
            Token::Identifier("print".to_string()),
            Token::OpenParen,
            Token::String("a".to_string()),
            Token::CloseParen,
            Token::EndLine,
            Token::CloseBlock,
            Token::CloseBlock,
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn basic_comment() {
        let actual = tokenize("i32 i = 10;\n//i32 e = 9;\ni32 g = 8;");
        let expected = vec![
            Token::Identifier("i32".to_string()),
            Token::Identifier("i".to_string()),
            Token::Assign,
            Token::ConstantNumber("10".to_string()),
            Token::EndLine,
            Token::Identifier("i32".to_string()),
            Token::Identifier("g".to_string()),
            Token::Assign,
            Token::ConstantNumber("8".to_string()),
            Token::EndLine,
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn multi_line_comment() {
        let actual = tokenize("i32 i = 10;\n/*unga\nbunga\nwunga\n*/i32 e = 0;");
        let expected = vec![
            Token::Identifier("i32".to_string()),
            Token::Identifier("i".to_string()),
            Token::Assign,
            Token::ConstantNumber("10".to_string()),
            Token::EndLine,
            Token::Identifier("i32".to_string()),
            Token::Identifier("e".to_string()),
            Token::Assign,
            Token::ConstantNumber("0".to_string()),
            Token::EndLine,
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn else_elif_test() {
        let actual = tokenize("if(i == 6){}elif(i == 7){}else{print(\"e\");}");
        let expected = vec![
            Token::If,
            Token::OpenParen,
            Token::Identifier("i".to_string()),
            Token::MathOp(MathOp::Equals),
            Token::ConstantNumber("6".to_string()),
            Token::CloseParen,
            Token::OpenBlock,
            Token::CloseBlock,
            Token::Elif,
            Token::OpenParen,
            Token::Identifier("i".to_string()),
            Token::MathOp(MathOp::Equals),
            Token::ConstantNumber("7".to_string()),
            Token::CloseParen,
            Token::OpenBlock,
            Token::CloseBlock,
            Token::Else,
            Token::OpenBlock,
            Token::Identifier("print".to_string()),
            Token::OpenParen,
            Token::String("e".to_string()),
            Token::CloseParen,
            Token::EndLine,
            Token::CloseBlock,
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn i32_i64_f32_f64() {
        let actual = tokenize("i32 i = 31;i64 e = 63;f32 f = 32; f64 g = 64;");
        let expected = vec![
            Token::Identifier("i32".to_string()),
            Token::Identifier("i".to_string()),
            Token::Assign,
            Token::ConstantNumber("31".to_string()),
            Token::EndLine,
            Token::Identifier("i64".to_string()),
            Token::Identifier("e".to_string()),
            Token::Assign,
            Token::ConstantNumber("63".to_string()),
            Token::EndLine,
            Token::Identifier("f32".to_string()),
            Token::Identifier("f".to_string()),
            Token::Assign,
            Token::ConstantNumber("32".to_string()),
            Token::EndLine,
            Token::Identifier("f64".to_string()),
            Token::Identifier("g".to_string()),
            Token::Assign,
            Token::ConstantNumber("64".to_string()),
            Token::EndLine,
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn one_dim_array() {
        let actual = tokenize("Array<i32> a = [];Array<i64> b=[100, 200];");
        let expected = vec![
            Token::Identifier("Array<i32>".to_string()),
            Token::Identifier("a".to_string()),
            Token::Assign,
            Token::OpenBracket,
            Token::CloseBracket,
            Token::EndLine,
            Token::Identifier("Array<i64>".to_string()),
            Token::Identifier("b".to_string()),
            Token::Assign,
            Token::OpenBracket,
            Token::ConstantNumber("100".to_string()),
            Token::Comma,
            Token::ConstantNumber("200".to_string()),
            Token::CloseBracket,
            Token::EndLine,
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn define_function() {
        let actual = tokenize("fn pwint(){print(\"i\");}pwint();");
        let expected = vec![
            Token::DefineFunction,
            Token::Identifier("pwint".to_string()),
            Token::OpenParen,
            Token::CloseParen,
            Token::OpenBlock,
            Token::Identifier("print".to_string()),
            Token::OpenParen,
            Token::String("i".to_string()),
            Token::CloseParen,
            Token::EndLine,
            Token::CloseBlock,
            Token::Identifier("pwint".to_string()),
            Token::OpenParen,
            Token::CloseParen,
            Token::EndLine,
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn complex_logic() {
        let actual = tokenize("print((false || true) && true);");
        let expected = vec![
            Token::Identifier("print".to_string()),
            Token::OpenParen,
            Token::OpenParen,
            Token::Boolean(false),
            Token::MathOp(MathOp::Or),
            Token::Boolean(true),
            Token::CloseParen,
            Token::MathOp(MathOp::And),
            Token::Boolean(true),
            Token::CloseParen,
            Token::EndLine,
        ];
        assert_eq!(actual, expected);
    }
}