//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/911fxx3nq
// modern101.cpp : Fibonacci console 
#include <iostream> 
int fib(int n) { 
    return n<=1 ? n : fib(n-2) + fib(n-1); 
} 
int main() { 
    std::cout << "Which Fibonacci number? "; 
    int n = 0; 
    std::cin >> n; 
    std::cout << "fib(" << n << ")=" << fib(n) << "\n"; 
}