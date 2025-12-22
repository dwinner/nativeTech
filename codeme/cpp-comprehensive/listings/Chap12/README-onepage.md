# Listings of Chap12.docx

This is the list of listings on one page.
You can also view a [linked summary](README.md).


## Listing 12.1: Creating your own data type with "struct".

Book listing [lst-0001-book.cpp](lst-0001-book.cpp):
```cpp
// https://godbolt.org/z/8YqoTEx58 
#include <string>
#include <iostream>                      // cout
#include <format>
using std::string; using std::cout; using std::format;
struct Person {                          // defines the new type Person
    string name_;
    int age_;
    string city_;
};                                       // closing semicolon
void print(Person p) {                   // entire Person as one parameter
    cout << format("{} ({}) from {}\n",
      p.name_, p.age_, p.city_);         // access via dot
}
int main() {
    Person john {"John", 45, "Boston" }; // initialization
    print(john);                         // call as a unit
}
```

Godbolt Listing [lst-0001-godb.cpp](lst-0001-godb.cpp), [https://godbolt.org/z/8YqoTEx58](https://godbolt.org/z/8YqoTEx58):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/8YqoTEx58 
#include <string>
#include <iostream>                      // cout
#include <format>
using std::string; using std::cout; using std::format;
struct Person {                          // defines the new type Person
    string name_;
    int age_;
    string city_;
};                                       // closing semicolon
void print(Person p) {                   // entire Person as one parameter
    cout << format("{} ({}) from {}\n",
      p.name_, p.age_, p.city_);         // access via dot
}
int main() {
    Person john {"John", 45, "Boston" }; // initialization
    print(john);                         // call as a unit
}
```

----------------


## Listing 12.2: The declaration of a class initially only mentions its name, while the definition contains all its elements.

Book listing [lst-0002-book.cpp](lst-0002-book.cpp):
```cpp
// https://godbolt.org/z/zPbK16zWE 
#include <memory>                   // shared_ptr
#include <vector>                   // vector
struct Employee;                    // class declaration
struct Boss;                        // class declaration
struct Employee {                   // class definition
    std::shared_ptr<Boss> boss_;    // pointer to Boss
    void print() const;             // method declaration
};
struct Boss {                       // definition
    std::vector<std::shared_ptr<Employee>> employees_; // Pointer to Employees
    void print() const;             // method declaration
};
void Employee::print() const {      // method definition
    // …
}
void Boss::print() const {          // method definition
    // …
}
```

Godbolt Listing [lst-0002-godb.cpp](lst-0002-godb.cpp), [https://godbolt.org/z/zPbK16zWE](https://godbolt.org/z/zPbK16zWE):
```cpp
//#(compile) c++; compiler:g141; options:; libs:-
// https://godbolt.org/z/zPbK16zWE 
#include <memory>                   // shared_ptr
#include <vector>                   // vector
struct Employee;                    // class declaration
struct Boss;                        // class declaration
struct Employee {                   // class definition
    std::shared_ptr<Boss> boss_;    // pointer to Boss
    void print() const;             // method declaration
};
struct Boss {                       // definition
    std::vector<std::shared_ptr<Employee>> employees_; // Pointer to Employees
    void print() const;             // method declaration
};
void Employee::print() const {      // method definition
    // …
}
void Boss::print() const {          // method definition
    // …
}
```

----------------


## Listing 12.3: With designated initializers, you specify the elements to be set.

Book listing [lst-0005-book.cpp](lst-0005-book.cpp):
```cpp
// https://godbolt.org/z/K8WePajvM 
Person john1 {"John", 45, "Boston" }; // correct
Person john2 {"Boston", 45, "John" }; // oops, swapped, and no one notices
Person jack { .name_ = "Jack", .age_ = 23, .city_ = "Dallas" }; // okay
Person jimmi { .name_ = "Jimmi", .age_ = 48  }; // okay, not all specified
Person carl { "Carl", .age_ = 53  };  // (ERR) all designated or none
Person paul { .age = 34, .name_ = "Paul", .city = "Reno" }; // (ERR) swapped
Person jim(.name_="Jim", .age_=34, .city_="NYC"); // (ERR) not parenthesis
```

Godbolt Listing [lst-0005-godb.cpp](lst-0005-godb.cpp), [https://godbolt.org/z/K8WePajvM](https://godbolt.org/z/K8WePajvM):
```cpp
//#(compile) c++; compiler:g141; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/K8WePajvM 
Person john1 {"John", 45, "Boston" }; // correct
Person john2 {"Boston", 45, "John" }; // oops, swapped, and no one notices
Person jack { .name_ = "Jack", .age_ = 23, .city_ = "Dallas" }; // okay
Person jimmi { .name_ = "Jimmi", .age_ = 48  }; // okay, not all specified
Person carl { "Carl", .age_ = 53  };  // (ERR) all designated or none
Person paul { .age = 34, .name_ = "Paul", .city = "Reno" }; // (ERR) swapped
Person jim(.name_="Jim", .age_=34, .city_="NYC"); // (ERR) not parenthesis
```

----------------


## GodboltId:rrjPorT1o

Book listing [lst-0006-book.cpp](lst-0006-book.cpp):
```cpp
// https://godbolt.org/z/rrjPorT1o
// Snippet
Person create(string name, int age, string city) { // return type
    Person result {name, age, city};
    return result;
}
int main() {
    Person john = create("John", 45, "Boston");    // store return value
    print(john);
}
```

Godbolt Listing [lst-0006-godb.cpp](lst-0006-godb.cpp), [https://godbolt.org/z/rrjPorT1o](https://godbolt.org/z/rrjPorT1o):
```cpp
//#(compile) c++; compiler:g141; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/rrjPorT1o
// Snippet
Person create(string name, int age, string city) { // return type
    Person result {name, age, city};
    return result;
}
int main() {
    Person john = create("John", 45, "Boston");    // store return value
    print(john);
}
```

----------------


## GodboltId:ME76bfbWa

Book listing [lst-0007-book.cpp](lst-0007-book.cpp):
```cpp
// https://godbolt.org/z/ME76bfbWa 
Person create(string name, int age, string city) {
    return Person{name, age, city};       // returned directly
}
int main() {
    print(create("John", 45, "Boston"));  // return value used directly
}
```

Godbolt Listing [lst-0007-godb.cpp](lst-0007-godb.cpp), [https://godbolt.org/z/ME76bfbWa](https://godbolt.org/z/ME76bfbWa):
```cpp
//#(compile) c++; compiler:g141; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/ME76bfbWa 
Person create(string name, int age, string city) {
    return Person{name, age, city};       // returned directly
}
int main() {
    print(create("John", 45, "Boston"));  // return value used directly
}
```

----------------


## Listing 12.4: Here, specifying Person in the return is necessary.

Book listing [lst-0010-book.cpp](lst-0010-book.cpp):
```cpp
// https://godbolt.org/z/jer44aWs9 
auto create(string name, int age, string city) {
    return Person{name, age, city};  // auto requires constructor name
}
auto create2(string name, int age, string city) {
    return {name, age, city};        // (ERR) auto with initializer_list does not work
}
```

Godbolt Listing [lst-0010-godb.cpp](lst-0010-godb.cpp), [https://godbolt.org/z/jer44aWs9](https://godbolt.org/z/jer44aWs9):
```cpp
//#(compile) c++; compiler:g141; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/jer44aWs9 
auto create(string name, int age, string city) {
    return Person{name, age, city};  // auto requires constructor name
}
auto create2(string name, int age, string city) {
    return {name, age, city};        // (ERR) auto with initializer_list does not work
}
```

----------------


## Listing 12.5: Methods bundle data and behavior together.

Book listing [lst-0011-book.cpp](lst-0011-book.cpp):
```cpp
// https://godbolt.org/z/5T1Er36P7 
#include <string>
#include <iostream>
#include <format>
using std::string; using std::cout; using std::format;
struct Person {
  string name_;
  int age_;
  string city_;
  void print();            // function as a method of the type
};
void Person::print() {     // method name is extended by Person::
  cout << format("{} ({}) from {}\n",
    name_, age_, city_);   // in a method you can directly access fields
}
int main() {
  Person john {"John", 45, "Boston" };
  john.print();            // calling the method for a variable of the type
}
```

Godbolt Listing [lst-0011-godb.cpp](lst-0011-godb.cpp), [https://godbolt.org/z/5T1Er36P7](https://godbolt.org/z/5T1Er36P7):
```cpp
//#(compile) c++; compiler:g141; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/5T1Er36P7 
#include <string>
#include <iostream>
#include <format>
using std::string; using std::cout; using std::format;
struct Person {
  string name_;
  int age_;
  string city_;
  void print();            // function as a method of the type
};
void Person::print() {     // method name is extended by Person::
  cout << format("{} ({}) from {}\n",
    name_, age_, city_);   // in a method you can directly access fields
}
int main() {
  Person john {"John", 45, "Boston" };
  john.print();            // calling the method for a variable of the type
}
```

----------------


## Listing 12.6: The "greeting()" method uses fields; via "this", it always refers to the field belonging to the called object.

Book listing [lst-0012-book.cpp](lst-0012-book.cpp):
```cpp
struct Person {
    //… rest as before …
    string greeting();
};
string Person::greeting() {
    return format("Hello {} from {}", this->name_, this->city_);
}
int main() {
    Person anna { "Anna", 33, "Eek" };
    Person nina { "Nina", 22, "Ojo" };
    anna.greeting();
    nina.greeting();
}
```

----------------


## Listing 12.7: How to separate methods and data from each other.

Book listing [lst-0014-book.cpp](lst-0014-book.cpp):
```cpp
string greeting(Person * const p) {  // implicit parameter made explicit
    return format("Hello {} from {}", p->name_, p->city_);
}
```

----------------


## GodboltId:hEvqWqqc6

Book listing [lst-0015-book.cpp](lst-0015-book.cpp):
```cpp
// https://godbolt.org/z/hEvqWqqc6 
int value = 5;                          // global variable
struct Wrap {
    int value = 3;                      // data field
    void set(int value) {               // parameter
        this->value = value + ::value;
    }
};
```

Godbolt Listing [lst-0015-godb.cpp](lst-0015-godb.cpp), [https://godbolt.org/z/hEvqWqqc6](https://godbolt.org/z/hEvqWqqc6):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/hEvqWqqc6 
int value = 5;                          // global variable
struct Wrap {
    int value = 3;                      // data field
    void set(int value) {               // parameter
        this->value = value + ::value;
    }
};
```

----------------


## Listing 12.8: "print" takes a stream as an argument.

Book listing [lst-0018-book.cpp](lst-0018-book.cpp):
```cpp
// https://godbolt.org/z/bz6n3Pjrv 
// Excerpt. Person as before
void Person::print(std::ostream& os) {
    os << format("{} ({}) from {}", name_, age_, city_);
}
int main() {
    Person carl {"Carl", 12, "Toledo"};
    carl.print(cout);          // on the screen
    cout << "\n";
    std::ofstream file {"persons.txt"};
    carl.print(file);          // to a file
    // automatic test:
    std::ostringstream oss{};  // writes to a string
    carl.print(oss);
    if(oss.str() == "Carl (12) from Toledo") {
        cout << "ok\n";
    } else {
        cout << "Error in Person::print!\n";
        return 1;              // propagate error outward
    }
}
```

Godbolt Listing [lst-0018-godb.cpp](lst-0018-godb.cpp), [https://godbolt.org/z/bz6n3Pjrv](https://godbolt.org/z/bz6n3Pjrv):
```cpp
//#(compile) c++; compiler:g141; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/bz6n3Pjrv 
// Excerpt. Person as before
void Person::print(std::ostream& os) {
    os << format("{} ({}) from {}", name_, age_, city_);
}
int main() {
    Person carl {"Carl", 12, "Toledo"};
    carl.print(cout);          // on the screen
    cout << "\n";
    std::ofstream file {"persons.txt"};
    carl.print(file);          // to a file
    // automatic test:
    std::ostringstream oss{};  // writes to a string
    carl.print(oss);
    if(oss.str() == "Carl (12) from Toledo") {
        cout << "ok\n";
    } else {
        cout << "Error in Person::print!\n";
        return 1;              // propagate error outward
    }
}
```

----------------


## Listing 12.9: You can overload the standard operator for output.

Book listing [lst-0019-book.cpp](lst-0019-book.cpp):
```cpp
std::ostream& Person::print(std::ostream& os) {
    return os << format("{} ({}) from {}", name_, age_, city_);
}
std::ostream& operator<<(std::ostream& os, Person p) {
    return p.print(os);
}
```

----------------


## Listing 12.10: The output with << is achieved by overloading a free function.

Book listing [lst-0021-book.cpp](lst-0021-book.cpp):
```cpp
// https://godbolt.org/z/a7q5xsWGj 
// Excerpt …
    std::ostream& print(std::ostream& os);
};
std::ostream& Person::print(std::ostream& os) {
    return os << format("{} ({}) from {}", name_, age_, city_);
}
std::ostream& operator<<(std::ostream& os, Person p) {
    return p.print(os);
}
int main() {
    Person paul {"Paul", 23, "Irvine"};
    cout << "You are " << paul << ", right?\n";
}
```

Godbolt Listing [lst-0021-godb.cpp](lst-0021-godb.cpp), [https://godbolt.org/z/a7q5xsWGj](https://godbolt.org/z/a7q5xsWGj):
```cpp
//#(compile) c++; compiler:g141; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/a7q5xsWGj 
// Excerpt …
    std::ostream& print(std::ostream& os);
};
std::ostream& Person::print(std::ostream& os) {
    return os << format("{} ({}) from {}", name_, age_, city_);
}
std::ostream& operator<<(std::ostream& os, Person p) {
    return p.print(os);
}
int main() {
    Person paul {"Paul", 23, "Irvine"};
    cout << "You are " << paul << ", right?\n";
}
```

----------------


## Listing 12.11: Methods can also be defined inline.

Book listing [lst-0022-book.cpp](lst-0022-book.cpp):
```cpp
// https://godbolt.org/z/Y3eW1Kh8v 
#include <string>
#include <iostream>    // ostream
#include <format>

using std::string; using std::ostream; using std::format;

struct Person {
  string name_;
  int age_;
  string city_;
  ostream& print(ostream& os) {       // method defined inline
    return os << format("{} ({}) from {}", name_, age_, city_);
  }
};
```

Godbolt Listing [lst-0022-godb.cpp](lst-0022-godb.cpp), [https://godbolt.org/z/Y3eW1Kh8v](https://godbolt.org/z/Y3eW1Kh8v):
```cpp
//#(compile) c++; compiler:g141; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/Y3eW1Kh8v 
#include <string>
#include <iostream>    // ostream
#include <format>

using std::string; using std::ostream; using std::format;

struct Person {
  string name_;
  int age_;
  string city_;
  ostream& print(ostream& os) {       // method defined inline
    return os << format("{} ({}) from {}", name_, age_, city_);
  }
};
```

----------------


## Listing 12.12: How to define a constructor.

Book listing [lst-0027-book.cpp](lst-0027-book.cpp):
```cpp
struct Person {
    string name_;
    int age_;
    string city_;
    Person();          // declare constructor
};
Person::Person()
  : name_{"no name"}   // initialization value for name_
  , age_{-1}           // initialization value for age_
  , city_{"no city"}   // initialization value for city_
{ }                    // empty function body
```

----------------


## Listing 12.13: Multiple constructors are also possible.

Book listing [lst-0029-book.cpp](lst-0029-book.cpp):
```cpp
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
```

Godbolt Listing [lst-0029-godb.cpp](lst-0029-godb.cpp), [https://godbolt.org/z/zndbvh5nK](https://godbolt.org/z/zndbvh5nK):
```cpp
//#(compile) c++; compiler:g141; options:-O3 -std=c++23; libs:-
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
```

----------------


## Listing 12.14: Member variables can be equipped with default values.

Book listing [lst-0030-book.cpp](lst-0030-book.cpp):
```cpp
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
```

Godbolt Listing [lst-0030-godb.cpp](lst-0030-godb.cpp), [https://godbolt.org/z/Wo8zaq9WW](https://godbolt.org/z/Wo8zaq9WW):
```cpp
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
```

----------------


## Listing 12.15: A constructor can pass part of the initialization to another constructor.

Book listing [lst-0031-book.cpp](lst-0031-book.cpp):
```cpp
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
```

Godbolt Listing [lst-0031-godb.cpp](lst-0031-godb.cpp), [https://godbolt.org/z/8h93f64Yj](https://godbolt.org/z/8h93f64Yj):
```cpp
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
```

----------------


## Listing 12.16: A constructor can also be overloaded with default parameters.

Book listing [lst-0032-book.cpp](lst-0032-book.cpp):
```cpp
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
```

Godbolt Listing [lst-0032-godb.cpp](lst-0032-godb.cpp), [https://godbolt.org/z/Gfrd5cceG](https://godbolt.org/z/Gfrd5cceG):
```cpp
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
```

----------------


## Listing 12.17: A constructor with all preset arguments becomes the default constructor.

Book listing [lst-0034-book.cpp](lst-0034-book.cpp):
```cpp
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
```

Godbolt Listing [lst-0034-godb.cpp](lst-0034-godb.cpp), [https://godbolt.org/z/1fE15Kf39](https://godbolt.org/z/1fE15Kf39):
```cpp
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
```

----------------


## Listing 12.18: Do not call an initializing method in the constructor body.

Book listing [lst-0035-book.cpp](lst-0035-book.cpp):
```cpp
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
```

Godbolt Listing [lst-0035-godb.cpp](lst-0035-godb.cpp), [https://godbolt.org/z/dzjaf6nbb](https://godbolt.org/z/dzjaf6nbb):
```cpp
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
```

----------------


## Listing 12.19: External access to the data fields of a type.

Book listing [lst-0037-book.cpp](lst-0037-book.cpp):
```cpp
ostream& operator<<(ostream& os, Person p) {
    return os << format("{} ({}) from {}", p.name_, p.age_, p.city_);
}
```

----------------


## Listing 12.20: Divide a type into multiple sections with "public" and "private".

Book listing [lst-0038-book.cpp](lst-0038-book.cpp):
```cpp
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
```

Godbolt Listing [lst-0038-godb.cpp](lst-0038-godb.cpp), [https://godbolt.org/z/4MzxaevKa](https://godbolt.org/z/4MzxaevKa):
```cpp
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
```

----------------


## Listing 12.21: A "class" starts with private visibility.

Book listing [lst-0039-book.cpp](lst-0039-book.cpp):
```cpp
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
```

Godbolt Listing [lst-0039-godb.cpp](lst-0039-godb.cpp), [https://godbolt.org/z/MYzsqfqdo](https://godbolt.org/z/MYzsqfqdo):
```cpp
//#(compile) c++; compiler:g141; options:-O3 -std=c++23; libs:-
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
```

----------------


## Listing 12.22: Parts of the data are private.

Book listing [lst-0040-book.cpp](lst-0040-book.cpp):
```cpp
// https://godbolt.org/z/Pr76Y55P4 
class Rect {
    int area_;  // private data
public:
    int x_, y_;
    void set(int x, int y) { x_=x; y_=y; area_=x_*y_; }
    int calc() { return area_; }
};
```

Godbolt Listing [lst-0040-godb.cpp](lst-0040-godb.cpp), [https://godbolt.org/z/Pr76Y55P4](https://godbolt.org/z/Pr76Y55P4):
```cpp
//#(execute) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/Pr76Y55P4 
class Rect {
    int area_;  // private data
public:
    int x_, y_;
    void set(int x, int y) { x_=x; y_=y; area_=x_*y_; }
    int calc() { return area_; }
};
```

----------------


## Listing 12.23: With "= default", you let the compiler generate code.

Book listing [lst-0041-book.cpp](lst-0041-book.cpp):
```cpp
// https://godbolt.org/z/PaG9nTs7s 
class Rect {
    int area_;          // private data

public:
    int x_, y_;
    void set(int x, int y) { x_=x; y_=y; area_=x_*y_; }
    int calc() { return area_; }
    Rect() = default;   // let the compiler generate a constructor
};
class Pow {
    int result_;        // private data; holds 'base' raised to 'exp'
public:
    int base_, exp_;
    void set(int b, int e) { /* ... */ }
    int calc() { return result_; }
    Pow() : result_{1},base_{1},exp_{1} {} // initialize sensibly
};
```

Godbolt Listing [lst-0041-godb.cpp](lst-0041-godb.cpp), [https://godbolt.org/z/PaG9nTs7s](https://godbolt.org/z/PaG9nTs7s):
```cpp
//#(execute) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/PaG9nTs7s 
class Rect {
    int area_;          // private data

public:
    int x_, y_;
    void set(int x, int y) { x_=x; y_=y; area_=x_*y_; }
    int calc() { return area_; }
    Rect() = default;   // let the compiler generate a constructor
};
class Pow {
    int result_;        // private data; holds 'base' raised to 'exp'
public:
    int base_, exp_;
    void set(int b, int e) { /* ... */ }
    int calc() { return result_; }
    Pow() : result_{1},base_{1},exp_{1} {} // initialize sensibly
};
```

----------------


## Listing 12.24: Custom data types can protect against errors. This is the first step toward that.

Book listing [lst-0042-book.cpp](lst-0042-book.cpp):
```cpp
// https://godbolt.org/z/aGd56xzsT 
#include <string>       // string, stoi
#include <iostream>     // cin, cout, ostream
#include <format>
using std::ostream; using std::format;
class Year {  /* Helper types for safe date */
    int value_; // e.g., 2024
public:
    Year(int v) : value_{v} {}
    int value() { return value_; }
};
class Month {
    int value_; // 1..12
public:
    Month(int v) : value_{v} {}
    int value() { return value_; }
};
class Day {
    int value_; // 1..31
public:
    Day(int v) : value_{v} {}
    int value() { return value_; }
};
/* type-safe constructing date */
class Date {
    Year year_;
    Month month_ = 1;
    Day day_ = 1;
public:
    Date(int y) : year_{y}           // 1-argument constructor
        {}                           // sets 1st January of the specified year
    Date(Year y, Month m, Day d)     // 3-argument constructor
        : year_{y}, month_{m}, day_{d}
        {}
    ostream& print(ostream& os);     // e.g., 2024-04-20
};
ostream& Date::print(ostream& os) {  // e.g., 2024-04-20
    return os << format("{}-{:02}-{:02}",
        year_.value(), month_.value(), day_.value());
}
ostream& operator<<(ostream& os, Date d) {
    return d.print(os);
}
// http://codegolf.stackexchange.com/a/11146/1405, user Fors, 2014-02-25
Date easter(Year year) {
    const int y = year.value();
    int a = y/100*1483 - y/400*2225 + 2613;
    int b = (y%19*3510 + a/25*319)/330%29;
    b = 148 - b - (y*5/4 + a - b)%7;
    return Date{Year{y}, Month{b/31}, Day{b%31 + 1}}; // Create date
}
int main(int argc, const char *argv[] ) {
    /* Input */
    int number {};
    if(argc > 1) {
        number = std::stoi(argv[1]);
    } else {
        std::cout << "Year? "; std::cin >> number;
    }
    /* Calculation */
    Date date = easter(number);  // implicit conversion to Year
    /* Output */
    std::cout << "Easter: " << date << "\n";
}
```

Godbolt Listing [lst-0042-godb.cpp](lst-0042-godb.cpp), [https://godbolt.org/z/aGd56xzsT](https://godbolt.org/z/aGd56xzsT):
```cpp
//#(compile) c++; compiler:g141; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/aGd56xzsT 
#include <string>       // string, stoi
#include <iostream>     // cin, cout, ostream
#include <format>
using std::ostream; using std::format;
class Year {  /* Helper types for safe date */
    int value_; // e.g., 2024
public:
    Year(int v) : value_{v} {}
    int value() { return value_; }
};
class Month {
    int value_; // 1..12
public:
    Month(int v) : value_{v} {}
    int value() { return value_; }
};
class Day {
    int value_; // 1..31
public:
    Day(int v) : value_{v} {}
    int value() { return value_; }
};
/* type-safe constructing date */
class Date {
    Year year_;
    Month month_ = 1;
    Day day_ = 1;
public:
    Date(int y) : year_{y}           // 1-argument constructor
        {}                           // sets 1st January of the specified year
    Date(Year y, Month m, Day d)     // 3-argument constructor
        : year_{y}, month_{m}, day_{d}
        {}
    ostream& print(ostream& os);     // e.g., 2024-04-20
};
ostream& Date::print(ostream& os) {  // e.g., 2024-04-20
    return os << format("{}-{:02}-{:02}",
        year_.value(), month_.value(), day_.value());
}
ostream& operator<<(ostream& os, Date d) {
    return d.print(os);
}
// http://codegolf.stackexchange.com/a/11146/1405, user Fors, 2014-02-25
Date easter(Year year) {
    const int y = year.value();
    int a = y/100*1483 - y/400*2225 + 2613;
    int b = (y%19*3510 + a/25*319)/330%29;
    b = 148 - b - (y*5/4 + a - b)%7;
    return Date{Year{y}, Month{b/31}, Day{b%31 + 1}}; // Create date
}
int main(int argc, const char *argv[] ) {
    /* Input */
    int number {};
    if(argc > 1) {
        number = std::stoi(argv[1]);
    } else {
        std::cout << "Year? "; std::cin >> number;
    }
    /* Calculation */
    Date date = easter(number);  // implicit conversion to Year
    /* Output */
    std::cout << "Easter: " << date << "\n";
}
```

----------------


## Listing 12.25: If all "return" statements return the same variable, the compiler can always avoid a copy.

Book listing [lst-0051-book.cpp](lst-0051-book.cpp):
```cpp
// https://godbolt.org/z/T7afh4vxz 
#include <vector>
std::vector<int> createData(unsigned size) {
    std::vector<int> result{};
    for(int idx=0; idx<size; ++idx) {
        result.push_back(idx);
    }
    return result;
}
```

Godbolt Listing [lst-0051-godb.cpp](lst-0051-godb.cpp), [https://godbolt.org/z/T7afh4vxz](https://godbolt.org/z/T7afh4vxz):
```cpp
//#(execute) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/T7afh4vxz 
#include <vector>
std::vector<int> createData(unsigned size) {
    std::vector<int> result{};
    for(int idx=0; idx<size; ++idx) {
        result.push_back(idx);
    }
    return result;
}
```

----------------


## Listing 12.26: With "explicit", you prevent automatic type conversion.

Book listing [lst-0060-book.cpp](lst-0060-book.cpp):
```cpp
// only excerpts
class Year {
    explicit Year(int v) : value_{v} {}
};
class Month {
    explicit Month(int v) : value_{v} {}
};
class Day {
    explicit Day(int v) : value_{v} {}
};
class Date {
    explicit Date(int y) : year_{y} {}
};
```

----------------


## Listing 12.27: "Year" no longer has "value()" and requires other methods.

Book listing [lst-0070-book.cpp](lst-0070-book.cpp):
```cpp
// https://godbolt.org/z/EPz4Kd6jP 
class Year {
    int value_;
public:
    explicit Year(int v) : value_{v} {}
    std::ostream& print(std::ostream& os) const;
    Year& advance(const Year& other);
    bool equals(const Year& other) const;
    bool less_than(const Year& other) const;
};
std::ostream& Year::print(std::ostream& os) const {
    return os << value_;
}
std::ostream& operator<<(std::ostream& os, const Year& year) {
    return year.print(os);
}
Year& Year::advance(const Year& other) {
    value_ += other.value_;
    return *this;
}
bool Year::equals(const Year& other) const {
    return value_ == other.value_;
}
bool Year::less_than(const Year& other) const {
    return value_ < other.value_;
}
```

Godbolt Listing [lst-0070-godb.cpp](lst-0070-godb.cpp), [https://godbolt.org/z/EPz4Kd6jP](https://godbolt.org/z/EPz4Kd6jP):
```cpp
//#(execute) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/EPz4Kd6jP 
class Year {
    int value_;
public:
    explicit Year(int v) : value_{v} {}
    std::ostream& print(std::ostream& os) const;
    Year& advance(const Year& other);
    bool equals(const Year& other) const;
    bool less_than(const Year& other) const;
};
std::ostream& Year::print(std::ostream& os) const {
    return os << value_;
}
std::ostream& operator<<(std::ostream& os, const Year& year) {
    return year.print(os);
}
Year& Year::advance(const Year& other) {
    value_ += other.value_;
    return *this;
}
bool Year::equals(const Year& other) const {
    return value_ == other.value_;
}
bool Year::less_than(const Year& other) const {
    return value_ < other.value_;
}
```

----------------


## Listing 12.28: If "advance" returns the object itself, then you can call another method afterward.

Book listing [lst-0071-book.cpp](lst-0071-book.cpp):
```cpp
Year year{2024};
year.advance(Year{1}).advance(Year{3});
cout << year; // Output: 2028
```

----------------


## Listing 12.29: A fluent programming interface sometimes allows for clear code.

Book listing [lst-0072-book.cpp](lst-0072-book.cpp):
```cpp
Page page = Html().body()
    .h1("Heading")
    .table().border(0)
      .tr()
        .td().css("head").text("Dog Breed").end()
        .td().text("Poodle").end()
      .end()
    .end()
    .toPage();
```

----------------


## Listing 12.30: The internal type "int" has become part of the class interface.

Book listing [lst-0073-book.cpp](lst-0073-book.cpp):
```cpp
// https://godbolt.org/z/hbsbMsGr1
class Year {
    int value_;                         // actually internally used type
public:
    explicit Year(int v) : value_{v} {} // type becomes part of the interface
    int value() { return value_; }      // also in the return
};
int main() {
   Year year{ 2024 };                   // type int
   int val = year.value();              // matching type
}
```

Godbolt Listing [lst-0073-godb.cpp](lst-0073-godb.cpp), [https://godbolt.org/z/hbsbMsGr1](https://godbolt.org/z/hbsbMsGr1):
```cpp
//#(compile) c++; compiler:g141; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/hbsbMsGr1
class Year {
    int value_;                         // actually internally used type
public:
    explicit Year(int v) : value_{v} {} // type becomes part of the interface
    int value() { return value_; }      // also in the return
};
int main() {
   Year year{ 2024 };                   // type int
   int val = year.value();              // matching type
}
```

----------------


## Listing 12.31: With "using", you can introduce type aliases that make it easier to maintain interfaces than with the types themselves.

Book listing [lst-0074-book.cpp](lst-0074-book.cpp):
```cpp
// https://godbolt.org/z/EYjo8E8Gn 
class Year {
public:
    using value_type = int;            // introduce type alias
    value_type value_;                 // actually internally used type
public:
    explicit Year(value_type v) : value_{v} {}
    value_type value() { return value_; }
};
int main() {
   Year year{ 2024 };                   // rely on compiler conversion here
   Year::value_type val = year.value(); // use ::
}
```

Godbolt Listing [lst-0074-godb.cpp](lst-0074-godb.cpp), [https://godbolt.org/z/EYjo8E8Gn](https://godbolt.org/z/EYjo8E8Gn):
```cpp
//#(compile) c++; compiler:g141; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/EYjo8E8Gn 
class Year {
public:
    using value_type = int;            // introduce type alias
    value_type value_;                 // actually internally used type
public:
    explicit Year(value_type v) : value_{v} {}
    value_type value() { return value_; }
};
int main() {
   Year year{ 2024 };                   // rely on compiler conversion here
   Year::value_type val = year.value(); // use ::
}
```

----------------


## Listing 12.32: The standard library also contains many useful type aliases.

Book listing [lst-0075-book.cpp](lst-0075-book.cpp):
```cpp
// https://godbolt.org/z/zdMfoa7Yj 
#include <vector>
#include <set>
#include <iostream>
using std::vector; using std::set; using std::cout;
using vector_t = vector<unsigned long long>; // Your own type alias
int main() {
    vector_t huge{ 12ULL, 10'000'000'000ULL, 9ULL, 0ULL,  };
    vector_t::size_type sz = huge.size();
    vector_t::value_type uiuiui = huge[1];
    for(vector_t::iterator it = huge.begin(); it != huge.end(); ++it)
        *it *= 2; // double
    /* sort via set */
    set<vector_t::value_type> sorted{huge.begin(), huge.end()};
    for(vector_t::value_type val : sorted)
        cout << val << " ";
    cout << "\n";
}
```

Godbolt Listing [lst-0075-godb.cpp](lst-0075-godb.cpp), [https://godbolt.org/z/zdMfoa7Yj](https://godbolt.org/z/zdMfoa7Yj):
```cpp
//#(compile) c++; compiler:g141; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/zdMfoa7Yj 
#include <vector>
#include <set>
#include <iostream>
using std::vector; using std::set; using std::cout;
using vector_t = vector<unsigned long long>; // Your own type alias
int main() {
    vector_t huge{ 12ULL, 10'000'000'000ULL, 9ULL, 0ULL,  };
    vector_t::size_type sz = huge.size();
    vector_t::value_type uiuiui = huge[1];
    for(vector_t::iterator it = huge.begin(); it != huge.end(); ++it)
        *it *= 2; // double
    /* sort via set */
    set<vector_t::value_type> sorted{huge.begin(), huge.end()};
    for(vector_t::value_type val : sorted)
        cout << val << " ";
    cout << "\n";
}
```

----------------


## Listing 12.33: When initializing a variable, the compiler can determine the type.

Book listing [lst-0079-book.cpp](lst-0079-book.cpp):
```cpp
// https://godbolt.org/z/Wh71cf7j1 
#include <vector>
#include <set>
#include <iostream> // cout
using std::vector; using std::set; using std::cout;
using vector_t = vector<unsigned long long>; // Your own type alias
int main() {
    vector_t huge{ 12ULL, 10000000000ULL, 9ULL, 0ULL,  };
    auto sz = huge.size();
    auto uiuiui = huge[1];
    for(auto it = huge.begin(); it != huge.end(); ++it)
        *it *= 2; // double
    /* sort via set */
    set sorted(huge.begin(), huge.end());  // set<vector_t::value_type>
    for(auto val : sorted)
        cout << val << " ";
    cout << "\n";
}
```

Godbolt Listing [lst-0079-godb.cpp](lst-0079-godb.cpp), [https://godbolt.org/z/Wh71cf7j1](https://godbolt.org/z/Wh71cf7j1):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/Wh71cf7j1 
#include <vector>
#include <set>
#include <iostream> // cout
using std::vector; using std::set; using std::cout;
using vector_t = vector<unsigned long long>; // Your own type alias
int main() {
    vector_t huge{ 12ULL, 10000000000ULL, 9ULL, 0ULL,  };
    auto sz = huge.size();
    auto uiuiui = huge[1];
    for(auto it = huge.begin(); it != huge.end(); ++it)
        *it *= 2; // double
    /* sort via set */
    set sorted(huge.begin(), huge.end());  // set<vector_t::value_type>
    for(auto val : sorted)
        cout << val << " ";
    cout << "\n";
}
```

----------------


## Listing 12.34: Type deduction with "auto" can be further restricted using a concept.

Book listing [lst-0080-book.cpp](lst-0080-book.cpp):
```cpp
// https://godbolt.org/z/qzWchWoeK 
#include <vector>
#include <set>
#include <iostream> // cout
#include <concepts> // integral
#include <iterator> // output_iterator, input_iterator
using namespace std;
using vector_t = vector<unsigned long long>;   // your own type alias
int main() {
  vector_t huge{ 12ULL, 10000000000ULL, 9ULL, 0ULL,  };
  unsigned_integral auto sz = huge.size();
  unsigned_integral auto uiuiui = huge[1];
  signed_integral auto meh = huge[1];          // (ERR) concept not fulfilled
  input_or_output_iterator auto itx = huge.begin(); // concept without parameter
  for(output_iterator<unsigned long long> auto it=huge.begin();
      it!=huge.end(); ++it)
    *it *= 2; // double
  /* sort using set */
  set sorted(huge.begin(), huge.end());        // set<vector_t::value_type>
  for(const unsigned_integral auto& val : sorted)
    cout << val << " ";
  cout << "\n";
}
```

Godbolt Listing [lst-0080-godb.cpp](lst-0080-godb.cpp), [https://godbolt.org/z/qzWchWoeK](https://godbolt.org/z/qzWchWoeK):
```cpp
//#(compile) c++; compiler:g141; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/qzWchWoeK 
#include <vector>
#include <set>
#include <iostream> // cout
#include <concepts> // integral
#include <iterator> // output_iterator, input_iterator
using namespace std;
using vector_t = vector<unsigned long long>;   // your own type alias
int main() {
  vector_t huge{ 12ULL, 10000000000ULL, 9ULL, 0ULL,  };
  unsigned_integral auto sz = huge.size();
  unsigned_integral auto uiuiui = huge[1];
  signed_integral auto meh = huge[1];          // (ERR) concept not fulfilled
  input_or_output_iterator auto itx = huge.begin(); // concept without parameter
  for(output_iterator<unsigned long long> auto it=huge.begin();
      it!=huge.end(); ++it)
    *it *= 2; // double
  /* sort using set */
  set sorted(huge.begin(), huge.end());        // set<vector_t::value_type>
  for(const unsigned_integral auto& val : sorted)
    cout << val << " ";
  cout << "\n";
}
```

----------------


## Listing 12.35: When you enrich "auto" with &, you get a modifiable reference.

Book listing [lst-0081-book.cpp](lst-0081-book.cpp):
```cpp
// https://godbolt.org/z/ro6fdsvT9 
#include <vector>
#include <iostream> // cout
using std::vector; using std::cout;
int main() {
    vector data{ 12, 100, -1, 0,  };    // vector<int>
    for(auto& val : data)
        val *= 2;                       // double it
    for(const auto val : data)
        cout << val << " ";
    cout << "\n";
}
```

Godbolt Listing [lst-0081-godb.cpp](lst-0081-godb.cpp), [https://godbolt.org/z/ro6fdsvT9](https://godbolt.org/z/ro6fdsvT9):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/ro6fdsvT9 
#include <vector>
#include <iostream> // cout
using std::vector; using std::cout;
int main() {
    vector data{ 12, 100, -1, 0,  };    // vector<int>
    for(auto& val : data)
        val *= 2;                       // double it
    for(const auto val : data)
        cout << val << " ";
    cout << "\n";
}
```

----------------


## Listing 12.36: To pack a custom data type into a "vector", it does not need to meet many requirements.

Book listing [lst-0085-book.cpp](lst-0085-book.cpp):
```cpp
// https://godbolt.org/z/8ejPs6zr6 
#include <vector>
struct Number {
    int value_ = 0;
    Number() {} // Default constructor
    explicit Number(int v) : value_{v} {}
};
int main() {
    std::vector<Number> numbers{}; // okay: Number meets the requirements
    numbers.push_back( Number{2} );
}
```

Godbolt Listing [lst-0085-godb.cpp](lst-0085-godb.cpp), [https://godbolt.org/z/8ejPs6zr6](https://godbolt.org/z/8ejPs6zr6):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/8ejPs6zr6 
#include <vector>
struct Number {
    int value_ = 0;
    Number() {} // Default constructor
    explicit Number(int v) : value_{v} {}
};
int main() {
    std::vector<Number> numbers{}; // okay: Number meets the requirements
    numbers.push_back( Number{2} );
}
```

----------------


## Listing 12.37: For a "set" of a custom data type, you need to override "operator<".

Book listing [lst-0086-book.cpp](lst-0086-book.cpp):
```cpp
// https://godbolt.org/z/Yhzqo6dxT
#include <set>
struct Number {
    int value_ = 0;
    explicit Number(int v) : value_{v} {}
};
bool operator<(const Number& left, const Number& right) {
    return left.value_ < right.value_;
}
int main() {
    std::set<Number> numbers{};   // okay
    numbers.insert( Number{3} );  // operator< is needed here
}
```

Godbolt Listing [lst-0086-godb.cpp](lst-0086-godb.cpp), [https://godbolt.org/z/Yhzqo6dxT](https://godbolt.org/z/Yhzqo6dxT):
```cpp
//#(compile) c++; compiler:g141; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/Yhzqo6dxT
#include <set>
struct Number {
    int value_ = 0;
    explicit Number(int v) : value_{v} {}
};
bool operator<(const Number& left, const Number& right) {
    return left.value_ < right.value_;
}
int main() {
    std::set<Number> numbers{};   // okay
    numbers.insert( Number{3} );  // operator< is needed here
}
```

----------------


## GodboltId:TdEnhKxGK

Book listing [lst-0087-book.cpp](lst-0087-book.cpp):
```cpp
// https://godbolt.org/z/TdEnhKxGK
#include <map>

struct Number {
    int value_ = 0;
    explicit Number(int v) : value_{v} {}
};

bool operator<(const Number& left, const Number& right) {
    return left.value_ < right.value_;
}

int main() {
    std::map<Number,int> numbers{};                  // okay
    numbers.insert( std::make_pair(Number{4},100) ); // needs operator<
    numbers[Number{5}] = 200;                        // here as well
}
```

Godbolt Listing [lst-0087-godb.cpp](lst-0087-godb.cpp), [https://godbolt.org/z/TdEnhKxGK](https://godbolt.org/z/TdEnhKxGK):
```cpp
//#(compile) c++; compiler:g141; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/TdEnhKxGK
#include <map>

struct Number {
    int value_ = 0;
    explicit Number(int v) : value_{v} {}
};

bool operator<(const Number& left, const Number& right) {
    return left.value_ < right.value_;
}

int main() {
    std::map<Number,int> numbers{};                  // okay
    numbers.insert( std::make_pair(Number{4},100) ); // needs operator<
    numbers[Number{5}] = 200;                        // here as well
}
```

----------------

