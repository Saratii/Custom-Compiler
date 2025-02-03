block b {
    f32 i = 2.1;
}


block a requires[b[i]] {
    print(i);
}