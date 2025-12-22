//#(compile) c++; compiler:g141; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/8h93f64Yj 
#include <string>
#include <string_view>
using std::string; using sview = std::string_view;

struct Person {
    string name_;
    int age_;
    string city_;

    Person(sview n, int a, sview c)                // delegated constructor
      : name_(n), age_(a), city_(c) { }            // … implemented
    Person() : Person{"no name",-1,"no city"} { }  // delegating
    Person(sview n, int a) : Person{n, a, "no city"} { } // delegating
    Person(sview n) : Person{n, -1, "no city"} { } // delegating
};