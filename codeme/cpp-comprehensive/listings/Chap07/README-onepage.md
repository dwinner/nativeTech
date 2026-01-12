# Listings of Chap07.docx

This is the list of listings on one page.
You can also view a [linked summary](README.md).


## Listing 07.1 Use functions.

Book listing [lst-0008-book.cpp](lst-0008-book.cpp):
```cpp
// https://godbolt.org/z/TdrebYodx 
#include <iostream>                     // cout 
#include <cmath>                        // sin 
#include <string> 
#include <vector> 
using std::sin; 
int main() { 
    std::cout << "sin(0.0): " << sin(0.0) << "\n";       // call sin() with literal 
    double angle = 3.1415/2; 
    std::cout << "sin("<<angle<<"): "<<sin(angle)<<"\n"; // call with variable 
    std::string name = "Han Solo"; 
    std::cout << name.length() << "\n"; // call a method 
                                        // ... conceptually like length(name) 
    std::vector<int> data{}; 
    data.push_back(5);                  // further method call with parameter 
    data.push_back(10); 
    std::cout << data.back() << " "; 
    data.pop_back(); 
    std::cout << data.back() << "\n"; 
    data.pop_back(); 
}
```

Godbolt Listing [lst-0008-godb.cpp](lst-0008-godb.cpp), [https://godbolt.org/z/TdrebYodx](https://godbolt.org/z/TdrebYodx):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/TdrebYodx 
#include <iostream>                     // cout 
#include <cmath>                        // sin 
#include <string> 
#include <vector> 
using std::sin; 
int main() { 
    std::cout << "sin(0.0): " << sin(0.0) << "\n";       // call sin() with literal 
    double angle = 3.1415/2; 
    std::cout << "sin("<<angle<<"): "<<sin(angle)<<"\n"; // call with variable 
    std::string name = "Han Solo"; 
    std::cout << name.length() << "\n"; // call a method 
                                        // ... conceptually like length(name) 
    std::vector<int> data{}; 
    data.push_back(5);                  // further method call with parameter 
    data.push_back(10); 
    std::cout << data.back() << " "; 
    data.pop_back(); 
    std::cout << data.back() << "\n"; 
    data.pop_back(); 
}
```

----------------


## Listing 07.2: Different return types of functions.

Book listing [lst-0009-book.cpp](lst-0009-book.cpp):
```cpp
int func();                        // returns an int
std::string func();                // a string from the standard library
void func();                       // no return value
std::pair<int,std::string> func(); // composite type from the stdlib
vector<int> func();                // returns a new container
vector<int>& func();               // reference to some container
const vector<int>& func();         // same, but you cannot change it
```

----------------


## Listing 07.3 Parameters are initially passed as a value.

Book listing [lst-0010-book.cpp](lst-0010-book.cpp):
```cpp
// https://godbolt.org/z/8zcvMdM3b 
#include <iostream>

void print_val8(int n) {           // parameter as value 
    std::cout << n << " ";
    n = 8;                         // sets parameter to 8 
    std::cout << n << "\n"; 
}

int main() { 
    int x = 5; 
    print_val8(x);                 // x as value: prints 5, then 8 
    std::cout << x << "\n";        // x remains unchanged at 5 
    print_val8(42);                // 42 as value: prints 42, then 8 
}
```

Godbolt Listing [lst-0010-godb.cpp](lst-0010-godb.cpp), [https://godbolt.org/z/8zcvMdM3b](https://godbolt.org/z/8zcvMdM3b):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/8zcvMdM3b 
#include <iostream>

void print_val8(int n) {           // parameter as value 
    std::cout << n << " ";
    n = 8;                         // sets parameter to 8 
    std::cout << n << "\n"; 
}

int main() { 
    int x = 5; 
    print_val8(x);                 // x as value: prints 5, then 8 
    std::cout << x << "\n";        // x remains unchanged at 5 
    print_val8(42);                // 42 as value: prints 42, then 8 
}
```

----------------


## Listing 07.4: Add an ampersand to make it a reference parameter.

Book listing [lst-0011-book.cpp](lst-0011-book.cpp):
```cpp
// https://godbolt.org/z/63dTshz3Y 
#include <iostream> 
void print_ref8(int& n) {          // parameter as reference 
    std::cout << n << " "; 
    n = 8;                         // sets parameter to 8 
    std::cout << n << "\n"; 
} 
int main() { 
    int x = 5; 
    print_ref8(x);                 // x as reference: prints 5, then 8 
    std::cout << x << "\n";        // x is now 8 
}
```

Godbolt Listing [lst-0011-godb.cpp](lst-0011-godb.cpp), [https://godbolt.org/z/63dTshz3Y](https://godbolt.org/z/63dTshz3Y):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/63dTshz3Y 
#include <iostream> 
void print_ref8(int& n) {          // parameter as reference 
    std::cout << n << " "; 
    n = 8;                         // sets parameter to 8 
    std::cout << n << "\n"; 
} 
int main() { 
    int x = 5; 
    print_ref8(x);                 // x as reference: prints 5, then 8 
    std::cout << x << "\n";        // x is now 8 
}
```

----------------


## Listing 07.5: You can use constant references as parameters for any call.

Book listing [lst-0012-book.cpp](lst-0012-book.cpp):
```cpp
// https://godbolt.org/z/dcKnx3Pno 
#include <iostream> 
void print_cref(const int& n) {  // parameter as constant reference 
    std::cout << n << " "; 
}

int main() { 
    int x = 5; 
    print_cref(x);               // call with a variable 
    print_cref(42);              // call with a constant literal 
}
```

Godbolt Listing [lst-0012-godb.cpp](lst-0012-godb.cpp), [https://godbolt.org/z/dcKnx3Pno](https://godbolt.org/z/dcKnx3Pno):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/dcKnx3Pno 
#include <iostream> 
void print_cref(const int& n) {  // parameter as constant reference 
    std::cout << n << " "; 
}

int main() { 
    int x = 5; 
    print_cref(x);               // call with a variable 
    print_cref(42);              // call with a constant literal 
}
```

----------------


## GodboltId:oh91YvaqT

Book listing [lst-0014-book.cpp](lst-0014-book.cpp):
```cpp
// https://godbolt.org/z/oh91YvaqT 
#include <iostream> 
void twice(double &num) {      // output parameter as modifiable reference 
    num *= 2.0; 
}

int main() { 
    double num = 7.25; 
    twice(num); 
    std::cout << num << "\n";  // now 14.5 
}
```

Godbolt Listing [lst-0014-godb.cpp](lst-0014-godb.cpp), [https://godbolt.org/z/oh91YvaqT](https://godbolt.org/z/oh91YvaqT):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/oh91YvaqT 
#include <iostream> 
void twice(double &num) {      // output parameter as modifiable reference 
    num *= 2.0; 
}

int main() { 
    double num = 7.25; 
    twice(num); 
    std::cout << num << "\n";  // now 14.5 
}
```

----------------


## GodboltId:q4ov41cnv

Book listing [lst-0015-book.cpp](lst-0015-book.cpp):
```cpp
// https://godbolt.org/z/q4ov41cnv
#include <iostream>
double twice(double num) {     // value parameter and return value 
    return num * 2.0; 
} 
int main() { 
    double num = 7.25;
    num = twice(num);          // change expressed by return value 
    std::cout << num << "\n";  // also 14.5 
}
```

Godbolt Listing [lst-0015-godb.cpp](lst-0015-godb.cpp), [https://godbolt.org/z/q4ov41cnv](https://godbolt.org/z/q4ov41cnv):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/q4ov41cnv
#include <iostream>
double twice(double num) {     // value parameter and return value 
    return num * 2.0; 
} 
int main() { 
    double num = 7.25;
    num = twice(num);          // change expressed by return value 
    std::cout << num << "\n";  // also 14.5 
}
```

----------------


## Listing 07.6 Various definitions of functions.

Book listing [lst-0016-book.cpp](lst-0016-book.cpp):
```cpp
std::vector<int> primes = {2};        // global variable
bool isPrime(int n) { // own function 
    for(int factor : primes) {        // access to global variable 
        if(factor*factor > n)         // access to parameter 
            return true; 
        if(n%factor==0) 
            return false; 
    } 
    return true; 
}
void calculatePrimesUpTo(int limit) { // another custom function 
    for(int n=3; n<limit; n=n+2) { 
        if(isPrime (n)) {             // use custom function 
            primes.push_back(n); 
        }
    }
 }
```

----------------


## Listing 07.7: A forward declaration without a function body.

Book listing [lst-0017-book.cpp](lst-0017-book.cpp):
```cpp
// ... extract ...
bool isPrime(int n);                  // Declaration of function defined later 
void calculatePrimesUpTo(int limit) { 
    for(int n=3; n<limit; n=n+2) { 
        if(isPrime(n)) {             // Use of function defined later 
            primes.push_back(n); 
        }
    }
}
bool isPrime(int n) {                // Definition after useage 
    // ... as before ... 
}
```

----------------


## Listing 07.8: Some conversions take place here. The conversions may look different on your system.

Book listing [lst-0019-book.cpp](lst-0019-book.cpp):
```cpp
// https://godbolt.org/z/zb1T7Kr3G 
#include <iostream>
#include <format>
void prints(short s, int i, float f, double d) {
  std::cout << std::format("short: {} int: {} float: {:.2f} double: {:.2f}\n",
    s, i, f, d);
}
int main() {
  int mill = 1000*1000;                // 1 Million
  prints(mill, mill, mill, mill);      // short overflows
  // Output: short: 16960 int: 1000000 float: 1000000.00 double: 1000000.00
  long bill = 1000L*1000L*1000L*1000L; // 1 Trillion
  prints(bill, bill, bill, bill);      // even int overflows, float becomes inaccurate
  // Output: short: 4096 int: –727379968
  // float:999999995904.00 double: 1000000000000.00
  float three = 3.75f;
  prints(three, three, three, three);  // decimal places are lost
  // Output: short: 3 int: 3 float: 3.75 double: 3.75
}
```

Godbolt Listing [lst-0019-godb.cpp](lst-0019-godb.cpp), [https://godbolt.org/z/zb1T7Kr3G](https://godbolt.org/z/zb1T7Kr3G):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/zb1T7Kr3G 
#include <iostream>
#include <format>
void prints(short s, int i, float f, double d) {
  std::cout << std::format("short: {} int: {} float: {:.2f} double: {:.2f}\n",
    s, i, f, d);
}
int main() {
  int mill = 1000*1000;                // 1 Million
  prints(mill, mill, mill, mill);      // short overflows
  // Output: short: 16960 int: 1000000 float: 1000000.00 double: 1000000.00
  long bill = 1000L*1000L*1000L*1000L; // 1 Trillion
  prints(bill, bill, bill, bill);      // even int overflows, float becomes inaccurate
  // Output: short: 4096 int: –727379968
  // float:999999995904.00 double: 1000000000000.00
  float three = 3.75f;
  prints(three, three, three, three);  // decimal places are lost
  // Output: short: 3 int: 3 float: 3.75 double: 3.75
}
```

----------------


## Listing 07.9: The "print" and "add" functions have been overloaded for multiple types.

Book listing [lst-0021-book.cpp](lst-0021-book.cpp):
```cpp
// https://godbolt.org/z/9YcPraT15 
#include <iostream>
void print(int value) { std::cout << "int value: " << value << " "; }
void print(double value) { std::cout << "double value: " << value << " "; }
void print(int v1, double v2) { std::cout << "Values: "<<v1<<", "<<v2<<" "; }
int add(int n, int m) { return n + m; }
double add(double a, double b) { return a + b; }
int main() {
    print( add(3, 4) );         // add(int, int) and print(int)
    print( add(3.25f, 1.5f) );  // add(double, double) and print(double)
    print( 7, 3.25 );           // print(int, double)
}
```

Godbolt Listing [lst-0021-godb.cpp](lst-0021-godb.cpp), [https://godbolt.org/z/9YcPraT15](https://godbolt.org/z/9YcPraT15):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/9YcPraT15 
#include <iostream>
void print(int value) { std::cout << "int value: " << value << " "; }
void print(double value) { std::cout << "double value: " << value << " "; }
void print(int v1, double v2) { std::cout << "Values: "<<v1<<", "<<v2<<" "; }
int add(int n, int m) { return n + m; }
double add(double a, double b) { return a + b; }
int main() {
    print( add(3, 4) );         // add(int, int) and print(int)
    print( add(3.25f, 1.5f) );  // add(double, double) and print(double)
    print( 7, 3.25 );           // print(int, double)
}
```

----------------


## GodboltId:58cTfvYhe

Book listing [lst-0024-book.cpp](lst-0024-book.cpp):
```cpp
// https://godbolt.org/z/58cTfvYhe 
int two() { return 2; }       // (ERR) once with int as return type…
double two() { return 2.0; }  // (ERR) … and once with double
int main() {
    int x = two();
    double y = two();
}
```

Godbolt Listing [lst-0024-godb.cpp](lst-0024-godb.cpp), [https://godbolt.org/z/58cTfvYhe](https://godbolt.org/z/58cTfvYhe):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/58cTfvYhe 
int two() { return 2; }       // (ERR) once with int as return type…
double two() { return 2.0; }  // (ERR) … and once with double
int main() {
    int x = two();
    double y = two();
}
```

----------------


## GodboltId:evGqG1oqa

Book listing [lst-0025-book.cpp](lst-0025-book.cpp):
```cpp
// https://godbolt.org/z/evGqG1oqa 
int twice(int a) { return a * 2; }
double twice(double a) { return a * 2.0; }
int main() {
    int x = twice(7);
    double y = twice(7.0);
}
```

Godbolt Listing [lst-0025-godb.cpp](lst-0025-godb.cpp), [https://godbolt.org/z/evGqG1oqa](https://godbolt.org/z/evGqG1oqa):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/evGqG1oqa 
int twice(int a) { return a * 2; }
double twice(double a) { return a * 2.0; }
int main() {
    int x = twice(7);
    double y = twice(7.0);
}
```

----------------


## Listing 07.10: Default parameters act like multiple overloads.

Book listing [lst-0028-book.cpp](lst-0028-book.cpp):
```cpp
// https://godbolt.org/z/fjc37rvEr 
int add(int n=0, int m=0, int o=0, int p=0, int q=0)  {
    return n+m+o+p+q;
}
int main() {
std::cout << add(1,2,3,4,5) << " ";
std::cout << add(1,2,3,4) << " "; // like add(1,2,3,4,0)
std::cout << add(1,2,3) << " ";   // like add(1,2,3,0,0)
std::cout << add(1,2) << " ";     // like add(1,2,0,0,0)
std::cout << add(1) << " ";       // like add(1,0,0,0,0)
std::cout << add() << " ";        // like add(0,0,0,0,0)
}
```

Godbolt Listing [lst-0028-godb.cpp](lst-0028-godb.cpp), [https://godbolt.org/z/fjc37rvEr](https://godbolt.org/z/fjc37rvEr):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/fjc37rvEr 
int add(int n=0, int m=0, int o=0, int p=0, int q=0)  {
    return n+m+o+p+q;
}
int main() {
std::cout << add(1,2,3,4,5) << " ";
std::cout << add(1,2,3,4) << " "; // like add(1,2,3,4,0)
std::cout << add(1,2,3) << " ";   // like add(1,2,3,0,0)
std::cout << add(1,2) << " ";     // like add(1,2,0,0,0)
std::cout << add(1) << " ";       // like add(1,0,0,0,0)
std::cout << add() << " ";        // like add(0,0,0,0,0)
}
```

----------------


## Listing 07.11: Alternative syntax for function declarations with trailing return type.

Book listing [lst-0033-book.cpp](lst-0033-book.cpp):
```cpp
auto  func() -> int;
auto  func() -> std::string;
auto  func() -> void;
auto  func() -> std::pair<int,std::string>;
auto  func() -> vector<int>;
auto  func() -> vector<int>&;
auto  func() -> const vector<int>&;
```

----------------


## Listing 8.12: Since C++14, you can let the compiler deduce the return type.

Book listing [lst-0034-book.cpp](lst-0034-book.cpp):
```cpp
// https://godbolt.org/z/6d5fxn9ed 
auto maxOf2(int a, int b) {
    return a<b ? b : a;   // one return: the compiler determines int
}
auto minOf3(int a, int b, int c) {
    if(a<b) return a<c ? a : c;
    else return b<c ? b : c;
}
auto medianOf3(int a, int b, int c) {
    // more complex, but no problem for the compiler
    return minOf3(maxOf2(a,b), maxOf2(b,c), maxOf2(a,c));
}
```

Godbolt Listing [lst-0034-godb.cpp](lst-0034-godb.cpp), [https://godbolt.org/z/6d5fxn9ed](https://godbolt.org/z/6d5fxn9ed):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/6d5fxn9ed 
auto maxOf2(int a, int b) {
    return a<b ? b : a;   // one return: the compiler determines int
}
auto minOf3(int a, int b, int c) {
    if(a<b) return a<c ? a : c;
    else return b<c ? b : c;
}
auto medianOf3(int a, int b, int c) {
    // more complex, but no problem for the compiler
    return minOf3(maxOf2(a,b), maxOf2(b,c), maxOf2(a,c));
}
```

----------------


## GodboltId:dMvqG85fr

Book listing [lst-0038-book.cpp](lst-0038-book.cpp):
```cpp
// https://godbolt.org/z/dMvqG85fr 
double add(double a, double b) { return a + b; }
double add(int, int) = delete;   // prohibit add(3,4)
```

Godbolt Listing [lst-0038-godb.cpp](lst-0038-godb.cpp), [https://godbolt.org/z/dMvqG85fr](https://godbolt.org/z/dMvqG85fr):
```cpp
//#(execute) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/dMvqG85fr 
double add(double a, double b) { return a + b; }
double add(int, int) = delete;   // prohibit add(3,4)
```

----------------


## GodboltId:8Gdso36zf

Book listing [lst-0039-book.cpp](lst-0039-book.cpp):
```cpp
// https://godbolt.org/z/8Gdso36zf 
class Widget : public Base {
    explicit Widget(int);      // no automatic conversion from int
    ~Widget();                 // Destructor with ~ before the name
    virtual void update();     // prefixed with virtual
    void calc1() override;     // suffixed with override
    void calc2() final;        // suffixed with final
    void draw() const;         // suffixed with const
    virtual void paint() = 0;  // abstract method
};
```

Godbolt Listing [lst-0039-godb.cpp](lst-0039-godb.cpp), [https://godbolt.org/z/8Gdso36zf](https://godbolt.org/z/8Gdso36zf):
```cpp
//#(execute) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/8Gdso36zf 
class Widget : public Base {
    explicit Widget(int);      // no automatic conversion from int
    ~Widget();                 // Destructor with ~ before the name
    virtual void update();     // prefixed with virtual
    void calc1() override;     // suffixed with override
    void calc2() final;        // suffixed with final
    void draw() const;         // suffixed with const
    virtual void paint() = 0;  // abstract method
};
```

----------------

