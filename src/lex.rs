use crate::tokenize::Token;
#[derive(PartialEq, Debug)]
pub enum Line {
    Print(Expression),
    DefineVariable(String, Expression, Type),
}
#[derive(PartialEq, Debug)]
pub enum Expression {
    String(String),
    Bool(bool),
    Variable(String),
    I32(String),
}
#[derive(PartialEq, Debug)]
pub enum Type{
    Bool,
    String,
    I32,
}
pub fn lex(tokens: Vec<Token>) -> Vec<Line> {
    let mut lines = vec![];
    for i in 0..tokens.len(){
        match &tokens[i]{
            Token::Print => {
                match &tokens[i+1]{
                    Token::String(expression) => lines.push(Line::Print(Expression::String(expression.to_string()))),
                    Token::VariableName(name) => lines.push(Line::Print(Expression::Variable(name.to_string()))),
                    _ => {}
                    
                }   
            }
            Token::TypeBool => {
                match &tokens[i+1]{
                    Token::VariableName(name) => {
                        match &tokens[i+2]{
                            Token::Boolean(expression) => lines.push(Line::DefineVariable(name.to_string(), Expression::Bool(expression.clone()), Type::Bool)),
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            Token::TypeString => {
                match &tokens[i+1]{
                    Token::VariableName(name) => {
                        match &tokens[i+2]{
                            Token::String(expression) => lines.push(Line::DefineVariable(name.to_string(), Expression::String(expression.to_string()), Type::String)),
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            Token::TypeI32 => {
                match &tokens[i+1]{
                    Token::VariableName(name) => {
                        match &tokens[i+2]{
                            Token::ConstantNumber(expression) => lines.push(Line::DefineVariable(name.to_string(), Expression::I32(expression.to_string()), Type::I32)),
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
    lines
}
#[cfg(test)]
mod test {
    use crate::{lex::Expression, tokenize::Token};

    use super::{lex, Line, Type};
    #[test]
    fn test_1() {
        let actual = lex(vec![
            Token::Print,
            Token::String("hello world".to_string()),
            Token::EndParen,
            Token::EndParen,
        ]);
        let expected = vec![Line::Print(Expression::String("hello world".to_string()))];
        assert_eq!(actual, expected);
    }
    #[test]
    fn bool_test(){
        let actual = lex(vec![Token::TypeBool, Token::VariableName("peepaw".to_string()), Token::Boolean(true)]);
        let expected = vec![Line::DefineVariable("peepaw".to_string(), Expression::Bool(true), Type::Bool)];
        assert_eq!(actual, expected);
    }
    #[test]
    fn print_variable_test(){
        let actual = lex(vec![
            Token::TypeBool,
            Token::VariableName("eee".to_string()),
            Token::Boolean(true),
            Token::Print,
            Token::VariableName("eee".to_string()),
            Token::EndParen,
        ]);
        let expected = vec![Line::DefineVariable("eee".to_string(), Expression::Bool(true), Type::Bool), Line::Print(Expression::Variable("eee".to_string()))];
        assert_eq!(actual, expected);
    }
    #[test]
    fn print_string_test(){
        let actual = lex(vec![
            Token::TypeString,
            Token::VariableName("ee".to_string()),
            Token::String("should I kill myself?".to_string()),
            Token::Print,
            Token::VariableName("ee".to_string()),
            Token::EndParen,
        ]);
        let expected = vec![
            Line::DefineVariable("ee".to_string(), Expression::String("should I kill myself?".to_string()), Type::String),
            Line::Print(Expression::Variable("ee".to_string())),
            ];
        assert_eq!(actual, expected);
    }
}
