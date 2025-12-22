//#(compile) c++; compiler:g141; options:-O1 -std=c++23 -Wno-unused-result; libs:boost@184
// https://godbolt.org/z/T4c1GnjY4
#include <iostream>
#include <thread>
#include <vector>
#include <exception>
using std::cout; using std::endl;

long fib(long n) { return n<=1 ? n : fib(n-1)+fib(n-2); }
void task1() { auto r = fib(40); cout << "fib(40)=" << r << endl; }

void main_program() {
    std::thread th{ &task1 };
    try {
        std::vector data{ 0,1,2 };
        data.at(666);                 // (ERR) triggers out_of_range
    } catch(std::runtime_error &ex) { // does not match out_of_range
         /* ... */                    // handle specific error here
    } catch( ... ) {
        th.join();
        throw;                        // continue error handling outside
    }
    th.join();                        // waits after Okay or specific error
}

int main() {
    try {
        main_program();
    } catch( ... ) {
        std::cout << "An error has occurred\n";
    }
}