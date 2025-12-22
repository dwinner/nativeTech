//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/a4zxbd99z 
#include <array> 
#include <algorithm>                     // accumulate 
#include <numeric>                       // iota
using January = std::array<int,31>;      // alias for repeated use

void initJanuary(January& jan) {         // the exact array as parameter 
    std::iota(begin(jan), end(jan), 1);  // fills with 1, 2, 3 ... 31 
} 
int sumJanuary(const January& jan) {     // the exact array as parameter 
    return std::accumulate(begin(jan), end(jan), 0); // helper function for sum 
} 
int main() {
    January jan;                         // declares an array<int,31> 
    initJanuary( jan ); 
    int sum = sumJanuary( jan ); 
}