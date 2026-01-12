//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/eW1cfYcGs 
#include <iostream> 
void output(int a, int b) { 
    std::cout << a << ' ' << b << '\n'; 
} 
int number() { 
    static int val = 0; 
    return ++val; 
} 
int main() {
    output(number(), number()); // in which order?
}