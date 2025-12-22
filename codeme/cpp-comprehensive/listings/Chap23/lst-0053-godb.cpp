//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/eh6jnjK9z
#include <vector>
#include <string>
#include <algorithm> // sort
#include <iostream>  // cout
using std::string; using std::vector; using std::cout;
// as before
int main() {
    vector dwarves{        // as before
    /* sort */
    bool backwards = true; // or false. Variable outside the lambda
    std::ranges::sort(dwarves,
        [backwards](const Dwarf& a, const Dwarf& b) {
            if(backwards)
                return a.name_ > b.name_;
            else
                return a.name_ < b.name_;
        }
    );
    /* output */
    for(const auto& z : dwarves) // backwards: "Thorin" to "Balin"
        cout << z.name_  << " ";
    cout << "\n";
}