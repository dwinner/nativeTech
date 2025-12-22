//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/MMP16Mj3W 
#include <iostream>
#include <array>
#include <string>
using std::array;
template<typename T, size_t S>
auto append(const array<T, S>& data, T elem) {
    array<T, S+1> result {};
    for(auto i=0u; i < data.size(); ++i)
        result[i] = data[i];
    result[S] =elem;
    return result;
}
int main() {
    // before
    array pics { 3, 4, 5 };
    std::cout << pics[0] << '\n'; // Output: 3
    // enlarge
    auto more = append(pics, 77);
    // after
    std::cout << more[3] << '\n'; // Output: 77
}