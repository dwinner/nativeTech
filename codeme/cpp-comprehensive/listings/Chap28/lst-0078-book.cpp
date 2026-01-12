
// https://godbolt.org/z/d1G9ez4KW
#include <iostream>
#include <typeinfo>
#include <typeindex>
#include <map>
#include <string>
struct Base {
    virtual ~Base() {}
};
struct Derived_One : public Base {};
struct Derived_Two : public Base {};
int main() {
  using std::string; using std::cout; using std::type_index;
  std::map<std::type_index, string> names {
      { type_index(typeid(int)), "int" },
      { type_index(typeid(double)), "double" },
      { type_index(typeid(Base)), "Base" },
      { type_index(typeid(Derived_One)), "Derived_One" },
      { type_index(typeid(Derived_Two)), "Derived_Two" },
      { type_index(typeid(string)), "string" },
      { type_index(typeid(string::const_iterator)), "string" },
  };
  names[type_index(typeid(names))] = "names-map";
  int integer;
  double floating;
  Base base{};
  Base *one = new Derived_One{};
  Base *two = new Derived_Two{};
  // typeid.name() is implementation- and runtime-dependent:
  cout << typeid(integer).name() << '\n';   // On my system: i
  cout << typeid(floating).name() << '\n';  // On my system: d
  cout << typeid(base).name() << '\n';      // On my system: 4Base
  cout << typeid(*one).name() << '\n';      // On my system: 11Derived_One
  cout << typeid(*two).name() << '\n';      // On my system: 11Derived_Two
  cout << typeid(string).name() << '\n';    // For me: Ss
  cout << typeid(string{"World"}.begin()).name() << '\n';
      // For me: N9__gnu_cxx17__normal_iteratorIPcSsEE
  cout << typeid(names).name() << '\n';
      // For me: St3mapISt10type_indexSsSt4lessIS0_ESaISt4pairIKS0_SsEEE
  cout << typeid(666/0).name() << '\n'; // Expression not executed! For me: i
  // type_index makes type_infos comparable:
  cout << names[type_index(typeid(integer))] << '\n';  // Output: int
  cout << names[type_index(typeid(floating))] << '\n'; // Output: double
  cout << names[type_index(typeid(base))] << '\n';     // Output: Base
  cout << names[type_index(typeid(*one))] << '\n';     // Output: Derived_One
  cout << names[type_index(typeid(*two))] << '\n';     // Output: Derived_Two
  cout << names[type_index(typeid(string))] << '\n';   // Output: string
  cout << names[type_index(typeid(names))] << '\n';    // Output: names-map
}