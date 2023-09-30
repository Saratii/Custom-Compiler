use std::collections::VecDeque;
use std::fs::File;
use std::io::Write;

use crate::compiler::Compiler;
use crate::parse::Statement;

impl Compiler{
    pub fn to_llvm_x86(&self, _statements: &VecDeque<Statement>){
        let mut x64_x86 = File::create("../build/x86.ll").expect("creation failed");
        let llvm_ir = 
r#"@.str = private unnamed_addr constant [13 x i8] c"hello world\0A\00"
declare i32 @puts(i8* nocapture) nounwind

define i32 @main() {
    %cast210 = getelementptr [13 x i8], [13 x i8]* @.str, i64 0, i64 0
    call i32 @puts(i8* %cast210)
    ret i32 0
}

!0 = !{i32 42, null, !"string"}
!foo = !{!0}"#;
    
    let _ = x64_x86.write_all(llvm_ir.as_bytes());
    
    println!("LLVM IR code written to build/x86.ll");
    }
}
//clang -o main.exe x86.ll     
//./main.exe