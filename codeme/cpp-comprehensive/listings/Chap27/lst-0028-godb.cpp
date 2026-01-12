//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/hPaz3a3sE
#include <fstream>
#include <iostream>
using std::cout;
int main() {
    std::ifstream file("44fstream07.cpp");
    if( !file ) { /* Error */ cout << "ERR\n"; return 1; }
    std::ofstream filecopy("backup.cpp");
    if( !filecopy ) { /* Error */ cout << "ERR\n"; return 1; }
    std::string buffer;
    while( getline(file, buffer) ) {
        filecopy << buffer << std::endl;
        cout << buffer << std::endl;
    }
    if( file.eof() ) {
        file.clear();
    }
}