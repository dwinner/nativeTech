//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/zfe7dM4ex 
#include <array> 
#include <iostream>
constexpr size_t MONTHS = 12; /* months in year */ 
int main() { 
    std::array<unsigned,MONTHS> mdays = {         // okay with a constant 
        31,28,31,30,31,30,31,31,30,31,30,30,30,31}; 
    unsigned age = 0; 
    std::cout << "How old are you? "; std::cin >> age; 
    std::array<int,age> years of life;           // (ERR) array size not by variable 
}