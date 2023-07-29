use std::collections::HashMap;

use crate::lex::{Expression, Line};

pub fn evaluate(lines: Vec<Line>) {
    let mut variables = HashMap::new();
    for line in lines {
        match line {
            Line::Print(expression) => match expression {
                Expression::String(s) => {
                    println!("{}", s)
                }
                Expression::Variable(name) => {
                    let (expression, _) = variables.get(&name).unwrap();
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
                _ => {}
            },
            Line::DefineVariable(name, expression, variable_type) => {
                variables.insert(name, (expression, variable_type));
            }
            _ => {}
        }
    }
}
