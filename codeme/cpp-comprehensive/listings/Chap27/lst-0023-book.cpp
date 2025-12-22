
// https://godbolt.org/z/GoPddTWq6
#include <fstream>
#include <iostream>
using std::cout;
int main() {
    std::ofstream file01("testfile.txt", std::ios::out|std::ios::app);
    if(file01.fail()) {
        cout << "Could not open testfile.txt\n";
    } else {
        cout << "ok.\n";
    }
    std::fstream file02;
    file02.open("database.db", std::ios::out|std::ios::trunc);
    if( !file02 ) {
        cout << "Could not open database.db\n";
    } else {
        cout << "ok.\n";
    }
}