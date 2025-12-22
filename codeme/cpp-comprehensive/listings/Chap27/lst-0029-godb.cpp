//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/P6MPabsKc
#include <fstream>
#include <iostream>
#include <vector>
using std::cout;
int main() {
    std::ifstream file("testfile.txt", std::ios::binary);
    if( !file ) { /* Error */ cout <<"ERR1\n"; return 1; }
    std::ofstream filecopy("backup.dat", std::ios::binary);
    if( !filecopy ) { /* Error */ return 1; }
    file.seekg(0, std::ios::end);
    auto size = file.tellg();
    cout << "File size: " << size << " bytes\n";
    file.seekg(0, std::ios::beg); // Important!
    std::vector<char> buffer(size);
    file.read(buffer.data(), size);
    if( !file ) { cout << "Error during read...\n"; return 1;}
    cout << "Read: " << file.gcount() << " bytes\n";
    filecopy.write( buffer.data(), size );
    if( !filecopy ) { cout << "Error during write...\n"; return 1;}
}