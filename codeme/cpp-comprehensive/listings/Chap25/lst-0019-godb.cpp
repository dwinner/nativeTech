//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/j6GPKc36P
#include <iostream>
#include <algorithm>
#include <string>
void one(std::string &seq) {
    std::ranges::next_permutation(seq);
    std::cout << seq << '\n';
}
int main() {
    std::string seq = "JQK";
    std::cout << seq << '\n'; // Output: JQK
    auto limit = 3*2*1;       // n!
    for(int i=0; i<limit; ++i)
        one(seq);
    // Here the sequence is back to its original state.
}