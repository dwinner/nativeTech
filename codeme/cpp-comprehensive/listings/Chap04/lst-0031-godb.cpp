//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/zK9Ke9n48 
#include <iostream>                      // cout 
int main(int argc, const char* argv[]) { 
    bool withParameters = argc > 1;      // comparison result stored 
    if(withParameters) {                 // ... and used 
        std::cout << "You have called the program with parameters.\n"; 
    } 
    return 0; 
}