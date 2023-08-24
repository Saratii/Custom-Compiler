use std::collections::VecDeque;

use crate::tokenize::{MathOp, Token};
#[derive(PartialEq, Debug, Clone)]
pub enum Statement {
    Print(Expression),
    DefineVariable(String, Expression, Type),
    WhileLoop(Expression, VecDeque<Statement>),
    If(Expression, VecDeque<Statement>, VecDeque<Statement>, Option<VecDeque<Statement>>),
    Elif(Expression, VecDeque<Statement>),
    ForLoop(Box<Statement>, Expression, Box<Statement>, VecDeque<Statement>),
    ModifyVariable(String, Expression),
}
#[derive(PartialEq, Debug, Clone)]
pub enum Expression {
    String(String),
    Bool(bool),
    Variable(String),
    I32(i32),
    Complete(Complete),
    BinaryOperator(BinaryOperator),
    Increment,
    Decrement,
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
    Variable,
}
pub fn parse(tokens: &mut VecDeque<Token>) -> VecDeque<Statement> {
    let mut statements = VecDeque::new();
    while tokens.len() > 0 && tokens[0] != Token::EndBlock{
        statements.push_back(parse_next_statement(tokens));
    }
    statements
}

fn parse_next_statement(tokens: &mut VecDeque<Token>) -> Statement{
    let next_token = tokens.pop_front().unwrap();
    match next_token {
        Token::Print => {
            let literal = parse_expression(tokens);
            tokens.pop_front(); //eat ;
            return Statement::Print(literal);
        }
        Token::If => {
            let condition = parse_expression(tokens);
            tokens.pop_front(); //eat {
            let body = parse(tokens);
            tokens.pop_front(); //eat }
            let mut elifs = VecDeque::new();
            while tokens.len() > 0 && tokens[0] == Token::Elif{
                let elif_condition = parse_expression(tokens);
                tokens.pop_front(); //eat {
                let elif_body = parse(tokens);
                tokens.pop_front(); //eat }
                elifs.push_back(Statement::Elif(elif_condition, elif_body));
            }
            let mut else_body = None;
            if tokens.len() > 0 && tokens[0] == Token::Else{
                tokens.pop_front(); //eat else
                tokens.pop_front(); //eat {
                else_body = Some(parse(tokens));
                tokens.pop_front(); //eat }, 
            }
            return Statement::If(condition, body, elifs, else_body)
        }
        Token::ForLoop => {
            tokens.pop_front(); //eat (
            let variable = parse_next_statement(tokens);
            let condition = parse_expression(tokens);
            let increment = parse_next_statement(tokens);
            tokens.pop_front(); //eat )
            tokens.pop_front(); //eat {
            let block = parse(tokens);
            return Statement::ForLoop(Box::new(variable), condition, Box::new(increment), block)
        }
        Token::TypeBool => match tokens.pop_front().unwrap() {
            Token::VariableName(name) => {
                let expression = parse_expression(tokens);                
                return Statement::DefineVariable(name, expression, Type::Bool);
            }
            _ => panic!("found on variable name after TypeBool"),
        }
        Token::TypeString => match tokens.pop_front().unwrap() {
            Token::VariableName(name) => {
                let expression = parse_expression(tokens);
                return Statement::DefineVariable(name, expression, Type::String);
            }
            _ => panic!("found non variable name after TypeString"),
        }
        Token::TypeI32 => match tokens.pop_front().unwrap() {
            Token::VariableName(name) => {
                let expression = parse_expression(tokens);
                return Statement::DefineVariable(name, expression, Type::I32);
            }
            _ => panic!("found on variable name after TypeI32"),
        }
        Token::WhileLoop => {
           let condition = parse_expression(tokens);
           println!("tokens after condition found: {:?}", tokens);
           tokens.pop_front(); //eat {
           let block = parse(tokens);
           return Statement::WhileLoop(condition, block)
        }
        Token::VariableName(name) => {
           let expression = parse_expression(tokens);
           return Statement::ModifyVariable(name, expression)
        }
        _ => {panic!("found {:?} trying to parse the start of a line", next_token)}
    }
}

