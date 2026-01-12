
// https://godbolt.org/z/TT13E3sz5
#include <sstream>  // ostringstream
#include <iostream>
int main() {
    std::ostringstream ostr;
    double dval = 3.1415;
    int ival = 4321;
    ostr << dval << " : " << ival;
    const std::string sval = ostr.str();
    std::cout << sval << std::endl;    // Output: 3.1415 : 4321
}