use std::collections::{HashMap, VecDeque};

use crate::{compiler::Type, eval::Primitive, parse::{Expression, Statement}};

pub fn get_buffer(statements: &VecDeque<Statement>, variable_map: HashMap<String, (Primitive, Type)>) -> String{
    let mut llvm_statements = VecDeque::new();
    llvm_statements.push_back("define i32 @main() {\nentry:\n".to_string());
    let var_index: u32 = 0;
    for statement in statements{
        match statement{
            Statement::FunctionCall(name, args) => {
                if name == "print"{                    
                    add_variable_definitions(&args, var_index, &mut llvm_statements);
                    if !llvm_statements.contains(&"declare i32 @printf(i8*, ...)\n".to_string()){
                        llvm_statements.push_front("declare i32 @printf(i8*, ...)\n".to_string());
                    }
                    let arg_string = get_arguments_string(&args, var_index, &variable_map);
                    match &args[0]{
                        Expression::String(value) => {
                            llvm_statements.push_back(format!("call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([{} x i8], [{} x i8]* @{}, i32 0, i32 0))\n", value.len() + 1, value.len() + 1, arg_string))
                        },
                        Expression::Bool(_) => todo!(),
                        Expression::Variable(name) => {
                            match variable_map.get(name){
                                Some(variable_object) => {
                                    match variable_object.1{
                                        Type::Bool => todo!(),
                                        Type::String => {
                                            llvm_statements.push_back(format!("call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([{} x i8], [{} x i8]* @{}, i32 0, i32 0))\n", variable_object.0.len() + 1, variable_object.0.len() + 1, name))
                                        },
                                        Type::I32 => {
                                            llvm_statements.push_back(format!("call i32 (i8*, ...) @printf(i8* @var{}, i32 @{})\n", var_index, name))
                                        },
                                        Type::I64 => todo!(),
                                        Type::F32 => todo!(),
                                        Type::F64 => todo!(),
                                        Type::Array(_) => todo!(),
                                    }
                                },
                                None => todo!(),
                            }
                        },
                        Expression::I32(value) => {
                            llvm_statements.push_back(format!("call i32 (i8*, ...) @printf(i8* @var{}, i32 {})\n", var_index, value))

                        },
                        Expression::I64(_) => todo!(),
                        Expression::F32(_) => todo!(),
                        Expression::F64(_) => todo!(),
                        Expression::Array(_) => todo!(),
                        Expression::Complete(_) => todo!(),
                        Expression::BinaryOperator(_) => todo!(),
                        Expression::UnaryOperator(_) => todo!(),
                        Expression::Increment => todo!(),
                        Expression::Decrement => todo!(),
                        Expression::FunctionCall(_, _) => todo!(),
                        Expression::CompleteU(_) => todo!(),
                    }
                } else {
                    add_variable_definitions(&args, var_index, &mut llvm_statements);
                    llvm_statements.push_back(format!("call void @{}({})\n", name, get_arguments_string(args, var_index, &variable_map)));
                }
            },
            Statement::DefineVariable(name, expression, _) => {
                llvm_statements.push_front(get_define_variable_line(expression.clone(), &var_index, Some(name.to_string())))
            }
            _ => {
                
            }
        }
    }
    llvm_statements.push_back("ret i32 0\n}".to_string());
    let mut buffer = "".to_string();
    for statement in llvm_statements{
        buffer.push_str(&statement);
    }
    buffer
}

fn add_variable_definitions(args: &Vec<Expression>, var_index: u32, llvm_statements: &mut VecDeque<String>){
    for exp in args{
        match exp{
            Expression::String(_) => {
                llvm_statements.push_front(get_define_variable_line(exp.clone(), &var_index, None));
            },
            Expression::I32(_) => {
                llvm_statements.push_front(get_define_variable_line(exp.clone(), &var_index, None));
            }
            _ => {}
        }
    }
}

