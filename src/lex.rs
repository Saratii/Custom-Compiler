use crate::tokenize::{MathOp, Token};
#[derive(PartialEq, Debug, Clone)]
pub enum Line {
    Print(Expression),
    DefineVariable(String, Expression, Type),
    WhileLoop(Expression, Vec<Line>),
    EndBlock,
    If(Expression, Vec<Line>),
    ForLoop(Box<Line>, Expression, Box<Line>, Vec<Line>),
}
#[derive(PartialEq, Debug, Clone)]
pub enum Expression {
    String(String),
    Bool(bool),
    Variable(String),
    I32(i32),
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
    Equals,
    LessThan,
    LessThanOrEqualTo,
    GreaterThan,
    GreaterThanOrEqualTo,
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
            BinaryOperator::Equals => 0,
            BinaryOperator::LessThan => 0,
            BinaryOperator::LessThanOrEqualTo => 0,
            BinaryOperator::GreaterThan => 0,
            BinaryOperator::GreaterThanOrEqualTo => 0,
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
            MathOp::Equals => Expression::BinaryOperator(BinaryOperator::Equals),
            MathOp::LessThan => Expression::BinaryOperator(BinaryOperator::LessThan),
            MathOp::LessThanOrEqualTo => {
                Expression::BinaryOperator(BinaryOperator::LessThanOrEqualTo)
            }
            MathOp::GreaterThan => Expression::BinaryOperator(BinaryOperator::GreaterThan),
            MathOp::GreaterThanOrEqualTo => {
                Expression::BinaryOperator(BinaryOperator::GreaterThanOrEqualTo)
            }
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
        }
        Token::If => {
            i += 1;
            let end_pos = tokens
                .iter()
                .enumerate()
                .skip(i)
                .find_map(|(i, token)| match token {
                    Token::EndParen => Some(i),
                    _ => None,
                });
            let (condition_literal, _) = lex_expression(&tokens[i..end_pos.unwrap()]);
            i = end_pos.unwrap() + 2;
            let end_of_if = tokens
                .iter()
                .enumerate()
                .skip(i)
                .find_map(|(i, token)| match token {
                    Token::EndBlock => Some(i),
                    _ => None,
                });

            let if_tokens = tokens[i..end_of_if.unwrap()].to_vec();
            let if_lines = lex(if_tokens);
            lines.push(Line::If(condition_literal, if_lines));
            i = end_of_if.unwrap() - 1;
        }
        Token::ForLoop => {
            i += 1;
            let starting_variable_type;
            match &tokens[i] {
                Token::TypeI32 => starting_variable_type = Type::I32,
                _ => starting_variable_type = Type::Bool,
            }
            i += 1;
            let starting_variable_name: String;
            match &tokens[i] {
                Token::VariableName(name) => starting_variable_name = name.to_string(),
                _ => starting_variable_name = "compiler did a fucky wucky".to_string(),
            }
            i += 1;
            let end_of_define_variable =
                tokens
                    .iter()
                    .enumerate()
                    .skip(i)
                    .find_map(|(i, token)| match token {
                        Token::Comma => Some(i),
                        _ => None,
                    });
            let if_variable_value_tokens = tokens[i..end_of_define_variable.unwrap()].to_vec();
            let (if_variable_value, _) = lex_expression(&if_variable_value_tokens);
            i = end_of_define_variable.unwrap() + 1;
            let end_of_if_condition =
                tokens
                    .iter()
                    .enumerate()
                    .skip(i)
                    .find_map(|(i, token)| match token {
                        Token::Comma => Some(i),
                        _ => None,
                    });
            let if_condition_tokens = tokens[i..end_of_if_condition.unwrap()].to_vec();
            let (if_condition, _) = lex_expression(&if_condition_tokens);
            i = end_of_if_condition.unwrap() + 1;
            let end_of_increment =
                tokens
                    .iter()
                    .enumerate()
                    .skip(i)
                    .find_map(|(i, token)| match token {
                        Token::EndParen => Some(i),
                        _ => None,
                    });
            let increment_tokens = tokens[i..end_of_increment.unwrap()].to_vec();
            let (increment, _) = lex_expression(&increment_tokens);
            i = end_of_increment.unwrap() + 2;
            let end_of_for_lines =
                tokens
                    .iter()
                    .enumerate()
                    .skip(i)
                    .find_map(|(i, token)| match token {
                        Token::EndBlock => Some(i),
                        _ => None,
                    });
            let for_tokens = tokens[i..end_of_for_lines.unwrap()].to_vec();
            let for_lines = lex(for_tokens);
            i = end_of_for_lines.unwrap() - 1;
            lines.push(Line::ForLoop(
                Box::new(Line::DefineVariable(
                    starting_variable_name.to_string(),
                    if_variable_value,
                    starting_variable_type.clone(),
                )),
                if_condition,
                Box::new(Line::DefineVariable(
                    starting_variable_name.to_string(),
                    increment,
                    starting_variable_type.clone(),
                )),
                for_lines,
            ));
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
                                Token::EndBlock => Some(i),
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
            lines.push(Line::DefineVariable(
                name.to_string(),
                literal,
                literal_type,
            ));
            i = end_pos.unwrap();
        }
        Token::EndBlock => lines.push(Line::EndBlock),
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
            Token::ConstantNumber(value) => {
                return (Expression::I32(value.parse::<i32>().unwrap()), Type::I32)
            }
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
                let mut right = Expression::I32(value.parse::<i32>().unwrap());
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
            Token::VariableName(name) => {
                let mut right = Expression::Variable(name.to_string());
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
            Token::StartBlock,
            Token::Print,
            Token::String("69".to_string()),
            Token::EndParen,
            Token::EndLine,
            Token::EndBlock,
        ]);
        let expected = vec![
            Line::WhileLoop(
                Expression::Bool(true),
                vec![Line::Print(Expression::String("69".to_string()))],
            ),
            Line::EndBlock,
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
            Line::DefineVariable("i".to_string(), Expression::I32(0), Type::I32),
            Line::DefineVariable("i".to_string(), Expression::I32(1), Type::I32),
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
                        left: Box::new(Expression::I32(4)),
                        right: Box::new(Expression::I32(4)),
                    })),
                    right: Box::new(Expression::I32(4)),
                })),
                right: Box::new(Expression::I32(4)),
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
                    left: Box::new(Expression::I32(1)),
                    right: Box::new(Expression::I32(2)),
                })),
                right: Box::new(Expression::Complete(Complete {
                    operator: BinaryOperator::Multiply,
                    left: Box::new(Expression::I32(3)),
                    right: Box::new(Expression::I32(4)),
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
            left: Box::new(Expression::I32(1)),
            right: Box::new(Expression::I32(69)),
        }))];
        assert_eq!(actual, expected);
    }
    #[test]
    fn variable_adding() {
        let actual = lex(vec![
            Token::TypeI32,
            Token::VariableName("e".to_string()),
            Token::ConstantNumber("1".to_string()),
            Token::EndLine,
            Token::TypeI32,
            Token::VariableName("ee".to_string()),
            Token::ConstantNumber("2".to_string()),
            Token::EndLine,
            Token::Print,
            Token::VariableName("e".to_string()),
            Token::MathOp(MathOp::Add),
            Token::VariableName("ee".to_string()),
            Token::EndParen,
            Token::EndLine,
        ]);
        let expected = vec![
            Line::DefineVariable("e".to_string(), Expression::I32(1), Type::I32),
            Line::DefineVariable("ee".to_string(), Expression::I32(2), Type::I32),
            Line::Print(Expression::Complete(Complete {
                operator: BinaryOperator::Add,
                left: Box::new(Expression::Variable("e".to_string())),
                right: Box::new(Expression::Variable("ee".to_string())),
            })),
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn basic_if() {
        let actual = lex(vec![
            Token::TypeI32,
            Token::VariableName("e".to_string()),
            Token::ConstantNumber("69".to_string()),
            Token::EndLine,
            Token::If,
            Token::VariableName("e".to_string()),
            Token::MathOp(MathOp::Equals),
            Token::ConstantNumber("69".to_string()),
            Token::EndParen,
            Token::StartBlock,
            Token::Print,
            Token::VariableName("e".to_string()),
            Token::EndParen,
            Token::EndLine,
            Token::EndBlock,
        ]);
        let expected = vec![
            Line::DefineVariable("e".to_string(), Expression::I32(69), Type::I32),
            Line::If(
                Expression::Complete(Complete {
                    operator: BinaryOperator::Equals,
                    left: Box::new(Expression::Variable("e".to_string())),
                    right: Box::new(Expression::I32(69)),
                }),
                vec![Line::Print(Expression::Variable("e".to_string()))],
            ),
            Line::EndBlock,
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn for_loop() {
        let actual = lex(vec![
            Token::ForLoop,
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
            Token::EndParen,
            Token::StartBlock,
            Token::Print,
            Token::VariableName("i".to_string()),
            Token::EndParen,
            Token::EndLine,
            Token::EndBlock,
        ]);
        let expected = vec![Line::ForLoop(
            Box::new(Line::DefineVariable(
                "i".to_string(),
                Expression::I32(0),
                Type::I32,
            )),
            Expression::Complete(Complete {
                operator: BinaryOperator::LessThan,
                left: Box::new(Expression::Variable("i".to_string())),
                right: Box::new(Expression::I32(10)),
            }),
            Box::new(Line::DefineVariable(
                "i".to_string(),
                Expression::Complete(Complete {
                    operator: BinaryOperator::Add,
                    left: Box::new(Expression::Variable("i".to_string())),
                    right: Box::new(Expression::I32(1)),
                }),
                Type::I32,
            )),
            vec![Line::Print(Expression::Variable("i".to_string()))],
        ), Line::EndBlock];
        assert_eq!(actual, expected);
    }
}
