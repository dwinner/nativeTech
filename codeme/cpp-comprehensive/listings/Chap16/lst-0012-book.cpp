
// https://godbolt.org/z/99PPn595E 
#include <iostream>  // cout
#include <stdexcept> // runtime_error
struct CanThrow {
    CanThrow(int whatShouldHappen) {
        std::cout << "Constructor " << whatShouldHappen << "...\n";
        if(whatShouldHappen == 666)
            throw std::runtime_error("Test error");
        std::cout << "...Constructor finished\n";
    }
    ~CanThrow() {
        std::cout << "Destructor.\n";
    }
};
int main() {
    try {
        CanThrow ct1{0};                // okay, does not throw an exception
    } catch(std::runtime_error &exc) {
        std::cout << "Caught-1: " << exc.what() << "\n";
    }
    try {
        CanThrow ct2{666};              // throws, ct2 is not created
    } catch(std::runtime_error &exc) {
        std::cout << "Caught-2: " << exc.what() << "\n";
    }
}