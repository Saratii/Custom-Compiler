declare i32 @printf(i8*, ...)
@a = private unnamed_addr constant [4 x i8] c"abc\00", align 1
define i32 @main() {
entry:
call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @a, i32 0, i32 0))
ret i32 0
}