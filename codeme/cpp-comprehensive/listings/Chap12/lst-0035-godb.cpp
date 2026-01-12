//#(compile) c++; compiler:g141; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/dzjaf6nbb 
#include <string>
#include <string_view>
using std::string; using sview = std::string_view;

struct Person {
    string name_;
    int age_;
    string city_;
    Person(sview n, int a, sview c)
    {                   // (ERR) Initialization list missing
        init(n, a, c);  // (ERR) questionable »initialization call«
    }
    void init(sview n, int a, sview c) {
      name_ = n; age_ = a; city_ = c;
    }
};