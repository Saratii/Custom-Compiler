use std::collections::HashMap;

use crate::{lex::{Expression, Line, Type, Complete, BinaryOperator}, tokenize::Token};

pub fn evaluate(lines: Vec<Line>) {
    let mut variables = HashMap::new();
    for i in 0..lines.len(){
        evaluate_line(&lines[i as usize], &mut variables);
    }
}

pub fn evaluate_line(line: &Line, variables: &mut HashMap<String, (Expression, Type)>){
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
            Expression::Complete(complete) => {
                let val = complete.evaluate(variables);
                match val{
                    Expression::I32(val) => println!("{}", val),
                    _ => println!("yaw {:?}", val)
                }
                
            }
            _ => {}
        },
        Line::DefineVariable(name, value, variable_type) => {
            variables.insert(name.clone(), (value.clone().evaluate(variables), variable_type.clone()));
        }
        Line::WhileLoop(condition, lines) => {
            match condition{
                Expression::Bool(value) => {
                    while *value{
                        for line in lines{
                            evaluate_line(line, variables);
                        }
                    }
                }
                _ => {}
            }
        }
        _ => {}
    }
    
}
impl Complete{
    fn evaluate(&self, variables: &HashMap<String, (Expression, Type)>) -> Expression{
        match (self.left.evaluate(variables), self.right.evaluate(variables)){
            (Expression::I32(left), Expression::I32(right)) => {
                match self.operator{
                    BinaryOperator::Add => Expression::I32((left.parse::<i32>().unwrap() + right.parse::<i32>().unwrap()).to_string()),
                    BinaryOperator::Subtract => Expression::I32((left.parse::<i32>().unwrap() - right.parse::<i32>().unwrap()).to_string()),
                    BinaryOperator::Multiply => Expression::I32((left.parse::<i32>().unwrap() * right.parse::<i32>().unwrap()).to_string()),
                    BinaryOperator::Divide => Expression::I32((left.parse::<i32>().unwrap() / right.parse::<i32>().unwrap()).to_string()),
                }
            }
            (a, b) => {
                panic!("learn how to fucking program, {:?} {:?}", a, b);
            }
        }
    }   
}
impl Expression{
    fn evaluate(&self, variables: &HashMap<String, (Expression, Type)>) -> Expression{
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
            // Expression::CompleteU(_) => panic!("eh"),
            // Expression::IncompleteU(_) => panic!("compiler done fucked up"),
        }
    }   
}