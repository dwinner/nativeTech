
#include <fstream>
#include <iostream>
using std::cout;
int main() {
    std::ifstream file("data.dat");
    if( !file ) { /* Error */ cout << "ERR\n"; return 1; }
    std::ofstream filecopy("backup.dat");
    if( !filecopy ) { /* Error */ cout << "ERR\n"; return 1; }
    char ch;
    while(file.get(ch) ) {
        filecopy.put(ch);
    }
}