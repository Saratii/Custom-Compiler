use crate::tokenize::{MathOp, Token};
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
    Complete(Complete),
    BinaryOperator(BinaryOperator),
    // CompleteU(CompleteU),
    // IncompleteU(IncompleteU),
}
#[derive(PartialEq, Debug, Clone)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
}
#[derive(PartialEq, Debug, Clone)]
pub struct Complete {
    pub operator: BinaryOperator,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}
// #[derive(PartialEq, Debug, Clone)]
// pub enum CompleteU {
//     Parenthesis(Box<Expression>),
// }

// #[derive(PartialEq, Debug, Clone)]
// pub enum IncompleteU {
//     Parenthesis,
// }
impl Expression {
    fn _get_precidence(&self) -> u8 {
        match self {
            Expression::Complete(opp) => opp.operator.get_precidence(),
            Expression::BinaryOperator(opp) => opp.get_precidence(),
            // Expression::CompleteU(opp) => match opp {
            //     CompleteU::Parenthesis(_) => 3,
            // },
            // Expression::IncompleteU(opp) => match opp {
            //     IncompleteU::Parenthesis => 3,
            // },
            _ => 255,
        }
    }
}
impl BinaryOperator {
    fn get_precidence(&self) -> u8 {
        match self {
            BinaryOperator::Add => 1,
            BinaryOperator::Subtract => 1,
            BinaryOperator::Divide => 2,
            BinaryOperator::Multiply => 2,
        }
    }
}
impl From<&MathOp> for Expression {
    fn from(op: &MathOp) -> Expression {
        match op {
            MathOp::Multiply => Expression::BinaryOperator(BinaryOperator::Multiply),
            MathOp::Divide => Expression::BinaryOperator(BinaryOperator::Divide),
            MathOp::Subtract => Expression::BinaryOperator(BinaryOperator::Subtract),
            MathOp::Add => Expression::BinaryOperator(BinaryOperator::Add),
        }
    }
}
impl From<(&BinaryOperator, &Expression, &Expression)> for Complete {
    fn from(
        (binary_operator, left, right): (&BinaryOperator, &Expression, &Expression),
    ) -> Complete {
        Complete {
            operator: binary_operator.clone(),
            left: Box::new(left.clone()),
            right: Box::new(right.clone()),
        }
    }
}
// impl From<(&IncompleteU, &Expression)> for CompleteU {
//     fn from((incomplete_u, exp): (&IncompleteU, &Expression)) -> CompleteU {
//         match incomplete_u {
//             IncompleteU::Parenthesis => CompleteU::Parenthesis(Box::new(exp.clone())),
//         }
//     }
// }
impl Complete {
    fn apply_precidence(self) -> Complete {
        match *self.left {
            Expression::Complete(c) => {
                if self.operator.get_precidence() > c.operator.get_precidence() {
                    Complete {
                        operator: c.operator,
                        left: c.left,
                        right: Box::new(Expression::Complete(
                            Complete {
                                operator: self.operator,
                                left: c.right,
                                right: self.right,
                            }
                            .apply_precidence(),
                        )),
                    }
                } else {
                    Complete {
                        operator: self.operator,
                        left: Box::new(Expression::Complete(c)),
                        right: self.right,
                    }
                }
            }
            _ => self.clone(),
        }
    }
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
            let end_pos = tokens
                .iter()
                .enumerate()
                .skip(i)
                .find_map(|(i, token)| match token {
                    Token::EndLine => Some(i),
                    _ => None,
                });
            let (literal, _) = lex_expression(&tokens[i..end_pos.unwrap()]);
            lines.push(Line::Print(literal));

            i = end_pos.unwrap();
            // match &tokens[i] {

            //     Token::String(expression) => {
            //         lines.push(Line::Print(Expression::String(expression.to_string())))
            //     }
            //     Token::VariableName(name) => {
            //         lines.push(Line::Print(Expression::Variable(name.to_string())))
            //     }
            //     Token::EndParen => lines.push(Line::Print(Expression::String("".to_string()))),
            //     Token::ConstantNumber(value) => {
            //         lines.push(Line::Print(Expression::String(value.to_string())))
            //     }
            //     Token::Boolean(value) => {
            //         lines.push(Line::Print(Expression::String(value.to_string())))
            //     }

            //     _ => println!(
            //         "{}",
            //         &format!(
            //             "Oopsie Woopsie: invalid token following print: {:?}",
            //             tokens[i]
            //         )
            //     ),
            // }
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
                    lines.push(Line::DefineVariable(
                        name.to_string(),
                        literal,
                        literal_type,
                    ));

                    i = end_pos.unwrap();
                }
                _ => {}
            }
        }
        Token::WhileLoop => {
            i += 1;
            match &tokens[i] {
                Token::Boolean(condition) => {
                    i += 3;
                    let end_pos =
                        tokens
                            .iter()
                            .enumerate()
                            .skip(i)
                            .find_map(|(i, token)| match token {
                                Token::EndLoop => Some(i),
                                _ => None,
                            });
                    let while_loop_tokens = tokens[i..end_pos.unwrap()].to_vec();
                    let while_loop_lines = lex(while_loop_tokens);
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
            let end_pos = tokens
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
            lines.push(Line::DefineVariable(
                name.to_string(),
                literal,
                literal_type,
            ));
            i = end_pos.unwrap();
        }
        Token::EndLoop => lines.push(Line::EndLoop),
        _ => {}
    }
    i
}

