//#(compile) c++; compiler:g141; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/Wo8zaq9WW 
#include <string>
#include <string_view>
using std::string; using sview = std::string_view;
struct Person {
    string name_ = "no name";
    int age_ = -1;
    string city_ = "no city";
    Person() {}
    Person(sview n, int a, sview c)
      : name_{n}, age_{a}, city_{c} { }
    Person(sview n, int a)
      : name_{n}, age_{a} { }
    Person(sview n)
      : name_{n} { }
};