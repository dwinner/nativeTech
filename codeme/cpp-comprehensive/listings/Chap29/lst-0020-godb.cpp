//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/TM6c7cdss 
#include <thread>
#include <iostream>
int main() {
    std::cout << "Main: " << std::this_thread::get_id() << '\n';
    std::jthread th{ []{
        std::cout << "Thread: " << std::this_thread::get_id() << '\n';
    }};
}