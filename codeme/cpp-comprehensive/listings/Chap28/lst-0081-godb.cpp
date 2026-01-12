//#(execute) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/Y3GsPd97d 
#include <set>
#include <string>
struct Dragon {
    std::string name_;
};
namespace std {
    template<> struct less<Dragon> { // Template specialization
        bool operator()(const Dragon &lhs, const Dragon &rhs) const {
            return lhs.name_ < rhs.name_;
} }; }
int main() {
  std::set<Dragon> dragons {
    Dragon{"Smaug"},    Dragon{"Glaurung"},
    Dragon{"Ancalagon"},Dragon{"Scatha"}};
}