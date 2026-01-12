
// https://godbolt.org/z/TMzdWhE51
#include <fstream>
#include <iostream>
int main() {
    std::ofstream file01("data.db");
    if( file01.fail() ) {
        std::cout << "Could not open data.db\n";
    } else {
        std::cout << "ok.\n";
    }
    file01 << "Text for the file\n";
    if( file01.is_open()) {
        file01.close();
    }
    file01.open("data001.db");
    // Automatically:
    {
        std::ofstream file02("data002.db");
    } // from here file02 is closed
} // from here file01 is also closed