use colored::Colorize;

use crate::parse::{
    BinaryOperator, Complete, CompleteU, Expression, Function, Statement, Type, UnaryOperator,
};
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
        (
            "print()".to_string(),
            Function {
                name: "print()".to_string(),
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
        Statement::FunctionCall(name, args) => {
            if name == "print()" {
                println!("{}", args[0].evaluate(variables));
            }
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
        Statement::ModifyVariable(name, expression) => {
            let literal = expression.evaluate(variables);
            match variables.get_mut(name) {
                Some(tuple) => {
                    let ty = match literal {
                        Primitive::Bool(_) => Type::Bool,
                        Primitive::I32(_) => Type::I32,
                        Primitive::String(_) => Type::String,
                        Primitive::F32(_) => Type::F32,
                        Primitive::I64(_) => Type::I64,
                        Primitive::F64(_) => Type::F64,
                    };
                    *tuple = (literal, ty);
                }
                None => {
                    let error_message = format!("ST:NAME ERROR -> name: {} does not exist", name);
                    panic!("{}", error_message.purple());
                }
            }
        }
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
        _ => {
            panic!("compiler found unexpected statement {:?}", statement)
        }
    }
}
impl CompleteU {
    fn evaluate(&self, variables: &HashMap<String, (Primitive, Type)>) -> Primitive {
        match (self.child.evaluate(variables), &self.operator) {
            (Primitive::Bool(value), UnaryOperator::Not) => Primitive::Bool(!value),
            (Primitive::I32(value), UnaryOperator::Parenthesis) => Primitive::I32(value),
            (Primitive::F64(value), UnaryOperator::Parenthesis) => Primitive::F64(value),
            (Primitive::I64(value), UnaryOperator::Parenthesis) => Primitive::I64(value),
            (Primitive::F32(value), UnaryOperator::Parenthesis) => Primitive::F32(value),
            (Primitive::String(value), UnaryOperator::Parenthesis) => Primitive::String(value),
            (Primitive::Bool(value), UnaryOperator::Parenthesis) => Primitive::Bool(value),
            _ => {
                let error_message = format!(
                    "ST: MISMATCHED-TYPES -> Operator {:?} is not defined for {:?}",
                    self.operator, self.child
                );
                panic!("{}", error_message.purple())
            }
        }
    }
}
impl Complete {
    fn evaluate(&self, variables: &HashMap<String, (Primitive, Type)>) -> Primitive {
        match (
            self.left.evaluate(variables),
            self.right.evaluate(variables),
        ) {
            (Primitive::Bool(left), Primitive::Bool(right)) => {
                match self.operator {
                    BinaryOperator::Or => Primitive::Bool(left || right),
                    BinaryOperator::And => Primitive::Bool(left && right),
                    _ => {
                        let error_message = format!("ST: MISMATCHED-TYPES -> Operator {:?} is not defined for bool and bool", self.operator);
                        panic!("{}", error_message.purple())
                    }
                }
            }
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
                _ => {
                    let error_message = format!(
                        "ST: MISMATCHED-TYPES -> Operator {:?} is not defined for i32 and i32",
                        self.operator
                    );
                    panic!("{}", error_message.purple())
                }
            },
            (Primitive::F32(left), Primitive::F32(right)) => match self.operator {
                BinaryOperator::Add => Primitive::F32(left + right),
                BinaryOperator::Subtract => Primitive::F32(left - right),
                BinaryOperator::Multiply => Primitive::F32(left * right),
                BinaryOperator::Divide => Primitive::F32(left / right),
                BinaryOperator::Equals => Primitive::Bool(left == right),
                BinaryOperator::LessThan => Primitive::Bool(left < right),
                BinaryOperator::LessThanOrEqualTo => Primitive::Bool(left <= right),
                BinaryOperator::GreaterThan => Primitive::Bool(left > right),
                BinaryOperator::GreaterThanOrEqualTo => Primitive::Bool(left >= right),
                BinaryOperator::Modulus => Primitive::F32(left % right),
                BinaryOperator::NotEqual => Primitive::Bool(left != right),
                _ => {
                    let error_message = format!(
                        "ST: MISMATCHED-TYPES -> Operator {:?} is not defined for i32 and i32",
                        self.operator
                    );
                    panic!("{}", error_message.purple())
                }
            },
            (Primitive::I64(left), Primitive::I64(right)) => match self.operator {
                BinaryOperator::Add => Primitive::I64(left + right),
                BinaryOperator::Subtract => Primitive::I64(left - right),
                BinaryOperator::Multiply => Primitive::I64(left * right),
                BinaryOperator::Divide => Primitive::I64(left / right),
                BinaryOperator::Equals => Primitive::Bool(left == right),
                BinaryOperator::LessThan => Primitive::Bool(left < right),
                BinaryOperator::LessThanOrEqualTo => Primitive::Bool(left <= right),
                BinaryOperator::GreaterThan => Primitive::Bool(left > right),
                BinaryOperator::GreaterThanOrEqualTo => Primitive::Bool(left >= right),
                BinaryOperator::Modulus => Primitive::I64(left % right),
                BinaryOperator::NotEqual => Primitive::Bool(left != right),
                _ => {
                    let error_message = format!(
                        "ST: MISMATCHED-TYPES -> Operator {:?} is not defined for i32 and i32",
                        self.operator
                    );
                    panic!("{}", error_message.purple())
                }
            },
            (Primitive::F64(left), Primitive::F64(right)) => match self.operator {
                BinaryOperator::Add => Primitive::F64(left + right),
                BinaryOperator::Subtract => Primitive::F64(left - right),
                BinaryOperator::Multiply => Primitive::F64(left * right),
                BinaryOperator::Divide => Primitive::F64(left / right),
                BinaryOperator::Equals => Primitive::Bool(left == right),
                BinaryOperator::LessThan => Primitive::Bool(left < right),
                BinaryOperator::LessThanOrEqualTo => Primitive::Bool(left <= right),
                BinaryOperator::GreaterThan => Primitive::Bool(left > right),
                BinaryOperator::GreaterThanOrEqualTo => Primitive::Bool(left >= right),
                BinaryOperator::Modulus => Primitive::F64(left % right),
                BinaryOperator::NotEqual => Primitive::Bool(left != right),
                _ => {
                    let error_message = format!(
                        "ST: MISMATCHED-TYPES -> Operator {:?} is not defined for i32 and i32",
                        self.operator
                    );
                    panic!("{}", error_message.purple())
                }
            },
            (Primitive::I64(left), Primitive::I32(right)) => match self.operator {
                BinaryOperator::Equals => Primitive::Bool(left == right as i64),
                BinaryOperator::LessThan => Primitive::Bool(left < right as i64),
                BinaryOperator::LessThanOrEqualTo => Primitive::Bool(left <= right as i64),
                BinaryOperator::GreaterThan => Primitive::Bool(left > right as i64),
                BinaryOperator::GreaterThanOrEqualTo => Primitive::Bool(left >= right as i64),
                BinaryOperator::NotEqual => Primitive::Bool(left != right as i64),
                _ => {
                    let error_message = format!(
                        "ST: MISMATCHED-TYPES -> Operator {:?} is not defined for i32 and i32",
                        self.operator
                    );
                    panic!("{}", error_message.purple())
                }
            },
            (a, b) => {
                let error_message = format!(
                    "ST: MISMATCHED-TYPES -> Operator {:?} is not defined for {} and {}",
                    self.operator, a, b
                );
                panic!("{}", error_message.purple())
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
            Expression::CompleteU(complete_u) => complete_u.evaluate(variables),
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
            _ => {
                panic!("compiler did an oopsie");
            }
        }
    }
}
