block a {
    //project euler problem
    let target: i64 = 600851475143;
    while target % i64(2) == 0 {
        target = target / 2;
    }
    let i: i64 = 3;
    while i * i <= target{
        while target % i == 0{
            target = target / i;
        }
        i = i + i64(2);
        i = i + i64(2);
    }
    print(target);

    //complex logic
    print((false || true) && true);

    //arrays
    let ieee: Array<Array<i32>> = [[1, 2, 3, 4]];
    print(ieee);
    print(rand_float());
}