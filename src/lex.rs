use crate::tokenize::Token;
#[derive(PartialEq, Debug)]
pub enum Line {
    Print(Expression),
    DefineVariable(String, Expression, Type),
    WhileLoop(Expression, Vec<Line>),
    EndLoop,
}
#[derive(PartialEq, Debug, Clone)]
pub enum Expression {
    String(String),
    Bool(bool),
    Variable(String),
    I32(String),
    Add(Box<Expression>, Box<Expression>),
    Subtract(Box<Expression>, Box<Expression>),
    Multiply(Box<Expression>, Box<Expression>),
    Divide(Box<Expression>, Box<Expression>),
}
#[derive(PartialEq, Debug, Clone)]
pub enum Type {
    Bool,
    String,
    I32,
}
pub fn lex(tokens: Vec<Token>) -> Vec<Line> {
    let mut lines = vec![];
    let mut i = 0;
    while i < tokens.len() {
        i = process_token(i, &tokens, &mut lines);
        i += 1;
    }
    lines
}

fn process_token(index: usize, tokens: &Vec<Token>, lines: &mut Vec<Line>) -> usize {
    let mut i = index;
    match &tokens[i] {
        Token::Print => {
            i += 1;
            match &tokens[i] {
                Token::String(expression) => {
                    lines.push(Line::Print(Expression::String(expression.to_string())))
                }
                Token::VariableName(name) => {
                    lines.push(Line::Print(Expression::Variable(name.to_string())))
                }
                Token::EndParen => lines.push(Line::Print(Expression::String("".to_string()))),
                Token::ConstantNumber(value) => {
                    lines.push(Line::Print(Expression::String(value.to_string())))
                }
                Token::Boolean(value) => {
                    lines.push(Line::Print(Expression::String(value.to_string())))
                }
                _ => println!(
                    "{}",
                    &format!(
                        "Oopsie Woopsie: invalid token following print: {:?}",
                        tokens[i]
                    )
                ),
            }
        }
        Token::TypeBool => {
            i += 1;
            match &tokens[i] {
                Token::VariableName(name) => {
                    i += 1;
                    match &tokens[i] {
                        Token::Boolean(expression) => lines.push(Line::DefineVariable(
                            name.to_string(),
                            Expression::Bool(expression.clone()),
                            Type::Bool,
                        )),
                        _ => println!(
                            "{}",
                            &format!(
                                "Oopsie Woopsie: invalid token following a variable name: {:?}",
                                tokens[i]
                            )
                        ),
                    }
                }
                _ => println!(
                    "{}",
                    &format!(
                        "Oopsie Woopsie: invalid token following Bool: {:?}",
                        tokens[i]
                    )
                ),
            }
        }
        Token::TypeString => {
            i += 1;
            match &tokens[i] {
                Token::VariableName(name) => {
                    i += 1;
                    match &tokens[i] {
                        Token::String(expression) => lines.push(Line::DefineVariable(
                            name.to_string(),
                            Expression::String(expression.to_string()),
                            Type::String,
                        )),
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        Token::TypeI32 => {
            i += 1;
            match &tokens[i] {
                Token::VariableName(name) => {
                    i += 1;
                    let end_pos =
                        tokens
                            .iter()
                            .enumerate()
                            .skip(i)
                            .find_map(|(i, token)| match token {
                                Token::EndLine => Some(i),
                                _ => None,
                            });
                    let (literal, literal_type) = lex_expression(&tokens[i..end_pos.unwrap()]);
                    println!("i:{} end point: {:?}", i, end_pos);
                    println!(
                        "added line with name: {}, and value: {:?}",
                        name.to_string(),
                        literal
                    );
                    lines.push(Line::DefineVariable(name.to_string(), literal, literal_type));

                    i = end_pos.unwrap();
                }
                _ => {}
            }
        }
        Token::WhileLoop => {
            i += 1;
            match &tokens[i] {
                Token::Boolean(condition) => {
                    let mut while_loop_lines = vec![];
                    let mut token_index = i + 1;
                    while token_index < tokens.len() {
                        if tokens[token_index] == Token::EndLoop {
                            break;
                        }
                        process_token(token_index, tokens, &mut while_loop_lines);
                        token_index += 1;
                    }
                    lines.push(Line::WhileLoop(
                        Expression::Bool(*condition),
                        while_loop_lines,
                    ));
                }

                _ => todo!(),
            }
        }
        Token::VariableName(name) => {
            i += 1;
            let end_pos =
                tokens
                    .iter()
                    .enumerate()
                    .skip(i)
                    .find_map(|(i, token)| match token {
                        Token::EndLine => Some(i),
                        _ => None,
                    });
            let (literal , literal_type) = lex_expression(&tokens[i..end_pos.unwrap()]);
            println!("i:{} end point: {:?}", i, end_pos);
            println!(
                "added line with name: {}, and value: {:?}",
                name.to_string(),
                literal
            );
            lines.push(Line::DefineVariable(name.to_string(), literal, literal_type));
            i = end_pos.unwrap();
        }
        Token::EndLoop => lines.push(Line::EndLoop),
        _ => {}
    }
    i
}

fn lex_expression(tokens: &[Token]) -> (Expression, Type) {
    if tokens.len() == 1 {
        match &tokens[0] {
            Token::ConstantNumber(value) => return (Expression::I32(value.to_string()), Type::I32),
            Token::String(value) => return (Expression::String(value.to_string()), Type::String),
            Token::Boolean(value) => return (Expression::Bool(value.clone()), Type::Bool),
            _ => {}
        }
    }
    (Expression::Bool(false), Type::Bool)
}
#[cfg(test)]
mod test {
    use crate::{
        lex::Expression,
        tokenize::{MathOp, Token},
    };
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
    fn bool_test() {
        let actual = lex(vec![
            Token::TypeBool,
            Token::VariableName("peepaw".to_string()),
            Token::Boolean(true),
        ]);
        let expected = vec![Line::DefineVariable(
            "peepaw".to_string(),
            Expression::Bool(true),
            Type::Bool,
        )];
        assert_eq!(actual, expected);
    }
    #[test]
    fn print_variable_test() {
        let actual = lex(vec![
            Token::TypeBool,
            Token::VariableName("eee".to_string()),
            Token::Boolean(true),
            Token::Print,
            Token::VariableName("eee".to_string()),
            Token::EndParen,
        ]);
        let expected = vec![
            Line::DefineVariable("eee".to_string(), Expression::Bool(true), Type::Bool),
            Line::Print(Expression::Variable("eee".to_string())),
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn print_string_test() {
        let actual = lex(vec![
            Token::TypeString,
            Token::VariableName("ee".to_string()),
            Token::String("should I kill myself?".to_string()),
            Token::Print,
            Token::VariableName("ee".to_string()),
            Token::EndParen,
        ]);
        let expected = vec![
            Line::DefineVariable(
                "ee".to_string(),
                Expression::String("should I kill myself?".to_string()),
                Type::String,
            ),
            Line::Print(Expression::Variable("ee".to_string())),
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn simple_while_loop() {
        let actual = lex(vec![
            Token::WhileLoop,
            Token::Boolean(true),
            Token::EndParen,
            Token::StartLoop,
            Token::Print,
            Token::ConstantNumber("69".to_string()),
            Token::EndParen,
            Token::EndLoop,
        ]);
        let expected = vec![
            Line::WhileLoop(
                Expression::Bool(true),
                vec![Line::Print(Expression::String("69".to_string()))],
            ),
            Line::Print(Expression::String("69".to_string())),
            Line::EndLoop,
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn change_variable() {
        let actual = lex(vec![
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
        ]);
        let expected = vec![
            Line::DefineVariable("i".to_string(), Expression::I32("0".to_string()), Type::I32),
            Line::DefineVariable("i".to_string(), Expression::I32("1".to_string()), Type::I32),
            Line::DefineVariable(
                "e".to_string(),
                Expression::String("hello".to_string()),
                Type::String,
            ),
            Line::DefineVariable(
                "e".to_string(),
                Expression::String("bye".to_string()),
                Type::String,
            ),
            Line::DefineVariable("yes".to_string(), Expression::Bool(true), Type::Bool),
            Line::DefineVariable("yes".to_string(), Expression::Bool(false), Type::Bool),
        ];
        assert_eq!(actual, expected);
    }

    fn simple_math() {
        let actual = lex(vec![
            Token::TypeI32,
            Token::VariableName("e".to_string()),
            Token::ConstantNumber("4".to_string()),
            Token::MathOp(MathOp::Add),
            Token::ConstantNumber("3".to_string()),
            Token::TypeI32,
            Token::VariableName("ee".to_string()),
            Token::ConstantNumber("4".to_string()),
            Token::MathOp(MathOp::Subtract),
            Token::ConstantNumber("3".to_string()),
            Token::TypeI32,
            Token::VariableName("eee".to_string()),
            Token::ConstantNumber("8".to_string()),
            Token::MathOp(MathOp::Divide),
            Token::ConstantNumber("2".to_string()),
            Token::TypeI32,
            Token::VariableName("eeee".to_string()),
            Token::ConstantNumber("8".to_string()),
            Token::MathOp(MathOp::Multiply),
            Token::ConstantNumber("2".to_string()),
            Token::EndLine,
        ]);
        let expected = vec![
            Line::DefineVariable(
                "e".to_string(),
                Expression::Add(
                    Box::new(Expression::I32("4".to_string())),
                    Box::new(Expression::I32("3".to_string())),
                ),
                Type::I32,
            ),
            Line::DefineVariable(
                "e".to_string(),
                Expression::Subtract(
                    Box::new(Expression::I32("4".to_string())),
                    Box::new(Expression::I32("3".to_string())),
                ),
                Type::I32,
            ),
            Line::DefineVariable(
                "e".to_string(),
                Expression::Divide(
                    Box::new(Expression::I32("8".to_string())),
                    Box::new(Expression::I32("2".to_string())),
                ),
                Type::I32,
            ),
            Line::DefineVariable(
                "e".to_string(),
                Expression::Multiply(
                    Box::new(Expression::I32("8".to_string())),
                    Box::new(Expression::I32("2".to_string())),
                ),
                Type::I32,
            ),
        ];
        assert_eq!(actual, expected);
    }
}
