//#(compile) c++; compiler:g141; options:; libs:-
// https://godbolt.org/z/YTK8PfcMc
#include <iostream>
struct Base {
    int eight_ = 8;
    int value() const { return eight_; }
    void print(std::ostream& os) const { os << value() << "\n"; }
};
struct Print : public Base {
    int nine_ = 9;
    void print(std::ostream& os) const { os << value() << "\n"; }
};
struct Value : public Base {
    int ten_ = 10;
    int value() const { return ten_; }
};
struct Both : public Base {
    int eleven_ = 11;
    int value() const { return eleven_; }
    void print(std::ostream& os) const { os << value() << "\n"; }
};
int main() {
    Base ba{}; ba.print(std::cout);   // Base call
    Print pr{}; pr.print(std::cout);  // print overridden
    Value va{}; va.print(std::cout);  // print from Base
    Both bo{}; bo.print(std::cout);   // everything overridden
}