@var0 = private constant [4 x i8] c"%d\0A\00"
declare i32 @printf(i8*, ...)
@a = private constant i32 888
define i32 @main() {
entry:
%a = load i32, i32* @a
call i32 (i8*, ...) @printf(i8* @var0, i32 %a)
ret i32 0
}