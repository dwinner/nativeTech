    // modern102.cpp : Fibonacci-Konsole
    #include <iostream>
    #include <map>
    int fib(int n) {
        static std::map<int, int> table{};
        table[n] = n<=1 ? n : table[n-2] + table[n-1];
        return table[n];
    }
    int main() {
        std::cout << "How many Fibonacci Number? ";
        int n = 0;
        std::cin >> n;
        for (int i = 1; i <= n; ++i)
            std::cout << "fib(" << i << ")=" << fib(i) << "\n";
    }
