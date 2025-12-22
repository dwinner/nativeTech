//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/j7qoMdG3n
#include <vector>
#include <string>
#include <algorithm> // sort
#include <iostream>  // cout
using std::string; using std::vector; using std::cout;
// as before
int main() {
    vector dwarfs{           // as before
    /* sort */
    bool backwards = true;   // or false. Variable outside the lambda
    unsigned rightway = 0;   // counts <
    unsigned wrongway = 0;   // counts >
    std::ranges::sort(dwarves,
        [backwards,&rightway,&wrongway](const Dwarf& a, const Dwarf& b) {
            bool result = backwards
                ? a.name_ > b.name_
                : a.name_ < b.name_;
            if(result==false) ++wrongway; else ++rightway;
            return result;
        }
    );
    /* output */
    cout << "Wrongway:" << wrongway << " Rightway: " << rightway << "\n";
    for(const auto& z : dwarves) // backwards: "Thorin" to "Balin"
        cout << z.name_  << " ";
    cout << "\n";
}