//#(compile) c++; compiler:g141; options:-O2 -std=c++23; libs:-
// https://godbolt.org/z/eK1Ervjs5 
#include <string>
#include <iostream>                      // cout
using std::string; using std::cout;

struct Value {
    int value_;
    Value(int value)                     // 1-arg constructor = type conversion
        : value_{value} {}
};

size_t length(string arg) {
    return arg.size();
}
Value twice(Value v) {
    return Value{ v.value_*2 };
}
int main() {
    cout << length("Hipphopp") << "\n";  // const char* to string
    cout << twice(10).value_ << "\n";    // int to Value
    string name {"Gandalf"};
    cout << ( name + " the Grey" ) << "\n"; // string + const char*
}