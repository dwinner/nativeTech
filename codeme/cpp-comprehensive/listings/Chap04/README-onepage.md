# Listings of Chap04.docx

This is the list of listings on one page.
You can also view a [linked summary](README.md).


## Listing 04.1: A very simple C++ program.

Book listing [lst-0002-book.cpp](lst-0002-book.cpp):
```cpp
// https://godbolt.org/z/Mdj7bGvar 
#include <iostream>                     // Include modules/libraries 
int main()                              // main() is the start of the program 
{ 
    int value = 100;                    // Variable with initial value 
    std::cout << "Divisors of " << value << " are:\n";        // output of text 
    for(int divider=1; divider <= value; divider = divider+1) // loop from 1 to 100 
    {                                   // begin of the repetition part
        if(value % divider == 0)        // test for conditional execution 
        std::cout << divider << ", ";   // only if the test is positive 
    }                                   // end of the loop 
    std::cout << "\n";                  // single output 
    return 0;                           // means end of program in main() 
}                                       // end of main()
```

Godbolt Listing [lst-0002-godb.cpp](lst-0002-godb.cpp), [https://godbolt.org/z/Mdj7bGvar](https://godbolt.org/z/Mdj7bGvar):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/Mdj7bGvar 
#include <iostream>                     // Include modules/libraries 
int main()                              // main() is the start of the program 
{ 
    int value = 100;                    // Variable with initial value 
    std::cout << "Divisors of " << value << " are:\n";        // output of text 
    for(int divider=1; divider <= value; divider = divider+1) // loop from 1 to 100 
    {                                   // begin of the repetition part
        if(value % divider == 0)        // test for conditional execution 
        std::cout << divider << ", ";   // only if the test is positive 
    }                                   // end of the loop 
    std::cout << "\n";                  // single output 
    return 0;                           // means end of program in main() 
}                                       // end of main()
```

----------------


## Listing 04.2 This program asks its users for a number.

Book listing [lst-0010-book.cpp](lst-0010-book.cpp):
```cpp
// https://godbolt.org/z/ceT6o5zxd
#include <iostream>    // for std::cin, std::cout, std::endl 
#include <string>      // std::stoi

void calculate(int n) {                           // a separate function
    using namespace std;                          // for std::cout and std::endl 
    /* output divisors */ 
    cout << "divisors of " << n << " are:\n"; 
    if(n == 0) { cout << "0\n"; return; }         // 0 is divisor of 0 
    for(int divider=1; divider <= n; ++divider) { // for divider=divider+1 
    if(n % divider == 0) 
      cout << divider << ", "; 
    } 
    cout << endl; 
} 
int main(int argc, const char* argv[]) {          // Arguments for main 
    /* Determine number */ 
    int value = 0; 
    if(argc<=1) { 
        std::cout << "Enter a number: "; 
        std::cin >> value;                        // read into variable wert 
        if(!std::cin) {                           // check whether reading worked 
            return 1;                             // error on user input 
        }    
    } else { 
        value = std::stoi(argv[1]); 
    }
    calculate(value);                             // function call 
    return 0; 
}
```

Godbolt Listing [lst-0010-godb.cpp](lst-0010-godb.cpp), [https://godbolt.org/z/ceT6o5zxd](https://godbolt.org/z/ceT6o5zxd):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/ceT6o5zxd
#include <iostream>    // for std::cin, std::cout, std::endl 
#include <string>      // std::stoi

void calculate(int n) {                           // a separate function
    using namespace std;                          // for std::cout and std::endl 
    /* output divisors */ 
    cout << "divisors of " << n << " are:\n"; 
    if(n == 0) { cout << "0\n"; return; }         // 0 is divisor of 0 
    for(int divider=1; divider <= n; ++divider) { // for divider=divider+1 
    if(n % divider == 0) 
      cout << divider << ", "; 
    } 
    cout << endl; 
} 
int main(int argc, const char* argv[]) {          // Arguments for main 
    /* Determine number */ 
    int value = 0; 
    if(argc<=1) { 
        std::cout << "Enter a number: "; 
        std::cin >> value;                        // read into variable wert 
        if(!std::cin) {                           // check whether reading worked 
            return 1;                             // error on user input 
        }    
    } else { 
        value = std::stoi(argv[1]); 
    }
    calculate(value);                             // function call 
    return 0; 
}
```

----------------


## Listing 04.3: A very unusually formatted piece of source code.

Book listing [lst-0011-book.cpp](lst-0011-book.cpp):
```cpp
// https://godbolt.org/z/33cc3xfo6 
#         include <iostream>       // # must be at the beginning of the line 
int            main( 
    ){ 
    std::cout 
<<"This is " 
        "text with <brackets>\n"   // string literal interrupted by new line 
        ; 
        /*type:*/ int 
        /*Variable:*/ a_Value 
        /*Init:*/ = 100;           // inner comments 
std::cout<<a_Value<<"\n";}         // no spaces
```

Godbolt Listing [lst-0011-godb.cpp](lst-0011-godb.cpp), [https://godbolt.org/z/33cc3xfo6](https://godbolt.org/z/33cc3xfo6):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/33cc3xfo6 
#         include <iostream>       // # must be at the beginning of the line 
int            main( 
    ){ 
    std::cout 
<<"This is " 
        "text with <brackets>\n"   // string literal interrupted by new line 
        ; 
        /*type:*/ int 
        /*Variable:*/ a_Value 
        /*Init:*/ = 100;           // inner comments 
std::cout<<a_Value<<"\n";}         // no spaces
```

----------------


## Listing 04.4 Comments with /* and */ can extend over several lines or interrupt a program line.

Book listing [lst-0012-book.cpp](lst-0012-book.cpp):
```cpp
int main() {
    /* My first program. It was 
       written by Max Muster.*/ 
    return /* The zero of success */ 0; 
}
```

----------------


## Listing 04.5: A custom C++ function.

Book listing [lst-0015-book.cpp](lst-0015-book.cpp):
```cpp
// https://godbolt.org/z/E4W9KhbnK
void calculate(int n) {                            // a separate function 
    using namespace std;                           // for std::cout and std::endl 
    /* output divisors */ 
    cout << " divisors of " << n << " are:\n"; 
    for(int divider=1; divider <= n; ++divider) {  // instead of divider=divider+1 
        if(n % divider == 0) 
            cout << divider << ", "; 
    } 
    cout << endl; 
}
```

Godbolt Listing [lst-0015-godb.cpp](lst-0015-godb.cpp), [https://godbolt.org/z/E4W9KhbnK](https://godbolt.org/z/E4W9KhbnK):
```cpp
//#(execute) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/E4W9KhbnK
void calculate(int n) {                            // a separate function 
    using namespace std;                           // for std::cout and std::endl 
    /* output divisors */ 
    cout << " divisors of " << n << " are:\n"; 
    for(int divider=1; divider <= n; ++divider) {  // instead of divider=divider+1 
        if(n % divider == 0) 
            cout << divider << ", "; 
    } 
    cout << endl; 
}
```

----------------


## Listing 04.6 Prefix operators are executed before the calculation.

Book listing [lst-0016-book.cpp](lst-0016-book.cpp):
```cpp
// https://godbolt.org/z/4Mr6fnaEr 
#include <iostream> // cout 
int main() { 
    int basis = 2; 
    int index = 10; 
    int number = 3 * (basis + ++index) - 1;  // index is incremented first 
    std::cout << number << '\n';             // output: 38 
}
```

Godbolt Listing [lst-0016-godb.cpp](lst-0016-godb.cpp), [https://godbolt.org/z/4Mr6fnaEr](https://godbolt.org/z/4Mr6fnaEr):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/4Mr6fnaEr 
#include <iostream> // cout 
int main() { 
    int basis = 2; 
    int index = 10; 
    int number = 3 * (basis + ++index) - 1;  // index is incremented first 
    std::cout << number << '\n';             // output: 38 
}
```

----------------


## GodboltId:r5hjrqqK9

Book listing [lst-0018-book.cpp](lst-0018-book.cpp):
```cpp
// https://godbolt.org/z/r5hjrqqK9 
#include <iostream> 
int main() { 
    int var = 10; 
    var += 2; 
    var *= 3; 
    var /= 4; 
    var -= 5; 
    std::cout << var << "\n"; // results in 4 
}
```

Godbolt Listing [lst-0018-godb.cpp](lst-0018-godb.cpp), [https://godbolt.org/z/r5hjrqqK9](https://godbolt.org/z/r5hjrqqK9):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/r5hjrqqK9 
#include <iostream> 
int main() { 
    int var = 10; 
    var += 2; 
    var *= 3; 
    var /= 4; 
    var -= 5; 
    std::cout << var << "\n"; // results in 4 
}
```

----------------


## Listing 04.7 A program that consists only of main and return.

Book listing [lst-0019-book.cpp](lst-0019-book.cpp):
```cpp
int main() { 
    if(2 < 1) return 1;    // one return 
    return 0;              // other return 
}                          // end of main
```

----------------


## Listing 04.8: Here you only set the outer curly braces.

Book listing [lst-0020-book.cpp](lst-0020-book.cpp):
```cpp
for(int divider=1; divider <= n; ++divider)  // for loop 
{                                            // start of the loop block 
    if(n % divider == 0) 
        std::cout << divider << ", "; 
}                                            // end of the loop block
```

----------------


## Listing 04.9 This is what it looks like if you set all curly brackets.

Book listing [lst-0021-book.cpp](lst-0021-book.cpp):
```cpp
if(argc<=1) {                                // start of the if statement 
    std::cout << "Enter a number: "; 
    std::cin >> number; 
    if(!std::cin) { 
        return 1; 
    } 
} else { 
    value = std::stoi(argv[1]); 
}                                            // end of the if statement
```

----------------


## Listing 04.10 The "if" is a statement and does not actually require curly brackets.

Book listing [lst-0022-book.cpp](lst-0022-book.cpp):
```cpp
for(int divider=1; divider <= value; ++divider) 
    if(value % divider == 0) 
        std::cout << divider << ", ";
```

----------------


## Listing 04.11: It is better to put individual statements in curly brackets.

Book listing [lst-0024-book.cpp](lst-0024-book.cpp):
```cpp
for(int divider=1; divider <= value; ++divider) { 
    if(value % divider == 0) { 
        std::cout << divider << ", ";
    }
}
```

----------------


## Listing 04.12 Assigning the result of a function call.

Book listing [lst-0027-book.cpp](lst-0027-book.cpp):
```cpp
value = std::stoi(argv[1]);
```

----------------


## Listing 04.13 Assigning the result of a calculation.

Book listing [lst-0028-book.cpp](lst-0028-book.cpp):
```cpp
divider = divider + 1
```

----------------


## Listing 04.14 An assignment is an expression with the type of the assigned variable.

Book listing [lst-0029-book.cpp](lst-0029-book.cpp):
```cpp
// https://godbolt.org/z/bh1MsPhje 
#include <iostream> 
int main() { 
    int a = 3; 
    int b = 7 + (a = 12) + 6;   // contains an assignment 
    std::cout << a << std::endl; 
}
```

Godbolt Listing [lst-0029-godb.cpp](lst-0029-godb.cpp), [https://godbolt.org/z/bh1MsPhje](https://godbolt.org/z/bh1MsPhje):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/bh1MsPhje 
#include <iostream> 
int main() { 
    int a = 3; 
    int b = 7 + (a = 12) + 6;   // contains an assignment 
    std::cout << a << std::endl; 
}
```

----------------


## Listing 04.15 The compiler helps to write correct programs by checking the types.

Book listing [lst-0030-book.cpp](lst-0030-book.cpp):
```cpp
// https://godbolt.org/z/9943MfExc 
#include <vector> 
class Image { 
    std::vector<char> data_; 
public: 
    void load(const char* filename); // loads image data 
}; 
class Screen { 
public: 
    void show(Image& image);         // (ERR) image should be const 
}; 
void paint(Screen &screen, const Image& image) { 
    screen.show(image); 
} 
int main() { 
    Image image {}; 
    image.load("peter.png"); 
    Screen screen {}; 
    paint(screen, image); 
}
```

Godbolt Listing [lst-0030-godb.cpp](lst-0030-godb.cpp), [https://godbolt.org/z/9943MfExc](https://godbolt.org/z/9943MfExc):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/9943MfExc 
#include <vector> 
class Image { 
    std::vector<char> data_; 
public: 
    void load(const char* filename); // loads image data 
}; 
class Screen { 
public: 
    void show(Image& image);         // (ERR) image should be const 
}; 
void paint(Screen &screen, const Image& image) { 
    screen.show(image); 
} 
int main() { 
    Image image {}; 
    image.load("peter.png"); 
    Screen screen {}; 
    paint(screen, image); 
}
```

----------------


## Listing 04.16 Variables of type bool can store the result of a comparison.

Book listing [lst-0031-book.cpp](lst-0031-book.cpp):
```cpp
// https://godbolt.org/z/zK9Ke9n48 
#include <iostream>                      // cout 
int main(int argc, const char* argv[]) { 
    bool withParameters = argc > 1;      // comparison result stored 
    if(withParameters) {                 // ... and used 
        std::cout << "You have called the program with parameters.\n"; 
    } 
    return 0; 
}
```

Godbolt Listing [lst-0031-godb.cpp](lst-0031-godb.cpp), [https://godbolt.org/z/zK9Ke9n48](https://godbolt.org/z/zK9Ke9n48):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/zK9Ke9n48 
#include <iostream>                      // cout 
int main(int argc, const char* argv[]) { 
    bool withParameters = argc > 1;      // comparison result stored 
    if(withParameters) {                 // ... and used 
        std::cout << "You have called the program with parameters.\n"; 
    } 
    return 0; 
}
```

----------------


## Listing 04.17: This is how you can write a C++ string as a literal in the source code.

Book listing [lst-0033-book.cpp](lst-0033-book.cpp):
```cpp
// https://godbolt.org/z/chTTqPjrM 
#include <string> 
#include <iostream> 
int main() { 
    std::cout << "string_literals\n"; 
    // using std::literals::string_literals::operator""s; 
    // using namespace std::string_literals; 
    // using namespace std::literals::string_literals; 
    using namespace std::literals; 
    std::cout << "Real string\n "s; 
}
```

Godbolt Listing [lst-0033-godb.cpp](lst-0033-godb.cpp), [https://godbolt.org/z/chTTqPjrM](https://godbolt.org/z/chTTqPjrM):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/chTTqPjrM 
#include <string> 
#include <iostream> 
int main() { 
    std::cout << "string_literals\n"; 
    // using std::literals::string_literals::operator""s; 
    // using namespace std::string_literals; 
    // using namespace std::literals::string_literals; 
    using namespace std::literals; 
    std::cout << "Real string\n "s; 
}
```

----------------


## Listing 04.18: Since C++17, the compiler determines the type parameters of class templates based on the constructor arguments.

Book listing [lst-0035-book.cpp](lst-0035-book.cpp):
```cpp
// https://godbolt.org/z/GbbcnYbqv 
std::vector vec { 1, 2, 3 };        // instead of vector<int>
std::tuple tpl { 5, 'x' };          // instead of tuple<int,char> 
std::shared_ptr<int> ptr { new int(5) };
std::shared_ptr ptr2 { ptr };       // instead of shared_ptr<int>
```

Godbolt Listing [lst-0035-godb.cpp](lst-0035-godb.cpp), [https://godbolt.org/z/GbbcnYbqv](https://godbolt.org/z/GbbcnYbqv):
```cpp
//#(execute) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/GbbcnYbqv 
std::vector vec { 1, 2, 3 };        // instead of vector<int>
std::tuple tpl { 5, 'x' };          // instead of tuple<int,char> 
std::shared_ptr<int> ptr { new int(5) };
std::shared_ptr ptr2 { ptr };       // instead of shared_ptr<int>
```

----------------


## Listing 04.19: You cannot assign anything to some variables; you can only initialize them.

Book listing [lst-0036-book.cpp](lst-0036-book.cpp):
```cpp
int main() {
    const int fest = 33;  // Initialization as constant
    fest = 80;            // (ERR) an assignment is impossible 
}
```

----------------


## Listing 04.20 Instead of = you can use {...} for initialization.

Book listing [lst-0037-book.cpp](lst-0037-book.cpp):
```cpp
int index = 1;        // old style, looks like an assignment 
int zaehler { 1 };    // C++11 style, clearly an initialization 
int counter = { 1 };  // in the C++11 style, the "=" is optional and is ignored
```

----------------


## Listing 04.21 You can include a namespace to make program text shorter.

Book listing [lst-0044-book.cpp](lst-0044-book.cpp):
```cpp
#include <iostream>                            // for std::cin, std::cout, std::endl 
#include <string>                              // for std::stoi 
void calculate(int n) {
    using namespace std;                       // for std::cout and std::endl 
    /* output divisors */
    cout << "divisors of " << n << " are:\n";  // cout instead of std::cout 
    for(int divider=1; divider <= n; ++divider) { 
    if(n % divider == 0)
        cout << divider << ", ";               // cout instead of std::cout 
    }
    cout << endl; 
}
```

----------------


## Listing 04.22 You can also use "using namespace" globally, but you should rarely do so.

Book listing [lst-0045-book.cpp](lst-0045-book.cpp):
```cpp
#include <iostream> 
#include <string>
using namespace std;            // (ERR) has a global effect; works, but is critical 
void calculate(int n) { 
    /* output divisors */
    cout << "divisors of " << n << " are:\n"; 
    // ...
} 
// ... also in other functions ...
```

----------------


## Listing 04.23 Get individual identifiers with "using".

Book listing [lst-0046-book.cpp](lst-0046-book.cpp):
```cpp
#include <iostream>                     // cin, cout, endl
using std::endl;                        // applies globally in this file 
void calculate(int n) {
    using std::cout;                    // applies locally in this function 
    /* output divider */
    cout << "divisors of " << n << " are:\n"; 
    for(int divider=1; divider <= n; ++divider) { 
    if(n % divider == 0)
        cout << divider << ", "; 
    }
    cout << endl; 
}
```

----------------


## Listing 04.24 Arithmetic operators in the application.

Book listing [lst-0047-book.cpp](lst-0047-book.cpp):
```cpp
// https://godbolt.org/z/cEGGfoeT7 
#include <iostream> 
int main() { 
std::cout << "3+4*5+6=" << 3+4*5+6 << "\n";              // dot before dash; = 29
    std::cout << "(3+4)*(5+6)=" << (3+4)*(5+6) << "\n";  // Parentheses; = 77 
    std::cout << "22/7=" << 22/7 
        << " remainder " << 22%7 << "\n";                // 22/7 = 3 remainder 1 
    for(int n=0; n < 10; ++n) { 
        std::cout << -2*n*n + 13*n - 4 << " ";           // with unary minus 
    } 
    std::cout << "\n"; 
    // output: -4 7 14 17 16 11 2 -11 -28 -49 
}
```

Godbolt Listing [lst-0047-godb.cpp](lst-0047-godb.cpp), [https://godbolt.org/z/cEGGfoeT7](https://godbolt.org/z/cEGGfoeT7):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/cEGGfoeT7 
#include <iostream> 
int main() { 
std::cout << "3+4*5+6=" << 3+4*5+6 << "\n";              // dot before dash; = 29
    std::cout << "(3+4)*(5+6)=" << (3+4)*(5+6) << "\n";  // Parentheses; = 77 
    std::cout << "22/7=" << 22/7 
        << " remainder " << 22%7 << "\n";                // 22/7 = 3 remainder 1 
    for(int n=0; n < 10; ++n) { 
        std::cout << -2*n*n + 13*n - 4 << " ";           // with unary minus 
    } 
    std::cout << "\n"; 
    // output: -4 7 14 17 16 11 2 -11 -28 -49 
}
```

----------------


## Listing 04.25 Programming example for the conversion of an integer into a bit sequence.

Book listing [lst-0050-book.cpp](lst-0050-book.cpp):
```cpp
// https://godbolt.org/z/nMcsbxvvv 
#include <iostream> 
void printBin(int x) { 
  while(x>0) {       // done? 
    int a = x/2;   // division by 2 
    int b = x%2;   // modulo, remainder of the division 
    std::cout << x <<" / 2 = " << a << ", remainder " << b<<'\n'; // output 
    x = a; 
  } 
} 
int main() { 
  printBin(412); 
}
```

Godbolt Listing [lst-0050-godb.cpp](lst-0050-godb.cpp), [https://godbolt.org/z/nMcsbxvvv](https://godbolt.org/z/nMcsbxvvv):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/nMcsbxvvv 
#include <iostream> 
void printBin(int x) { 
  while(x>0) {       // done? 
    int a = x/2;   // division by 2 
    int b = x%2;   // modulo, remainder of the division 
    std::cout << x <<" / 2 = " << a << ", remainder " << b<<'\n'; // output 
    x = a; 
  } 
} 
int main() { 
  printBin(412); 
}
```

----------------


## GodboltId:qdY41d5a9

Book listing [lst-0051-book.cpp](lst-0051-book.cpp):
```cpp
// https://godbolt.org/z/qdY41d5a9
#include <iostream> 
#include <bitset> 
constexpr unsigned n_bits = sizeof(unsigned short)*8; // 8 bits per char 
auto reverse_bits(unsigned val) -> unsigned short { 
  unsigned short ret = 0; 
  for (unsigned i = 0; i < n_bits; ++i ) { 
    ret = (ret << 1) | (val & 1); // one to the side, set lowest bit
    val >>= 1;                    // one in the other direction 
  } 
  return ret; 
} 
void tell(unsigned short val) { 
  std::bitset<n_bits> bits{val}; 
  std::cout << val << "=" << bits << " -> "; 
  auto lav = reverse_bits(val); 
  std::bitset<n_bits> stib{lav}; 
  std::cout << lav << "=" << stib << "\n"; 
} 
int main() { 
  tell(36u);  // output: 36=0000000000100100 -> 9216=0010010000000000 
  tell(199u); // output: 199=0000000011000111 -> 58112=1110001100000000
  tell(255u); // Output: 255=0000000011111111 -> 65280=1111111100000000 
  tell(256u); // Output: 256=0000000100000000 -> 128=0000000010000000 
}
```

Godbolt Listing [lst-0051-godb.cpp](lst-0051-godb.cpp), [https://godbolt.org/z/qdY41d5a9](https://godbolt.org/z/qdY41d5a9):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/qdY41d5a9
#include <iostream> 
#include <bitset> 
constexpr unsigned n_bits = sizeof(unsigned short)*8; // 8 bits per char 
auto reverse_bits(unsigned val) -> unsigned short { 
  unsigned short ret = 0; 
  for (unsigned i = 0; i < n_bits; ++i ) { 
    ret = (ret << 1) | (val & 1); // one to the side, set lowest bit
    val >>= 1;                    // one in the other direction 
  } 
  return ret; 
} 
void tell(unsigned short val) { 
  std::bitset<n_bits> bits{val}; 
  std::cout << val << "=" << bits << " -> "; 
  auto lav = reverse_bits(val); 
  std::bitset<n_bits> stib{lav}; 
  std::cout << lav << "=" << stib << "\n"; 
} 
int main() { 
  tell(36u);  // output: 36=0000000000100100 -> 9216=0010010000000000 
  tell(199u); // output: 199=0000000011000111 -> 58112=1110001100000000
  tell(255u); // Output: 255=0000000011111111 -> 65280=1111111100000000 
  tell(256u); // Output: 256=0000000100000000 -> 128=0000000010000000 
}
```

----------------


## GodboltId:5hqanTeYh

Book listing [lst-0053-book.cpp](lst-0053-book.cpp):
```cpp
// https://godbolt.org/z/5hqanTeYh 
#include <iostream> 
#include <bitset>  // helps with the output of numbers as a bit sequence 
int main() { 
    int a = 0; 
    for(int idx=0; idx<8; idx++) { 
        a <<= 2;                         // shift two bits to the left: "...100"
        a |= 1;                          // set the lowest bit: "...1"
    }
    std::cout << std::bitset<16>(a) << "\n"; // 0101010101010101 
    std::cout << a << "\n";              // 21845 
}
```

Godbolt Listing [lst-0053-godb.cpp](lst-0053-godb.cpp), [https://godbolt.org/z/5hqanTeYh](https://godbolt.org/z/5hqanTeYh):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/5hqanTeYh 
#include <iostream> 
#include <bitset>  // helps with the output of numbers as a bit sequence 
int main() { 
    int a = 0; 
    for(int idx=0; idx<8; idx++) { 
        a <<= 2;                         // shift two bits to the left: "...100"
        a |= 1;                          // set the lowest bit: "...1"
    }
    std::cout << std::bitset<16>(a) << "\n"; // 0101010101010101 
    std::cout << a << "\n";              // 21845 
}
```

----------------


## GodboltId:xrMYz93WP

Book listing [lst-0062-book.cpp](lst-0062-book.cpp):
```cpp
// https://godbolt.org/z/xrMYz93WP 
int main() { 
    for(int w1 = 1; w1 <= 6; ++w1) { // 1..6 
        for(int w2 = 0; w2 < 10; ++w2) { // 0..9 
            int max = w1 > w2 ? w1 : w2; // ternary operator 
        }
    }
}
```

Godbolt Listing [lst-0062-godb.cpp](lst-0062-godb.cpp), [https://godbolt.org/z/xrMYz93WP](https://godbolt.org/z/xrMYz93WP):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/xrMYz93WP 
int main() { 
    for(int w1 = 1; w1 <= 6; ++w1) { // 1..6 
        for(int w2 = 0; w2 < 10; ++w2) { // 0..9 
            int max = w1 > w2 ? w1 : w2; // ternary operator 
        }
    }
}
```

----------------


## Listing 04.26: You can concatenate several expressions with commas in brackets.

Book listing [lst-0063-book.cpp](lst-0063-book.cpp):
```cpp
// https://godbolt.org/z/rq66cWr3d 
int main() { 
    int a = 0; 
    int b = 0; 
    for(int w1 = 1; w1 <= 6; ++w1) { // 1..6 
        for(int w2 = 0; w2 < 10; ++w2) { // 0..9 
            int max = w1 > w2 ? (a+=b , w1) :( b+=1 , w2); // sequence operator
        }
    }
}
```

Godbolt Listing [lst-0063-godb.cpp](lst-0063-godb.cpp), [https://godbolt.org/z/rq66cWr3d](https://godbolt.org/z/rq66cWr3d):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/rq66cWr3d 
int main() { 
    int a = 0; 
    int b = 0; 
    for(int w1 = 1; w1 <= 6; ++w1) { // 1..6 
        for(int w2 = 0; w2 < 10; ++w2) { // 0..9 
            int max = w1 > w2 ? (a+=b , w1) :( b+=1 , w2); // sequence operator
        }
    }
}
```

----------------


## Listing 04.27: The sequence comma can be useful in for loops.

Book listing [lst-0064-book.cpp](lst-0064-book.cpp):
```cpp
// https://godbolt.org/z/vd85f7rMj 
#include <iostream> 
int main() { 
    int arr[] = { 8,3,7,3,11,999,5,6,7 }; 
    int len = 9; 
    for(int i=0, *p=arr; i<len && *p!=999; ++i, ++p) { // first ++i, then ++p 
        std::cout << i << ":" << *p << " "; 
    } 
    std::cout << "\n"; 
    // output: 0:8 1:3 2:7 3:3 4:11 
}
```

Godbolt Listing [lst-0064-godb.cpp](lst-0064-godb.cpp), [https://godbolt.org/z/vd85f7rMj](https://godbolt.org/z/vd85f7rMj):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/vd85f7rMj 
#include <iostream> 
int main() { 
    int arr[] = { 8,3,7,3,11,999,5,6,7 }; 
    int len = 9; 
    for(int i=0, *p=arr; i<len && *p!=999; ++i, ++p) { // first ++i, then ++p 
        std::cout << i << ":" << *p << " "; 
    } 
    std::cout << "\n"; 
    // output: 0:8 1:3 2:7 3:3 4:11 
}
```

----------------


## Listing 04.28: Some new data types are used here.

Book listing [lst-0068-book.cpp](lst-0068-book.cpp):
```cpp
// https://godbolt.org/z/MPPsbdq9c 
#include <iostream> // cin, cout for input and output

void input(unsigned &birthDay_, 
           unsigned &birthMonth_, 
           unsigned &birthYear_, 
           unsigned long long &taxnumber_, 
           double &bodyheight_)  
{ 
    /* Inputs still without good error handling... */ 
    std::cout << "Date of birth: "; std::cin >> birthDay_; 
    std::cout << "Month of birth: "; std::cin >> birthMonth_; 
    std::cout << "Year of birth: "; std::cin >> birthYear_; 
    std::cout << "Tax number: "; std::cin >> taxnumber_; 
    std::cout << "Height: "; std::cin >> bodyheight_; 
} 

int main() { 
    /* data */ 
    unsigned birthDay_ = 0;
    unsigned birthMonth_ = 0; 
    unsigned birthYear_ = 0; 
    unsigned long long taxnumber_ = 0; 
    double bodyheight_ = 0.0; 
    /* Input */ 
    input(birthDay_, birthMonth_, birthYear_, taxnumber_, bodyheight_); 
    /* Calculations */ 
    // ... 
}
```

Godbolt Listing [lst-0068-godb.cpp](lst-0068-godb.cpp), [https://godbolt.org/z/MPPsbdq9c](https://godbolt.org/z/MPPsbdq9c):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/MPPsbdq9c 
#include <iostream> // cin, cout for input and output

void input(unsigned &birthDay_, 
           unsigned &birthMonth_, 
           unsigned &birthYear_, 
           unsigned long long &taxnumber_, 
           double &bodyheight_)  
{ 
    /* Inputs still without good error handling... */ 
    std::cout << "Date of birth: "; std::cin >> birthDay_; 
    std::cout << "Month of birth: "; std::cin >> birthMonth_; 
    std::cout << "Year of birth: "; std::cin >> birthYear_; 
    std::cout << "Tax number: "; std::cin >> taxnumber_; 
    std::cout << "Height: "; std::cin >> bodyheight_; 
} 

int main() { 
    /* data */ 
    unsigned birthDay_ = 0;
    unsigned birthMonth_ = 0; 
    unsigned birthYear_ = 0; 
    unsigned long long taxnumber_ = 0; 
    double bodyheight_ = 0.0; 
    /* Input */ 
    input(birthDay_, birthMonth_, birthYear_, taxnumber_, bodyheight_); 
    /* Calculations */ 
    // ... 
}
```

----------------


## Listing 04.29: Arithmetic with integers.

Book listing [lst-0070-book.cpp](lst-0070-book.cpp):
```cpp
// https://godbolt.org/z/ebPcdxzvj 
#include <iostream> 
int main() { 
    std::cout << 3 + 4 * 5 + 6 << "\n";               // 29 
    std::cout << 20/7 << " Rest " << 20%7 << "\n";    // 2 remainder 6 
}
```

Godbolt Listing [lst-0070-godb.cpp](lst-0070-godb.cpp), [https://godbolt.org/z/ebPcdxzvj](https://godbolt.org/z/ebPcdxzvj):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/ebPcdxzvj 
#include <iostream> 
int main() { 
    std::cout << 3 + 4 * 5 + 6 << "\n";               // 29 
    std::cout << 20/7 << " Rest " << 20%7 << "\n";    // 2 remainder 6 
}
```

----------------


## Listing 04.30 Bit operations.

Book listing [lst-0071-book.cpp](lst-0071-book.cpp):
```cpp
// https://godbolt.org/z/jeTs5Exqx 
#include <iostream> 
int main() { 
    unsigned a = 0b1111'0000;        // 240 
    unsigned b = 0b0011'1100;        // 60 
    std::cout << ( a | b ) << "\n";  // Bit-Or: 252, in bits 1111'1100 
    std::cout << ( a & b ) << "\n";  // Bit-And: 48, in bits 0011'0000 
    std::cout << ( a ^ b ) << "\n";  // Exclusive-Or: 204, in bits 1100'1100 
    unsigned int c = 170;            // in bits 0..(24x0)..0'1010'1010 
    std::cout << ( ~c ) << "\n";     // Inv.: 4294967125, Bits: 1...(24x1)..1'0101'0101 
}
```

Godbolt Listing [lst-0071-godb.cpp](lst-0071-godb.cpp), [https://godbolt.org/z/jeTs5Exqx](https://godbolt.org/z/jeTs5Exqx):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/jeTs5Exqx 
#include <iostream> 
int main() { 
    unsigned a = 0b1111'0000;        // 240 
    unsigned b = 0b0011'1100;        // 60 
    std::cout << ( a | b ) << "\n";  // Bit-Or: 252, in bits 1111'1100 
    std::cout << ( a & b ) << "\n";  // Bit-And: 48, in bits 0011'0000 
    std::cout << ( a ^ b ) << "\n";  // Exclusive-Or: 204, in bits 1100'1100 
    unsigned int c = 170;            // in bits 0..(24x0)..0'1010'1010 
    std::cout << ( ~c ) << "\n";     // Inv.: 4294967125, Bits: 1...(24x1)..1'0101'0101 
}
```

----------------


## Listing 04.31 Index variables can be of type "size_t".

Book listing [lst-0073-book.cpp](lst-0073-book.cpp):
```cpp
// https://godbolt.org/z/zGso3xGcj 
#include <vector> 
#include <cstddef> // size_t 
int main() { 
    std::vector<int> data = { 100, -4, 6'699, 88, 0, } ; 
    int sum = 0; 
    for(size_t idx = 0; idx < data.size(); ++idx) { // a specific int type 
        sum += data[idx]; 
    }
 }
```

Godbolt Listing [lst-0073-godb.cpp](lst-0073-godb.cpp), [https://godbolt.org/z/zGso3xGcj](https://godbolt.org/z/zGso3xGcj):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/zGso3xGcj 
#include <vector> 
#include <cstddef> // size_t 
int main() { 
    std::vector<int> data = { 100, -4, 6'699, 88, 0, } ; 
    int sum = 0; 
    for(size_t idx = 0; idx < data.size(); ++idx) { // a specific int type 
        sum += data[idx]; 
    }
 }
```

----------------


## Listing 04.32 "double" cannot always store numbers exactly. Calculating and comparing with "==" is an error.

Book listing [lst-0074-book.cpp](lst-0074-book.cpp):
```cpp
// https://godbolt.org/z/8ae83nPYs
#include <iostream>     // cout 
#include <iomanip>      // setprecision, etc.
#include <cmath>        // fabs 
using std::cout;        // cout as abbreviation for std::cout 
int main() { 
    cout << std::fixed << std::setprecision(25);     // for better readable output 
    // 0.1 and 0.01 cannot store double exactly
    double x = 0.1 * 0.1; 
    cout << "0.1*0.1: " << x << "\n"; 
    // output: 0.1*0.1: 0.0100000000000000019428903 
    if(x == 0.01) {     // (ERR) never compare double with == 
        cout << "Yes! x == 0.01" << "\n"; 
    } else {
        cout << "Uh-oh! x != 0.01" << "\n";          // you see this output 
    } 
    // Attention especially when comparing with 0.0
    double null = x - 0.01; 
    cout << "null: " << null << "\n"; 
    // Output: null: 0.0000000000000000017347235 
    if(std::fabs(null) < 0.00000001) {               // compare with an "epsilon"
        cout << "Yes! null is close to 0.0" << "\n"; // you see this output 
    } else { 
        cout << "Uh-oh! null not close to 0.0" << "\n"; 
    } 
    // fractions of powers of 2 are less critical 
    double y = 0.5 * 0.5; 
    cout << "0.5*0.5: " << y << "\n"; 
    // Output: 0.5*0.5: 0.2500000000000000000000000 
    if(y == 0.25) {     // here the dangerous comparison works exceptionally
        cout << "Yes! y == 0.25" << "\n";            // you see this output 
    } else { 
        cout << "Oh-oh! y != 0.25" << "\n"; 
    } 
    // 
    return 0; 
}
```

Godbolt Listing [lst-0074-godb.cpp](lst-0074-godb.cpp), [https://godbolt.org/z/8ae83nPYs](https://godbolt.org/z/8ae83nPYs):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/8ae83nPYs
#include <iostream>     // cout 
#include <iomanip>      // setprecision, etc.
#include <cmath>        // fabs 
using std::cout;        // cout as abbreviation for std::cout 
int main() { 
    cout << std::fixed << std::setprecision(25);     // for better readable output 
    // 0.1 and 0.01 cannot store double exactly
    double x = 0.1 * 0.1; 
    cout << "0.1*0.1: " << x << "\n"; 
    // output: 0.1*0.1: 0.0100000000000000019428903 
    if(x == 0.01) {     // (ERR) never compare double with == 
        cout << "Yes! x == 0.01" << "\n"; 
    } else {
        cout << "Uh-oh! x != 0.01" << "\n";          // you see this output 
    } 
    // Attention especially when comparing with 0.0
    double null = x - 0.01; 
    cout << "null: " << null << "\n"; 
    // Output: null: 0.0000000000000000017347235 
    if(std::fabs(null) < 0.00000001) {               // compare with an "epsilon"
        cout << "Yes! null is close to 0.0" << "\n"; // you see this output 
    } else { 
        cout << "Uh-oh! null not close to 0.0" << "\n"; 
    } 
    // fractions of powers of 2 are less critical 
    double y = 0.5 * 0.5; 
    cout << "0.5*0.5: " << y << "\n"; 
    // Output: 0.5*0.5: 0.2500000000000000000000000 
    if(y == 0.25) {     // here the dangerous comparison works exceptionally
        cout << "Yes! y == 0.25" << "\n";            // you see this output 
    } else { 
        cout << "Oh-oh! y != 0.25" << "\n"; 
    } 
    // 
    return 0; 
}
```

----------------


## Listing 04.33: Several ways to mark "double" literals.

Book listing [lst-0075-book.cpp](lst-0075-book.cpp):
```cpp
// https://godbolt.org/z/qx8h7a4j9
#include <iostream> 
#include <iomanip>                                 // fixed, setprecision 
using std::cout;                                   // abbreviated cout 
int main() { 
    cout << std::setprecision(2) << std::fixed;    // two decimal places 
   cout << "1/4: " << 0.25 << "\n";                // comma notation for double 
    // output: 1/4: 0.25

    cout << "2/4: " << 0.5 << "\n"; 
    // Output: 2/4: 0.50 
    cout << "3/4: " << 0.75 << "\n"; 
    // Output: 3/4: 0.75 
    cout << "4/4: " << 1 << " or " << 1.0 << "\n"; // (ERR) recognizes 1 as int 
    // Output 4/4: 1 or 1.00 
    cout << "1e0: " << 1e0 << "\n";                // scientific notation 
    // output: 1e0: 1.00 
    cout << "0x10.1p0: " << 0x10.1p0 << "\n";      // hexadecimal notation 
    // output: 0x10.1p0: 16.06 
}
```

Godbolt Listing [lst-0075-godb.cpp](lst-0075-godb.cpp), [https://godbolt.org/z/qx8h7a4j9](https://godbolt.org/z/qx8h7a4j9):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/qx8h7a4j9
#include <iostream> 
#include <iomanip>                                 // fixed, setprecision 
using std::cout;                                   // abbreviated cout 
int main() { 
    cout << std::setprecision(2) << std::fixed;    // two decimal places 
   cout << "1/4: " << 0.25 << "\n";                // comma notation for double 
    // output: 1/4: 0.25

    cout << "2/4: " << 0.5 << "\n"; 
    // Output: 2/4: 0.50 
    cout << "3/4: " << 0.75 << "\n"; 
    // Output: 3/4: 0.75 
    cout << "4/4: " << 1 << " or " << 1.0 << "\n"; // (ERR) recognizes 1 as int 
    // Output 4/4: 1 or 1.00 
    cout << "1e0: " << 1e0 << "\n";                // scientific notation 
    // output: 1e0: 1.00 
    cout << "0x10.1p0: " << 0x10.1p0 << "\n";      // hexadecimal notation 
    // output: 0x10.1p0: 16.06 
}
```

----------------


## Listing 04.34 Floating-point literals become imprecise at some point.

Book listing [lst-0076-book.cpp](lst-0076-book.cpp):
```cpp
// https://godbolt.org/z/6qfo4oYnr 
#include <iostream> 
#include <iomanip> // fixed, setprecision 
int main() { 
    using std::cout;
    cout << std::setprecision(30) << std::fixed; // always output 30 digits 
    cout << 1.111222333444555666777888999f << "\n"; // float literal 
    // output: 1.111222386360168457031250000000 
    cout << 1.111222333444555666777888999 << "\n";  // double is default 
    // Output: 1.111222333444555676607023997349 
    cout << 1.111222333444555666777888999d << "\n"; // double literal 
    // output: 1.111222333444555676607023997349 
    cout << 1.111222333444555666777888999L << "\n"; // long double 
    // output: 1.111222333444555666740784227731 
}
```

Godbolt Listing [lst-0076-godb.cpp](lst-0076-godb.cpp), [https://godbolt.org/z/6qfo4oYnr](https://godbolt.org/z/6qfo4oYnr):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/6qfo4oYnr 
#include <iostream> 
#include <iomanip> // fixed, setprecision 
int main() { 
    using std::cout;
    cout << std::setprecision(30) << std::fixed; // always output 30 digits 
    cout << 1.111222333444555666777888999f << "\n"; // float literal 
    // output: 1.111222386360168457031250000000 
    cout << 1.111222333444555666777888999 << "\n";  // double is default 
    // Output: 1.111222333444555676607023997349 
    cout << 1.111222333444555666777888999d << "\n"; // double literal 
    // output: 1.111222333444555676607023997349 
    cout << 1.111222333444555666777888999L << "\n"; // long double 
    // output: 1.111222333444555666740784227731 
}
```

----------------


## Listing 04.35 Among other things, you can discover special values with "fpclassify".

Book listing [lst-0077-book.cpp](lst-0077-book.cpp):
```cpp
// https://godbolt.org/z/Tvx4qcrE6 
#include <iostream> 
#include <cmath>   // fpclassify 
#include <limits>  // numeric_limits 
#include <string> 
std::string fpclass(double x) { 
    switch(std::fpclassify(x)) { 
        case FP_INFINITE:  return "infinite"; 
        case FP_NAN:       return "NaN"; 
        case FP_NORMAL:    return "normal"; 
        case FP_SUBNORMAL: return "subnormal"; 
        case FP_ZERO:      return "zero"; 
        default:           return "unknown"; 
    } 
} 
int main() { 
    const auto dmin = std::numeric_limits<double>::min(); 
    std::cout 
      <<"1.0/0.0 is "<<fpclass(1/0.0)<<'\n'   // output: 1.0/0.0 is infinite
      <<"0.0/0.0 is "<<fpclass(0.0/0.0)<<'\n' // Output: 0.0/0.0 is NaN 
      <<"dmin/2 is "<<fpclass(dmin/2)<<'\n'   // Output: dmin/2 is subnormal 
      <<"-0.0 is "<<fpclass(-0.0)<<'\n'       // Output: -0.0 is zero 
      <<"1.0 is "<<fpclass(1.0)<<'\n';        // Output: 1.0 is normal 
}
```

Godbolt Listing [lst-0077-godb.cpp](lst-0077-godb.cpp), [https://godbolt.org/z/Tvx4qcrE6](https://godbolt.org/z/Tvx4qcrE6):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/Tvx4qcrE6 
#include <iostream> 
#include <cmath>   // fpclassify 
#include <limits>  // numeric_limits 
#include <string> 
std::string fpclass(double x) { 
    switch(std::fpclassify(x)) { 
        case FP_INFINITE:  return "infinite"; 
        case FP_NAN:       return "NaN"; 
        case FP_NORMAL:    return "normal"; 
        case FP_SUBNORMAL: return "subnormal"; 
        case FP_ZERO:      return "zero"; 
        default:           return "unknown"; 
    } 
} 
int main() { 
    const auto dmin = std::numeric_limits<double>::min(); 
    std::cout 
      <<"1.0/0.0 is "<<fpclass(1/0.0)<<'\n'   // output: 1.0/0.0 is infinite
      <<"0.0/0.0 is "<<fpclass(0.0/0.0)<<'\n' // Output: 0.0/0.0 is NaN 
      <<"dmin/2 is "<<fpclass(dmin/2)<<'\n'   // Output: dmin/2 is subnormal 
      <<"-0.0 is "<<fpclass(-0.0)<<'\n'       // Output: -0.0 is zero 
      <<"1.0 is "<<fpclass(1.0)<<'\n';        // Output: 1.0 is normal 
}
```

----------------


## GodboltId:We3oWrcWW

Book listing [lst-0080-book.cpp](lst-0080-book.cpp):
```cpp
// https://godbolt.org/z/We3oWrcWW 
#include <iostream> 
#include <cmath>     // sqrt 
#include <concepts>  // floating_point 
using std::min; using std::max; using std::floating_point;

template<floating_point T> T heron(T a, T b, T c) { 
    auto s = (a+b+c) / 2; 
    return sqrt(s*(s-a)*(s-b)*(s-c)); 
}

template<floating_point T> T kahan(T a, T b, T c) { 
    auto x = max(a,max(b,c)); 
    auto y = max(min(a,b), min(max(a,b),c)); 
    auto z = min(a,min(b,c)); return 
    sqrt( (x+(y+z))*(z-(x-y))*(z+(x-y))*(x+(y-z)) )/4 ; 
}

template<floating_point T> void triangle(T a, T b, T c) { 
    std::cout << "heron: " << heron(a,b,c) << '\n'; 
    std::cout << "kahan: " << kahan(a,b,c) << '\n'; 
}

int main() { 
    triangle(3.0f, 4.0f, 5.0f); 
}
```

Godbolt Listing [lst-0080-godb.cpp](lst-0080-godb.cpp), [https://godbolt.org/z/We3oWrcWW](https://godbolt.org/z/We3oWrcWW):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/We3oWrcWW 
#include <iostream> 
#include <cmath>     // sqrt 
#include <concepts>  // floating_point 
using std::min; using std::max; using std::floating_point;

template<floating_point T> T heron(T a, T b, T c) { 
    auto s = (a+b+c) / 2; 
    return sqrt(s*(s-a)*(s-b)*(s-c)); 
}

template<floating_point T> T kahan(T a, T b, T c) { 
    auto x = max(a,max(b,c)); 
    auto y = max(min(a,b), min(max(a,b),c)); 
    auto z = min(a,min(b,c)); return 
    sqrt( (x+(y+z))*(z-(x-y))*(z+(x-y))*(x+(y-z)) )/4 ; 
}

template<floating_point T> void triangle(T a, T b, T c) { 
    std::cout << "heron: " << heron(a,b,c) << '\n'; 
    std::cout << "kahan: " << kahan(a,b,c) << '\n'; 
}

int main() { 
    triangle(3.0f, 4.0f, 5.0f); 
}
```

----------------


## GodboltId:863b1z4GY

Book listing [lst-0081-book.cpp](lst-0081-book.cpp):
```cpp
// https://godbolt.org/z/863b1z4GY 
int main() { 
    std::cout << std::setprecision(15) << std::fixed; 
    triangle(100'000.0f, 99'999.999'79f, 0.000'29f); 
    triangle(100'000.0,  99'999.999'79,  0.000'29); 
}
```

Godbolt Listing [lst-0081-godb.cpp](lst-0081-godb.cpp), [https://godbolt.org/z/863b1z4GY](https://godbolt.org/z/863b1z4GY):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/863b1z4GY 
int main() { 
    std::cout << std::setprecision(15) << std::fixed; 
    triangle(100'000.0f, 99'999.999'79f, 0.000'29f); 
    triangle(100'000.0,  99'999.999'79,  0.000'29); 
}
```

----------------


## Listing 04.36 Accumulated time measurement.

Book listing [lst-0082-book.cpp](lst-0082-book.cpp):
```cpp
// https://godbolt.org/z/sEE94bh5K
#include <iostream>
#include <iomanip> // setprecision, fixed 
constexpr int framesPerSec = 25; 
constexpr int runtimeInSecs = 3600; 
constexpr int framesTotal = runtimeInSecs * framesPerSec; 
constexpr float frametime = 1.0f / framesPerSec;

int main() { 
    float filmtime = 0.f; 
    for(int n=1; n <= framesTotal; ++n) { // 1 .. framesTotal
        filmtime += frametime;            // accumulate 
        // ... here code for this frame ...
    } 
    std::cout << std::setprecision(10) << std::fixed 
        << filmtime << '\n';              // output: 3602.2695312500 
}
```

Godbolt Listing [lst-0082-godb.cpp](lst-0082-godb.cpp), [https://godbolt.org/z/sEE94bh5K](https://godbolt.org/z/sEE94bh5K):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/sEE94bh5K
#include <iostream>
#include <iomanip> // setprecision, fixed 
constexpr int framesPerSec = 25; 
constexpr int runtimeInSecs = 3600; 
constexpr int framesTotal = runtimeInSecs * framesPerSec; 
constexpr float frametime = 1.0f / framesPerSec;

int main() { 
    float filmtime = 0.f; 
    for(int n=1; n <= framesTotal; ++n) { // 1 .. framesTotal
        filmtime += frametime;            // accumulate 
        // ... here code for this frame ...
    } 
    std::cout << std::setprecision(10) << std::fixed 
        << filmtime << '\n';              // output: 3602.2695312500 
}
```

----------------


## Listing 04.37 Closed time calculation.

Book listing [lst-0083-book.cpp](lst-0083-book.cpp):
```cpp
// https://godbolt.org/z/dKPo7andE 
#include <iostream> 
#include <iomanip> // setprecision, fixed 
constexpr int framesPerSec = 25; 
constexpr int runtimeInSecs = 3600; 
constexpr int framesTotal = runtimeInSecs * framesPerSec; 
constexpr float frametime = 1.0f / framesPerSec;
int main() {
    float filmtime = 0.f; 
    for(int n=1; n <= framesTotal; ++n) { // 1 .. framesTotal, because of formula
        filmtime = frametime * n;         // scale 
        // ... here code for this frame ... 
    } 
    std::cout << std::setprecision(10) << std::fixed 
        << filmtime << '\n';              // output: 3600.0000000000 
}
```

Godbolt Listing [lst-0083-godb.cpp](lst-0083-godb.cpp), [https://godbolt.org/z/dKPo7andE](https://godbolt.org/z/dKPo7andE):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/dKPo7andE 
#include <iostream> 
#include <iomanip> // setprecision, fixed 
constexpr int framesPerSec = 25; 
constexpr int runtimeInSecs = 3600; 
constexpr int framesTotal = runtimeInSecs * framesPerSec; 
constexpr float frametime = 1.0f / framesPerSec;
int main() {
    float filmtime = 0.f; 
    for(int n=1; n <= framesTotal; ++n) { // 1 .. framesTotal, because of formula
        filmtime = frametime * n;         // scale 
        // ... here code for this frame ... 
    } 
    std::cout << std::setprecision(10) << std::fixed 
        << filmtime << '\n';              // output: 3600.0000000000 
}
```

----------------


## Listing 04.38: You can calculate arithmetically with complex numbers.

Book listing [lst-0087-book.cpp](lst-0087-book.cpp):
```cpp
// https://godbolt.org/z/43rYYeKx9 
#include <iostream> 
#include <iomanip> // setprecision, fixed
#include <complex> 
using std::cout; using std::complex;

int main() {
    using namespace std::complex_literals;   // for i-suffix 
    cout << std::fixed << std::setprecision(1); 
    complex<double> z1 = 1i * 1i;            // i times i 
    cout << z1 << '\n';                      // output: (-1.0,0.0)
    complex<double> z2 = std::pow(1i, 2);    // i-square 
    cout << z2 << '\n';                      // Output: (-1.0,0.0) 
    double PI = std::acos(-1);               // Length of half a unit circle 
    complex<double> z3 = std::exp(1i * PI);  // Euler formula 
    cout << z3 << '\n';                      // Output: (-1.0,0.0) 
    complex<double> a(3, 4);                 // usually as a constructor 
    complex<double> b = 1. - 2i;             // practically as a literal

    // Calculations: 
    cout << "a + b = " << a + b << "\n";     // Output: a + b = (4.0,2.0) 
    cout << "a * b = " << a * b << "\n";     // Output: a * b = (11.0,-2.0) 
    cout << "a / b = " << a / b << "\n";     // Output: a / b = (-1.0,2.0) 
    cout << "|a| = " << abs(a) << "\n";      // Output: |a| = 5.0 
    cout << "conj(a) = " << conj(a) << "\n"; // Output: conj(a) = (3.0,-4.0) 
    cout << "norm(a) = " << norm(a) << "\n"; // Output: norm(a) = 25.0 
    cout << "abs(a) = " << abs(a) << "\n";   // Output: abs(a) = 5.0 
    cout << "exp(a) = " << exp(a) << "\n";   // Output: exp(a) = (-13.1,-15.2) 
}
```

Godbolt Listing [lst-0087-godb.cpp](lst-0087-godb.cpp), [https://godbolt.org/z/43rYYeKx9](https://godbolt.org/z/43rYYeKx9):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/43rYYeKx9 
#include <iostream> 
#include <iomanip> // setprecision, fixed
#include <complex> 
using std::cout; using std::complex;

int main() {
    using namespace std::complex_literals;   // for i-suffix 
    cout << std::fixed << std::setprecision(1); 
    complex<double> z1 = 1i * 1i;            // i times i 
    cout << z1 << '\n';                      // output: (-1.0,0.0)
    complex<double> z2 = std::pow(1i, 2);    // i-square 
    cout << z2 << '\n';                      // Output: (-1.0,0.0) 
    double PI = std::acos(-1);               // Length of half a unit circle 
    complex<double> z3 = std::exp(1i * PI);  // Euler formula 
    cout << z3 << '\n';                      // Output: (-1.0,0.0) 
    complex<double> a(3, 4);                 // usually as a constructor 
    complex<double> b = 1. - 2i;             // practically as a literal

    // Calculations: 
    cout << "a + b = " << a + b << "\n";     // Output: a + b = (4.0,2.0) 
    cout << "a * b = " << a * b << "\n";     // Output: a * b = (11.0,-2.0) 
    cout << "a / b = " << a / b << "\n";     // Output: a / b = (-1.0,2.0) 
    cout << "|a| = " << abs(a) << "\n";      // Output: |a| = 5.0 
    cout << "conj(a) = " << conj(a) << "\n"; // Output: conj(a) = (3.0,-4.0) 
    cout << "norm(a) = " << norm(a) << "\n"; // Output: norm(a) = 25.0 
    cout << "abs(a) = " << abs(a) << "\n";   // Output: abs(a) = 5.0 
    cout << "exp(a) = " << exp(a) << "\n";   // Output: exp(a) = (-13.1,-15.2) 
}
```

----------------


## Listing 04.39 Output in unspecified order.

Book listing [lst-0089-book.cpp](lst-0089-book.cpp):
```cpp
// https://godbolt.org/z/eW1cfYcGs 
#include <iostream> 
void output(int a, int b) { 
    std::cout << a << ' ' << b << '\n'; 
} 
int number() { 
    static int val = 0; 
    return ++val; 
} 
int main() {
    output(number(), number()); // in which order?
}
```

Godbolt Listing [lst-0089-godb.cpp](lst-0089-godb.cpp), [https://godbolt.org/z/eW1cfYcGs](https://godbolt.org/z/eW1cfYcGs):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/eW1cfYcGs 
#include <iostream> 
void output(int a, int b) { 
    std::cout << a << ' ' << b << '\n'; 
} 
int number() { 
    static int val = 0; 
    return ++val; 
} 
int main() {
    output(number(), number()); // in which order?
}
```

----------------