fn get_arguments_string(args: &Vec<Expression>, mut var_index: u32, variable_map: &HashMap<String, (Primitive, Type)>) -> String{
    let mut buffer = "".to_string();
    for exp in args{
        match exp{
            Expression::String(_) => {
                buffer.push_str(format!("var{}, ", var_index).as_str());
                var_index += 1;
            },
            Expression::Bool(_) => todo!(),
            Expression::Variable(name) => {
                match variable_map.get(name){
                    Some(var_object) => {
                        match var_object.1{
                            Type::Bool => todo!(),
                            Type::String => {
                                buffer.push_str(&format!("getelementptr inbounds ([{} x i8], [{} x i8]* @{}, i32 0, i32 0), ", var_object.0.len(), var_object.0.len(), name))
                            },
                            Type::I32 => buffer.push_str(&format!("")),
                            Type::I64 => todo!(),
                            Type::F32 => todo!(),
                            Type::F64 => todo!(),
                            Type::Array(_) => todo!(),
                        }
                    },
                    None => panic!("Undefined variable: {}", name),
                }
            },
            Expression::I32(value) => {
                buffer.push_str(&format!("@var{}, i32 {}, ", var_index, value))
            },
            Expression::I64(_) => todo!(),
            Expression::F32(_) => todo!(),
            Expression::F64(_) => todo!(),
            Expression::Array(_) => todo!(),
            Expression::Complete(_) => todo!(),
            Expression::BinaryOperator(_) => todo!(),
            Expression::UnaryOperator(_) => todo!(),
            Expression::Increment => todo!(),
            Expression::Decrement => todo!(),
            Expression::FunctionCall(_, _) => todo!(),
            Expression::CompleteU(_) => todo!(),
        }
    }
    if buffer.len() > 0{
        buffer.pop();
        buffer.pop();
    }
    buffer
}

fn get_define_variable_line(exp: Expression, var_index: &u32, name: Option<String>) -> String{
    match exp{
        Expression::String(value) => {
            match name{
                Some(name) => return format!("@{} = private unnamed_addr constant [{} x i8] c\"{}\\00\", align 1\n", name, value.len() + 1, value)
                ,
                None => return format!("@var{} = private unnamed_addr constant [{} x i8] c\"{}\\00\", align 1\n", var_index, value.len() + 1, value),
            }
        },
        Expression::Bool(_) => todo!(),
        Expression::Variable(_) => todo!(),
        Expression::I32(value) => {
            match name{
                Some(name) => return format!("@{} = private constant i32 {}\n", name, value)
                ,
                None => return format!("@var{} = private constant [4 x i8] c\"%d\\0A\\00\"\n", var_index),
            }
        },
        Expression::I64(_) => todo!(),
        Expression::F32(_) => todo!(),
        Expression::F64(_) => todo!(),
        Expression::Array(_) => todo!(),
        Expression::Complete(_) => todo!(),
        Expression::BinaryOperator(_) => todo!(),
        Expression::UnaryOperator(_) => todo!(),
        Expression::Increment => todo!(),
        Expression::Decrement => todo!(),
        Expression::FunctionCall(_, _) => todo!(),
        Expression::CompleteU(_) => todo!(),
    }
}

#[cfg(test)]
mod test {
    use crate::{
        assembler::get_buffer, compiler::Type, eval::Primitive, parse::{Expression, Statement}
    };
    use std::{collections::{HashMap, VecDeque}, fs};
    #[test]
    fn hello_world() {
        let mut statements = VecDeque::new();
        statements.push_back(
            Statement::FunctionCall(
                "print".to_owned(),
                vec![
                    Expression::String("Hello, world!!".to_owned()),
                ]
            )
        );
        let actual = get_buffer(&statements, HashMap::new());
        let expected = fs::read_to_string("llvm_tests/hello_world.ll").expect("go fuck yourself").replace("\r", "");
        assert_eq!(actual, expected);
    }

    #[test]
    fn define_string_variable(){
        let mut statements = VecDeque::new();
        statements.push_back(
            Statement::DefineVariable(
                "abc".to_owned(),
                Expression::String("this is a string".to_owned()),
                Type::String
            )
        );
        let actual = get_buffer(&statements, HashMap::new());
        let expected = fs::read_to_string("llvm_tests/define_string_variable.ll").expect("go fuck yourself").replace("\r", "");
        assert_eq!(actual, expected);
    }

    #[test]
    fn print_string_variable(){
        let mut statements = VecDeque::new();
        statements.push_back(
            Statement::DefineVariable(
                "a".to_owned(),
                Expression::String("abc".to_owned()),
                Type::String
            )
        );
        statements.push_back(
            Statement::FunctionCall(
                "print".to_owned(),
                vec![
                    Expression::Variable("a".to_owned()),
                ]
            )
        );
        let mut variable_map = HashMap::new();
        variable_map.insert("a".to_string(), (Primitive::String("abc".to_string()), Type::String));
        let actual = get_buffer(&statements, variable_map);
        let expected = fs::read_to_string("llvm_tests/print_string_variable.ll").expect("go fuck yourself").replace("\r", "");
        assert_eq!(actual, expected);
    }

    #[test]
    fn print_i32(){
        let mut statements = VecDeque::new();
        statements.push_back(
            Statement::FunctionCall(
                "print".to_owned(),
                vec![
                    Expression::I32(777),
                ]
            )
        );
        let actual = get_buffer(&statements, HashMap::new());
        let expected = fs::read_to_string("llvm_tests/print_i32.ll").expect("go fuck yourself").replace("\r", "");
        assert_eq!(actual, expected);
    }
}