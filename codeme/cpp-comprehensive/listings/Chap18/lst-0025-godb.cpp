//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/8jYeYPs7W 
#include <iostream>
using std::cout;
class Base {
public:
  int data_ = 0;
};
class Derived1 : public virtual Base {
};
class Derived2 : public virtual Base {
};
class DerivedDerived : public Derived1, public Derived2 {
public:
  void method() {
     data_ = 1;  // unambiguous, because there is only one data_
  }
};
void output(const DerivedDerived &dd) {
    cout << dd.data_
        << (((Derived1&)dd).data_)
        << (((Derived2&)dd).data_)
        << (((Base&)dd).data_) << '\n';
}
int main() {
  DerivedDerived dd{};
  output(dd);    // Output: 0000
  dd.method();   // sets data_ to 1
  output(dd);    // Output: 1111
}