//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/sjaGaq48v
#include <fstream>
#include <iomanip> // setw
#include <iostream>
int main() {
    std::ifstream file("data.dat");
    if( !file ) {
        std::cout << "Error opening file\n";
        return 1;
    }
    char ch;
    while(file.get(ch) ) {
        std::cout.put(ch);
    }
    if( file.eof() ) {
        file.clear();
    }
    file.close();
}