//#(compile) c++; compiler:g141; options:; libs:-
// https://godbolt.org/z/vW5s3Kh6M 
#include <iostream>

using std::ostream; using std::cout;
struct Base2 {
    int eight_ = 8;
    virtual int value() const          // virtual method
        { return eight_; }
    void print(ostream& os) const
        { os << value() << "\n"; }
};
struct Value2 : public Base2 {
    int ten_ = 10;
    virtual int value() const override // override
        { return ten_; }
};
int main() {
    Value2 v2{}; v2.print(cout);       // use
}