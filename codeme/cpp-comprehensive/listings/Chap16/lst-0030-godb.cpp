//#(compile) c++; compiler:g141; options:-O2 -std=c++23; libs:-
// https://godbolt.org/z/P99563KoP 
#include <iostream>
using std::cout; using std::ostream;
struct Widget {
    bool operator<(const Widget&) const {      // method notation
        return true;                           // always true
    }
};
bool operator<(const Widget&, const Widget&) { // function notation
    return false;                              // always false
}
int main() {
    Widget x{};
    Widget y{};
    cout << (operator<(x, y)      // calls function notation
        ? "Method1\n" : "Function1\n");
    cout << (y.operator<(x)       // calls method notation
        ? "Method2\n" : "Function2\n");
    cout << (x < y                // infix notation, allows choice, here function
        ? "Method3\n" : "Function3\n");
}