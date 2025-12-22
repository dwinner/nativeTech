//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/TsP33njar 
#include <iostream>
#include <stdexcept> // exception

int main() {
    try {
        // … your other code …
    } catch(std::exception& exc) {
        std::cerr << "main: " << exc.what() << "\n";
    }
}