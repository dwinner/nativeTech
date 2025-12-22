#include <iostream>
#include <random>
#include <string>
#include <iterator>
#include <algorithm>
 int main() {
     std::default_random_engine randy{};
     const std::string in = "abcdefgh";
     for(auto idx : {0,1,2,3,4}) {
         std::string out;
         std::sample(in.begin(), in.end(), std::back_inserter(out), 5, randy);
         std::cout << out << '\n';
     }
}
