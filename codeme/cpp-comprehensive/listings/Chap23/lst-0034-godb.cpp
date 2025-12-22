//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/c9Eh346cq
#include <functional> // function
#include <iostream>   // cout
int calculate(int a, int b, std::function<int(int,int)> binop) {
    return binop(a,b);
}
int plus(int a, int b) { return a+b; }
int times(int a, int b) { return a*b; }
int main() {
    std::cout << calculate(3, 4, plus) << "\n";   // Value notation
    std::cout << calculate(3, 4, times) << "\n";  // Value notation
    std::cout << calculate(3, 4, &plus) << "\n";  // Pointer notation
    std::cout << calculate(3, 4, &times) << "\n"; // Pointer notation
}