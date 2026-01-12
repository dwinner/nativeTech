//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/Yv3vcxPv5 
#include <vector>
#include <iostream>
int main() {
    std::vector<int> numbers{ 1,2,3,4,5 };
    for(auto val : numbers) {
        std::cout << val << ' ';
    }
    std::cout << '\n';
}