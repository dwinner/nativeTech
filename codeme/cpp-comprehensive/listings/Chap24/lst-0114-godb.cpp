//#(compile) c++; compiler:g141; options:-O3 -std=c++23 -DNDEBUG; libs:-
// https://godbolt.org/z/cdeGW6a3P
#include <unordered_map>
#include <iostream>
#include <string>
using std::string; using std::unordered_map; using std::cout;
struct City {
    string name_;
    explicit City(const string &name) : name_{name} {}
    auto operator<=>(const City &b) const = default;
};
struct CityHash {
     std::hash<string> sHash;
     size_t operator()(const City& a) const {
         return sHash(a.name_);
     }
};
int main() {
    unordered_map<City, string, CityHash> cty{
        {City{"San Francisco"}, "CA"},
        {City{"Austin"}, "TX"},
        {City{"Miami"}, "FL"},
    };
    cout << cty[City{"San Francisco"}] << '\n'; // Output: CA
}