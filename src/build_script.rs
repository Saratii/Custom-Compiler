use std::{collections::{HashMap, VecDeque}, fs, path::Path, process::Command};
use crate::{llvm_ir::get_buffer, compiler::Type, interpreter::Primitive, parse::Statement};

pub fn run(statements: &VecDeque<Statement>, variable_map: HashMap<String, (Primitive, Type)>) {
    fs::write("build/llvm.ll", get_buffer(statements, variable_map)).expect("Unable to write file");
    let exe_path = Path::new("build/main.exe");
    if exe_path.exists() {
        fs::remove_file(exe_path).expect("go fuck yourself");
    }
    let clang = Command::new("clang")
                        .arg("build/llvm.ll")
                        .arg("-o")
                        .arg("build/main.exe")
                        .output()
                        .expect("Failed to execute command");
    if !clang.status.success() {
        let error = String::from_utf8_lossy(&clang.stderr);
        eprintln!("Error: {}", error);
    }
    let execute = Command::new("./build/main.exe")
                  .output()
                  .expect("Failed to execute command");
    if execute.status.success() {
        let output: std::borrow::Cow<str> = String::from_utf8_lossy(&execute.stdout);
        println!("Compiled Output:\n{}", output);
    } else {
        let error = String::from_utf8_lossy(&execute.stderr);
        println!("Std out: {}", String::from_utf8_lossy(&execute.stdout));
        eprintln!("Error: {}", error);
    }
}
