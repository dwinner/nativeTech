
// https://godbolt.org/z/MYzsqfqdo 
#include <string>
#include <string_view>
using std::string; using std::string_view;

class Person { // a class starts with private visibility
    string name_;
    int age_;
    string city_;
public:        // everything from here can be used externally
    Person(string_view n, int a, string_view c)
      : name_{n}, age_{a}, city_{c} { }
    void print();
};