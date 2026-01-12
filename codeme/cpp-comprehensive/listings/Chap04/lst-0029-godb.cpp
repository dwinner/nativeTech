//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/bh1MsPhje 
#include <iostream> 
int main() { 
    int a = 3; 
    int b = 7 + (a = 12) + 6;   // contains an assignment 
    std::cout << a << std::endl; 
}