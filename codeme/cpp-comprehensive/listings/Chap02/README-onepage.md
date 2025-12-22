# Listings of Chap02.docx

This is the list of listings on one page.
You can also view a [linked summary](README.md).


## GodboltId:911fxx3nq

Book listing [lst-0001-book.cpp](lst-0001-book.cpp):
```cpp
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
```

Godbolt Listing [lst-0001-godb.cpp](lst-0001-godb.cpp), [https://godbolt.org/z/911fxx3nq](https://godbolt.org/z/911fxx3nq):
```cpp
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
```

----------------


## GodboltId:3W1qY7b9x

Book listing [lst-0002-book.cpp](lst-0002-book.cpp):
```cpp
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
```

Godbolt Listing [lst-0002-godb.cpp](lst-0002-godb.cpp), [https://godbolt.org/z/3W1qY7b9x](https://godbolt.org/z/3W1qY7b9x):
```cpp
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
```

----------------

