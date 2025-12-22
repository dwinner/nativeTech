//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:boost@184
// https://godbolt.org/z/G4z8Y5E6P
#include <variant>
#include <iostream>
using std::cout;
struct TypeGreeting {
  void operator()(int) const { cout << "Hello int"; }
  void operator()(float) const { cout << "Hello float"; }
};
int main() {
    std::variant<int, float> var{};
    var = 12;                                   // State int
    std::visit([](auto a) { cout << a; }, var); // generic lambda
    cout << std::endl;
    var = 3.456f;                               // State float
    std::visit(TypeGreeting{}, var);            // Functor with overloads
    cout << std::endl;
}