
// https://godbolt.org/z/ebPcdxzvj 
#include <iostream> 
int main() { 
    std::cout << 3 + 4 * 5 + 6 << "\n";               // 29 
    std::cout << 20/7 << " Rest " << 20%7 << "\n";    // 2 remainder 6 
}