//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/sG4E66YhW
#include <iostream>
#include <string>
#include <tuple>
#include <vector>
#include <algorithm> // ranges::sort
using std::tuple; using std::get; using std::cout; using std::string;
int main() {
    std::vector<tuple<string,string,int>> armstrongs =
      { {"Armstrong", "Louis", 1901}  // Initialize using initializer list
      , {"Armstrong", "Lance", 1971}
      , {"Armstrong", "Neil", 1930} };
    std::ranges::sort(armstrongs);    // Lance < Louis < Neil
    for(const auto& a : armstrongs) {
      cout << get<0>(a) << ", " << get<1>(a) << ", " << get<2>(a) << "\n";
    }
}