//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/xz64qK3e5 
#include <random>
#include <iostream>
int main() {
    std::default_random_engine e{};
    using Dstr = std::uniform_int_distribution<int>;      // uniformly distributed int
    Dstr card{};                                          // generate distribution
    for(int n=32; n>=1; --n)
      std::cout <<" "<< card(e, Dstr::param_type{1,n} );  // parameters only here
    std::cout << '\n';
    // output for example:
    // 1 5 23 14 15 6 2 17 17 22 9 11 17 1 1 10 11 1 6 1 6 8 6 9 7 4 1 4 2 3 2 1
}