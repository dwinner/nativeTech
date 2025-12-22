//#(compile) c++; compiler:g141; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/5T1Er36P7 
#include <string>
#include <iostream>
#include <format>
using std::string; using std::cout; using std::format;
struct Person {
  string name_;
  int age_;
  string city_;
  void print();            // function as a method of the type
};
void Person::print() {     // method name is extended by Person::
  cout << format("{} ({}) from {}\n",
    name_, age_, city_);   // in a method you can directly access fields
}
int main() {
  Person john {"John", 45, "Boston" };
  john.print();            // calling the method for a variable of the type
}