//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/cEGGfoeT7 
#include <iostream> 
int main() { 
std::cout << "3+4*5+6=" << 3+4*5+6 << "\n";              // dot before dash; = 29
    std::cout << "(3+4)*(5+6)=" << (3+4)*(5+6) << "\n";  // Parentheses; = 77 
    std::cout << "22/7=" << 22/7 
        << " remainder " << 22%7 << "\n";                // 22/7 = 3 remainder 1 
    for(int n=0; n < 10; ++n) { 
        std::cout << -2*n*n + 13*n - 4 << " ";           // with unary minus 
    } 
    std::cout << "\n"; 
    // output: -4 7 14 17 16 11 2 -11 -28 -49 
}