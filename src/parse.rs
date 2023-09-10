use colored::Colorize;
use core::panic;
use std::collections::VecDeque;

use crate::{tokenize::{MathOp, Token}, compiler::{Type, Compiler}};
#[derive(PartialEq, Debug, Clone)]
pub enum Statement {
    DefineVariable(String, Expression, Type),
    WhileLoop(Expression, VecDeque<Statement>),
    If(
        Expression,
        VecDeque<Statement>,
        VecDeque<Statement>,
        Option<VecDeque<Statement>>,
    ),
    Elif(Expression, VecDeque<Statement>),
    ForLoop(
        Box<Statement>,
        Expression,
        Box<Statement>,
        VecDeque<Statement>,
    ),
    ModifyVariable(String, Expression),
    _DefineFunction(String, Vec<Type>, VecDeque<Statement>),
    FunctionCall(String, Vec<Expression>),
}

#[derive(PartialEq, Debug, Clone)]
pub enum Expression {
    String(String),
    Bool(bool),
    Variable(String),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    Array(Vec<Expression>),
    Complete(Complete),
    BinaryOperator(BinaryOperator),
    UnaryOperator(UnaryOperator),
    Increment,
    Decrement,
    FunctionCall(String, Vec<Expression>),
    CompleteU(CompleteU),
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
    And,
    Or,
}
#[derive(PartialEq, Debug, Clone)]
pub enum UnaryOperator {
    Not,
    Parenthesis,
}
#[derive(PartialEq, Debug, Clone)]
pub struct Complete {
    pub operator: BinaryOperator,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}
#[derive(PartialEq, Debug, Clone)]
pub struct CompleteU {
    pub operator: UnaryOperator,
    pub child: Box<Expression>,
}
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
            BinaryOperator::And => 0,
            BinaryOperator::Or => 0,
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
            MathOp::And => Expression::BinaryOperator(BinaryOperator::And),
            MathOp::Or => Expression::BinaryOperator(BinaryOperator::Or),
            MathOp::Not => Expression::UnaryOperator(UnaryOperator::Not),
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
impl From<(&UnaryOperator, &Expression)> for CompleteU {
    fn from((unary_operator, child): (&UnaryOperator, &Expression)) -> CompleteU {
        CompleteU {
            operator: unary_operator.clone(),
            child: Box::new(child.clone()),
        }
    }
}
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
pub struct Function {
    pub name: String,
    pub block: VecDeque<Statement>,
}
impl Compiler{
    pub fn parse(
        &self,
        tokens: &mut VecDeque<Token>,
    ) -> VecDeque<Statement> {
        let mut statements = VecDeque::new();
        while tokens.len() > 0 && tokens[0] != Token::CloseBlock {
            statements.push_back(self.parse_next_statement(tokens));
        }
        statements
    }

