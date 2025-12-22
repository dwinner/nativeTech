//#(compile) c++; compiler:g141; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/jhPaEjh1r
#include <iostream>
int main() {
    try {                                        // beginning of the try block
        for(int n=1; ; n=n*2) {
            if (n > std::numeric_limits<int>::max()/2) { // check for coming overflow
                throw "There would be an overflow";
            }
        }
    }                                            // End of the try block
    catch(const char *error) {                   // if this error occurs, …
        std::cout << "Error: " << error << " ";  // … handle it this way
    }
}