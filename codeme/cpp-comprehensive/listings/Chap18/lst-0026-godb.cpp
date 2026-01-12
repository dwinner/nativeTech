//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/s9xnn7c5j 
#include <iostream>
using std::cout;
struct Base {                           // abstract class
  virtual void provider() = 0;
  virtual void user() = 0;
};
struct Derived1 : public virtual Base { // still abstract
  virtual void user() override { provider(); }
};
struct Derived2 : public virtual Base { // still abstract
  virtual void provider() override { cout << "Derived2::provider!\n"; }
};
struct DerivedDerived : public Derived1, public Derived2 { // concrete
};
int main() {
  DerivedDerived dd{};
  DerivedDerived *pdd = &dd;
  Derived1* pd1 = pdd; // Cast within the hierarchy
  Derived2* pd2 = pdd; // Cast within the hierarchy
  pdd->user();         // Output: Derived2::provider()!
  pd1->user();         // Output: Derived2::provider()!
  pd2->user();         // Output: Derived2::provider()!
}