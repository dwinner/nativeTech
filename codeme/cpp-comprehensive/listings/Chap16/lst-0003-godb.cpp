//#(compile) c++; compiler:g141; options:-O2 -std=c++23; libs:-
// https://godbolt.org/z/WPcTv3zKM 
#include <string>
#include <iostream>
#include <iomanip>   // setw
using std::cout; using std::setw; using std::string;
struct MyValue {
    static int counter;         // static: exists only once for all instances
    int number_;                // indentation level of this instance for output
    string name_;               // name of this instance for output
    explicit MyValue(string name)
        : number_{++counter}    // increment indentation level per instance
        , name_{name}           // remember the object's name for output
    {
        cout << setw(number_) << " " // use number_ for indentation
             << "Constructor " << name_ << "\n"; // output instance name        
    }
    ~MyValue() {                // destructor
        cout << setw(number_) << " " << "Destructor " << name_ << "\n";
    }
};
int MyValue::counter = 0;       // initialization of the static class variable