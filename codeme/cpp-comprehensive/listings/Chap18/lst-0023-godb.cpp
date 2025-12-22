//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/oc7jsbrEK 
#include <iostream>
using std::cout;
struct Base1 {
    virtual void f1() {}
};
struct Base2 {
    virtual void f2() {}
};
struct Derived : public Base1, public Base2 {
    virtual void g() {};
};
void compare(void* a, void* b) {
    cout << (a==b ? "identical\n" : "different\n");
}
int main() {
    Derived d{};
    auto *pd = &d;
    cout << pd << '\n';           // for example 0x1000
    auto pb1 = static_cast<Base1*>(pd);
    cout << pb1 << '\n';          // still 0x1000
    auto pb2 = static_cast<Base2*>(pd);
    cout << pb2 << '\n';          // now 0x1008!
    cout << (pd==pb1 ? "same\n" : "different\n"); // Output: same
    cout << (pd==pb2 ? "same\n" : "different\n"); // Output: same
    compare(pb1, pd);             // Output: identical
    compare(pb2, pd);             // Output: different
}