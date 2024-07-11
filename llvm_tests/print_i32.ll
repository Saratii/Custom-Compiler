@var0 = private constant [4 x i8] c"%d\0A\00"
declare i32 @printf(i8*, ...)
define i32 @main() {
entry:
call i32 (i8*, ...) @printf(i8* @var0, i32 777)
ret i32 0
}