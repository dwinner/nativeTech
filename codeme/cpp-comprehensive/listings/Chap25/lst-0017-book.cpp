
// https://godbolt.org/z/s53qx664K 
#include <iostream>
#include <random>       // default_random_engine
#include <string>
#include <iterator>     // back_inserter
#include <algorithm>    // sample

int main() {
   std::default_random_engine random{};
   const std::string in = "abcdefgh";
   for(auto idx : {0,1,2,3}) {
     std::string out;
     std::ranges::sample(in, std::back_inserter(out), 5, random);
     std::cout << out << '\n';
   }
}