fn lex_expression(mut tokens: &[Token]) -> (Expression, Type) {
    let mut stack = Vec::new();
    for token in tokens {
        match token {
            Token::EndParen => tokens = &tokens[0..tokens.len() - 1],
            _ => {}
        }
    }
    if tokens.len() == 1 {
        match &tokens[0] {
            Token::ConstantNumber(value) => return (Expression::I32(value.to_string()), Type::I32),
            Token::String(value) => return (Expression::String(value.to_string()), Type::String),
            Token::Boolean(value) => return (Expression::Bool(value.clone()), Type::Bool),
            Token::VariableName(name) => {
                return (Expression::Variable(name.to_string()), Type::String)
            }
            _ => {}
        }
    }
    let mut i = 0;
    while i < tokens.len() {
        match &tokens[i] {
            Token::ConstantNumber(value) => {
                let mut right = Expression::I32(value.to_string());
                loop {
                    if stack.len() > 1 {
                        let operator = stack.pop().unwrap();
                        let left = stack.pop().unwrap();
                        match operator {
                            Expression::BinaryOperator(binary_operator) => {
                                right = Expression::Complete(
                                    Complete::from((&binary_operator, &left, &right))
                                        .apply_precidence(),
                                );
                            }
                            _ => {
                                stack.push(left);
                                stack.push(operator);
                                stack.push(right);
                                break;
                            }
                        }
                    } else {
                        stack.push(right);
                        break;
                    }
                }
            }
            Token::MathOp(opp) => {
                stack.push(Expression::from(opp));
            }
            _ => {}
        }
        i += 1;
    }
    (stack[0].clone(), Type::I32)
}
#[cfg(test)]
mod test {
    use super::{lex, Line, Type};
    use crate::{
        lex::{BinaryOperator, Complete, Expression},
        tokenize::{MathOp, Token},
    };
    #[test]
    fn test_1() {
        let actual = lex(vec![
            Token::Print,
            Token::String("hello world".to_string()),
            Token::EndParen,
            Token::EndLine,
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
            Token::EndLine,
            Token::Print,
            Token::VariableName("eee".to_string()),
            Token::EndParen,
            Token::EndLine,
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
            Token::EndLine,
            Token::Print,
            Token::VariableName("ee".to_string()),
            Token::EndParen,
            Token::EndLine,
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
            Token::String("69".to_string()),
            Token::EndParen,
            Token::EndLine,
            Token::EndLoop,
        ]);
        let expected = vec![
            Line::WhileLoop(
                Expression::Bool(true),
                vec![Line::Print(Expression::String("69".to_string()))],
            ),
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
    #[test]
    fn math_test() {
        let actual = lex(vec![
            Token::TypeI32,
            Token::VariableName("e".to_string()),
            Token::ConstantNumber("4".to_string()),
            Token::MathOp(MathOp::Add),
            Token::ConstantNumber("4".to_string()),
            Token::MathOp(MathOp::Add),
            Token::ConstantNumber("4".to_string()),
            Token::MathOp(MathOp::Add),
            Token::ConstantNumber("4".to_string()),
            Token::EndLine,
        ]);
        let expected = vec![Line::DefineVariable(
            "e".to_string(),
            Expression::Complete(Complete {
                operator: BinaryOperator::Add,
                left: Box::new(Expression::Complete(Complete {
                    operator: BinaryOperator::Add,
                    left: Box::new(Expression::Complete(Complete {
                        operator: BinaryOperator::Add,
                        left: Box::new(Expression::I32("4".to_string())),
                        right: Box::new(Expression::I32("4".to_string())),
                    })),
                    right: Box::new(Expression::I32("4".to_string())),
                })),
                right: Box::new(Expression::I32("4".to_string())),
            }),
            Type::I32,
        )];
        assert_eq!(actual, expected);
    }
    #[test]
    fn oop_test() {
        let actual = lex(vec![
            Token::TypeI32,
            Token::VariableName("e".to_string()),
            Token::ConstantNumber("1".to_string()),
            Token::MathOp(MathOp::Add),
            Token::ConstantNumber("2".to_string()),
            Token::MathOp(MathOp::Subtract),
            Token::ConstantNumber("3".to_string()),
            Token::MathOp(MathOp::Multiply),
            Token::ConstantNumber("4".to_string()),
            Token::EndLine,
        ]);
        let expected = vec![Line::DefineVariable(
            "e".to_string(),
            Expression::Complete(Complete {
                operator: BinaryOperator::Subtract,
                left: Box::new(Expression::Complete(Complete {
                    operator: BinaryOperator::Add,
                    left: Box::new(Expression::I32("1".to_string())),
                    right: Box::new(Expression::I32("2".to_string())),
                })),
                right: Box::new(Expression::Complete(Complete {
                    operator: BinaryOperator::Multiply,
                    left: Box::new(Expression::I32("3".to_string())),
                    right: Box::new(Expression::I32("4".to_string())),
                })),
            }),
            Type::I32,
        )];
        assert_eq!(actual, expected);
    }
    #[test]
    fn simple_print_add() {
        let actual = lex(vec![
            Token::Print,
            Token::ConstantNumber("1".to_string()),
            Token::MathOp(MathOp::Add),
            Token::ConstantNumber("69".to_string()),
            Token::EndParen,
            Token::EndLine,
        ]);
        let expected = vec![Line::Print(Expression::Complete(Complete {
            operator: BinaryOperator::Add,
            left: Box::new(Expression::I32("1".to_string())),
            right: Box::new(Expression::I32("69".to_string())),
        }))];
        assert_eq!(actual, expected);
    }
}
