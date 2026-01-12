//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/babK485oT 
#include <vector>
#include <iostream>
int main() {
    std::vector<int> numbers{ 1,2,3,4,5 };
    for(auto it = begin(numbers); it != end(numbers); ++it) {
        auto val = *it;
        std::cout << val << ' ';
    }
    std::cout << '\n';
}