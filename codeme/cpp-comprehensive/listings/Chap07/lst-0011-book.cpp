
// https://godbolt.org/z/63dTshz3Y 
#include <iostream> 
void print_ref8(int& n) {          // parameter as reference 
    std::cout << n << " "; 
    n = 8;                         // sets parameter to 8 
    std::cout << n << "\n"; 
} 
int main() { 
    int x = 5; 
    print_ref8(x);                 // x as reference: prints 5, then 8 
    std::cout << x << "\n";        // x is now 8 
}