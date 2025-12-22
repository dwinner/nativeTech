
// https://godbolt.org/z/Mr6E3Ez48 
#include <iostream>
int main() {
    unsigned int val;
    std::cout << "Enter value: ";
    std::cin >> val;
    if( std::cin ) { // operator bool()
        /* ... */                               // Input correct
    } else {
        std::cout << "Error with std::cin\n";   // Error with input
    }
}