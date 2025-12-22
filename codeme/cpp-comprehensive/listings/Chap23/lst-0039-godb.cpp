//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/3ch51ov87 
#include <set>
#include <string>
#include <iostream> // cout
using std::string; using std::set; using std::cout;
struct Dwarf {
    string name_;
    unsigned year_;
};
bool operator<(const Dwarf& a, const Dwarf& b) {
    return a.name_ < b.name_;
}
int main() {
    set dwarves{ Dwarf{"Balin", 2763}, Dwarf{"Dwalin", 2772},
        Dwarf{"Oin", 2774}, Dwarf{"Gloin", 2783}, Dwarf{"Thorin", 2746},
        Dwarf{"Fili", 2859}, Dwarf{"Kili", 2864} };
    for(const auto& z : dwarves) // sorted output: "Balin" to "Thorin"
        cout << z.name_  << " ";
    cout << "\n";
}