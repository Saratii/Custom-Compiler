use crate::parse::{BinaryOperator, Complete, Expression, Statement, Type};
use std::collections::{HashMap, VecDeque};

pub fn evaluate(lines: VecDeque<Statement>) {
    let mut variables = HashMap::new();
    for i in 0..lines.len() {
        evaluate_line(&lines[i as usize], &mut variables);
    }
}

pub fn evaluate_line(statement: &Statement, variables: &mut HashMap<String, (Expression, Type)>) {
    match statement {
        Statement::Print(expression) => match expression {
            Expression::String(s) => {
                println!("{}", s)
            }
            Expression::Variable(name) => {
                let (expression, _) = variables.get(name).unwrap();
                match expression {
                    Expression::Bool(value) => {
                        println!("{:?}", value)
                    }
                    Expression::String(value) => {
                        println!("{}", value)
                    }
                    Expression::I32(value) => {
                        println!("{}", value)
                    }
                    _ => {}
                }
            }
            Expression::Complete(complete) => {
                let val = complete.evaluate(variables);
                match val {
                    Expression::I32(val) => println!("{}", val),
                    _ => println!("yaw {:?}", val),
                }
            }
            _ => {}
        },
        Statement::DefineVariable(name, value, variable_type) => {
            variables.insert(
                name.clone(),
                (value.clone().evaluate(variables), variable_type.clone()),
            );
        }
        Statement::WhileLoop(condition, lines) => {
            let mut literal_condition = condition.evaluate(variables);
            match literal_condition {
                Expression::Bool(mut value) => {
                    while value {
                        for statement in lines {
                            evaluate_line(statement, variables);
                        }
                        literal_condition = condition.evaluate(variables);
                        match literal_condition {
                            Expression::Bool(val) => value = val,
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
        Statement::If(condition, statements, elifs, else_) => {
            match condition.evaluate(variables) {
                Expression::Bool(literal) => {
                    if literal {
                        for statement in statements {
                            evaluate_line(statement, variables);
                        }
                    } else {
                        'break_when_found: for elif in elifs{
                            match elif{
                                Statement::Elif(elif_condition, elif_block) => {
                                    match elif_condition.evaluate(&variables){
                                        Expression::Bool(elif_literal) => {
                                            if elif_literal{
                                                for statement in elif_block{
                                                    evaluate_line(statement, variables);
                                                }
                                                break 'break_when_found;
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                                _ => {}
                            }
                        }
                        for statement in else_.clone().unwrap(){
                            evaluate_line(&statement, variables);
                        }
                    }
                }
                _ => panic!("compiler made an oopsie woopsie"),
            }
        }
        Statement::ForLoop(define_variable, condition, increment, lines) => {
            evaluate_line(define_variable, variables);
            let mut evaluated_condition = condition.evaluate(variables);
            match evaluated_condition {
                Expression::Bool(mut value) => {
                    while value {
                        for statement in lines {
                            evaluate_line(statement, variables);
                        }
                        evaluate_line(&**&increment, variables);
                        evaluated_condition = condition.evaluate(variables);
                        match evaluated_condition {
                            Expression::Bool(updated_value) => value = updated_value,
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
        _ => {}
    }
}
impl Complete {
    fn evaluate(&self, variables: &HashMap<String, (Expression, Type)>) -> Expression {
        match (
            self.left.evaluate(variables),
            self.right.evaluate(variables),
        ) {
            (Expression::I32(left), Expression::I32(right)) => match self.operator {
                BinaryOperator::Add => Expression::I32(left + right),
                BinaryOperator::Subtract => Expression::I32(left - right),
                BinaryOperator::Multiply => Expression::I32(left * right),
                BinaryOperator::Divide => Expression::I32(left / right),
                BinaryOperator::Equals => Expression::Bool(left == right),
                BinaryOperator::LessThan => Expression::Bool(left < right),
                BinaryOperator::LessThanOrEqualTo => Expression::Bool(left <= right),
                BinaryOperator::GreaterThan => Expression::Bool(left > right),
                BinaryOperator::GreaterThanOrEqualTo => Expression::Bool(left >= right),
                BinaryOperator::Modulus => Expression::I32(left % right),
                BinaryOperator::NotEqual => Expression::Bool(left != right),
            },
            (a, b) => {
                panic!("learn how to fucking program, {:?} {:?}", a, b);
            }
        }
    }
}
impl Expression {
    fn evaluate(&self, variables: &HashMap<String, (Expression, Type)>) -> Expression {
        match self {
            Expression::String(_) => self.clone(),
            Expression::Bool(_) => self.clone(),
            Expression::Variable(name) => match variables.get(name) {
                Some((ex, _)) => ex.evaluate(variables),
                None => todo!(),
            },
            Expression::I32(_) => self.clone(),
            Expression::Complete(complete) => complete.evaluate(variables),
            Expression::BinaryOperator(_) => panic!("compiler done fucked up"),
            Expression::Increment => self.clone(),
            Expression::Decrement => self.clone()
            // Expression::CompleteU(_) => panic!("eh"),
            // Expression::IncompleteU(_) => panic!("compiler done fucked up"),
        }
    }
}
