//#(compile) c++; compiler:g141; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/Y3eW1Kh8v 
#include <string>
#include <iostream>    // ostream
#include <format>

using std::string; using std::ostream; using std::format;

struct Person {
  string name_;
  int age_;
  string city_;
  ostream& print(ostream& os) {       // method defined inline
    return os << format("{} ({}) from {}", name_, age_, city_);
  }
};