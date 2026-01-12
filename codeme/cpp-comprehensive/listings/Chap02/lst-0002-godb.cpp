//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/3W1qY7b9x 
// modern102.cpp : Fibonacci console 
#include <iostream> 
#include <map> 
int fib(int n) { 
    static std::map<int, int> table{}; 
    table[n] = n<=1 ? n : table[n-2] + table[n-1]; 
    return table[n]; 
} 
int main() { 
    std::cout << "How many Fibonacci numbers? "; 
    int n = 0; 
    std::cin >> n; 
    for (int i = 1; i <= n; ++i) 
        std::cout << "fib(" << i << ")=" << fib(i) << "\n"; 
}