//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/nPGY47r4K 
std::jthread myThread{ [] {
    task1();
    task2();
    task3();
} };