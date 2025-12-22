
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