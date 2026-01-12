//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/o7YP7or8s
#include <fstream>
#include <iostream>
#include <string>
int main() {
    std::string name = "textfile.txt";
    std::ifstream file01;
    file01.open(name);
    if( file01.fail() ) {
        std::cout << "Could not open " << name << "\n";
    }
    std::ofstream file02("data.dat");
    if( file02.good() ) {
        std::cout << "data.dat opened or created\n";
    }
    std::fstream file03;
    file03.open("database.db");
    if( !file03 ) {
        std::cout << "Could not open database.db\n";
    }
}