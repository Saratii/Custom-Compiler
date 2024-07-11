declare i32 @printf(i8*, ...)
@a = private unnamed_addr constant [12 x i8] c"hello world\00", align 1
define i32 @main() {
entry:
call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([12 x i8], [12 x i8]* @a, i32 0, i32 0))
ret i32 0
}