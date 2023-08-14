use std::collections::VecDeque;

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
    Modulus,
    Equals,
    NotEqual,
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
            BinaryOperator::NotEqual => 0,
            BinaryOperator::LessThan => 0,
            BinaryOperator::LessThanOrEqualTo => 0,
            BinaryOperator::GreaterThan => 0,
            BinaryOperator::GreaterThanOrEqualTo => 0,
            BinaryOperator::Modulus => 2,
        }
    }
}
impl From<&MathOp> for Expression {
    fn from(op: &MathOp) -> Expression {
        match op {
            MathOp::NotEqual => Expression::BinaryOperator(BinaryOperator::NotEqual),
            MathOp::Modulus => Expression::BinaryOperator(BinaryOperator::Modulus),
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
pub fn parse(mut tokens: VecDeque<Token>) -> Vec<Line> {
    let mut lines = vec![];
    while tokens.len() > 0 {
        let mut block_count = 0;
        process_token(&mut tokens, &mut lines, &mut block_count);
    }
    lines
}

fn process_token(tokens: &mut VecDeque<Token>, lines: &mut Vec<Line>, block_count: &mut usize) {
    match tokens.pop_front().unwrap() {
        Token::Print => {
            let mut expression_tokens = Vec::new();
            tokens.pop_front(); //remove start paren
            loop {
                let next_token = tokens.pop_front().unwrap();
                match next_token {
                    Token::CloseParen => break,
                    _ => expression_tokens.push(next_token),
                }
            }
            tokens.pop_front(); //remove end line
            let (literal, _) = lex_expression(&expression_tokens);
            lines.push(Line::Print(literal));
        }
        Token::If => {
            tokens.pop_front(); //remove paren
            let mut condition_tokens = Vec::new();
            loop {
                let next_token = tokens.pop_front().unwrap();
                match next_token {
                    Token::CloseParen => break,
                    _ => condition_tokens.push(next_token),
                }
            }
            tokens.pop_front(); //remove start block
            *block_count += 1;
            let mut block_tokens = VecDeque::new();
            loop {
                let next_token = tokens.pop_front().unwrap();
                match next_token {
                    Token::EndBlock if *block_count == 1 => {
                        let (condition, _) = lex_expression(&condition_tokens);
                        let block_lines = parse(block_tokens);
                        lines.push(Line::If(condition, block_lines));
                        lines.push(Line::EndBlock);
                        break;
                    }
                    Token::EndBlock => {
                        block_tokens.push_back(next_token);
                        *block_count -= 1;
                    }
                    Token::StartBlock => {
                        *block_count += 1;
                        block_tokens.push_back(next_token);
                    }
                    _ => block_tokens.push_back(next_token),
                }
            }
        }
        Token::ForLoop => {
            tokens.pop_front(); //remove start paren
            let starting_variable_type;
            match tokens.pop_front().unwrap() {
                Token::TypeI32 => starting_variable_type = Type::I32,
                _ => starting_variable_type = Type::Bool,
            }
            let starting_variable_name: String;
            match tokens.pop_front().unwrap() {
                Token::VariableName(name) => starting_variable_name = name.to_string(),
                _ => panic!("parser found non variable name after variable type in ForLoop"),
            }
            let mut variable_init_tokens = Vec::new();
            loop {
                let next_token = tokens.pop_front().unwrap();
                match next_token {
                    Token::Comma => break,
                    _ => variable_init_tokens.push(next_token),
                }
            }
            let mut condition_tokens = Vec::new();
            loop {
                let next_token = tokens.pop_front().unwrap();
                match next_token {
                    Token::Comma => break,
                    _ => condition_tokens.push(next_token),
                }
            }
            let mut increment_tokens = Vec::new();
            loop {
                let next_token = tokens.pop_front().unwrap();
                match next_token {
                    Token::CloseParen => break,
                    _ => increment_tokens.push(next_token),
                }
            }
            tokens.pop_front(); //remove start block
            *block_count += 1;
            let mut block_tokens = VecDeque::new();
            loop {
                let next_token = tokens.pop_front().unwrap();
                match next_token {
                    Token::EndBlock if *block_count == 1 => {
                        let (variable_init_value, _) = lex_expression(&variable_init_tokens);
                        let (condition, _) = lex_expression(&condition_tokens);
                        let (increment, _) = lex_expression(&increment_tokens);
                        let block_lines = parse(block_tokens);
                        lines.push(Line::ForLoop(
                            Box::new(Line::DefineVariable(
                                starting_variable_name.to_string(),
                                variable_init_value,
                                starting_variable_type.clone(),
                            )),
                            condition,
                            Box::new(Line::DefineVariable(
                                starting_variable_name.to_string(),
                                increment,
                                starting_variable_type.clone(),
                            )),
                            block_lines,
                        ));
                        lines.push(Line::EndBlock);
                        break;
                    }
                    Token::EndBlock => {
                        block_tokens.push_back(next_token);
                        *block_count -= 1;
                    }
                    Token::StartBlock => {
                        *block_count += 1;
                        block_tokens.push_back(next_token)
                    }
                    _ => block_tokens.push_back(next_token),
                }
            }
        }
        Token::TypeBool => match tokens.pop_front().unwrap() {
            Token::VariableName(name) => {
                let mut expression_tokens = Vec::new();
                loop {
                    let next_token = tokens.pop_front().unwrap();
                    match next_token {
                        Token::EndLine => break,
                        _ => expression_tokens.push(next_token),
                    }
                }
                let (expression, _) = lex_expression(&expression_tokens);
                lines.push(Line::DefineVariable(name, expression, Type::Bool));
            }
            _ => panic!("found on variable name after TypeBool"),
        },
        Token::TypeString => match tokens.pop_front().unwrap() {
            Token::VariableName(name) => {
                let mut expression_tokens = Vec::new();
                loop {
                    let next_token = tokens.pop_front().unwrap();
                    match next_token {
                        Token::EndLine => break,
                        _ => expression_tokens.push(next_token),
                    }
                }
                let (expression, _) = lex_expression(&expression_tokens);
                lines.push(Line::DefineVariable(name, expression, Type::String));
            }
            _ => panic!("found on variable name after TypeString"),
        },
        Token::TypeI32 => match tokens.pop_front().unwrap() {
            Token::VariableName(name) => {
                let mut expression_tokens = Vec::new();
                loop {
                    let next_token = tokens.pop_front().unwrap();
                    match next_token {
                        Token::EndLine => break,
                        _ => expression_tokens.push(next_token),
                    }
                }
                let (expression, _) = lex_expression(&expression_tokens);
                lines.push(Line::DefineVariable(name, expression, Type::I32));
            }
            _ => panic!("found on variable name after TypeBool"),
        },
        Token::WhileLoop => {
            tokens.pop_front(); //remove paren
            let mut condition_tokens = Vec::new();
            loop {
                let next_token = tokens.pop_front().unwrap();
                match next_token {
                    Token::CloseParen => break,
                    _ => condition_tokens.push(next_token),
                }
            }
            tokens.pop_front(); //remove start block
            *block_count += 1;
            let mut block_tokens = VecDeque::new();
            loop {
                let next_token = tokens.pop_front().unwrap();
                match next_token {
                    Token::EndBlock if *block_count == 1 => {
                        let (condition, _) = lex_expression(&condition_tokens);
                        let block_lines = parse(block_tokens);
                        lines.push(Line::WhileLoop(condition, block_lines));
                        lines.push(Line::EndBlock);
                        break;
                    }
                    Token::EndBlock => {
                        block_tokens.push_back(next_token);
                        *block_count -= 1;
                    }
                    Token::StartBlock => {
                        *block_count += 1;
                        block_tokens.push_back(next_token)
                    }
                    _ => block_tokens.push_back(next_token),
                }
            }
        }
        Token::VariableName(name) => {
            let next = tokens.pop_front().unwrap();
            let mut expression_tokens = Vec::new();
            match &next {
                Token::IncrementDown => {
                    expression_tokens.push(Token::VariableName(name.clone()));
                }
                Token::IncrementUp => {
                    expression_tokens.push(Token::VariableName(name.clone()));
                }
                _ => {}
            }
            tokens.push_front(next);
            loop {
                let next_token = tokens.pop_front().unwrap();
                match next_token {
                    Token::EndLine => break,
                    _ => expression_tokens.push(next_token),
                }
            }
            let (expression, literal_type) = lex_expression(&expression_tokens);
            lines.push(Line::DefineVariable(
                name.to_string(),
                expression,
                literal_type,
            ));
        }
        _ => {}
    }
}

fn lex_expression(mut tokens: &[Token]) -> (Expression, Type) {
    let mut stack = Vec::new();
    for token in tokens {
        match token {
            Token::CloseParen => tokens = &tokens[0..tokens.len() - 1],
            Token::OpenParen => tokens = &tokens[1..],
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

    match &tokens[0] {
        Token::VariableName(name) => match &tokens[1] {
            Token::IncrementDown => {
                return (
                    Expression::Complete(Complete {
                        operator: BinaryOperator::Subtract,
                        left: Box::new(Expression::Variable(name.to_string())),
                        right: Box::new(Expression::I32(1)),
                    }),
                    Type::I32,
                )
            }
            Token::IncrementUp => {
                return (
                    Expression::Complete(Complete {
                        operator: BinaryOperator::Add,
                        left: Box::new(Expression::Variable(name.to_string())),
                        right: Box::new(Expression::I32(1)),
                    }),
                    Type::I32,
                )
            }
            _ => {}
        },
        _ => {}
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
    use super::{parse, Line, Type};
    use crate::{
        lex::{BinaryOperator, Complete, Expression},
        tokenize::{MathOp, Token},
    };
    use std::collections::VecDeque;
    #[test]
    fn test_1() {
        let actual = parse(VecDeque::from([
            Token::Print,
            Token::OpenParen,
            Token::String("hello world".to_string()),
            Token::CloseParen,
            Token::EndLine,
        ]));
        let expected = vec![Line::Print(Expression::String("hello world".to_string()))];
        assert_eq!(actual, expected);
    }
    #[test]
    fn bool_test() {
        let actual = parse(VecDeque::from([
            Token::TypeBool,
            Token::VariableName("peepaw".to_string()),
            Token::Boolean(true),
            Token::EndLine,
        ]));
        let expected = vec![Line::DefineVariable(
            "peepaw".to_string(),
            Expression::Bool(true),
            Type::Bool,
        )];
        assert_eq!(actual, expected);
    }
    #[test]
    fn print_variable_test() {
        let actual = parse(VecDeque::from([
            Token::TypeBool,
            Token::VariableName("eee".to_string()),
            Token::Boolean(true),
            Token::EndLine,
            Token::Print,
            Token::OpenParen,
            Token::VariableName("eee".to_string()),
            Token::CloseParen,
            Token::EndLine,
        ]));
        let expected = vec![
            Line::DefineVariable("eee".to_string(), Expression::Bool(true), Type::Bool),
            Line::Print(Expression::Variable("eee".to_string())),
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn print_string_test() {
        let actual = parse(VecDeque::from([
            Token::TypeString,
            Token::VariableName("ee".to_string()),
            Token::String("should I kill myself?".to_string()),
            Token::EndLine,
            Token::Print,
            Token::OpenParen,
            Token::VariableName("ee".to_string()),
            Token::CloseParen,
            Token::EndLine,
        ]));
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
        let actual = parse(VecDeque::from([
            Token::WhileLoop,
            Token::OpenParen,
            Token::Boolean(true),
            Token::CloseParen,
            Token::StartBlock,
            Token::Print,
            Token::OpenParen,
            Token::String("69".to_string()),
            Token::CloseParen,
            Token::EndLine,
            Token::EndBlock,
        ]));
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
        let actual = parse(VecDeque::from([
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
        ]));
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
        let actual = parse(VecDeque::from([
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
        ]));
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
        let actual = parse(VecDeque::from([
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
        ]));
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
        let actual = parse(VecDeque::from([
            Token::Print,
            Token::OpenParen,
            Token::ConstantNumber("1".to_string()),
            Token::MathOp(MathOp::Add),
            Token::ConstantNumber("69".to_string()),
            Token::CloseParen,
            Token::EndLine,
        ]));
        let expected = vec![Line::Print(Expression::Complete(Complete {
            operator: BinaryOperator::Add,
            left: Box::new(Expression::I32(1)),
            right: Box::new(Expression::I32(69)),
        }))];
        assert_eq!(actual, expected);
    }
    #[test]
    fn variable_adding() {
        let actual = parse(VecDeque::from([
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
        ]));
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
        let actual = parse(VecDeque::from([
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
        ]));
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
        let actual = parse(VecDeque::from([
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
            Token::IncrementUp,
            Token::CloseParen,
            Token::StartBlock,
            Token::Print,
            Token::OpenParen,
            Token::VariableName("i".to_string()),
            Token::CloseParen,
            Token::EndLine,
            Token::EndBlock,
        ]));
        let expected = vec![
            Line::ForLoop(
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
            ),
            Line::EndBlock,
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn double_if() {
        let actual = parse(VecDeque::from([
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
        ]));
        let expected = vec![
            Line::If(
                Expression::Bool(true),
                vec![
                    Line::If(
                        Expression::Bool(false),
                        vec![Line::Print(Expression::String("a".to_string()))],
                    ),
                    Line::EndBlock,
                ],
            ),
            Line::EndBlock,
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn more_if() {
        let actual = parse(VecDeque::from([
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
            Token::If,
            Token::OpenParen,
            Token::Boolean(true),
            Token::CloseParen,
            Token::StartBlock,
            Token::Print,
            Token::OpenParen,
            Token::String("n".to_string()),
            Token::CloseParen,
            Token::EndLine,
            Token::EndBlock,
            Token::EndBlock,
        ]));
        let expected = vec![
            Line::If(
                Expression::Bool(true),
                vec![
                    Line::If(
                        Expression::Bool(false),
                        vec![Line::Print(Expression::String("a".to_string()))],
                    ),
                    Line::EndBlock,
                    Line::If(
                        Expression::Bool(true),
                        vec![Line::Print(Expression::String("n".to_string()))],
                    ),
                    Line::EndBlock,
                ],
            ),
            Line::EndBlock,
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn increment_test() {
        let actual = parse(VecDeque::from([
            Token::TypeI32,
            Token::VariableName("w".to_string()),
            Token::ConstantNumber("68".to_string()),
            Token::EndLine,
            Token::VariableName("w".to_string()),
            Token::IncrementDown,
            Token::EndLine,
        ]));
        let expected = vec![
            Line::DefineVariable("w".to_string(), Expression::I32(68), Type::I32),
            Line::DefineVariable(
                "w".to_string(),
                Expression::Complete(Complete {
                    operator: BinaryOperator::Subtract,
                    left: Box::new(Expression::Variable("w".to_string())),
                    right: Box::new(Expression::I32(1)),
                }),
                Type::I32,
            ),
        ];
        assert_eq!(actual, expected);
    }
}
