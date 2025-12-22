//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/P1fn5q8z8
// Call this program with 'prog.exe > file.txt', for example. 
#include <iostream>  // cout, cerr 
int main() {
    std::cout << "Output to cout\n";       // is output after 'file.txt'
    std::cerr << "Error message!\n";       // still appears on the console
    std::cout << "Normal output again\n";  // back into the file 
}