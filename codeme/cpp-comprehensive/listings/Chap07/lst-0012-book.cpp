
// https://godbolt.org/z/dcKnx3Pno 
#include <iostream> 
void print_cref(const int& n) {  // parameter as constant reference 
    std::cout << n << " "; 
}

int main() { 
    int x = 5; 
    print_cref(x);               // call with a variable 
    print_cref(42);              // call with a constant literal 
}