
// https://godbolt.org/z/8zcvMdM3b 
#include <iostream>

void print_val8(int n) {           // parameter as value 
    std::cout << n << " ";
    n = 8;                         // sets parameter to 8 
    std::cout << n << "\n"; 
}

int main() { 
    int x = 5; 
    print_val8(x);                 // x as value: prints 5, then 8 
    std::cout << x << "\n";        // x remains unchanged at 5 
    print_val8(42);                // 42 as value: prints 42, then 8 
}