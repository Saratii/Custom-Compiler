use regex::Regex;

#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    Print,
    String(String),
    EndParen,
    StartLoop,
    EndLoop,
    TypeI32,
    TypeString,
    TypeBool,
    VariableName(String),
    ConstantNumber(String),
    Boolean(bool),
    WhileLoop,
    MathOp(MathOp),
    EndLine,
    OpenParen,
}
#[derive(PartialEq, Debug, Clone)]
pub enum MathOp{
    Add,
    Multiply,
    Divide,
    Subtract,
}

pub fn parse_to_tokens(raw: &str) -> Vec<Token> {
    let remove_tabs = Regex::new(r"\n\s+").unwrap();
    let mut inputs = remove_tabs.replace_all(raw, "\n").to_string();
    inputs = inputs.replace("\r", "").replace("\n", "");
    let number_regex = Regex::new(r"^(\d+)").unwrap();
    let name_regex = Regex::new(r"^([a-zA-Z_][a-zA-Z0-9_]*)").unwrap();

    let mut tokens = vec![];
    while inputs.len() > 0 {
        if inputs.starts_with("print(") {
            tokens.push(Token::Print);
            inputs = inputs[6..].to_string();
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
            tokens.push(Token::EndParen);
            inputs = inputs[1..].to_string();
        } else if inputs.starts_with("}") {
            tokens.push(Token::EndLoop);
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
        } else if inputs.starts_with("Bool "){
            tokens.push(Token::TypeBool);
            inputs = inputs[5..].to_string();
        } else if inputs.starts_with("True"){
            tokens.push(Token::Boolean(true));
            inputs = inputs[4..].to_string();
        } else if inputs.starts_with("False"){
            tokens.push(Token::Boolean(false));
            inputs = inputs[5..].to_string();
        } else if inputs.starts_with("while ("){
            tokens.push(Token::WhileLoop);
            inputs = inputs[7..].to_string();
        } else if inputs.starts_with("{"){
            tokens.push(Token::StartLoop);
            inputs = inputs[1..].to_string();
        } else if inputs.starts_with(" + "){
            tokens.push(Token::MathOp(MathOp::Add));
            inputs = inputs[3..].to_string();
        } else if inputs.starts_with(" - "){
            tokens.push(Token::MathOp(MathOp::Subtract));
            inputs = inputs[3..].to_string();
        } else if inputs.starts_with(" * "){
            tokens.push(Token::MathOp(MathOp::Multiply));
            inputs = inputs[3..].to_string();
        } else if inputs.starts_with(" / "){
            tokens.push(Token::MathOp(MathOp::Divide));
            inputs = inputs[3..].to_string();
        } else if inputs.starts_with(";"){
            tokens.push(Token::EndLine);
            inputs = inputs[1..].to_string();
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
            Token::String("hello world".to_string()),
            Token::EndParen,
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
    fn print_variable_test(){
        let actual = parse_to_tokens("Bool eee = True;\nprint(eee);");
        let expected = vec![
            Token::TypeBool,
            Token::VariableName("eee".to_string()),
            Token::Boolean(true),
            Token::EndLine,
            Token::Print,
            Token::VariableName("eee".to_string()),
            Token::EndParen,
            Token::EndLine,
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn print_string_variable(){
        let actual = parse_to_tokens("String ee = \"should I kill myself?\";\nprint(ee);");
        let expected = vec![
            Token::TypeString,
            Token::VariableName("ee".to_string()),
            Token::String("should I kill myself?".to_string()),
            Token::EndLine,
            Token::Print,
            Token::VariableName("ee".to_string()),
            Token::EndParen,
            Token::EndLine,
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn while_true(){
        let actual = parse_to_tokens("while (True){\nprint(69);\n}");
        let expected = vec![
            Token::WhileLoop,
            Token::Boolean(true),
            Token::EndParen,
            Token::StartLoop,
            Token::Print,
            Token::ConstantNumber("69".to_string()),
            Token::EndParen,
            Token::EndLine,
            Token::EndLoop,
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn format_tab(){
        let actual = parse_to_tokens("
i32 e = 69;
while (True){
    print(e);
}"
        );
        let expected = vec![
            Token::TypeI32,
            Token::VariableName("e".to_string()),
            Token::ConstantNumber("69".to_string()),
            Token::EndLine,
            Token::WhileLoop,
            Token::Boolean(true),
            Token::EndParen,
            Token::StartLoop,
            Token::Print,
            Token::VariableName("e".to_string()),
            Token::EndParen,
            Token::EndLine,
            Token::EndLoop,
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn change_variable(){
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
    fn simple_math(){
        let actual = parse_to_tokens("i32 e = 4 + 3;\ni32 ee = 4 - 3;\ni32 eee = 8 / 2;\ni32 eeee = 8 * 2;");
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
    fn multi_term_simple_math(){
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
}
