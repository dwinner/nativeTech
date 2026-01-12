//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/3756v8rzK
#include <fstream>
#include <iomanip> // setw
#include <iostream>
int main() {
    std::ofstream file("data.dat");
    if( !file ) {
        std::cout << "Could not open data.dat\n";
        return 1;
    }
    file << std::setw(10) << std::setfill( '*' )
         << 1234 << std::endl;
}