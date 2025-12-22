//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/nqbKT9GEa 
#include <array>
#include <numeric> // accumulate
#include <iostream>
template<size_t SZ>
int sumSz(const std::array<int,SZ>& data) {
    int result = 0;
    for(auto i=0uz; i<SZ; ++i)        // uz is the suffix for size_t since C++23
        result += data[i];
    return result;
}
template<typename Elem, size_t SZ>
Elem sumElem(const std::array<Elem,SZ>& data) {
    Elem result {0};
    for(auto i=0uz; i<SZ; ++i)
        result += data[i];
    return result;
}
// C++20 concept input_iterator, otherwise typename
template<std::input_iterator It>
int sumIt(It begin, It end) {
    return std::accumulate(begin, end, 0);
}
// abbreviated function template with auto
int sumAuto(std::input_iterator auto begin, std::input_iterator auto end) {
    return std::accumulate(begin, end, 0);
}
int main() {
   using namespace std;
   array<int,4> a4 { 1, 2, 5, 8 };
   cout << sumSz<4>(a4) << '\n';                  // Output: 16
   array<int,5> a5 { 1, -5, 3, 7, 2 };
   cout << sumElem(a5) << '\n';                   // Output: 8
   array<int,6> a6 { 1,2,3, 4,5,6 };
   cout << sumIt(a6.begin(), a6.end()) << '\n';   // Output: 21
   array a7 { 1,2,3, 4,5,6, 7 };                  // array<int,7>
   cout << sumIt(a7.begin(), a7.end()) << '\n';   // Output: 28
   cout << sumAuto(a7.begin(), a7.end()) << '\n'; // Output: 28
}