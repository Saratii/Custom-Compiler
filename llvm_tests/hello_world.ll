declare i32 @printf(i8*, ...)
@var0 = private unnamed_addr constant [15 x i8] c"Hello, world!!\00", align 1
define i32 @main() {
entry:
call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([15 x i8], [15 x i8]* @var0, i32 0, i32 0))
ret i32 0
}