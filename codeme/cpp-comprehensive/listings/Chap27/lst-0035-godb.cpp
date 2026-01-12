//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/vcoEcYYcx
#include <sstream> // stringstream
#include <iostream>
#include <stdexcept> // invalid_argument
template <class T1, class T2>
void myConvert(const T1& in, T2& out) {
    std::stringstream ss;
    ss << in;
    ss >> out;
    if( ! ss.eof() ) {
        throw std::invalid_argument("Error during conversion");
    }
}
int main() {
    std::string sval;
    float fval=3.1415f;
    std::string sdval("5.321");
    double dsval=0;
    std::string doesnotwork("does not work");
    try {
        myConvert(fval, sval);
        std::cout << sval << std::endl;  // Output: 3.1415
        myConvert(sdval, dsval);
        std::cout << dsval << std::endl; // Output: 5.321
        myConvert(doesnotwork, dsval);   // triggers "Error during conversion"
    }
    catch(const std::invalid_argument& e) {
        std::cout << e.what() << std::endl;
    }
}