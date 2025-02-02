block a {
    sleep(5);
}

block b {
    sleep(10);
}

block c {
    sleep(3);
}


block d requires[a, b, c] {
    sleep(4);
}

block e {
    sleep(2);
}