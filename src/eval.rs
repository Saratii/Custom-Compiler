use colored::Colorize;

use crate::parse::{BinaryOperator, Complete, Expression, Function, Statement, Type};
use std::collections::{HashMap, VecDeque};
#[derive(PartialEq, Debug, Clone)]
pub enum Primitive {
    String(String),
    I32(i32),
    F32(f32),
    I64(i64),
    F64(f64),
    Bool(bool),
}
impl std::fmt::Display for Primitive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Primitive::I32(value) => write!(f, "{}", value),
            Primitive::F32(value) => write!(f, "{}", value),
            Primitive::String(value) => write!(f, "{}", value),
            Primitive::I64(value) => write!(f, "{}", value),
            Primitive::F64(value) => write!(f, "{}", value),
            Primitive::Bool(value) => write!(f, "{}", value),
        }
    }
}
pub fn evaluate(lines: VecDeque<Statement>) {
    let mut variables = HashMap::new();
    let function_map = HashMap::from([
        (
            "i32()".to_string(),
            Function {
                name: "i32()".to_string(),
                block: VecDeque::new(),
            },
        ),
        (
            "f32()".to_string(),
            Function {
                name: "f32()".to_string(),
                block: VecDeque::new(),
            },
        ),
        (
            "i64()".to_string(),
            Function {
                name: "i64()".to_string(),
                block: VecDeque::new(),
            },
        ),
        (
            "f64()".to_string(),
            Function {
                name: "f64()".to_string(),
                block: VecDeque::new(),
            },
        ),
        (
            "string()".to_string(),
            Function {
                name: "string()".to_string(),
                block: VecDeque::new(),
            },
        ),
    ]);
    for i in 0..lines.len() {
        evaluate_line(&lines[i as usize], &mut variables, &function_map);
    }
}

pub fn evaluate_line(
    statement: &Statement,
    variables: &mut HashMap<String, (Primitive, Type)>,
    function_map: &HashMap<String, Function>,
) {
    match statement {
        Statement::Print(expression) => {
            println!("{}", expression.evaluate(variables));
        }
        Statement::DefineVariable(name, value, variable_type) => {
            variables.insert(
                name.clone(),
                (value.clone().evaluate(variables), variable_type.clone()),
            );
        }
        Statement::WhileLoop(condition, lines) => {
            let mut literal_condition = condition.evaluate(variables);
            match literal_condition {
                Primitive::Bool(mut value) => {
                    while value {
                        for statement in lines {
                            evaluate_line(statement, variables, function_map);
                        }
                        literal_condition = condition.evaluate(variables);
                        match literal_condition {
                            Primitive::Bool(val) => value = val,
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
        Statement::If(condition, statements, elifs, else_) => match condition.evaluate(variables) {
            Primitive::Bool(literal) => {
                if literal {
                    for statement in statements {
                        evaluate_line(statement, variables, function_map);
                    }
                } else {
                    'break_when_found: for elif in elifs {
                        match elif {
                            Statement::Elif(elif_condition, elif_block) => {
                                match elif_condition.evaluate(&variables) {
                                    Primitive::Bool(elif_literal) => {
                                        if elif_literal {
                                            for statement in elif_block {
                                                evaluate_line(statement, variables, &function_map);
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
                    for statement in else_.clone().unwrap() {
                        evaluate_line(&statement, variables, &function_map);
                    }
                }
            }
            _ => panic!("compiler made an oopsie woopsie"),
        },
        Statement::ForLoop(define_variable, condition, increment, lines) => {
            evaluate_line(define_variable, variables, function_map);
            let mut evaluated_condition = condition.evaluate(variables);
            match evaluated_condition {
                Primitive::Bool(mut value) => {
                    while value {
                        for statement in lines {
                            evaluate_line(statement, variables, function_map);
                        }
                        evaluate_line(&**&increment, variables, &function_map);
                        evaluated_condition = condition.evaluate(variables);
                        match evaluated_condition {
                            Primitive::Bool(updated_value) => value = updated_value,
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
    fn evaluate(&self, variables: &HashMap<String, (Primitive, Type)>) -> Primitive {
        match (
            self.left.evaluate(variables),
            self.right.evaluate(variables),
        ) {
            (Primitive::I32(left), Primitive::I32(right)) => match self.operator {
                BinaryOperator::Add => Primitive::I32(left + right),
                BinaryOperator::Subtract => Primitive::I32(left - right),
                BinaryOperator::Multiply => Primitive::I32(left * right),
                BinaryOperator::Divide => Primitive::I32(left / right),
                BinaryOperator::Equals => Primitive::Bool(left == right),
                BinaryOperator::LessThan => Primitive::Bool(left < right),
                BinaryOperator::LessThanOrEqualTo => Primitive::Bool(left <= right),
                BinaryOperator::GreaterThan => Primitive::Bool(left > right),
                BinaryOperator::GreaterThanOrEqualTo => Primitive::Bool(left >= right),
                BinaryOperator::Modulus => Primitive::I32(left % right),
                BinaryOperator::NotEqual => Primitive::Bool(left != right),
            },
            (a, b) => {
                let error_message = format!("\nST:TYPE ERROR types {:?}, {:?}\n", a, b);
                panic!("{}", error_message.purple());
            }
        }
    }
}
impl Expression {
    fn evaluate(&self, variables: &HashMap<String, (Primitive, Type)>) -> Primitive {
        match self {
            Expression::String(value) => Primitive::String(value.clone()),
            Expression::Bool(value) => Primitive::Bool(*value),
            Expression::Variable(name) => match variables.get(name) {
                Some((value, _)) => value.clone(),
                None => {
                    let error_message = format!("ST:DNE -> Variable name: {} does not exist", name);
                    panic!("{}", error_message.purple());
                }
            },
            Expression::Complete(complete) => complete.evaluate(variables),
            Expression::BinaryOperator(_) => panic!("compiler done fucked up"),
            Expression::Increment => todo!(),
            Expression::Decrement => todo!(),
            Expression::I32(value) => Primitive::I32(*value),
            Expression::I64(value) => Primitive::I64(*value),
            Expression::F32(value) => Primitive::F32(*value),
            Expression::F64(value) => Primitive::F64(*value),
            Expression::FunctionCall(name, args) => match args[0].evaluate(variables) {
                Primitive::I32(value) => {
                    if name == "i32()" {
                        return Primitive::I32(value as i32);
                    } else if name == "i64()" {
                        return Primitive::I64(value as i64);
                    } else if name == "f64()" {
                        return Primitive::F64(value as f64);
                    } else if name == "f32()" {
                        return Primitive::F32(value as f32);
                    } else if name == "string()" {
                        return Primitive::String(value.to_string());
                    } else {
                        let error_message =
                            format!("ST:NAME ERROR -> Function: {} does not exist", name);
                        panic!("{}", error_message.purple());
                    }
                }
                Primitive::F32(value) => {
                    if name == "i32()" {
                        return Primitive::I32(value as i32);
                    } else if name == "i64()" {
                        return Primitive::I64(value as i64);
                    } else if name == "f64()" {
                        return Primitive::F64(value as f64);
                    } else if name == "f32()" {
                        return Primitive::F32(value as f32);
                    } else if name == "string()" {
                        return Primitive::String(value.to_string());
                    } else {
                        let error_message =
                            format!("ST:NAME ERROR -> Function: {} does not exist", name);
                        panic!("{}", error_message.purple());
                    }
                }
                _ => {
                    panic!("yawwwwww")
                }
            },
            // Expression::CompleteU(_) => panic!("eh"),
            // Expression::IncompleteU(_) => panic!("compiler done fucked up"),
        }
    }
}
