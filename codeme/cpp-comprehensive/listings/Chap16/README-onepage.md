# Listings of Chap16.docx

This is the list of listings on one page.
You can also view a [linked summary](README.md).


## Listing 16.1: Which variables can you use for "x1" to "x5"?

Book listing [lst-0001-book.cpp](lst-0001-book.cpp):
```cpp
// https://godbolt.org/z/9jGKPbzqo
struct MyValue { /* something */ };
MyValue globalValue{};                   // global class instance

void function(const MyValue &paramRef) {
    if( /*...*/ ) function( /*x1?*/ );   // call some function
    MyValue localValue{};                // local class instance
}                                        // end of function

int main() {
    MyValue mvalue1{};
    function( /*x2?*/ );
    function( MyValue{} );               // temporary value
    {
        MyValue mvalue2{};
        function( /*x3?*/ );
        MyValue mvalue3{};
    }                                    // end of inner block
    function( /*x4?*/ );
    MyValue mvalue4{};
    function( /*x5?*/ );
}                                        // end of main function
```

Godbolt Listing [lst-0001-godb.cpp](lst-0001-godb.cpp), [https://godbolt.org/z/9jGKPbzqo](https://godbolt.org/z/9jGKPbzqo):
```cpp
//#(compile) c++; compiler:g141; options:-O2 -std=c++23; libs:-
// https://godbolt.org/z/9jGKPbzqo
struct MyValue { /* something */ };
MyValue globalValue{};                   // global class instance

void function(const MyValue &paramRef) {
    if( /*...*/ ) function( /*x1?*/ );   // call some function
    MyValue localValue{};                // local class instance
}                                        // end of function

int main() {
    MyValue mvalue1{};
    function( /*x2?*/ );
    function( MyValue{} );               // temporary value
    {
        MyValue mvalue2{};
        function( /*x3?*/ );
        MyValue mvalue3{};
    }                                    // end of inner block
    function( /*x4?*/ );
    MyValue mvalue4{};
    function( /*x5?*/ );
}                                        // end of main function
```

----------------


## Listing 16.2: When you call a function with a parameter for which the compiler invokes a constructor to perform a conversion, it creates a temp-value.

Book listing [lst-0002-book.cpp](lst-0002-book.cpp):
```cpp
// https://godbolt.org/z/eK1Ervjs5 
#include <string>
#include <iostream>                      // cout
using std::string; using std::cout;

struct Value {
    int value_;
    Value(int value)                     // 1-arg constructor = type conversion
        : value_{value} {}
};

size_t length(string arg) {
    return arg.size();
}
Value twice(Value v) {
    return Value{ v.value_*2 };
}
int main() {
    cout << length("Hipphopp") << "\n";  // const char* to string
    cout << twice(10).value_ << "\n";    // int to Value
    string name {"Gandalf"};
    cout << ( name + " the Grey" ) << "\n"; // string + const char*
}
```

Godbolt Listing [lst-0002-godb.cpp](lst-0002-godb.cpp), [https://godbolt.org/z/eK1Ervjs5](https://godbolt.org/z/eK1Ervjs5):
```cpp
//#(compile) c++; compiler:g141; options:-O2 -std=c++23; libs:-
// https://godbolt.org/z/eK1Ervjs5 
#include <string>
#include <iostream>                      // cout
using std::string; using std::cout;

struct Value {
    int value_;
    Value(int value)                     // 1-arg constructor = type conversion
        : value_{value} {}
};

size_t length(string arg) {
    return arg.size();
}
Value twice(Value v) {
    return Value{ v.value_*2 };
}
int main() {
    cout << length("Hipphopp") << "\n";  // const char* to string
    cout << twice(10).value_ << "\n";    // int to Value
    string name {"Gandalf"};
    cout << ( name + " the Grey" ) << "\n"; // string + const char*
}
```

----------------


## Listing 16.3: A destructor is executed when an object is removed.

Book listing [lst-0003-book.cpp](lst-0003-book.cpp):
```cpp
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
```

Godbolt Listing [lst-0003-godb.cpp](lst-0003-godb.cpp), [https://godbolt.org/z/WPcTv3zKM](https://godbolt.org/z/WPcTv3zKM):
```cpp
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
```

----------------


## Listing 16.4: Here, many objects are created and destroyed.

Book listing [lst-0004-book.cpp](lst-0004-book.cpp):
```cpp
// https://godbolt.org/z/ennEoWPzb 
void function(const MyValue &paramRef) {
    MyValue localValue{"local"};
}

int main() {
    MyValue mvalue1{"mvalue1"};
    function( MyValue{"temp"} );
    function( mvalue1 );
    {
        MyValue mvalue2{"mvalue2"};
    }
}
```

Godbolt Listing [lst-0004-godb.cpp](lst-0004-godb.cpp), [https://godbolt.org/z/ennEoWPzb](https://godbolt.org/z/ennEoWPzb):
```cpp
//#(compile) c++; compiler:g141; options:-O2 -std=c++23; libs:-
// https://godbolt.org/z/ennEoWPzb 
void function(const MyValue &paramRef) {
    MyValue localValue{"local"};
}

int main() {
    MyValue mvalue1{"mvalue1"};
    function( MyValue{"temp"} );
    function( mvalue1 );
    {
        MyValue mvalue2{"mvalue2"};
    }
}
```

----------------


## Listing 16.5: This output shows when objects are created and destroyed.

Book listing [lst-0005-book.cpp](lst-0005-book.cpp):
```cpp
Constructor mvalue1
 Constructor temp
  Constructor local
  Destructor local
 Destructor temp
   Constructor local
   Destructor local
    Constructor mvalue2
    Destructor mvalue2
Destructor mvalue1
```

----------------


## Listing 16.6: A simple example of a C interface to a resource.

Book listing [lst-0006-book.cpp](lst-0006-book.cpp):
```cpp
// https://godbolt.org/z/54dzsb31z 
#ifndef DATABASE_HPP
#define DATABASE_HPP

typedef void* db_handle_t;

db_handle_t db_open(const char* filename);
void db_close(db_handle_t db);
int db_execute(db_handle_t db, const char* query);

#endif
```

Godbolt Listing [lst-0006-godb.cpp](lst-0006-godb.cpp), [https://godbolt.org/z/54dzsb31z](https://godbolt.org/z/54dzsb31z):
```cpp
//#(execute) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/54dzsb31z 
#ifndef DATABASE_HPP
#define DATABASE_HPP

typedef void* db_handle_t;

db_handle_t db_open(const char* filename);
void db_close(db_handle_t db);
int db_execute(db_handle_t db, const char* query);

#endif
```

----------------


## Listing 16.7: If you need to close a resource, the destructor is suitable for this.

Book listing [lst-0007-book.cpp](lst-0007-book.cpp):
```cpp
// https://godbolt.org/z/6399qK9Ts 
#include <iostream>                // cout
#include "database.hpp"            // include the foreign API
class Database {
    db_handle_t db_;               // wrapped resource
public:
    Database(const char* filename);
    ~Database();
    int execute(const char* query);
};
Database::Database(const char* filename)
    : db_{db_open(filename)}       // requesting the resource
    { }
Database::~Database() {
    db_close(db_);                 // releasing the resource
}
int Database::execute(const char* query) {
    return db_execute(db_, query); // using the resource
}
int main() {
    Database db{"customers.dat"};  // creating the wrapper
    std::cout << "Count: "<< db.execute("select * from cust") << "\n";
}                                  // automatically removing the wrapper
```

Godbolt Listing [lst-0007-godb.cpp](lst-0007-godb.cpp), [https://godbolt.org/z/6399qK9Ts](https://godbolt.org/z/6399qK9Ts):
```cpp
//#(compile) c++; compiler:g141; options:-O2 -std=c++23; libs:-
// https://godbolt.org/z/6399qK9Ts 
#include <iostream>                // cout
#include "database.hpp"            // include the foreign API
class Database {
    db_handle_t db_;               // wrapped resource
public:
    Database(const char* filename);
    ~Database();
    int execute(const char* query);
};
Database::Database(const char* filename)
    : db_{db_open(filename)}       // requesting the resource
    { }
Database::~Database() {
    db_close(db_);                 // releasing the resource
}
int Database::execute(const char* query) {
    return db_execute(db_, query); // using the resource
}
int main() {
    Database db{"customers.dat"};  // creating the wrapper
    std::cout << "Count: "<< db.execute("select * from cust") << "\n";
}                                  // automatically removing the wrapper
```

----------------


## Listing 16.8: The constructor initializes or throws an exception.

Book listing [lst-0008-book.cpp](lst-0008-book.cpp):
```cpp
Database::Database(const char* filename)
    : db_{ db_open(filename) }
    {
        if(nullptr == db_) { // Error opening
            throw IllegalArgumentException("Error opening the DB");
        }
    }
```

----------------


## Listing 16.9: Examples of Yoda conditions with "==".

Book listing [lst-0010-book.cpp](lst-0010-book.cpp):
```cpp
if("Yoda" == character) …
if(42 == answer) …
```

----------------


## Listing 16.10: A Yoda condition with a method call.

Book listing [lst-0011-book.cpp](lst-0011-book.cpp):
```cpp
#include "my_string.hpp"
static const my_string ZEBRA { "zebra" };
int main() {
    my_string animal{ "horse" };
    if(ZEBRA.equals(animal)) return 0;
    else return 1;
}
```

----------------


## Listing 16.11: The constructor of "CanThrow" can terminate with an exception.

Book listing [lst-0012-book.cpp](lst-0012-book.cpp):
```cpp
// https://godbolt.org/z/99PPn595E 
#include <iostream>  // cout
#include <stdexcept> // runtime_error
struct CanThrow {
    CanThrow(int whatShouldHappen) {
        std::cout << "Constructor " << whatShouldHappen << "...\n";
        if(whatShouldHappen == 666)
            throw std::runtime_error("Test error");
        std::cout << "...Constructor finished\n";
    }
    ~CanThrow() {
        std::cout << "Destructor.\n";
    }
};
int main() {
    try {
        CanThrow ct1{0};                // okay, does not throw an exception
    } catch(std::runtime_error &exc) {
        std::cout << "Caught-1: " << exc.what() << "\n";
    }
    try {
        CanThrow ct2{666};              // throws, ct2 is not created
    } catch(std::runtime_error &exc) {
        std::cout << "Caught-2: " << exc.what() << "\n";
    }
}
```

Godbolt Listing [lst-0012-godb.cpp](lst-0012-godb.cpp), [https://godbolt.org/z/99PPn595E](https://godbolt.org/z/99PPn595E):
```cpp
//#(compile) c++; compiler:g141; options:-O2 -std=c++23; libs:-
// https://godbolt.org/z/99PPn595E 
#include <iostream>  // cout
#include <stdexcept> // runtime_error
struct CanThrow {
    CanThrow(int whatShouldHappen) {
        std::cout << "Constructor " << whatShouldHappen << "...\n";
        if(whatShouldHappen == 666)
            throw std::runtime_error("Test error");
        std::cout << "...Constructor finished\n";
    }
    ~CanThrow() {
        std::cout << "Destructor.\n";
    }
};
int main() {
    try {
        CanThrow ct1{0};                // okay, does not throw an exception
    } catch(std::runtime_error &exc) {
        std::cout << "Caught-1: " << exc.what() << "\n";
    }
    try {
        CanThrow ct2{666};              // throws, ct2 is not created
    } catch(std::runtime_error &exc) {
        std::cout << "Caught-2: " << exc.what() << "\n";
    }
}
```

----------------


## Listing 16.12: Partially initialized data fields are also cleaned up in the case of an exception.

Book listing [lst-0013-book.cpp](lst-0013-book.cpp):
```cpp
class Mega {
    std::vector<int>     data_;
    CanThrow             canThrow_;
    std::map<string,int> more_;
public:
    Mega()
      : data_{}
      , canThrow_{666}  // triggers an exception
      , more_{}
      { }
};
```

----------------


## Listing 16.13: Passing by value creates objects with the compiler-generated copy constructor.

Book listing [lst-0014-book.cpp](lst-0014-book.cpp):
```cpp
// https://godbolt.org/z/3T869arc8 
void function(MyValue paramValue) {
    std::cout << "(function)\n";
    MyValue localValue{"local"};
}
int main() {
    MyValue mvalue1{"mvalue1"};
    function( MyValue{"temp"} );
    function( mvalue1 );
    {
        MyValue mvalue2{"mvalue2"};
    }
}
```

Godbolt Listing [lst-0014-godb.cpp](lst-0014-godb.cpp), [https://godbolt.org/z/3T869arc8](https://godbolt.org/z/3T869arc8):
```cpp
//#(compile) c++; compiler:g141; options:-O2 -std=c++23; libs:-
// https://godbolt.org/z/3T869arc8 
void function(MyValue paramValue) {
    std::cout << "(function)\n";
    MyValue localValue{"local"};
}
int main() {
    MyValue mvalue1{"mvalue1"};
    function( MyValue{"temp"} );
    function( mvalue1 );
    {
        MyValue mvalue2{"mvalue2"};
    }
}
```

----------------


## Listing 16.14: The copy constructor takes a constant reference of the class as an argument.

Book listing [lst-0015-book.cpp](lst-0015-book.cpp):
```cpp
// https://godbolt.org/z/sGan39a8z 
struct MyValue {
   static int counter;
   int number_;
   string name_;
   explicit MyValue(string name) // as before
      : number_{++counter} , name_{name}
      { cout << setw(number_) << '_'  << "Constructor " << name_ << "\n"; }
   MyValue(const MyValue &orig) // new copy constructor
      : number_{++counter} , name_{orig.name_ + "-Copy"}
      { cout << setw(number_)<<" " << "Copy Constructor " << name_ << "\n"; }
   ~MyValue() { // as before
      cout << setw(number_)<<" " << "Destructor " << name_ << "\n";
   }
};
int MyValue::counter = 0;
```

Godbolt Listing [lst-0015-godb.cpp](lst-0015-godb.cpp), [https://godbolt.org/z/sGan39a8z](https://godbolt.org/z/sGan39a8z):
```cpp
//#(compile) c++; compiler:g141; options:-O2 -std=c++23; libs:-
// https://godbolt.org/z/sGan39a8z 
struct MyValue {
   static int counter;
   int number_;
   string name_;
   explicit MyValue(string name) // as before
      : number_{++counter} , name_{name}
      { cout << setw(number_) << '_'  << "Constructor " << name_ << "\n"; }
   MyValue(const MyValue &orig) // new copy constructor
      : number_{++counter} , name_{orig.name_ + "-Copy"}
      { cout << setw(number_)<<" " << "Copy Constructor " << name_ << "\n"; }
   ~MyValue() { // as before
      cout << setw(number_)<<" " << "Destructor " << name_ << "\n";
   }
};
int MyValue::counter = 0;
```

----------------


## Listing 16.15: Copy and assignment.

Book listing [lst-0016-book.cpp](lst-0016-book.cpp):
```cpp
void byVal(MyValue arg) { }
int main() {
    MyValue value1{"ABCD"};  // new instance, constructed via string
    MyValue value2{value1};  // new instance, constructed via copy
    MyValue value3 = value1; // new instance, also via copy, despite =
    byVal(value1);           // a new instance via copy
    value1 = value2;         // not a new instance, but an assignment
}
```

----------------


## GodboltId:5M8xKabjK

Book listing [lst-0017-book.cpp](lst-0017-book.cpp):
```cpp
// https://godbolt.org/z/5M8xKabjK
int main() {
    MyValue value1{"ABCD"};
    MyValue value2{"WXYZ"};
    value1 = value2;         // assignment
}
```

Godbolt Listing [lst-0017-godb.cpp](lst-0017-godb.cpp), [https://godbolt.org/z/5M8xKabjK](https://godbolt.org/z/5M8xKabjK):
```cpp
//#(compile) c++; compiler:g141; options:-O2 -std=c++23; libs:-
// https://godbolt.org/z/5M8xKabjK
int main() {
    MyValue value1{"ABCD"};
    MyValue value2{"WXYZ"};
    value1 = value2;         // assignment
}
```

----------------


## Listing 16.16: The schema for implementing a custom assignment operator.

Book listing [lst-0019-book.cpp](lst-0019-book.cpp):
```cpp
// https://godbolt.org/z/s65GvY9xj
struct MyValue {
    // … everything else as before
    MyValue& operator=(const MyValue& right) {
        if(this != &right) { // 1. check for self-assignment
            // 2. Release previous resources; none here
            // 3. element-wise transfer by assignment or similar
            name_ = right.name_ + "-Assignment (previously " + name_ + ")";
            /* number_ remains, and thus the original indentation */
        }
        return *this; // 4. return self
    }
};
```

Godbolt Listing [lst-0019-godb.cpp](lst-0019-godb.cpp), [https://godbolt.org/z/s65GvY9xj](https://godbolt.org/z/s65GvY9xj):
```cpp
//#(compile) c++; compiler:g141; options:-O2 -std=c++23; libs:-
// https://godbolt.org/z/s65GvY9xj
struct MyValue {
    // … everything else as before
    MyValue& operator=(const MyValue& right) {
        if(this != &right) { // 1. check for self-assignment
            // 2. Release previous resources; none here
            // 3. element-wise transfer by assignment or similar
            name_ = right.name_ + "-Assignment (previously " + name_ + ")";
            /* number_ remains, and thus the original indentation */
        }
        return *this; // 4. return self
    }
};
```

----------------


## Listing 16.17: The "const" data element is initially without an assignment operator.

Book listing [lst-0020-book.cpp](lst-0020-book.cpp):
```cpp
struct MyNumber {

    const int number_;         // constant data field

    explicit MyNumber(int v)
        : number_{v}           // initialization of the constant data field
        {}
};

int main() {
    MyNumber c1{4};
    MyNumber c2{7};
    c1 = c2;                   // (ERR) Error: Assignment removed by compiler
}
```

----------------


## Listing 16.18: Use "= delete" to manually remove operations.

Book listing [lst-0021-book.cpp](lst-0021-book.cpp):
```cpp
// https://godbolt.org/z/398GhcbaE 
struct MyNumber {
    int number_;           // variable data field

    explicit MyNumber(int v)
        : number_{v} {}
    MyNumber& operator=(const MyNumber&) = delete; // remove assignment
    MyNumber(const MyNumber&) = delete;            // remove copy
};
int main() {
    MyNumber c1{4};
    MyNumber c2{7};
    c1 = c2;               // (ERR) Error – assignment removed by programmer
    MyNumber c3{c1};       // (ERR) Error – copy removed by programmer
}
```

Godbolt Listing [lst-0021-godb.cpp](lst-0021-godb.cpp), [https://godbolt.org/z/398GhcbaE](https://godbolt.org/z/398GhcbaE):
```cpp
//#(compile) c++; compiler:g141; options:-O2 -std=c++23; libs:-
// https://godbolt.org/z/398GhcbaE 
struct MyNumber {
    int number_;           // variable data field

    explicit MyNumber(int v)
        : number_{v} {}
    MyNumber& operator=(const MyNumber&) = delete; // remove assignment
    MyNumber(const MyNumber&) = delete;            // remove copy
};
int main() {
    MyNumber c1{4};
    MyNumber c2{7};
    c1 = c2;               // (ERR) Error – assignment removed by programmer
    MyNumber c3{c1};       // (ERR) Error – copy removed by programmer
}
```

----------------


## Listing 16.19: With the deleted functions, the compiler prevents incorrect usage of the class.

Book listing [lst-0022-book.cpp](lst-0022-book.cpp):
```cpp
// https://godbolt.org/z/vb5a3EjcY 
#include <iostream>              // cout
#include "database.hpp"          // include the foreign API

class Database {
    const db_handle_t db_;                         // make constant
public:
    explicit Database(const char* filename);
    ~Database();
    int execute(const char* query);
    Database(const Database&) = delete;            // prohibit copying
    Database& operator=(const Database&) = delete; // prohibit assignment
};
// … Implementations as before …
int main() {
    Database db{ "customers.dat" };
    std::cout << "Count: "<< db.execute("select * from cust") << "\n";
    Database db2{ db };          // (ERR) compiler prevents dangerous copy
    db = db2;                    // (ERR) compiler prevents dangerous assignment
}
```

Godbolt Listing [lst-0022-godb.cpp](lst-0022-godb.cpp), [https://godbolt.org/z/vb5a3EjcY](https://godbolt.org/z/vb5a3EjcY):
```cpp
//#(compile) c++; compiler:g141; options:-O2 -std=c++23; libs:-
// https://godbolt.org/z/vb5a3EjcY 
#include <iostream>              // cout
#include "database.hpp"          // include the foreign API

class Database {
    const db_handle_t db_;                         // make constant
public:
    explicit Database(const char* filename);
    ~Database();
    int execute(const char* query);
    Database(const Database&) = delete;            // prohibit copying
    Database& operator=(const Database&) = delete; // prohibit assignment
};
// … Implementations as before …
int main() {
    Database db{ "customers.dat" };
    std::cout << "Count: "<< db.execute("select * from cust") << "\n";
    Database db2{ db };          // (ERR) compiler prevents dangerous copy
    db = db2;                    // (ERR) compiler prevents dangerous assignment
}
```

----------------


## Listing 16.20: The class probably contains large amounts of data that are expensive to copy. But what is being copied here?

Book listing [lst-0023-book.cpp](lst-0023-book.cpp):
```cpp
// https://godbolt.org/z/75Kh5Ws45 
#include <vector>
class Image {
    std::vector<std::byte> data_;
public:
    explicit Image(const char *fn) { /*...*/ }
    // Compiler generates (among others): 
    // Copy constructor, assignment, but also moves
};
std::vector<Image> loadCollection(bool empty) {
    if(empty) return std::vector<Image>{};
    std::vector<Image> result {};                 // for return; initially empty
    // three images in the collection … copy?
    result.push_back( Image{"MonaLisa.png"} );
    result.push_back( Image{"TheScream.png"} );
    result.push_back( Image{"BoyWithPipe.png"} );
    return result; // return collection by value
}
int main() {
    // store return in variable
    std::vector<Image> collection = loadCollection(false);
}
```

Godbolt Listing [lst-0023-godb.cpp](lst-0023-godb.cpp), [https://godbolt.org/z/75Kh5Ws45](https://godbolt.org/z/75Kh5Ws45):
```cpp
//#(compile) c++; compiler:g141; options:-O2 -std=c++23; libs:-
// https://godbolt.org/z/75Kh5Ws45 
#include <vector>
class Image {
    std::vector<std::byte> data_;
public:
    explicit Image(const char *fn) { /*...*/ }
    // Compiler generates (among others): 
    // Copy constructor, assignment, but also moves
};
std::vector<Image> loadCollection(bool empty) {
    if(empty) return std::vector<Image>{};
    std::vector<Image> result {};                 // for return; initially empty
    // three images in the collection … copy?
    result.push_back( Image{"MonaLisa.png"} );
    result.push_back( Image{"TheScream.png"} );
    result.push_back( Image{"BoyWithPipe.png"} );
    return result; // return collection by value
}
int main() {
    // store return in variable
    std::vector<Image> collection = loadCollection(false);
}
```

----------------


## Listing 16.21: Implementation of the two move operations.

Book listing [lst-0024-book.cpp](lst-0024-book.cpp):
```cpp
// https://godbolt.org/z/aoqW86vs8 
#include <vector>
class Image {
    std::vector<std::byte> data_;              // byte has been available since C++17
public:
    explicit Image(const char *fn) { /*...*/ }
    Image(Image&& other) noexcept              // move constructor
        : data_{} // create empty
    {
        using std::swap;
        swap(data_, other.data_);
    }
    Image& operator=(Image&& other) noexcept { // move operator
        using std::swap;
        swap(data_, other.data_);
        return *this;
    }
};
```

Godbolt Listing [lst-0024-godb.cpp](lst-0024-godb.cpp), [https://godbolt.org/z/aoqW86vs8](https://godbolt.org/z/aoqW86vs8):
```cpp
//#(compile) c++; compiler:g141; options:-O2 -std=c++23; libs:-
// https://godbolt.org/z/aoqW86vs8 
#include <vector>
class Image {
    std::vector<std::byte> data_;              // byte has been available since C++17
public:
    explicit Image(const char *fn) { /*...*/ }
    Image(Image&& other) noexcept              // move constructor
        : data_{} // create empty
    {
        using std::swap;
        swap(data_, other.data_);
    }
    Image& operator=(Image&& other) noexcept { // move operator
        using std::swap;
        swap(data_, other.data_);
        return *this;
    }
};
```

----------------


## Listing 16.22: Infix, function, and method notation for operators.

Book listing [lst-0030-book.cpp](lst-0030-book.cpp):
```cpp
// https://godbolt.org/z/P99563KoP 
#include <iostream>
using std::cout; using std::ostream;
struct Widget {
    bool operator<(const Widget&) const {      // method notation
        return true;                           // always true
    }
};
bool operator<(const Widget&, const Widget&) { // function notation
    return false;                              // always false
}
int main() {
    Widget x{};
    Widget y{};
    cout << (operator<(x, y)      // calls function notation
        ? "Method1\n" : "Function1\n");
    cout << (y.operator<(x)       // calls method notation
        ? "Method2\n" : "Function2\n");
    cout << (x < y                // infix notation, allows choice, here function
        ? "Method3\n" : "Function3\n");
}
```

Godbolt Listing [lst-0030-godb.cpp](lst-0030-godb.cpp), [https://godbolt.org/z/P99563KoP](https://godbolt.org/z/P99563KoP):
```cpp
//#(compile) c++; compiler:g141; options:-O2 -std=c++23; libs:-
// https://godbolt.org/z/P99563KoP 
#include <iostream>
using std::cout; using std::ostream;
struct Widget {
    bool operator<(const Widget&) const {      // method notation
        return true;                           // always true
    }
};
bool operator<(const Widget&, const Widget&) { // function notation
    return false;                              // always false
}
int main() {
    Widget x{};
    Widget y{};
    cout << (operator<(x, y)      // calls function notation
        ? "Method1\n" : "Function1\n");
    cout << (y.operator<(x)       // calls method notation
        ? "Method2\n" : "Function2\n");
    cout << (x < y                // infix notation, allows choice, here function
        ? "Method3\n" : "Function3\n");
}
```

----------------


## Listing 16.23: A "friend" function is not a method, even if it is inside the class or defined there.

Book listing [lst-0031-book.cpp](lst-0031-book.cpp):
```cpp
// https://godbolt.org/z/Wvr3aK75G 
#include <compare>               // for <=>
class Value {
    int value_; 
public:
    explicit Value(int value) : value_{value} {}

    // free functions, but declared as friend
    friend Value operator+(const Value& li, const Value& re);
    friend Value operator-(const Value& li, const Value& re)
        { return Value{li.value_ - re.value_};  // also defined inside
    }
    // operator<=> does not need to be friend.
    // for all comparisons: <, >, <=, >= as well as == and !=
    auto operator<=>(const Value& re) const = default; 
};

// Definition of the previously declared friend function:
Value operator+(const Value& li, const Value& re) {
    return Value{li.value_ + re.value_}; // access to private members allowed
}

int main() {
    Value seven{7}; Value three{3}; Value two{2};
    if((three+two) < seven) {    // operator+, then operator< via <=>
        return 0;                // okay
    } else {
        return 1;                // something went wrong
    }
}
```

Godbolt Listing [lst-0031-godb.cpp](lst-0031-godb.cpp), [https://godbolt.org/z/Wvr3aK75G](https://godbolt.org/z/Wvr3aK75G):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/Wvr3aK75G 
#include <compare>               // for <=>
class Value {
    int value_; 
public:
    explicit Value(int value) : value_{value} {}

    // free functions, but declared as friend
    friend Value operator+(const Value& li, const Value& re);
    friend Value operator-(const Value& li, const Value& re)
        { return Value{li.value_ - re.value_};  // also defined inside
    }
    // operator<=> does not need to be friend.
    // for all comparisons: <, >, <=, >= as well as == and !=
    auto operator<=>(const Value& re) const = default; 
};

// Definition of the previously declared friend function:
Value operator+(const Value& li, const Value& re) {
    return Value{li.value_ + re.value_}; // access to private members allowed
}

int main() {
    Value seven{7}; Value three{3}; Value two{2};
    if((three+two) < seven) {    // operator+, then operator< via <=>
        return 0;                // okay
    } else {
        return 1;                // something went wrong
    }
}
```

----------------


## Listing 16.24: A data type almost fully equipped with all operators.

Book listing [lst-0032-book.cpp](lst-0032-book.cpp):
```cpp
// https://godbolt.org/z/YonbGKj8M 
#include <iostream>  // istream, ostream, cout
class Num {
  int val_ = 0;
public:
  int& operator*();  // Dereference: Direct access to the value
  const int& operator*() const; // Dereference: Read access to the value
  Num() {}
  explicit Num(int value) : val_{value} {}
  // unary operators
  Num& operator++(); // Preincrement
  Num& operator--(); // Predecrement
  Num operator+();   // Positive
  Num operator-();   // Negate
  Num operator~();   // Bitwise invert
  // binary operators
  // - compound assignments, arithmetic
  Num& operator+=(const Num& re) { val_ += *re; return *this; }
  Num& operator-=(const Num& re) { val_ -= *re; return *this; }
  Num& operator*=(const Num& re) { val_ *= *re; return *this; }
  Num& operator/=(const Num& re) { val_ /= *re; return *this; }
  Num& operator%=(const Num& re) { val_ %= *re; return *this; }
  // - compound assignments, bitwise
  Num& operator|=(const Num& re) { val_ |= *re; return *this; }
  Num& operator&=(const Num& re) { val_ &= *re; return *this; }
  Num& operator^=(const Num& re) { val_ ^= *re; return *this; }
  Num& operator<<=(int n) { val_ <<= n; return *this; }
  Num& operator>>=(int n) { val_ >>= n; return *this; }
  // - Variation of compound assignments for easier handling
  Num& operator+=(int re) { val_ += re; return *this; }
  Num& operator-=(int re) { val_ -= re; return *this; }
  // binary operators, with call-by-value for the first parameter
  // and using compound assignment for assistance
  // - Arithmetic
  friend Num operator+(Num li, const Num& re) { return li += re; }
  friend Num operator-(Num li, const Num& re) { return li -= re; }
  friend Num operator*(Num li, const Num& re) { return li *= re; }
  friend Num operator/(Num li, const Num& re) { return li /= re; }
  friend Num operator%(Num li, const Num& re) { return li %= re; }
  // - bitwise
  friend Num operator|(Num li, const Num& re) { return li |= re; }
  friend Num operator&(Num li, const Num& re) { return li &= re; }
  friend Num operator^(Num li, const Num& re) { return li ^= re; }
  // - comparisons
  // - … fundamental for standard containers and algorithms
  friend bool operator==(const Num& li, const Num& re) { return *li == *re; }
  auto operator<=>(const Num& re) const {return val_ <=> *re; } // C++20
  // - input and output
  friend std::ostream& operator<<(std::ostream& os, const Num& arg);
  friend std::istream& operator>>(std::istream& is,  Num& arg);
};
// unary operators
Num& Num::operator++() { ++val_; return *this; }
Num& Num::operator--() { --val_; return *this; }
Num Num::operator+() { return Num{val_}; }
Num Num::operator-() { return Num{-val_}; }
Num Num::operator~() { return Num{~val_}; }
int& Num::operator*() { return val_; }
const int& Num::operator*() const { return val_; }
// Input and Output
std::ostream& operator<<(std::ostream&os, const Num& arg) { return os<<*arg; }
std::istream& operator>>(std::istream&is, Num& arg) { return is>>*arg; }
```

Godbolt Listing [lst-0032-godb.cpp](lst-0032-godb.cpp), [https://godbolt.org/z/YonbGKj8M](https://godbolt.org/z/YonbGKj8M):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/YonbGKj8M 
#include <iostream>  // istream, ostream, cout
class Num {
  int val_ = 0;
public:
  int& operator*();  // Dereference: Direct access to the value
  const int& operator*() const; // Dereference: Read access to the value
  Num() {}
  explicit Num(int value) : val_{value} {}
  // unary operators
  Num& operator++(); // Preincrement
  Num& operator--(); // Predecrement
  Num operator+();   // Positive
  Num operator-();   // Negate
  Num operator~();   // Bitwise invert
  // binary operators
  // - compound assignments, arithmetic
  Num& operator+=(const Num& re) { val_ += *re; return *this; }
  Num& operator-=(const Num& re) { val_ -= *re; return *this; }
  Num& operator*=(const Num& re) { val_ *= *re; return *this; }
  Num& operator/=(const Num& re) { val_ /= *re; return *this; }
  Num& operator%=(const Num& re) { val_ %= *re; return *this; }
  // - compound assignments, bitwise
  Num& operator|=(const Num& re) { val_ |= *re; return *this; }
  Num& operator&=(const Num& re) { val_ &= *re; return *this; }
  Num& operator^=(const Num& re) { val_ ^= *re; return *this; }
  Num& operator<<=(int n) { val_ <<= n; return *this; }
  Num& operator>>=(int n) { val_ >>= n; return *this; }
  // - Variation of compound assignments for easier handling
  Num& operator+=(int re) { val_ += re; return *this; }
  Num& operator-=(int re) { val_ -= re; return *this; }
  // binary operators, with call-by-value for the first parameter
  // and using compound assignment for assistance
  // - Arithmetic
  friend Num operator+(Num li, const Num& re) { return li += re; }
  friend Num operator-(Num li, const Num& re) { return li -= re; }
  friend Num operator*(Num li, const Num& re) { return li *= re; }
  friend Num operator/(Num li, const Num& re) { return li /= re; }
  friend Num operator%(Num li, const Num& re) { return li %= re; }
  // - bitwise
  friend Num operator|(Num li, const Num& re) { return li |= re; }
  friend Num operator&(Num li, const Num& re) { return li &= re; }
  friend Num operator^(Num li, const Num& re) { return li ^= re; }
  // - comparisons
  // - … fundamental for standard containers and algorithms
  friend bool operator==(const Num& li, const Num& re) { return *li == *re; }
  auto operator<=>(const Num& re) const {return val_ <=> *re; } // C++20
  // - input and output
  friend std::ostream& operator<<(std::ostream& os, const Num& arg);
  friend std::istream& operator>>(std::istream& is,  Num& arg);
};
// unary operators
Num& Num::operator++() { ++val_; return *this; }
Num& Num::operator--() { --val_; return *this; }
Num Num::operator+() { return Num{val_}; }
Num Num::operator-() { return Num{-val_}; }
Num Num::operator~() { return Num{~val_}; }
int& Num::operator*() { return val_; }
const int& Num::operator*() const { return val_; }
// Input and Output
std::ostream& operator<<(std::ostream&os, const Num& arg) { return os<<*arg; }
std::istream& operator>>(std::istream&is, Num& arg) { return is>>*arg; }
```

----------------


## Listing 16.25: You use the data type equipped with operators as usual.

Book listing [lst-0033-book.cpp](lst-0033-book.cpp):
```cpp
// https://godbolt.org/z/YonbGKj8M 
#include <iostream>                       // cout
int main() {
    using std::cout;
    Num a{1};
    *a = 7;                               // operator* also returns int&
    a += Num{3};                          // Increment with Num
    cout << ( ++( ++a ) ) << "\n";        // Output: 12
    a -= 2;                               // Variation with int
    cout << --(--a) << "\n";              // Output: 8
    Num b{99};
    cout << (a<b ? "yes\n" : "xxx\n");    // Output: yes
    cout << (a>b ? "xxx\n" : "no\n");     // Output: no
    b /= Num{3};           // b: 33
    b %= Num{10};          // b: 3
    b <<= 4;               // b: 48
    b >>= 2;               // b: 12
    Num c = b / Num{3} + a * Num{2};      // c: 20
}
```

Godbolt Listing [lst-0033-godb.cpp](lst-0033-godb.cpp), [https://godbolt.org/z/YonbGKj8M](https://godbolt.org/z/YonbGKj8M):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/YonbGKj8M 
#include <iostream>                       // cout
int main() {
    using std::cout;
    Num a{1};
    *a = 7;                               // operator* also returns int&
    a += Num{3};                          // Increment with Num
    cout << ( ++( ++a ) ) << "\n";        // Output: 12
    a -= 2;                               // Variation with int
    cout << --(--a) << "\n";              // Output: 8
    Num b{99};
    cout << (a<b ? "yes\n" : "xxx\n");    // Output: yes
    cout << (a>b ? "xxx\n" : "no\n");     // Output: no
    b /= Num{3};           // b: 33
    b %= Num{10};          // b: 3
    b <<= 4;               // b: 48
    b >>= 2;               // b: 12
    Num c = b / Num{3} + a * Num{2};      // c: 20
}
```

----------------


## Listing 16.26: This data type demonstrates the Boolean operators.

Book listing [lst-0034-book.cpp](lst-0034-book.cpp):
```cpp
// https://godbolt.org/z/vjrr6d5TW 
#include <iostream>               // istream, ostream, cout
class Bool {
    bool val_ = false;
    bool& operator*()             // dereference; mutable
        { return val_; };
    const bool& operator*() const // dereference; read-only
        { return val_; }
public:
    constexpr Bool() {}
    explicit constexpr Bool(bool value)
        : val_{value} {}
    // unary operators
    Bool operator!() const        // not operator
        { return Bool{!val_}; };
    // binary operators
    friend Bool operator&&(const Bool &re, const Bool &li)
        { return Bool{*re && *li}; }
    friend Bool operator||(const Bool &re, const Bool &li)
        { return Bool{*re || *li}; }
    // input and output
    friend std::ostream& operator<<(std::ostream& os, const Bool& arg);
    friend std::istream& operator>>(std::istream& is,  Bool& arg);
};
std::ostream& operator<<(std::ostream& os, const Bool& arg)
    { return os << *arg; }
std::istream& operator>>(std::istream& is, Bool& arg)
    { return is >> *arg; }
// Constants
static constexpr Bool False{false};
static constexpr Bool True{true};
int main() {
    Bool yesno = True && ( Bool{false} || !Bool{} ); // uses &&, ||, and !
    std::cout << yesno << "\n"; // Output: 1
}
```

Godbolt Listing [lst-0034-godb.cpp](lst-0034-godb.cpp), [https://godbolt.org/z/vjrr6d5TW](https://godbolt.org/z/vjrr6d5TW):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/vjrr6d5TW 
#include <iostream>               // istream, ostream, cout
class Bool {
    bool val_ = false;
    bool& operator*()             // dereference; mutable
        { return val_; };
    const bool& operator*() const // dereference; read-only
        { return val_; }
public:
    constexpr Bool() {}
    explicit constexpr Bool(bool value)
        : val_{value} {}
    // unary operators
    Bool operator!() const        // not operator
        { return Bool{!val_}; };
    // binary operators
    friend Bool operator&&(const Bool &re, const Bool &li)
        { return Bool{*re && *li}; }
    friend Bool operator||(const Bool &re, const Bool &li)
        { return Bool{*re || *li}; }
    // input and output
    friend std::ostream& operator<<(std::ostream& os, const Bool& arg);
    friend std::istream& operator>>(std::istream& is,  Bool& arg);
};
std::ostream& operator<<(std::ostream& os, const Bool& arg)
    { return os << *arg; }
std::istream& operator>>(std::istream& is, Bool& arg)
    { return is >> *arg; }
// Constants
static constexpr Bool False{false};
static constexpr Bool True{true};
int main() {
    Bool yesno = True && ( Bool{false} || !Bool{} ); // uses &&, ||, and !
    std::cout << yesno << "\n"; // Output: 1
}
```

----------------


## Listing 16.27: The comparison categories contain constants for the return.

Book listing [lst-0036-book.cpp](lst-0036-book.cpp):
```cpp
// https://godbolt.org/z/KxWb4q8q9 
#include <compare>  // partial_ordering etc
#include <set>
#include <iostream>
using namespace std;
struct Fraction {
  int z, n;  // numerator / denominator
  partial_ordering operator<=>(const Fraction& rhs) const {
    if(n==0 || rhs.n==0) return partial_ordering::unordered;
    return (double)z / n <=> (double)rhs.z / rhs.n;
  }
};
int main() {
  set<Fraction> fractions{ {1,2}, {2,4}, {1,3}, {2,3}, {1,4}, {2,5}, {3,8}, {99,0} };
  for(auto f : fractions)
    cout << f.z << "/" << f.n << " ";
  cout << "\n”; // Output: 1/4 1/3 3/8 2/5 1/2 2/3
}
```

Godbolt Listing [lst-0036-godb.cpp](lst-0036-godb.cpp), [https://godbolt.org/z/KxWb4q8q9](https://godbolt.org/z/KxWb4q8q9):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/KxWb4q8q9 
#include <compare>  // partial_ordering etc
#include <set>
#include <iostream>
using namespace std;
struct Fraction {
  int z, n;  // numerator / denominator
  partial_ordering operator<=>(const Fraction& rhs) const {
    if(n==0 || rhs.n==0) return partial_ordering::unordered;
    return (double)z / n <=> (double)rhs.z / rhs.n;
  }
};
int main() {
  set<Fraction> fractions{ {1,2}, {2,4}, {1,3}, {2,3}, {1,4}, {2,5}, {3,8}, {99,0} };
  for(auto f : fractions)
    cout << f.z << "/" << f.n << " ";
  cout << "\n”; // Output: 1/4 1/3 3/8 2/5 1/2 2/3
}
```

----------------


## Listing 16.28: You can use this helper function to implement a simple ordered comparison.

Book listing [lst-0038-book.cpp](lst-0038-book.cpp):
```cpp
auto operator<=>(const Address& re) const {
  array a{street, zip, city};
  array b{re.street, re.zip, re.city};
  return lexicographical_compare_three_way(begin(a), end(a), begin(b), end(b));
}
```

----------------


## Listing 16.29: A virtual method that is = 0 is called pure virtual or abstract.

Book listing [lst-0039-book.cpp](lst-0039-book.cpp):
```cpp
// https://godbolt.org/z/1zq4jfdYb
#include <string>
#include <iostream>
using std::string; using std::ostream;
class Shape {
    string color_;
public:
    virtual double calcArea() const = 0; // pure virtual method
    string getColor() const { return color_; }
    void setColor(const string& color) { color_ = color; }
    virtual ~Shape() {}
};
class Square : public Shape {
    double len_;
public:
    explicit Square(double len) : len_{len} {}
    double calcArea() const override { return len_*len_; }
};
class Circle : public Shape {
    double rad_;
public:
    explicit Circle(double rad) : rad_{rad} {}
    double calcArea() const override { return 3.1415*rad_*rad_; }
};
struct Calculator {
    Shape& shape_;
    Calculator(Shape& shape) : shape_{shape} { }
    void run(ostream& os) const {
        os << "The area of the shape is " << shape_.calcArea() << "\n";
    }
};
int main() {
    Square square {5.0};
    Calculator ti { square };
    ti.run(std::cout); // Output: The area of the shape is 25
}
```

Godbolt Listing [lst-0039-godb.cpp](lst-0039-godb.cpp), [https://godbolt.org/z/1zq4jfdYb](https://godbolt.org/z/1zq4jfdYb):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/1zq4jfdYb
#include <string>
#include <iostream>
using std::string; using std::ostream;
class Shape {
    string color_;
public:
    virtual double calcArea() const = 0; // pure virtual method
    string getColor() const { return color_; }
    void setColor(const string& color) { color_ = color; }
    virtual ~Shape() {}
};
class Square : public Shape {
    double len_;
public:
    explicit Square(double len) : len_{len} {}
    double calcArea() const override { return len_*len_; }
};
class Circle : public Shape {
    double rad_;
public:
    explicit Circle(double rad) : rad_{rad} {}
    double calcArea() const override { return 3.1415*rad_*rad_; }
};
struct Calculator {
    Shape& shape_;
    Calculator(Shape& shape) : shape_{shape} { }
    void run(ostream& os) const {
        os << "The area of the shape is " << shape_.calcArea() << "\n";
    }
};
int main() {
    Square square {5.0};
    Calculator ti { square };
    ti.run(std::cout); // Output: The area of the shape is 25
}
```

----------------


## Listing 16.30: With an "enum", you define a type with its own value range.

Book listing [lst-0044-book.cpp](lst-0044-book.cpp):
```cpp
// https://godbolt.org/z/xhd7s1YhE 
#include <string>
using std::string;
enum class StoplightColor {
    RED, YELLOW, REDYELLOW, GREEN
};
struct Stoplight {
    StoplightColor color_;
    Stoplight(StoplightColor color, string name) : color_{color} {}
};

Stoplight createStoplight(StoplightColor color, string name) {
    return Stoplight{color, name};
}

int main() {
    Stoplight stoplight = createStoplight(StoplightColor::RED, "AX-001");
}
```

Godbolt Listing [lst-0044-godb.cpp](lst-0044-godb.cpp), [https://godbolt.org/z/xhd7s1YhE](https://godbolt.org/z/xhd7s1YhE):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/xhd7s1YhE 
#include <string>
using std::string;
enum class StoplightColor {
    RED, YELLOW, REDYELLOW, GREEN
};
struct Stoplight {
    StoplightColor color_;
    Stoplight(StoplightColor color, string name) : color_{color} {}
};

Stoplight createStoplight(StoplightColor color, string name) {
    return Stoplight{color, name};
}

int main() {
    Stoplight stoplight = createStoplight(StoplightColor::RED, "AX-001");
}
```

----------------


## GodboltId:f8on7bs1Y

Book listing [lst-0046-book.cpp](lst-0046-book.cpp):
```cpp
// https://godbolt.org/z/f8on7bs1Y
string drivingSchool(Stoplight light) {
    switch(light.color) {
      using enum StoplightColor;   // "StoplightColor::" can now be omitted
      case RED: return "stop";
      case REDYELLOW: return "get ready";
      case YELLOW: return "slow down";
      case GREEN: return "go";
    }
    abort(); // exhaustive switch, should not happen
}
```

Godbolt Listing [lst-0046-godb.cpp](lst-0046-godb.cpp), [https://godbolt.org/z/f8on7bs1Y](https://godbolt.org/z/f8on7bs1Y):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/f8on7bs1Y
string drivingSchool(Stoplight light) {
    switch(light.color) {
      using enum StoplightColor;   // "StoplightColor::" can now be omitted
      case RED: return "stop";
      case REDYELLOW: return "get ready";
      case YELLOW: return "slow down";
      case GREEN: return "go";
    }
    abort(); // exhaustive switch, should not happen
}
```

----------------


## Listing 17.31: In an "enum", you can also specify the desired numerical values.

Book listing [lst-0047-book.cpp](lst-0047-book.cpp):
```cpp
// https://godbolt.org/z/Yjbzrrnob
enum class Weekday {
    MO=1, TU, WE, TH, FR, SA, SU         // TU becomes 2, WE becomes 3, etc.
};
enum class Level {
   TRACE=1, DEBUG, INFO=10, ERROR, FATAL // gaps are also possible
};
void log(Level level) {
   int intLevel = (int)level;            // explicitly cast to an int
   if(intLevel > 10) { /* ... */ }
}
```

Godbolt Listing [lst-0047-godb.cpp](lst-0047-godb.cpp), [https://godbolt.org/z/Yjbzrrnob](https://godbolt.org/z/Yjbzrrnob):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/Yjbzrrnob
enum class Weekday {
    MO=1, TU, WE, TH, FR, SA, SU         // TU becomes 2, WE becomes 3, etc.
};
enum class Level {
   TRACE=1, DEBUG, INFO=10, ERROR, FATAL // gaps are also possible
};
void log(Level level) {
   int intLevel = (int)level;            // explicitly cast to an int
   if(intLevel > 10) { /* ... */ }
}
```

----------------

