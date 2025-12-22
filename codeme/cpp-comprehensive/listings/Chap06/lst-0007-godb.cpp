//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/a6qqbWKMs 
#include <cstdlib> 
#include <iostream>

int main(int argc, char* argv[]) { 
    for(int i=1; i<argc; ++i) {                // start at 1 
        for(char* p=argv[i]; *p!='\0'; ++p) { 
            char c = toupper(*p); 
            std::cout << c; 
        } 
        std::cout << ' '; 
    } 
    std::cout << '\n'; 
}