    fn parse_next_statement(&self, tokens: &mut VecDeque<Token>) -> Statement {
        let token_stream_0 = tokens.pop_front().unwrap();
        match &token_stream_0 {
            Token::Identifier(ident_1) => {
                let token_stream_1 = tokens.pop_front().unwrap();
                match &token_stream_1{
                    Token::Identifier(ident_2) => {
                        let type_hint = self.parse_type_hint(ident_1);
                        self.eat_token(tokens, Token::Assign);
                        let expression = self.parse_expression(tokens, Some(type_hint));
                        return Statement::DefineVariable(ident_2.clone(), expression, type_hint);
                    }
                    Token::OpenParen => {
                        let args = self.parse_function_args(tokens);
                        self.eat_token(tokens, Token::EndLine);
                        return Statement::FunctionCall(ident_1.clone(), args)
                    }
                    Token::MathOp(MathOp::LessThan) =>{
                        let token_stream_3 = tokens.pop_front().unwrap();
                        match token_stream_3{
                            Token::Identifier(ident_3) => {
                                let type_hint = self.parse_type_hint(ident_1);
                                self.eat_token(tokens, Token::MathOp(MathOp::GreaterThan));
                                let expression = self.parse_expression(tokens, None);
                                return Statement::DefineVariable(ident_3.clone(), expression, type_hint);
                            }
                            _ => {panic!("Unexpected token: {:?}", token_stream_3)}
                        }
                    }
                    Token::Increment =>{
                        let next_token = tokens.pop_front().unwrap();
                        match next_token{
                            Token::EndLine =>{},
                            Token::CloseParen =>{
                                tokens.push_front(next_token);
                            }
                            _ => {panic!("unexpected token {:?}", next_token)}
                        }
                        return Statement::ModifyVariable(ident_1.clone(), Expression::Increment)
                    }
                    Token::Decrement =>{
                        let next_token = tokens.pop_front().unwrap();
                        match next_token{
                            Token::EndLine =>{},
                            Token::CloseParen =>{
                                tokens.push_front(next_token);
                            }
                            _ => {panic!("unexpected token {:?}", next_token)}
                        }
                        return Statement::ModifyVariable(ident_1.clone(), Expression::Decrement)
                    }
                    Token::Assign =>{
                        let expression = self.parse_expression(tokens, None);
                        return Statement::ModifyVariable(ident_1.clone(), expression)
                    }
                    _ => {panic!("Unexpected token {:?}", token_stream_1)}
                }
            },
            Token::If => {
                self.eat_token(tokens, Token::OpenParen);
                let condition = self.parse_expression(tokens, None);
                self.eat_token(tokens, Token::CloseParen);
                self.eat_token(tokens, Token::OpenBlock);
                let body = self.parse(tokens);
                self.eat_token(tokens, Token::CloseBlock);
                let mut elifs = VecDeque::new();
                while tokens.len() > 0 && tokens[0] == Token::Elif {
                    self.eat_token(tokens, Token::Elif);
                    self.eat_token(tokens, Token::OpenParen);
                    let elif_condition = self.parse_expression(tokens, None);
                    self.eat_token(tokens, Token::CloseParen);
                    self.eat_token(tokens, Token::OpenBlock);
                    let elif_body = self.parse(tokens);
                    self.eat_token(tokens, Token::CloseBlock);
                    elifs.push_back(Statement::Elif(elif_condition, elif_body));
                }
                let mut else_body = None;
                if tokens.len() > 0 && tokens[0] == Token::Else {
                    self.eat_token(tokens, Token::Else);
                    self.eat_token(tokens, Token::OpenBlock);
                    else_body = Some(self.parse(tokens));
                    self.eat_token(tokens, Token::CloseBlock);
                }
                return Statement::If(condition, body, elifs, else_body);
            }
            Token::ForLoop => {
                self.eat_token(tokens, Token::OpenParen);
                let variable = self.parse_next_statement(tokens);
                let condition = self.parse_expression(tokens, None);
                let increment = self.parse_next_statement(tokens);
                self.eat_token(tokens, Token::CloseParen);
                self.eat_token(tokens, Token::OpenBlock);
                let block = self.parse(tokens);
                return Statement::ForLoop(Box::new(variable), condition, Box::new(increment), block);
            },
            Token::WhileLoop => {
                self.eat_token(tokens, Token::OpenParen);
                let condition = self.parse_expression(tokens, None);
                self.eat_token(tokens, Token::CloseParen);
                self.eat_token(tokens, Token::OpenBlock);
                let block = self.parse(tokens);
                self.eat_token(tokens, Token::CloseBlock);
                return Statement::WhileLoop(condition, block);
            }
            _ => {
                panic!("found {:?} trying to parse the start of a line", token_stream_0)
            }
        }
    }
    fn parse_expression(&self, tokens: &mut VecDeque<Token>, expected_type: Option<Type>) -> Expression {
        let mut stack: Vec<Expression> = Vec::new();
        while tokens.len() > 0 {
            match tokens.pop_front().unwrap() {
                Token::ConstantNumber(value) => {
                    if expected_type == None {
                        self.stack_helper(&mut stack, Expression::I32(value.parse::<i32>().unwrap()));
                    } else {
                        match expected_type.as_ref().unwrap() {
                            Type::I32 => {
                                self.stack_helper(
                                    &mut stack,
                                    Expression::I32(value.parse::<i32>().unwrap()),
                                );
                            }
                            Type::I64 => {
                                self.stack_helper(
                                    &mut stack,
                                    Expression::I64(value.parse::<i64>().unwrap()),
                                );
                            }
                            Type::F32 => {
                                self.stack_helper(
                                    &mut stack,
                                    Expression::F32(value.parse::<f32>().unwrap()),
                                );
                            }
                            Type::F64 => {
                                self.stack_helper(
                                    &mut stack,
                                    Expression::F64(value.parse::<f64>().unwrap()),
                                );
                            }
                            _ => {}
                        }
                    }
                }
                Token::Identifier(name) => {
                    let next_token = tokens.pop_front().unwrap();
                    match next_token{
                        Token::OpenParen => {
                            let args = self.parse_function_args(tokens);
                            self.stack_helper(&mut stack, Expression::FunctionCall(name, args));
                        }
                        _ => {
                            tokens.push_front(next_token);
                            self.stack_helper(&mut stack, Expression::Variable(name))
                        }
                    }
                },
                Token::String(literal) => self.stack_helper(&mut stack, Expression::String(literal)),
                Token::Boolean(literal) => self.stack_helper(&mut stack, Expression::Bool(literal)),
                Token::MathOp(opp) => {
                    stack.push(Expression::from(&opp));
                }
                Token::EndLine => return stack[0].clone(),
                Token::CloseParen => {
                    tokens.push_front(Token::CloseParen);
                    return stack[0].clone()
                }
                Token::Comma => return stack[0].clone(),
                Token::CloseBracket => return stack[0].clone(),

                Token::OpenParen => {
                    self.stack_helper(
                        &mut stack,
                        Expression::CompleteU(CompleteU {
                            operator: UnaryOperator::Parenthesis,
                            child: Box::new(self.parse_expression(tokens, None)),
                        }),
                    );
                }
                Token::Increment => {
                    if tokens.len() > 0 {
                        let next = tokens.pop_front().unwrap(); //eat ;
                        match next {
                            Token::EndLine => {}
                            _ => tokens.push_front(next),
                        }
                    }
                    return Expression::Increment;
                }
                Token::Decrement => {
                    if tokens.len() > 0 {
                        let next = tokens.pop_front().unwrap(); //eat ;
                        match next {
                            Token::EndLine => {}
                            _ => tokens.push_front(next),
                        }
                    }
                    return Expression::Decrement;
                }
                _ => {}
            }
        }
        stack[0].clone()
    }
    fn stack_helper(&self, stack: &mut Vec<Expression>, expression: Expression) {
        let mut right = expression;
        loop {
            if stack.len() > 0 {
                let operator = stack.pop().unwrap();
                match operator {
                    Expression::BinaryOperator(binary_operator) => {
                        if stack.len() > 0 {
                            let left = stack.pop().unwrap();
                            right = Expression::Complete(
                                Complete::from((&binary_operator, &left, &right)).apply_precidence(),
                            );
                        } else {
                            stack.push(Expression::BinaryOperator(binary_operator));
                            stack.push(right);
                            break;
                        }
                    }
                    Expression::UnaryOperator(unary_operator) => {
                        right = Expression::CompleteU(CompleteU {
                            operator: unary_operator,
                            child: Box::new(right),
                        })
                    }
                    _ => {
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

    fn eat_token(&self, tokens: &mut VecDeque<Token>, expected: Token) {
        if tokens.len() == 0 || tokens[0] != expected {
            panic!("Tried to eat a {:?}, but found {:?}", expected, tokens);
        }
        tokens.pop_front();
    }
    fn parse_type_hint(&self, ident: &String) -> Type{
        match ident.as_str(){
            "i32" => return Type::I32,
            "i64" => return Type::I64,
            "f32" => return Type::F32,
            "f64" => return Type::F64,
            "Bool" => return Type::Bool,
            "String" => return Type::String,
            _=>{
                panic!("unexpected type: {}", ident)
            }
        }
    }
    fn parse_function_args(&self, tokens: &mut VecDeque<Token>) -> Vec<Expression>{
        let mut args = Vec::new();
        loop{
            let next_token = tokens.pop_front().unwrap();
            match next_token{
                Token::CloseParen => return args,
                _ => {
                    tokens.push_front(next_token);
                    args.push(self.parse_expression(tokens, None))
                }
            }
        }
    }
}
#[cfg(test)]
mod test {
    use super::{CompleteU, Statement, Type, UnaryOperator};
    use crate::{
        parse::{BinaryOperator, Complete, Expression},
        tokenize::{MathOp, Token}, compiler::Compiler,
    };
    use std::collections::VecDeque;
    #[test]
    fn hello_world() {
        let compiler = Compiler::new();
        let actual = compiler.parse(&mut VecDeque::from([
            Token::Identifier("print".to_string()),
            Token::OpenParen,
            Token::String("hello world".to_string()),
            Token::CloseParen,
            Token::EndLine,
        ]));
        let expected = vec![Statement::FunctionCall(
            "print".to_string(),
            vec![Expression::String("hello world".to_string())],
        )];
        assert_eq!(actual, expected);
    }
    #[test]
    fn bool_test() {
        let compiler = Compiler::new();
        let actual = compiler.parse(&mut VecDeque::from([
            Token::Identifier("Bool".to_string()),
            Token::Identifier("peepaw".to_string()),
            Token::Assign,
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
        let compiler = Compiler::new();
        let actual = compiler.parse(&mut VecDeque::from([
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
        ]));
        let expected = vec![
            Statement::DefineVariable("eee".to_string(), Expression::Bool(true), Type::Bool),
            Statement::FunctionCall(
                "print".to_string(),
                vec![Expression::Variable("eee".to_string())],
            ),
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn print_string_test() {
        let compiler = Compiler::new();
        let actual = compiler.parse(&mut VecDeque::from([
            Token::Identifier("String".to_string()),
            Token::Identifier("ee".to_string()),
            Token::Assign,
            Token::String("yeet?".to_string()),
            Token::EndLine,
            Token::Identifier("print".to_string()),
            Token::OpenParen,
            Token::Identifier("ee".to_string()),
            Token::CloseParen,
            Token::EndLine,
        ]));
        let expected = vec![
            Statement::DefineVariable(
                "ee".to_string(),
                Expression::String("yeet?".to_string()),
                Type::String,
            ),
            Statement::FunctionCall(
                "print".to_string(),
                vec![Expression::Variable("ee".to_string())],
            ),
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn simple_while_loop() {
        let compiler = Compiler::new();
        let actual = compiler.parse(&mut VecDeque::from([
            Token::WhileLoop,
            Token::OpenParen,
            Token::Boolean(true),
            Token::CloseParen,
            Token::OpenBlock,
            Token::Identifier("print".to_string()),
            Token::OpenParen,
            Token::String("69".to_string()),
            Token::CloseParen,
            Token::EndLine,
            Token::CloseBlock,
        ]));
        let expected = vec![Statement::WhileLoop(
            Expression::Bool(true),
            VecDeque::from([Statement::FunctionCall(
                "print".to_string(),
                vec![Expression::String("69".to_string())],
            )]),
        )];
        assert_eq!(actual, expected);
    }
    #[test]
    fn change_variable() {
        let compiler = Compiler::new();
        let actual = compiler.parse(&mut VecDeque::from([
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
        ]));
        let expected = vec![
            Statement::DefineVariable("i".to_string(), Expression::I32(0), Type::I32),
            Statement::ModifyVariable("i".to_string(), Expression::I32(1)),
            Statement::DefineVariable(
                "e".to_string(),
                Expression::String("hello".to_string()),
                Type::String,
            ),
            Statement::ModifyVariable("e".to_string(), Expression::String("bye".to_string())),
            Statement::DefineVariable("yes".to_string(), Expression::Bool(true), Type::Bool),
            Statement::ModifyVariable("yes".to_string(), Expression::Bool(false)),
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn math_test() {
        let compiler = Compiler::new();
        let actual = compiler.parse(&mut VecDeque::from([
            Token::Identifier("i32".to_string()),
            Token::Identifier("e".to_string()),
            Token::Assign,
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
        let compiler = Compiler::new();
        let actual = compiler.parse(&mut VecDeque::from([
            Token::Identifier("i32".to_string()),
            Token::Identifier("e".to_string()),
            Token::Assign,
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
        let compiler = Compiler::new();
        let actual = compiler.parse(&mut VecDeque::from([
            Token::Identifier("print".to_string()),
            Token::OpenParen,
            Token::ConstantNumber("1".to_string()),
            Token::MathOp(MathOp::Add),
            Token::ConstantNumber("69".to_string()),
            Token::CloseParen,
            Token::EndLine,
        ]));
        let expected = vec![Statement::FunctionCall(
            "print".to_string(),
            vec![Expression::Complete(Complete {
                operator: BinaryOperator::Add,
                left: Box::new(Expression::I32(1)),
                right: Box::new(Expression::I32(69)),
            })],
        )];
        assert_eq!(actual, expected);
    }
    #[test]
    fn variable_adding() {
        let compiler = Compiler::new();
        let actual = compiler.parse(&mut VecDeque::from([
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
        ]));
        let expected = vec![
            Statement::DefineVariable("e".to_string(), Expression::I32(1), Type::I32),
            Statement::DefineVariable("ee".to_string(), Expression::I32(2), Type::I32),
            Statement::FunctionCall(
                "print".to_string(),
                vec![Expression::Complete(Complete {
                    operator: BinaryOperator::Add,
                    left: Box::new(Expression::Variable("e".to_string())),
                    right: Box::new(Expression::Variable("ee".to_string())),
                })],
            ),
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn basic_if() {
        let compiler = Compiler::new();
        let actual = compiler.parse(&mut VecDeque::from([
            Token::Identifier("i32".to_string()),
            Token::Identifier("e".to_string()),
            Token::Assign,
            Token::ConstantNumber("69".to_string()),
            Token::EndLine,
            Token::If,
            Token::OpenParen,
            Token::Identifier("e".to_string()),
            Token::MathOp(MathOp::Equals),
            Token::ConstantNumber("69".to_string()),
            Token::CloseParen,
            Token::OpenBlock,
            Token::Identifier("print".to_string()),
            Token::OpenParen,
            Token::Identifier("e".to_string()),
            Token::CloseParen,
            Token::EndLine,
            Token::CloseBlock,
        ]));
        let expected = vec![
            Statement::DefineVariable("e".to_string(), Expression::I32(69), Type::I32),
            Statement::If(
                Expression::Complete(Complete {
                    operator: BinaryOperator::Equals,
                    left: Box::new(Expression::Variable("e".to_string())),
                    right: Box::new(Expression::I32(69)),
                }),
                VecDeque::from([Statement::FunctionCall(
                    "print".to_string(),
                    vec![Expression::Variable("e".to_string())],
                )]),
                VecDeque::new(),
                None,
            ),
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn for_loop() {
        let compiler = Compiler::new();
        let actual = compiler.parse(&mut VecDeque::from([
            Token::ForLoop,
            Token::OpenParen,
            Token::Identifier("i32".to_string()),
            Token::Identifier("i".to_string()),
            Token::Assign,
            Token::ConstantNumber("0".to_string()),
            Token::EndLine,
            Token::Identifier("i".to_string()),
            Token::MathOp(MathOp::LessThan),
            Token::ConstantNumber("10".to_string()),
            Token::EndLine,
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
        ]));
        let expected = vec![Statement::ForLoop(
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
            VecDeque::from([Statement::FunctionCall(
                "print".to_string(),
                vec![Expression::Variable("i".to_string())],
            )]),
        )];
        assert_eq!(actual, expected);
    }
    #[test]
    fn double_if() {
        let compiler = Compiler::new();
        let actual = compiler.parse(&mut VecDeque::from([
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
        ]));
        let expected = vec![Statement::If(
            Expression::Bool(true),
            VecDeque::from([Statement::If(
                Expression::Bool(false),
                VecDeque::from([Statement::FunctionCall(
                    "print".to_string(),
                    vec![Expression::String("a".to_string())],
                )]),
                VecDeque::new(),
                None,
            )]),
            VecDeque::new(),
            None,
        )];
        assert_eq!(actual, expected);
    }
    #[test]
    fn more_if() {
        let compiler = Compiler::new();
        let actual = compiler.parse(&mut VecDeque::from([
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
            Token::If,
            Token::OpenParen,
            Token::Boolean(true),
            Token::CloseParen,
            Token::OpenBlock,
            Token::Identifier("print".to_string()),
            Token::OpenParen,
            Token::String("n".to_string()),
            Token::CloseParen,
            Token::EndLine,
            Token::CloseBlock,
            Token::CloseBlock,
        ]));
        let expected = vec![Statement::If(
            Expression::Bool(true),
            VecDeque::from([
                Statement::If(
                    Expression::Bool(false),
                    VecDeque::from([Statement::FunctionCall(
                        "print".to_string(),
                        vec![Expression::String("a".to_string())],
                    )]),
                    VecDeque::new(),
                    None,
                ),
                Statement::If(
                    Expression::Bool(true),
                    VecDeque::from([Statement::FunctionCall(
                        "print".to_string(),
                        vec![Expression::String("n".to_string())],
                    )]),
                    VecDeque::new(),
                    None,
                ),
            ]),
            VecDeque::new(),
            None,
        )];
        assert_eq!(actual, expected);
    }
    #[test]
    fn increment_test() {
        let compiler = Compiler::new();
        let actual = compiler.parse(&mut VecDeque::from([
            Token::Identifier("i32".to_string()),
            Token::Identifier("w".to_string()),
            Token::Assign,
            Token::ConstantNumber("68".to_string()),
            Token::EndLine,
            Token::Identifier("w".to_string()),
            Token::Decrement,
            Token::EndLine,
        ]));
        let expected = vec![
            Statement::DefineVariable("w".to_string(), Expression::I32(68), Type::I32),
            Statement::ModifyVariable("w".to_string(), Expression::Decrement),
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn if_elif_elif_else() {
        let compiler = Compiler::new();
        let actual = compiler.parse(&mut VecDeque::from([
            Token::Identifier("i32".to_string()),
            Token::Identifier("i".to_string()),
            Token::Assign,
            Token::ConstantNumber("69".to_string()),
            Token::EndLine,
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
            Token::Elif,
            Token::OpenParen,
            Token::Identifier("i".to_string()),
            Token::MathOp(MathOp::Equals),
            Token::ConstantNumber("69".to_string()),
            Token::CloseParen,
            Token::OpenBlock,
            Token::CloseBlock,
            Token::Else,
            Token::OpenBlock,
            Token::CloseBlock,
        ]));
        let expected = vec![
            Statement::DefineVariable("i".to_string(), Expression::I32(69), Type::I32),
            Statement::If(
                Expression::Complete(Complete {
                    operator: BinaryOperator::Equals,
                    left: Box::new(Expression::Variable("i".to_string())),
                    right: Box::new(Expression::I32(6)),
                }),
                VecDeque::from([]),
                VecDeque::from([
                    Statement::Elif(
                        Expression::Complete(Complete {
                            operator: BinaryOperator::Equals,
                            left: Box::new(Expression::Variable("i".to_string())),
                            right: Box::new(Expression::I32(7)),
                        }),
                        VecDeque::from([]),
                    ),
                    Statement::Elif(
                        Expression::Complete(Complete {
                            operator: BinaryOperator::Equals,
                            left: Box::new(Expression::Variable("i".to_string())),
                            right: Box::new(Expression::I32(69)),
                        }),
                        VecDeque::from([]),
                    ),
                ]),
                Some(VecDeque::new()),
            ),
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn i32_i64_f32_f64() {
        let compiler = Compiler::new();
        let actual = compiler.parse(&mut VecDeque::from([
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
        ]));
        let expected = vec![
            Statement::DefineVariable("i".to_string(), Expression::I32(31), Type::I32),
            Statement::DefineVariable("e".to_string(), Expression::I64(63), Type::I64),
            Statement::DefineVariable("f".to_string(), Expression::F32(32.0), Type::F32),
            Statement::DefineVariable("g".to_string(), Expression::F64(64.0), Type::F64),
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn f32_test() {
        let compiler = Compiler::new();
        let actual = compiler.parse(&mut VecDeque::from([
            Token::Identifier("f32".to_string()),
            Token::Identifier("e".to_string()),
            Token::Assign,
            Token::ConstantNumber("32".to_string()),
            Token::EndLine,
        ]));
        let expected = vec![Statement::DefineVariable(
            "e".to_string(),
            Expression::F32(32.0),
            Type::F32,
        )];
        assert_eq!(actual, expected);
    }
    #[test]
    fn simple_cast() {
        let compiler = Compiler::new();
        let actual = compiler.parse(&mut VecDeque::from([
            Token::Identifier("i32".to_string()),
            Token::Identifier("i".to_string()),
            Token::Assign,
            Token::ConstantNumber("6".to_string()),
            Token::EndLine,
            Token::Identifier("i64".to_string()),
            Token::Identifier("e".to_string()),
            Token::Assign,
            Token::Identifier("i64".to_string()),
            Token::OpenParen,
            Token::Identifier("i".to_string()),
            Token::CloseParen,
            Token::EndLine,
        ]));
        let expected = vec![
            Statement::DefineVariable("i".to_string(), Expression::I32(6), Type::I32),
            Statement::DefineVariable(
                "e".to_string(),
                Expression::FunctionCall(
                    "i64".to_string(),
                    vec![Expression::Variable("i".to_string())],
                ),
                Type::I64,
            ),
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn cast_in_while() {
        let compiler = Compiler::new();
        let actual = compiler.parse(&mut VecDeque::from([
            Token::Identifier("i64".to_string()),
            Token::Identifier("i".to_string()),
            Token::Assign,
            Token::ConstantNumber("6".to_string()),
            Token::EndLine,
            Token::WhileLoop,
            Token::OpenParen,
            Token::Identifier("i".to_string()),
            Token::MathOp(MathOp::NotEqual),
            Token::Identifier("i64".to_string()),
            Token::OpenParen,
            Token::ConstantNumber("2".to_string()),
            Token::CloseParen,
            Token::CloseParen,
            Token::OpenBlock,
            Token::CloseBlock,
        ]));
        let expected = vec![
            Statement::DefineVariable("i".to_string(), Expression::I64(6), Type::I64),
            Statement::WhileLoop(
                Expression::Complete(Complete {
                    operator: BinaryOperator::NotEqual,
                    left: Box::new(Expression::Variable("i".to_string())),
                    right: Box::new(Expression::FunctionCall(
                        "i64".to_string(),
                        vec![Expression::I32(2)],
                    )),
                }),
                VecDeque::new(),
            ),
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn statement_after_loop() {
        let compiler = Compiler::new();
        let actual = compiler.parse(&mut VecDeque::from([
            Token::Identifier("i64".to_string()),
            Token::Identifier("i".to_string()),
            Token::Assign,
            Token::ConstantNumber("9".to_string()),
            Token::EndLine,
            Token::WhileLoop,
            Token::OpenParen,
            Token::Boolean(false),
            Token::CloseParen,
            Token::OpenBlock,
            Token::CloseBlock,
            Token::Identifier("print".to_string()),
            Token::OpenParen,
            Token::Identifier("i".to_string()),
            Token::CloseParen,
            Token::EndLine,
        ]));
        let expected = vec![
            Statement::DefineVariable("i".to_string(), Expression::I64(9), Type::I64),
            Statement::WhileLoop(Expression::Bool(false), VecDeque::new()),
            Statement::FunctionCall(
                "print".to_string(),
                vec![Expression::Variable("i".to_string())],
            ),
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn order_of_ops_parenthesis_1() {
        let mut tokens = VecDeque::from([
            Token::OpenParen,
            Token::ConstantNumber("1".to_string()),
            Token::MathOp(MathOp::Add),
            Token::ConstantNumber("2".to_string()),
            Token::CloseParen,
            Token::MathOp(MathOp::Multiply),
            Token::ConstantNumber("3".to_string()),
        ]);
        let compiler = Compiler::new();
        let actual = compiler.parse_expression(&mut tokens, Some(Type::I32));
        let expected = Expression::Complete(Complete {
            operator: BinaryOperator::Multiply,
            left: Box::new(Expression::CompleteU(CompleteU {
                operator: UnaryOperator::Parenthesis,
                child: Box::new(Expression::Complete(Complete {
                    operator: BinaryOperator::Add,
                    left: Box::new(Expression::I32(1)),
                    right: Box::new(Expression::I32(2)),
                })),
            })),
            right: Box::new(Expression::I32(3)),
        });
        assert_eq!(actual, expected);
    }
    #[test]
    fn order_of_ops_parenthesis_2() {
        let mut tokens = VecDeque::from([
            Token::ConstantNumber("3".to_string()),
            Token::MathOp(MathOp::Multiply),
            Token::OpenParen,
            Token::ConstantNumber("2".to_string()),
            Token::MathOp(MathOp::Add),
            Token::ConstantNumber("6".to_string()),
            Token::CloseParen,
        ]);
        let compiler = Compiler::new();
        let actual = compiler.parse_expression(&mut tokens, Some(Type::I32));
        let expected = Expression::Complete(Complete {
            operator: BinaryOperator::Multiply,
            left: Box::new(Expression::I32(3)),
            right: Box::new(Expression::CompleteU(CompleteU {
                operator: UnaryOperator::Parenthesis,
                child: Box::new(Expression::Complete(Complete {
                    operator: BinaryOperator::Add,
                    left: Box::new(Expression::I32(2)),
                    right: Box::new(Expression::I32(6)),
                })),
            })),
        });
        assert_eq!(actual, expected);
    }
    #[test]
    fn complex_print_1() {
        let compiler = Compiler::new();
        let actual = compiler.parse(&mut VecDeque::from([
            Token::Identifier("print".to_string()),
            Token::OpenParen,
            Token::OpenParen,
            Token::ConstantNumber("1".to_string()),
            Token::MathOp(MathOp::Add),
            Token::ConstantNumber("2".to_string()),
            Token::CloseParen,
            Token::MathOp(MathOp::Multiply),
            Token::ConstantNumber("3".to_string()),
            Token::CloseParen,
            Token::EndLine,
        ]));
        let expected = vec![Statement::FunctionCall(
            "print".to_string(),
            vec![Expression::Complete(Complete {
                operator: BinaryOperator::Multiply,
                left: Box::new(Expression::CompleteU(CompleteU {
                    operator: UnaryOperator::Parenthesis,
                    child: Box::new(Expression::Complete(Complete {
                        operator: BinaryOperator::Add,
                        left: Box::new(Expression::I32(1)),
                        right: Box::new(Expression::I32(2)),
                    })),
                })),
                right: Box::new(Expression::I32(3)),
            })],
        )];
        assert_eq!(actual, expected);
    }
    #[test]
    fn boolean_operators() {
        let compiler = Compiler::new();
        let actual = compiler.parse(&mut VecDeque::from([
            Token::Identifier("print".to_string()),
            Token::OpenParen,
            Token::Boolean(false),
            Token::MathOp(MathOp::And),
            Token::Boolean(true),
            Token::CloseParen,
            Token::EndLine,
            Token::Identifier("print".to_string()),
            Token::OpenParen,
            Token::Boolean(false),
            Token::MathOp(MathOp::Or),
            Token::Boolean(true),
            Token::CloseParen,
            Token::EndLine,
            Token::Identifier("print".to_string()),
            Token::OpenParen,
            Token::MathOp(MathOp::Not),
            Token::Boolean(false),
            Token::MathOp(MathOp::And),
            Token::Boolean(true),
            Token::CloseParen,
            Token::EndLine,
        ]));
        let expected = vec![
            Statement::FunctionCall(
                "print".to_string(),
                vec![Expression::Complete(Complete {
                    operator: BinaryOperator::And,
                    left: Box::new(Expression::Bool(false)),
                    right: Box::new(Expression::Bool(true)),
                })],
            ),
            Statement::FunctionCall(
                "print".to_string(),
                vec![Expression::Complete(Complete {
                    operator: BinaryOperator::Or,
                    left: Box::new(Expression::Bool(false)),
                    right: Box::new(Expression::Bool(true)),
                })],
            ),
            Statement::FunctionCall(
                "print".to_string(),
                vec![Expression::Complete(Complete {
                    operator: BinaryOperator::And,
                    left: Box::new(Expression::CompleteU(CompleteU {
                        operator: UnaryOperator::Not,
                        child: Box::new(Expression::Bool(false)),
                    })),
                    right: Box::new(Expression::Bool(true)),
                })],
            ),
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn one_dim_array() {
        let compiler = Compiler::new();
        let actual = compiler.parse(&mut VecDeque::from([
            Token::Identifier("Array<i32>".to_string()),
            Token::Identifier("a".to_string()),
            Token::Assign,
            Token::OpenBracket,
            Token::ConstantNumber("69".to_string()),
            Token::Comma,
            Token::ConstantNumber("420".to_string()),
            Token::CloseBracket,
            Token::EndLine,
        ]));
        let expected = vec![Statement::DefineVariable(
            "a".to_string(),
            Expression::Array(vec![Expression::F32(69.0), Expression::F32(420.0)]),
            Type::F32,
        )];
        assert_eq!(actual, expected);
    }
    #[test]
    fn empty_one_dim_array() {
        let compiler = Compiler::new();
        let actual = compiler.parse(&mut VecDeque::from([
            Token::Identifier("Array<String>".to_string()),
            Token::Identifier("a".to_string()),
            Token::Assign,
            Token::OpenBracket,
            Token::CloseBracket,
            Token::EndLine,
        ]));
        let expected = vec![Statement::DefineVariable(
            "a".to_string(),
            Expression::Array(vec![]),
            Type::String,
        )];
        assert_eq!(actual, expected);
    }
}