fn parse_expression(tokens: &mut VecDeque<Token>) -> Expression {
    let mut stack: Vec<Expression> = Vec::new();
    while tokens.len() > 0 {
        match tokens.pop_front().unwrap() {
            Token::ConstantNumber(value) => {
                stack_helper(&mut stack, Type::I32, Some(value), None)
            }
            Token::VariableName(name) => {
                stack_helper(&mut stack, Type::Variable,  Some(name), None)
            }
            Token::String(literal) => {
                stack_helper(&mut stack, Type::String, Some(literal), None)
            }
            Token::Boolean(literal) => {
                stack_helper(&mut stack, Type::Bool, None, Some(literal))
            }
            Token::MathOp(opp) => {
                stack.push(Expression::from(&opp));
            }
            Token::EndLine => {
                return stack[0].clone()
            }
            Token::CloseParen => {
                return stack[0].clone()
            }
            Token::OpenParen => {
                return parse_expression(tokens);
            }
            Token::Increment => {
                if tokens.len() > 0{
                    let next = tokens.pop_front().unwrap(); //eat ;
                    match next{
                        Token::EndLine => {}
                        _ => {tokens.push_front(next)}
                    }
                }
                return Expression::Increment
            }
            Token::Decrement => {
                if tokens.len() > 0{
                    let next = tokens.pop_front().unwrap(); //eat ;
                    match next{
                        Token::EndLine => {}
                        _ => {tokens.push_front(next)}
                    }
                }
                return Expression::Decrement
            }
            _ => {}
        }
    }
    stack[0].clone()
}
fn stack_helper(stack: &mut Vec<Expression>, type_: Type, string_value: Option<String>, bool_value: Option<bool>){
    let mut right;
    match type_{
        Type::Bool => {right = Expression::Bool(bool_value.unwrap());}
        Type::I32 => {right = Expression::I32(string_value.unwrap().parse::<i32>().unwrap());}
        Type::String => {right = Expression::String(string_value.unwrap());}
        Type::Variable => {right = Expression::Variable(string_value.unwrap());}
    }
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
#[cfg(test)]
mod test {
    use super::{parse, Statement, Type};
    use crate::{
        parse::{BinaryOperator, Complete, Expression},
        tokenize::{MathOp, Token},
    };
    use std::collections::VecDeque;
    #[test]
    fn hello_world() {
        let actual = parse(&mut VecDeque::from([
            Token::Print,
            Token::OpenParen,
            Token::String("hello world".to_string()),
            Token::CloseParen,
            Token::EndLine,
        ]));
        let expected = vec![Statement::Print(Expression::String("hello world".to_string()))];
        assert_eq!(actual, expected);
    }
    #[test]
    fn bool_test() {
        let actual = parse(&mut VecDeque::from([
            Token::TypeBool,
            Token::VariableName("peepaw".to_string()),
            Token::Boolean(true),
            Token::EndLine,
        ]));
        let expected = vec![Statement::DefineVariable(
            "peepaw".to_string(),
            Expression::Bool(true),
            Type::Bool,
        )];
        assert_eq!(actual, expected);
    }
    #[test]
    fn print_variable_test() {
        let actual = parse(&mut VecDeque::from([
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
            Statement::DefineVariable("eee".to_string(), Expression::Bool(true), Type::Bool),
            Statement::Print(Expression::Variable("eee".to_string())),
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn print_string_test() {
        let actual = parse(&mut VecDeque::from([
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
            Statement::DefineVariable(
                "ee".to_string(),
                Expression::String("should I kill myself?".to_string()),
                Type::String,
            ),
            Statement::Print(Expression::Variable("ee".to_string())),
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn simple_while_loop() {
        let actual = parse(&mut VecDeque::from([
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
            Statement::WhileLoop(
                Expression::Bool(true),
                VecDeque::from([Statement::Print(Expression::String("69".to_string()))]),
            ),
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn change_variable() {
        let actual = parse(&mut VecDeque::from([
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
            Statement::DefineVariable("i".to_string(), Expression::I32(0), Type::I32),
            Statement::ModifyVariable("i".to_string(), Expression::I32(1)),
            Statement::DefineVariable(
                "e".to_string(),
                Expression::String("hello".to_string()),
                Type::String,
            ),
            Statement::ModifyVariable(
                "e".to_string(),
                Expression::String("bye".to_string()),
            ),
            Statement::DefineVariable("yes".to_string(), Expression::Bool(true), Type::Bool),
            Statement::ModifyVariable("yes".to_string(), Expression::Bool(false)),
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn math_test() {
        let actual = parse(&mut VecDeque::from([
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
        let expected = vec![Statement::DefineVariable(
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
        let actual = parse(&mut VecDeque::from([
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
        let expected = vec![Statement::DefineVariable(
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
        let actual = parse(&mut VecDeque::from([
            Token::Print,
            Token::OpenParen,
            Token::ConstantNumber("1".to_string()),
            Token::MathOp(MathOp::Add),
            Token::ConstantNumber("69".to_string()),
            Token::CloseParen,
            Token::EndLine,
        ]));
        let expected = vec![Statement::Print(Expression::Complete(Complete {
            operator: BinaryOperator::Add,
            left: Box::new(Expression::I32(1)),
            right: Box::new(Expression::I32(69)),
        }))];
        assert_eq!(actual, expected);
    }
    #[test]
    fn variable_adding() {
        let actual = parse(&mut VecDeque::from([
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
            Statement::DefineVariable("e".to_string(), Expression::I32(1), Type::I32),
            Statement::DefineVariable("ee".to_string(), Expression::I32(2), Type::I32),
            Statement::Print(Expression::Complete(Complete {
                operator: BinaryOperator::Add,
                left: Box::new(Expression::Variable("e".to_string())),
                right: Box::new(Expression::Variable("ee".to_string())),
            })),
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn basic_if() {
        let actual = parse(&mut VecDeque::from([
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
            Statement::DefineVariable("e".to_string(), Expression::I32(69), Type::I32),
            Statement::If(
                Expression::Complete(Complete {
                    operator: BinaryOperator::Equals,
                    left: Box::new(Expression::Variable("e".to_string())),
                    right: Box::new(Expression::I32(69)),
                }),
                VecDeque::from([Statement::Print(Expression::Variable("e".to_string()))]),
                VecDeque::new(),
                None
            ),
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn for_loop() {
        let actual = parse(&mut VecDeque::from([
            Token::ForLoop,
            Token::OpenParen,
            Token::TypeI32,
            Token::VariableName("i".to_string()),
            Token::ConstantNumber("0".to_string()),
            Token::EndLine,
            Token::VariableName("i".to_string()),
            Token::MathOp(MathOp::LessThan),
            Token::ConstantNumber("10".to_string()),
            Token::EndLine,
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
        ]));
        let expected = vec![
            Statement::ForLoop(
                Box::new(Statement::DefineVariable(
                    "i".to_string(),
                    Expression::I32(0),
                    Type::I32,
                )),
                Expression::Complete(Complete {
                    operator: BinaryOperator::LessThan,
                    left: Box::new(Expression::Variable("i".to_string())),
                    right: Box::new(Expression::I32(10)),
                }),
                Box::new(Statement::ModifyVariable(
                    "i".to_string(),
                    Expression::Increment,
                )),
                VecDeque::from([Statement::Print(Expression::Variable("i".to_string()))]),
            ),
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn double_if() {
        let actual = parse(&mut VecDeque::from([
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
            Statement::If(
                Expression::Bool(true),
                VecDeque::from([
                    Statement::If(
                        Expression::Bool(false),
                        VecDeque::from([Statement::Print(Expression::String("a".to_string()))]),
                        VecDeque::new(),
                        None
                    ),
                ]),
                VecDeque::new(),
                None
            ),
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn more_if() {
        let actual = parse(&mut VecDeque::from([
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
            Statement::If(
                Expression::Bool(true),
                VecDeque::from([
                    Statement::If(
                        Expression::Bool(false),
                        VecDeque::from([Statement::Print(Expression::String("a".to_string()))]),
                        VecDeque::new(),
                        None
                    ),
                    Statement::If(
                        Expression::Bool(true),
                        VecDeque::from([Statement::Print(Expression::String("n".to_string()))]),
                        VecDeque::new(),
                        None
                    )
                ]),
                VecDeque::new(),
                None
            )
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn increment_test() {
        let actual = parse(&mut VecDeque::from([
            Token::TypeI32,
            Token::VariableName("w".to_string()),
            Token::ConstantNumber("68".to_string()),
            Token::EndLine,
            Token::VariableName("w".to_string()),
            Token::Decrement,
            Token::EndLine,
        ]));
        let expected = vec![
            Statement::DefineVariable("w".to_string(), Expression::I32(68), Type::I32),
            Statement::ModifyVariable(
                "w".to_string(),
                Expression::Decrement,
            ),
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn if_elif_elif_else(){
        let actual = parse(&mut VecDeque::from([
            Token::TypeI32,
            Token::VariableName("i".to_string()),
            Token::ConstantNumber("69".to_string()),
            Token::EndLine,
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
            Token::Elif,
            Token::OpenParen,
            Token::VariableName("i".to_string()),
            Token::MathOp(MathOp::Equals),
            Token::ConstantNumber("69".to_string()),
            Token::CloseParen,
            Token::StartBlock,
            Token::EndBlock,
            Token::Else,
            Token::StartBlock,
            Token::EndBlock,
        ]));
        let expected = vec![
            Statement::DefineVariable("i".to_string(), Expression::I32(69), Type::I32),
            Statement::If(
                Expression::Complete(Complete{operator: BinaryOperator::Equals, left: Box::new(Expression::Variable("i".to_string())), right: Box::new(Expression::I32(6))}),
                VecDeque::from([]),
                VecDeque::from([
                    Statement::Elif(
                        Expression::Complete(Complete{operator: BinaryOperator::Equals, left: Box::new(Expression::Variable("i".to_string())), right: Box::new(Expression::I32(7))}),
                        VecDeque::from([])
                    ),
                    Statement::Elif(
                        Expression::Complete(Complete{operator: BinaryOperator::Equals, left: Box::new(Expression::Variable("i".to_string())), right: Box::new(Expression::I32(69))}),
                        VecDeque::from([])
                    )
                ]),
                Some(VecDeque::new())
            ),
        ];
            assert_eq!(actual, expected);
    }
}
