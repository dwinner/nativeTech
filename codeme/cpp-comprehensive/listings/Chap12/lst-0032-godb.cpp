//#(compile) c++; compiler:g141; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/Gfrd5cceG 
#include <string>
#include <string_view>
using std::string; using sview = std::string_view;

struct Person {
    string name_;
    int age_;
    string city_;

    Person(sview n = "N.N.", int a = 18, sview c = "Berlin")
      : name_(n), age_(a), city_(c) { }
};