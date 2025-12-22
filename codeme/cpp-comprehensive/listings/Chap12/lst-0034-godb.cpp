//#(compile) c++; compiler:g141; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/1fE15Kf39
#include <vector>
#include <string>
#include <string_view>
#include <iostream>
using std::string; using std::cout; using sview = std::string_view;

struct Person {
    string name_;
    int age_;
    string city_;
    // acts as the default constructor:
    Person(sview n = "N.N.", int a = 18, sview c = "Berlin")
      : name_(n), age_(a), city_(c) { }
};

int main() {
    std::vector<Person> people{};     // initially empty
    people.resize(5);                 // expand to five 'empty' people
    cout << people[3].city_ << "\n";  // Output: Berlin
}