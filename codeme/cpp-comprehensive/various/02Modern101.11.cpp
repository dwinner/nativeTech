#include <iostream>

int fib(int n) {
    return n <= 2 ? 1 : fib(n-2) + fib(n-1);
}

int main() {
    std::cout << "Which Fibonacci Number? ";
    int n = 0;
    std::cin >> n;
    std::cout << "fib(" << n << ") is " << fib(n) << "\n";
    for(auto i = 1; i <= n; ++i) {
        std::cout << "  fib(" << i << ")" << " = " << fib(i) << "\n";
    }
    return 0;
}


