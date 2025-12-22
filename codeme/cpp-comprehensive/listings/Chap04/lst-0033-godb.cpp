//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/chTTqPjrM 
#include <string> 
#include <iostream> 
int main() { 
    std::cout << "string_literals\n"; 
    // using std::literals::string_literals::operator""s; 
    // using namespace std::string_literals; 
    // using namespace std::literals::string_literals; 
    using namespace std::literals; 
    std::cout << "Real string\n "s; 
}