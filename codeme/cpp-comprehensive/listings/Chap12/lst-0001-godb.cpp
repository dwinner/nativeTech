//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/8YqoTEx58 
#include <string>
#include <iostream>                      // cout
#include <format>
using std::string; using std::cout; using std::format;
struct Person {                          // defines the new type Person
    string name_;
    int age_;
    string city_;
};                                       // closing semicolon
void print(Person p) {                   // entire Person as one parameter
    cout << format("{} ({}) from {}\n",
      p.name_, p.age_, p.city_);         // access via dot
}
int main() {
    Person john {"John", 45, "Boston" }; // initialization
    print(john);                         // call as a unit
}