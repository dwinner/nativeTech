//#(execute) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/ssW3eKEGq 
#include <iostream> // cerr 
#include <fstream> 
int main(int argc, char* argv[]) { 
    int value; 
    std::ifstream myInput{"input1.txt"}; 
    if(!myInput) { 
        std::cerr << "Error opening file!\n"; 
    } else { 
        myInput >> value; 
    }
}