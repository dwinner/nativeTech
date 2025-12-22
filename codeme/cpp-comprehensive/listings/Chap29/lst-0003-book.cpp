
// https://godbolt.org/z/nPGY47r4K 
std::jthread myThread{ [] {
    task1();
    task2();
    task3();
} };