//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/Pq5aWq8xr
#include <iostream>
using std::cout; using std::ostream;
// ... as before ...
struct NormalCase : public DerivedPublic {
    void print() {
        cout << xPublic;
        cout << xProtected;
    }
};
struct SpecialCase : public DerivedPrivate {
    void print() {
        cout << xPublic;                   // (ERR) no access
        cout << xProtected;                // (ERR) no access
    }
};
int main() {
    NormalCase n {};
    n.print();                // output: 12
    SpecialCase s {};
    s.print();
}