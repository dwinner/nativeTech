//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/4Mr6fnaEr 
#include <iostream> // cout 
int main() { 
    int basis = 2; 
    int index = 10; 
    int number = 3 * (basis + ++index) - 1;  // index is incremented first 
    std::cout << number << '\n';             // output: 38 
}