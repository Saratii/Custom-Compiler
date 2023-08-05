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
    // OpenParen,
}
#[derive(PartialEq, Debug, Clone)]
pub enum MathOp {
    Add,
    Multiply,
    Divide,
    Subtract,
    Equals,
    LessThan,
    GreaterThan,
    LessThanOrEqualTo,
    GreaterThanOrEqualTo,
}

pub fn parse_to_tokens(raw: &str) -> Vec<Token> {
    let remove_tabs = Regex::new(r"\n\s+").unwrap();
    let mut inputs = remove_tabs.replace_all(raw, "\n").to_string();
    inputs = inputs.replace("\r", "").replace("\n", "");
    let number_regex = Regex::new(r"^(\d+)").unwrap();
    let name_regex = Regex::new(r"^([a-zA-Z_][a-zA-Z0-9_]*)").unwrap();

    let mut tokens = vec![];
    while inputs.len() > 0 {
        if inputs.starts_with("print") {
            tokens.push(Token::Print);
            inputs = inputs[5..].to_string();
        } else if inputs.starts_with("\"") {
            inputs = inputs[1..].to_string();
            let mut string = "".to_string();
            while !inputs.starts_with("\"") {
                string = string + &inputs.chars().nth(0).unwrap().to_string();
                inputs = inputs[1..].to_string();
            }
            tokens.push(Token::String(string));
            inputs = inputs[1..].to_string();
        } else if inputs.starts_with(")") {
            tokens.push(Token::CloseParen);
            inputs = inputs[1..].to_string();
        } else if inputs.starts_with("}") {
            tokens.push(Token::EndBlock);
            inputs = inputs[1..].to_string();
        } else if inputs.starts_with("String ") {
            inputs = inputs[7..].to_string();
            tokens.push(Token::TypeString);
        } else if inputs.starts_with("i32 ") {
            inputs = inputs[4..].to_string();
            tokens.push(Token::TypeI32);
        } else if number_regex.is_match(&inputs) {
            let constant_number = number_regex
                .captures(&inputs)
                .unwrap()
                .get(0)
                .unwrap()
                .as_str();
            tokens.push(Token::ConstantNumber(constant_number.to_string()));
            inputs = inputs[constant_number.len()..].to_string();
        } else if inputs.starts_with(" = ") {
            inputs = inputs[3..].to_string();
        } else if inputs.starts_with("++") {
            inputs = inputs[2..].to_string();
            tokens.push(Token::MathOp(MathOp::Add));
            tokens.push(Token::ConstantNumber("1".to_string()));
        } else if inputs.starts_with("--") {
            inputs = inputs[2..].to_string();
            tokens.push(Token::MathOp(MathOp::Subtract));
            tokens.push(Token::ConstantNumber("1".to_string()));
        } else if inputs.starts_with("Bool ") {
            tokens.push(Token::TypeBool);
            inputs = inputs[5..].to_string();
        } else if inputs.starts_with("True") {
            tokens.push(Token::Boolean(true));
            inputs = inputs[4..].to_string();
        } else if inputs.starts_with("true") {
            panic!("you typed: true, did you mean True?");
        } else if inputs.starts_with("False") {
            tokens.push(Token::Boolean(false));
            inputs = inputs[5..].to_string();
        } else if inputs.starts_with("false") {
            panic!("you typed: false, did you mean False?");
        } else if inputs.starts_with("while ") {
            tokens.push(Token::WhileLoop);
            inputs = inputs[6..].to_string();
        } else if inputs.starts_with("while") {
            tokens.push(Token::WhileLoop);
            inputs = inputs[5..].to_string();
        } else if inputs.starts_with("{") {
            tokens.push(Token::StartBlock);
            inputs = inputs[1..].to_string();
        } else if inputs.starts_with(" + ") {
            tokens.push(Token::MathOp(MathOp::Add));
            inputs = inputs[3..].to_string();
        } else if inputs.starts_with(" - ") {
            tokens.push(Token::MathOp(MathOp::Subtract));
            inputs = inputs[3..].to_string();
        } else if inputs.starts_with(" * ") {
            tokens.push(Token::MathOp(MathOp::Multiply));
            inputs = inputs[3..].to_string();
        } else if inputs.starts_with(" / ") {
            tokens.push(Token::MathOp(MathOp::Divide));
            inputs = inputs[3..].to_string();
        } else if inputs.starts_with(";") {
            tokens.push(Token::EndLine);
            inputs = inputs[1..].to_string();
        } else if inputs.starts_with("if") {
            tokens.push(Token::If);
            inputs = inputs[2..].to_string();
        } else if inputs.starts_with(" == ") {
            tokens.push(Token::MathOp(MathOp::Equals));
            inputs = inputs[4..].to_string()
        } else if inputs.starts_with(" < ") {
            tokens.push(Token::MathOp(MathOp::LessThan));
            inputs = inputs[3..].to_string()
        } else if inputs.starts_with(" <= ") {
            tokens.push(Token::MathOp(MathOp::LessThanOrEqualTo));
            inputs = inputs[4..].to_string()
        } else if inputs.starts_with(" > ") {
            tokens.push(Token::MathOp(MathOp::GreaterThan));
            inputs = inputs[3..].to_string()
        } else if inputs.starts_with(" >= ") {
            tokens.push(Token::MathOp(MathOp::GreaterThanOrEqualTo));
            inputs = inputs[4..].to_string()
        } else if inputs.starts_with(", ") {
            tokens.push(Token::Comma);
            inputs = inputs[2..].to_string()
        } else if inputs.starts_with("for") {
            tokens.push(Token::ForLoop);
            inputs = inputs[3..].to_string()
        } else if inputs.starts_with("(") {
            tokens.push(Token::OpenParen);
            inputs = inputs[1..].to_string()
        } else if name_regex.is_match(&inputs) {
            let variable_name = name_regex
                .captures(&inputs)
                .unwrap()
                .get(0)
                .unwrap()
                .as_str();
            tokens.push(Token::VariableName(variable_name.to_string()));
            inputs = inputs[variable_name.len()..].to_string();
        } else {
            if inputs.len() != 0 {
                println!("Oopsie Woopsie: Code contains something that doesnt parse or hidden characters: {}", &inputs[0..])
            }
            break;
        }
    }
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
            Token::MathOp(MathOp::Add),
            Token::ConstantNumber("1".to_string()),
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
}
