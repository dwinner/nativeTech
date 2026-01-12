
// https://godbolt.org/z/61EdcjPn4
#include <iostream>
using std::cout; using std::ostream;
// ... as before ...
int main() {
    // public inheritance
    DerivedPublic dpu{};
    cout << dpu.xPublic << '\n';    // output: 1
    cout << dpu.xProtected << '\n'; // no access from outside
    // protected inheritance
    DerivedProtected dpt{};
    cout << dpt.xPublic << '\n';    // no access from outside
    cout << dpt.xProtected << '\n'; // no access from outside
    // private inheritance
    DerivedPrivate dpv{};
    cout << dpv.xPublic << '\n';    // no access from outside
    cout << dpv.xProtected << '\n'; // no access from outside
}