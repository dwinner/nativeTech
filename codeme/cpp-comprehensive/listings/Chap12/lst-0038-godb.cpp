//#(compile) c++; compiler:g141; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/4MzxaevKa 
#include <string>
#include <string_view>
using std::string; using std::string_view;

struct Person {
private: // everything from here cannot be used externally
    string name_;
    int age_;
    string city_;
public:  // everything from here can be used externally
    Person(string_view n, int a, string_view c)
      : name_{n}, age_{a}, city_{c} { }
    void print();
};