# Listings of Chap15.docx

This is the list of listings on one page.
You can also view a [linked summary](README.md).


## Listing 15.1: The common ancestor of our "Year", "Month", and "Day" helper classes.

Book listing [lst-0005-book.cpp](lst-0005-book.cpp):
```cpp
// https://godbolt.org/z/hhGo46z5a 
#include <iostream>  // ostream
#include <format>    // format, vformat, make_format_args
using std::ostream;
class Value {
protected: // not public, only for own and derived use
    int value_;
    const std::string fmt_;   // e.g. "{:02}" or "{:04}"
    Value(int v, unsigned w)  // constructor with two arguments
      : value_{v}, fmt_{std::format("{{:0{}}}", w)} {}
public:
    ostream& print(ostream& os) const;
};
ostream& operator<<(ostream& os, const Value& right) {
    return right.print(os);
}
ostream& Value::print(ostream& os) const {
    return os << std::vformat(fmt_, std::make_format_args(value_));
}
```

Godbolt Listing [lst-0005-godb.cpp](lst-0005-godb.cpp), [https://godbolt.org/z/hhGo46z5a](https://godbolt.org/z/hhGo46z5a):
```cpp
//#(execute) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/hhGo46z5a 
#include <iostream>  // ostream
#include <format>    // format, vformat, make_format_args
using std::ostream;
class Value {
protected: // not public, only for own and derived use
    int value_;
    const std::string fmt_;   // e.g. "{:02}" or "{:04}"
    Value(int v, unsigned w)  // constructor with two arguments
      : value_{v}, fmt_{std::format("{{:0{}}}", w)} {}
public:
    ostream& print(ostream& os) const;
};
ostream& operator<<(ostream& os, const Value& right) {
    return right.print(os);
}
ostream& Value::print(ostream& os) const {
    return os << std::vformat(fmt_, std::make_format_args(value_));
}
```

----------------


## Listing 15.2: The duplicate code of the helper classes has now disappeared.

Book listing [lst-0006-book.cpp](lst-0006-book.cpp):
```cpp
// https://godbolt.org/z/eYY1js4Te 
class Year : public Value {               // derive from class Value
public:
    explicit Year(int v) : Value{v, 4} {} // initialize base class
};
class Month : public Value {
public:
    explicit Month(int v) : Value{v, 2} {}
};
struct Day : public Value {               // class-public corresponds to struct
    explicit Day(int v) : Value{v, 2} {}
};
```

Godbolt Listing [lst-0006-godb.cpp](lst-0006-godb.cpp), [https://godbolt.org/z/eYY1js4Te](https://godbolt.org/z/eYY1js4Te):
```cpp
//#(execute) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/eYY1js4Te 
class Year : public Value {               // derive from class Value
public:
    explicit Year(int v) : Value{v, 4} {} // initialize base class
};
class Month : public Value {
public:
    explicit Month(int v) : Value{v, 2} {}
};
struct Day : public Value {               // class-public corresponds to struct
    explicit Day(int v) : Value{v, 2} {}
};
```

----------------


## Listing 15.3: This is how 'Date' uses the new classes.

Book listing [lst-0007-book.cpp](lst-0007-book.cpp):
```cpp
// https://godbolt.org/z/9rY7qhK89 
class Date {
    Year year_;
    Month month_ {1};
    Day day_ {1};
public:
    explicit Date(int y) : year_{y} {} // year-01-01
    Date(Year y, Month m, Day d) : year_{y}, month_{m}, day_{d} {}
    ostream& print(ostream& os) const;
};
ostream& Date::print(ostream& os) const {
    return os << year_ << "-" << month_ << "-"  << day_;
}
ostream& operator<<(ostream& os, const Date& right) {
    return right.print(os);
}
int main() {
    Date d1 { Year{2024}, Month{11}, Day{19} };
    std::cout << d1 << "\n"; // Output: 2024-11-19
}
```

Godbolt Listing [lst-0007-godb.cpp](lst-0007-godb.cpp), [https://godbolt.org/z/9rY7qhK89](https://godbolt.org/z/9rY7qhK89):
```cpp
//#(execute) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/9rY7qhK89 
class Date {
    Year year_;
    Month month_ {1};
    Day day_ {1};
public:
    explicit Date(int y) : year_{y} {} // year-01-01
    Date(Year y, Month m, Day d) : year_{y}, month_{m}, day_{d} {}
    ostream& print(ostream& os) const;
};
ostream& Date::print(ostream& os) const {
    return os << year_ << "-" << month_ << "-"  << day_;
}
ostream& operator<<(ostream& os, const Date& right) {
    return right.print(os);
}
int main() {
    Date d1 { Year{2024}, Month{11}, Day{19} };
    std::cout << d1 << "\n"; // Output: 2024-11-19
}
```

----------------


## Listing 15.4: Now "easter" is a method of "Year".

Book listing [lst-0008-book.cpp](lst-0008-book.cpp):
```cpp
// https://godbolt.org/z/zqKvMedz7 (full example)
class Date;                         // forward declaration
class Year : public Value {
public:
    explicit Year(int v) : Value{v, 4} {}
    Date easter() const;            // declare new method
};
// Declare Month, Day, and Date here. Then:
Date Year::easter() const {         // define new method
    const int y = value_;
    int a = value_/100*1483 - value_/400*2225 + 2613;
    int b = (value_%19*3510 + a/25*319)/330%29;
    b = 148 - b - (value_*5/4 + a - b)%7;
    return Date{Year{value_}, Month{b/31}, Day{b%31 + 1}};
}
int main() {
    using std::cout;
    Year year{2025};
    cout << year.easter() << "\n";  // Output: 2025-04-20
}
```

Godbolt Listing [lst-0008-godb.cpp](lst-0008-godb.cpp), [https://godbolt.org/z/zqKvMedz7](https://godbolt.org/z/zqKvMedz7):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/zqKvMedz7 (full example)
class Date;                         // forward declaration
class Year : public Value {
public:
    explicit Year(int v) : Value{v, 4} {}
    Date easter() const;            // declare new method
};
// Declare Month, Day, and Date here. Then:
Date Year::easter() const {         // define new method
    const int y = value_;
    int a = value_/100*1483 - value_/400*2225 + 2613;
    int b = (value_%19*3510 + a/25*319)/330%29;
    b = 148 - b - (value_*5/4 + a - b)%7;
    return Date{Year{value_}, Month{b/31}, Day{b%31 + 1}};
}
int main() {
    using std::cout;
    Year year{2025};
    cout << year.easter() << "\n";  // Output: 2025-04-20
}
```

----------------


## Listing 15.5: All components have a white color; only the button will be gray.

Book listing [lst-0009-book.cpp](lst-0009-book.cpp):
```cpp
struct Component {
    Color getColor() const { return white; }
};
struct Window : public Component { };
struct MainWindow : public Window { };
struct Dialog : public Window { };
struct TextInput : public Component { };
struct Button : public Component {
    Color getColor() const  { return gray; }
};
```

----------------


## Listing 15.6: What does "print" output? The "value" method appears more often.

Book listing [lst-0010-book.cpp](lst-0010-book.cpp):
```cpp
// https://godbolt.org/z/YTK8PfcMc
#include <iostream>
struct Base {
    int eight_ = 8;
    int value() const { return eight_; }
    void print(std::ostream& os) const { os << value() << "\n"; }
};
struct Print : public Base {
    int nine_ = 9;
    void print(std::ostream& os) const { os << value() << "\n"; }
};
struct Value : public Base {
    int ten_ = 10;
    int value() const { return ten_; }
};
struct Both : public Base {
    int eleven_ = 11;
    int value() const { return eleven_; }
    void print(std::ostream& os) const { os << value() << "\n"; }
};
int main() {
    Base ba{}; ba.print(std::cout);   // Base call
    Print pr{}; pr.print(std::cout);  // print overridden
    Value va{}; va.print(std::cout);  // print from Base
    Both bo{}; bo.print(std::cout);   // everything overridden
}
```

Godbolt Listing [lst-0010-godb.cpp](lst-0010-godb.cpp), [https://godbolt.org/z/YTK8PfcMc](https://godbolt.org/z/YTK8PfcMc):
```cpp
//#(compile) c++; compiler:g141; options:; libs:-
// https://godbolt.org/z/YTK8PfcMc
#include <iostream>
struct Base {
    int eight_ = 8;
    int value() const { return eight_; }
    void print(std::ostream& os) const { os << value() << "\n"; }
};
struct Print : public Base {
    int nine_ = 9;
    void print(std::ostream& os) const { os << value() << "\n"; }
};
struct Value : public Base {
    int ten_ = 10;
    int value() const { return ten_; }
};
struct Both : public Base {
    int eleven_ = 11;
    int value() const { return eleven_; }
    void print(std::ostream& os) const { os << value() << "\n"; }
};
int main() {
    Base ba{}; ba.print(std::cout);   // Base call
    Print pr{}; pr.print(std::cout);  // print overridden
    Value va{}; va.print(std::cout);  // print from Base
    Both bo{}; bo.print(std::cout);   // everything overridden
}
```

----------------


## Listing 15.7: Methods marked with "virtual" are resolved at runtime.

Book listing [lst-0011-book.cpp](lst-0011-book.cpp):
```cpp
// https://godbolt.org/z/vW5s3Kh6M 
#include <iostream>

using std::ostream; using std::cout;
struct Base2 {
    int eight_ = 8;
    virtual int value() const          // virtual method
        { return eight_; }
    void print(ostream& os) const
        { os << value() << "\n"; }
};
struct Value2 : public Base2 {
    int ten_ = 10;
    virtual int value() const override // override
        { return ten_; }
};
int main() {
    Value2 v2{}; v2.print(cout);       // use
}
```

Godbolt Listing [lst-0011-godb.cpp](lst-0011-godb.cpp), [https://godbolt.org/z/vW5s3Kh6M](https://godbolt.org/z/vW5s3Kh6M):
```cpp
//#(compile) c++; compiler:g141; options:; libs:-
// https://godbolt.org/z/vW5s3Kh6M 
#include <iostream>

using std::ostream; using std::cout;
struct Base2 {
    int eight_ = 8;
    virtual int value() const          // virtual method
        { return eight_; }
    void print(ostream& os) const
        { os << value() << "\n"; }
};
struct Value2 : public Base2 {
    int ten_ = 10;
    virtual int value() const override // override
        { return ten_; }
};
int main() {
    Value2 v2{}; v2.print(cout);       // use
}
```

----------------


## Listing 15.8: The derived class inherits methods but not constructors from the parent class.

Book listing [lst-0013-book.cpp](lst-0013-book.cpp):
```cpp
// https://godbolt.org/z/59WKfxenE 
class Base {
public:
    Base() {}                 // null-argument constructor
    explicit Base(int i) {}   // one argument
    Base(int i, int j) {}     // two arguments
    void func() {};           // method
};

class Derived : public Base { // no own constructor
};

int main() {
    Base b0{};                // okay, null-argument constructor
    Base b1{12};              // okay, one argument
    Base b2{6,18};            // okay, two arguments
    Derived d0{};             // okay, compiler generates default constructor
    d0.func();                // okay, method is inherited
    Derived d1{7};            // (ERR) Error: no constructor for one argument
    Derived d2{3,13};         // (ERR) Error: no constructor for two arguments
}
```

Godbolt Listing [lst-0013-godb.cpp](lst-0013-godb.cpp), [https://godbolt.org/z/59WKfxenE](https://godbolt.org/z/59WKfxenE):
```cpp
//#(compile) c++; compiler:g141; options:; libs:-
// https://godbolt.org/z/59WKfxenE 
class Base {
public:
    Base() {}                 // null-argument constructor
    explicit Base(int i) {}   // one argument
    Base(int i, int j) {}     // two arguments
    void func() {};           // method
};

class Derived : public Base { // no own constructor
};

int main() {
    Base b0{};                // okay, null-argument constructor
    Base b1{12};              // okay, one argument
    Base b2{6,18};            // okay, two arguments
    Derived d0{};             // okay, compiler generates default constructor
    d0.func();                // okay, method is inherited
    Derived d1{7};            // (ERR) Error: no constructor for one argument
    Derived d2{3,13};         // (ERR) Error: no constructor for two arguments
}
```

----------------


## Listing 15.9: Using "using" to import all parent class constructors.

Book listing [lst-0014-book.cpp](lst-0014-book.cpp):
```cpp
// https://godbolt.org/z/h15eGahjh 
class Base {
public:
    Base() {}
    explicit Base(int i) {}
    Base(int i, int j) {}
    void func() {};           // method
};

class Derived : public Base {
public:
    using Base::Base;         // importing all parent class constructors
};

int main() {
    Derived d0{};             // okay, imported, no longer generated
    Derived d1{7};            // okay, was imported
    Derived d2{3,13};         // okay, was imported
}
```

Godbolt Listing [lst-0014-godb.cpp](lst-0014-godb.cpp), [https://godbolt.org/z/h15eGahjh](https://godbolt.org/z/h15eGahjh):
```cpp
//#(compile) c++; compiler:g141; options:; libs:-
// https://godbolt.org/z/h15eGahjh 
class Base {
public:
    Base() {}
    explicit Base(int i) {}
    Base(int i, int j) {}
    void func() {};           // method
};

class Derived : public Base {
public:
    using Base::Base;         // importing all parent class constructors
};

int main() {
    Derived d0{};             // okay, imported, no longer generated
    Derived d1{7};            // okay, was imported
    Derived d2{3,13};         // okay, was imported
}
```

----------------


## Listing 15.10: Passing by value only copies the common part of the type.

Book listing [lst-0015-book.cpp](lst-0015-book.cpp):
```cpp
// https://godbolt.org/z/Ec7vqEdY3 
//… Base2 and Value2 as before …
void output(Base2 x) {          // pass by value
    x.print(cout);
}
int main() {
    Base2 ba2{}; output(ba2);   // outputs 8
    Value2 va2{}; output(va2);  // also outputs 8
}
```

Godbolt Listing [lst-0015-godb.cpp](lst-0015-godb.cpp), [https://godbolt.org/z/Ec7vqEdY3](https://godbolt.org/z/Ec7vqEdY3):
```cpp
//#(compile) c++; compiler:g141; options:; libs:-
// https://godbolt.org/z/Ec7vqEdY3 
//… Base2 and Value2 as before …
void output(Base2 x) {          // pass by value
    x.print(cout);
}
int main() {
    Base2 ba2{}; output(ba2);   // outputs 8
    Value2 va2{}; output(va2);  // also outputs 8
}
```

----------------


## Listing 15.11: The derived class as an argument type to a function does not allow calling with a base class variable as a value parameter.

Book listing [lst-0016-book.cpp](lst-0016-book.cpp):
```cpp
// https://godbolt.org/z/17nah59eP
//… Base2 and Value2 as before …
void output(Value2 x) {         // derived class as value
    x.print(cout);
}
int main() {
    Base2 ba2{}; output(ba2);   // (ERR) ba2 cannot be converted to Value2
    Value2 va2{}; output(va2);  // outputs 10
}
```

Godbolt Listing [lst-0016-godb.cpp](lst-0016-godb.cpp), [https://godbolt.org/z/17nah59eP](https://godbolt.org/z/17nah59eP):
```cpp
//#(compile) c++; compiler:g141; options:; libs:-
// https://godbolt.org/z/17nah59eP
//… Base2 and Value2 as before …
void output(Value2 x) {         // derived class as value
    x.print(cout);
}
int main() {
    Base2 ba2{}; output(ba2);   // (ERR) ba2 cannot be converted to Value2
    Value2 va2{}; output(va2);  // outputs 10
}
```

----------------


## Listing 15.12: Passing by reference does not change the instance.

Book listing [lst-0017-book.cpp](lst-0017-book.cpp):
```cpp
// https://godbolt.org/z/sW7MexWrh 
//… Base2 and Value2 as before …
void output(Base2& x) {       // passing as a reference
    x.print(cout);
}

int main() {
    Base2 ba2{}; output(ba2);  // outputs 8
    Value2 va2{}; output(va2); // outputs 10, because the object is not copied
}
```

Godbolt Listing [lst-0017-godb.cpp](lst-0017-godb.cpp), [https://godbolt.org/z/sW7MexWrh](https://godbolt.org/z/sW7MexWrh):
```cpp
//#(compile) c++; compiler:g141; options:; libs:-
// https://godbolt.org/z/sW7MexWrh 
//… Base2 and Value2 as before …
void output(Base2& x) {       // passing as a reference
    x.print(cout);
}

int main() {
    Base2 ba2{}; output(ba2);  // outputs 8
    Value2 va2{}; output(va2); // outputs 10, because the object is not copied
}
```

----------------

