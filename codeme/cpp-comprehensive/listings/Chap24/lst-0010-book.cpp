
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