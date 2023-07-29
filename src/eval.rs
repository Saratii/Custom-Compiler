use std::collections::HashMap;

use crate::lex::{Expression, Line, Type};

pub fn evaluate(lines: Vec<Line>) {
    let mut variables = HashMap::new();
    for i in 0..lines.len(){
        evaluate_line(&lines[i as usize], &mut variables, &lines);
    }
}

fn evaluate_line(line: &Line, variables: &mut HashMap<String, (Expression, Type)>, lines: &Vec<Line>){
    match line {
        Line::Print(expression) => match expression {
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
            _ => {}
        },
        Line::DefineVariable(name, value, variable_type) => {
            variables.insert(name.clone(), (value.clone(), variable_type.clone()));
        }
        Line::WhileLoop(condition, lines) => {
            match condition{
                Expression::Bool(value) => {
                    while *value{
                        for line in lines{
                            evaluate_line(line, variables, lines);
                        }
                    }
                }
                _ => {}
            }
        }
        _ => {}
    }
    
}