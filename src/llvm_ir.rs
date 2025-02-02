use std::collections::{HashMap, VecDeque};

use crate::{interpreter::{Primitive, Type}, parse::Statement};

pub fn get_buffer(statements: &VecDeque<Statement>, variable_map: HashMap<String, (Primitive, Type)>) -> String{
    let mut llvm_statements = VecDeque::new();
    llvm_statements.push_back("define i32 @main() {\nentry:\n".to_string());
    let var_index: u32 = 0;
    for statement in statements{
        match statement{
            Statement::FunctionCall(name, args) => {
                if name == "print"{                    
                    if !llvm_statements.contains(&"declare i32 @printf(i8*, ...)\n".to_string()){
                        llvm_statements.push_front("declare i32 @printf(i8*, ...)\n".to_string());
                    }
                    let primitive = args[0].evaluate(&variable_map);
                    match &primitive{
                        Primitive::String(value) => {
                            llvm_define_variable(&mut llvm_statements, &primitive, &var_index, None);
                            llvm_statements.push_back(format!("call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([{} x i8], [{} x i8]* @var{}, i32 0, i32 0))\n", value.len() + 1, value.len() + 1, var_index))
                        },
                        Primitive::I32(value) => {
                            llvm_define_variable(&mut llvm_statements, &primitive, &var_index, None);
                            llvm_statements.push_back(format!("call i32 (i8*, ...) @printf(i8* @var{}, i32 {})\n", var_index, value));
                        }
                        Primitive::F32(_) => todo!(),
                        Primitive::I64(_) => todo!(),
                        Primitive::F64(_) => todo!(),
                        Primitive::Bool(_) => todo!(),
                        Primitive::Array(_) => todo!(),
                    }
                } else {
                    todo!()
                }
            },
            Statement::DefineVariable(name, expression, _) => {
                llvm_define_variable(&mut llvm_statements, &expression.evaluate(&variable_map), &var_index, Some(name.to_string()));
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

fn llvm_define_variable(llvm_statements: &mut VecDeque<String>, primitive: &Primitive, var_index: &u32, name: Option<String>){
    match primitive{
        Primitive::String(value) => {
            match name{
                Some(name) => llvm_statements.push_front(format!("@{} = private unnamed_addr constant [{} x i8] c\"{}\\00\", align 1\n", name, value.len() + 1, value))
                ,
                None => return llvm_statements.push_front(format!("@var{} = private unnamed_addr constant [{} x i8] c\"{}\\00\", align 1\n", var_index, value.len() + 1, value))
            }
        },
        Primitive::I32(value) => {
            match name{
                Some(name) => llvm_statements.push_front(format!("@{} = private constant i32 {}\n", name, value))
                ,
                None => llvm_statements.push_front(format!("@var{} = private constant [4 x i8] c\"%d\\0A\\00\"\n", var_index))
            }
        },
        Primitive::F32(_) => todo!(),
        Primitive::I64(_) => todo!(),
        Primitive::F64(_) => todo!(),
        Primitive::Bool(_) => todo!(),
        Primitive::Array(_) => todo!(),
    }
}

#[cfg(test)]
mod test {
    use crate::{
        interpreter::{Primitive, Type}, llvm_ir::get_buffer, parse::{Expression, Statement}
    };
    use std::{collections::{HashMap, VecDeque}, fs};
    #[test]
    fn hello_world() {
        let mut statements = VecDeque::new();
        statements.push_back(
            Statement::FunctionCall(
                "print".to_owned(),
                vec![
                    Expression::String("hello world".to_owned()),
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

    #[test]
    fn print_i32_variable(){
        let mut statements = VecDeque::new();
        statements.push_back(
            Statement::DefineVariable("a".to_string(), Expression::I32(888), Type::I32)
        );
        statements.push_back(
            Statement::FunctionCall("print".to_owned(), vec![Expression::Variable("a".to_string())])
        );
        let mut variable_map: HashMap<String, (Primitive, Type)> = HashMap::new();
        variable_map.insert("a".to_string(), (Primitive::I32(888), Type::I32));
        let actual = get_buffer(&statements, variable_map);
        let expected = fs::read_to_string("llvm_tests/print_i32_variable.ll").expect("go fuck yourself").replace("\r", "");
        assert_eq!(actual, expected);
    }
}