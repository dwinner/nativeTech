
// https://godbolt.org/z/zndbvh5nK 
#include <string>
#include <string_view>
using std::string; using sview = std::string_view;

struct Person {
    string name_;
    int age_;
    string city_;
    Person();                         // constructor without arguments
    Person(sview n, int a, sview c);  // constructor with three arguments
    Person(sview n, int a);           // constructor with two arguments
    Person(sview n);                  // constructor with one argument
};

Person::Person() 
  : name_{"no name"}, age_{-1}, city_{"no city"} { } 
Person::Person(sview n, int a, sview c) 
  : name_{n}, age_{a}, city_{c} { } 
Person::Person(sview n, int a)
  : name_{n}, age_{a}, city_{"no city"} { }
Person::Person(sview n)
  : name_{n}, age_{-1}, city_{"no city"} { }