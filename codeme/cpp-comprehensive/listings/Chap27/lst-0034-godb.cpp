//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/1EMox3hW5 
#include <sstream>  // istringstream
#include <iostream>
int main() {
    std::istringstream istr;
    std::string sval("3.1415 : 4321");
    std::string none;
    double dval=0.0;
    int ival=0;
    istr.str(sval);                 // initialize
    istr >> dval >> none >> ival;   // read
    if( ! istr.eof() ) {
        std::cout << "Error converting\n"; return 1;
    }
    std::cout << dval << " == " << none << " == " << ival << "\n";
    // Output: 3.1415 == : == 4321
}