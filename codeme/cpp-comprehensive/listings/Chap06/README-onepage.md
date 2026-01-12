# Listings of Chap06.docx

This is the list of listings on one page.
You can also view a [linked summary](README.md).


## Listing 06.1 Some possible uses of strings.

Book listing [lst-0001-book.cpp](lst-0001-book.cpp):
```cpp
// https://godbolt.org/z/cv1qfcEd4
#include <iostream>      // cin, cout for input and output
#include <string>        // You need this header of the standard library

void input(
    std::string &name,   // as parameter 
    unsigned &birthYear) 
{ 
    /* Input still without good error handling... */ 
    std::cout << "Name: ";
    std::getline(std::cin, name);  // getline reads a string 
    if(name.length() == 0) {       // length is a method of string 
        std::cout << "You have entered an empty name.\n"; 
        exit(1); 
    } 
    std::cout << "Birth year: "; 
    std::cin >> birthYear; 
} 
int main() { 
    /* data */
    std::string name;              // defines and initializes a string variable 
    unsigned birthJahr = 0; 
    /* input */ 
    input(name, birthJahr); 
    /* calculations */ 
    // ... 
}
```

Godbolt Listing [lst-0001-godb.cpp](lst-0001-godb.cpp), [https://godbolt.org/z/cv1qfcEd4](https://godbolt.org/z/cv1qfcEd4):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/cv1qfcEd4
#include <iostream>      // cin, cout for input and output
#include <string>        // You need this header of the standard library

void input(
    std::string &name,   // as parameter 
    unsigned &birthYear) 
{ 
    /* Input still without good error handling... */ 
    std::cout << "Name: ";
    std::getline(std::cin, name);  // getline reads a string 
    if(name.length() == 0) {       // length is a method of string 
        std::cout << "You have entered an empty name.\n"; 
        exit(1); 
    } 
    std::cout << "Birth year: "; 
    std::cin >> birthYear; 
} 
int main() { 
    /* data */
    std::string name;              // defines and initializes a string variable 
    unsigned birthJahr = 0; 
    /* input */ 
    input(name, birthJahr); 
    /* calculations */ 
    // ... 
}
```

----------------


## Listing 06.2 A string_view can be easily created from a string.

Book listing [lst-0004-book.cpp](lst-0004-book.cpp):
```cpp
// https://godbolt.org/z/1zfo7fPcs
#include <iostream> 
#include <string>
#include <string_view> 
void show_middle(std::string_view msg) {       // string_view is a good parameter
    auto middle = msg.substr(2, msg.size()-4); // substr returns string_view 
    std::cout << middle << "\n"; 
} 
int main() { 
    using namespace std::literals; 
    const std::string aaa = "##Some text##"s; 
    show_middle(aaa);                          // conversion to string_view 
    auto bbb = "++More text++"sv;              // string_view as literal 
    show_middle(bbb); 
}
```

Godbolt Listing [lst-0004-godb.cpp](lst-0004-godb.cpp), [https://godbolt.org/z/1zfo7fPcs](https://godbolt.org/z/1zfo7fPcs):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/1zfo7fPcs
#include <iostream> 
#include <string>
#include <string_view> 
void show_middle(std::string_view msg) {       // string_view is a good parameter
    auto middle = msg.substr(2, msg.size()-4); // substr returns string_view 
    std::cout << middle << "\n"; 
} 
int main() { 
    using namespace std::literals; 
    const std::string aaa = "##Some text##"s; 
    show_middle(aaa);                          // conversion to string_view 
    auto bbb = "++More text++"sv;              // string_view as literal 
    show_middle(bbb); 
}
```

----------------


## Listing 06.3 The error stream is separated from the standard output.

Book listing [lst-0005-book.cpp](lst-0005-book.cpp):
```cpp
// https://godbolt.org/z/P1fn5q8z8
// Call this program with 'prog.exe > file.txt', for example. 
#include <iostream>  // cout, cerr 
int main() {
    std::cout << "Output to cout\n";       // is output after 'file.txt'
    std::cerr << "Error message!\n";       // still appears on the console
    std::cout << "Normal output again\n";  // back into the file 
}
```

Godbolt Listing [lst-0005-godb.cpp](lst-0005-godb.cpp), [https://godbolt.org/z/P1fn5q8z8](https://godbolt.org/z/P1fn5q8z8):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/P1fn5q8z8
// Call this program with 'prog.exe > file.txt', for example. 
#include <iostream>  // cout, cerr 
int main() {
    std::cout << "Output to cout\n";       // is output after 'file.txt'
    std::cerr << "Error message!\n";       // still appears on the console
    std::cout << "Normal output again\n";  // back into the file 
}
```

----------------


## Listing 06.4 A function with inputs and outputs.

Book listing [lst-0006-book.cpp](lst-0006-book.cpp):
```cpp
// https://godbolt.org/z/xYvPxbjr5 
#include <iostream>                    // cin, cout for input and output 
#include <string> 
#include <array> 
using std::cin; using std::cout;       // abbreviations cin and cout 
void input( 
    std::string &name, 
    unsigned &birthDay, 
    unsigned &birthMonth, 
    unsigned &birthYear, 
    long long &socialSecurity, 
    std::array<int,12> &monthIncomes)  // array is a container 
{ 
    /* input still without good error handling... */ 
    cout << "Name: "; 
    std::getline(cin, name);           // getline takes input stream and string 
    if(name.length() == 0) { 
        cout << "You have entered an empty name.\n"; 
        exit(1); 
    } 
    cout << "birth day: "; cin >> birthDay; 
    cout << "birth month: "; cin >> birthMonth; 
    cout << "birth year: "; cin >> birthYear; 
    cout << "social security: "; cin >> socialSecurity; 
    for(int m=0; m<12; ++m) { 
        cout << "income month " << m+1 << ": "; // multiple outputs
        cin >> monthIncomes[m];                 // read in with operator 
    } 
    cout << std::endl; 
} 
int main() { 
    std::string name{}; 
    unsigned day = 0; 
    unsigned month = 0; 
    unsigned year = 0; 
    long long ssid = 0; 
    std::array<int,12> incomes{};
    input(name, day, month, year, ssid, incomes); 
    // ... calculations ... 
}
```

Godbolt Listing [lst-0006-godb.cpp](lst-0006-godb.cpp), [https://godbolt.org/z/xYvPxbjr5](https://godbolt.org/z/xYvPxbjr5):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/xYvPxbjr5 
#include <iostream>                    // cin, cout for input and output 
#include <string> 
#include <array> 
using std::cin; using std::cout;       // abbreviations cin and cout 
void input( 
    std::string &name, 
    unsigned &birthDay, 
    unsigned &birthMonth, 
    unsigned &birthYear, 
    long long &socialSecurity, 
    std::array<int,12> &monthIncomes)  // array is a container 
{ 
    /* input still without good error handling... */ 
    cout << "Name: "; 
    std::getline(cin, name);           // getline takes input stream and string 
    if(name.length() == 0) { 
        cout << "You have entered an empty name.\n"; 
        exit(1); 
    } 
    cout << "birth day: "; cin >> birthDay; 
    cout << "birth month: "; cin >> birthMonth; 
    cout << "birth year: "; cin >> birthYear; 
    cout << "social security: "; cin >> socialSecurity; 
    for(int m=0; m<12; ++m) { 
        cout << "income month " << m+1 << ": "; // multiple outputs
        cin >> monthIncomes[m];                 // read in with operator 
    } 
    cout << std::endl; 
} 
int main() { 
    std::string name{}; 
    unsigned day = 0; 
    unsigned month = 0; 
    unsigned year = 0; 
    long long ssid = 0; 
    std::array<int,12> incomes{};
    input(name, day, month, year, ssid, incomes); 
    // ... calculations ... 
}
```

----------------


## GodboltId:a6qqbWKMs

Book listing [lst-0007-book.cpp](lst-0007-book.cpp):
```cpp
// https://godbolt.org/z/a6qqbWKMs 
#include <cstdlib> 
#include <iostream>

int main(int argc, char* argv[]) { 
    for(int i=1; i<argc; ++i) {                // start at 1 
        for(char* p=argv[i]; *p!='\0'; ++p) { 
            char c = toupper(*p); 
            std::cout << c; 
        } 
        std::cout << ' '; 
    } 
    std::cout << '\n'; 
}
```

Godbolt Listing [lst-0007-godb.cpp](lst-0007-godb.cpp), [https://godbolt.org/z/a6qqbWKMs](https://godbolt.org/z/a6qqbWKMs):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/a6qqbWKMs 
#include <cstdlib> 
#include <iostream>

int main(int argc, char* argv[]) { 
    for(int i=1; i<argc; ++i) {                // start at 1 
        for(char* p=argv[i]; *p!='\0'; ++p) { 
            char c = toupper(*p); 
            std::cout << c; 
        } 
        std::cout << ' '; 
    } 
    std::cout << '\n'; 
}
```

----------------


## GodboltId:McqfcWb8j

Book listing [lst-0008-book.cpp](lst-0008-book.cpp):
```cpp
// https://godbolt.org/z/McqfcWb8j 
#include <fstream> 
int main(int argc, char* argv[]) {
    std::ofstream myOutput{"output1.txt"}; 
    myOutput << "line 1\n"; 
    myOutput << "line 2\n"; 
}
```

Godbolt Listing [lst-0008-godb.cpp](lst-0008-godb.cpp), [https://godbolt.org/z/McqfcWb8j](https://godbolt.org/z/McqfcWb8j):
```cpp
//#(execute) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/McqfcWb8j 
#include <fstream> 
int main(int argc, char* argv[]) {
    std::ofstream myOutput{"output1.txt"}; 
    myOutput << "line 1\n"; 
    myOutput << "line 2\n"; 
}
```

----------------


## GodboltId:P9z8n8v16

Book listing [lst-0009-book.cpp](lst-0009-book.cpp):
```cpp
// https://godbolt.org/z/P9z8n8v16 
#include <fstream> 
int main(int argc, char* argv[]) { 
    int value = 0; 
    std::ifstream myInput{"input1.txt"}; 
    myInput >> value; 
}
```

Godbolt Listing [lst-0009-godb.cpp](lst-0009-godb.cpp), [https://godbolt.org/z/P9z8n8v16](https://godbolt.org/z/P9z8n8v16):
```cpp
//#(execute) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/P9z8n8v16 
#include <fstream> 
int main(int argc, char* argv[]) { 
    int value = 0; 
    std::ifstream myInput{"input1.txt"}; 
    myInput >> value; 
}
```

----------------


## Listing 06.5 Use the ! operator to check the state of the stream.

Book listing [lst-0010-book.cpp](lst-0010-book.cpp):
```cpp
// https://godbolt.org/z/ssW3eKEGq 
#include <iostream> // cerr 
#include <fstream> 
int main(int argc, char* argv[]) { 
    int value; 
    std::ifstream myInput{"input1.txt"}; 
    if(!myInput) { 
        std::cerr << "Error opening file!\n"; 
    } else { 
        myInput >> value; 
    }
}
```

Godbolt Listing [lst-0010-godb.cpp](lst-0010-godb.cpp), [https://godbolt.org/z/ssW3eKEGq](https://godbolt.org/z/ssW3eKEGq):
```cpp
//#(execute) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/ssW3eKEGq 
#include <iostream> // cerr 
#include <fstream> 
int main(int argc, char* argv[]) { 
    int value; 
    std::ifstream myInput{"input1.txt"}; 
    if(!myInput) { 
        std::cerr << "Error opening file!\n"; 
    } else { 
        myInput >> value; 
    }
}
```

----------------


## Listing 06.6: Use stream manipulators from <iomanip> to influence the format of the output.

Book listing [lst-0011-book.cpp](lst-0011-book.cpp):
```cpp
//https://godbolt.org/z/EcnWz3r3j 
#include <iostream>
#include <iomanip>                            // fixed, setprecision 
#include <format>                             // C++20 
using std::cout; using std::format;           // abbreviation cout, format 
int main() { 
    cout << std::fixed                        // dot notation, not scientific 
         << std::setprecision(15);            // 15 decimal places 
    cout << 0.5 << "\n";                      // output: 0.500000000000000* 
    cout << std::setprecision(5);             // 5 decimal places 
    cout << 0.25 << "\n";                     // output: 0.25000 
    cout << format("{:0.4f}", 0.75) << "\n";  // (C++20) output: 0.7500 
    return 0; 
}
```

Godbolt Listing [lst-0011-godb.cpp](lst-0011-godb.cpp), [https://godbolt.org/z/EcnWz3r3j](https://godbolt.org/z/EcnWz3r3j):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
//https://godbolt.org/z/EcnWz3r3j 
#include <iostream>
#include <iomanip>                            // fixed, setprecision 
#include <format>                             // C++20 
using std::cout; using std::format;           // abbreviation cout, format 
int main() { 
    cout << std::fixed                        // dot notation, not scientific 
         << std::setprecision(15);            // 15 decimal places 
    cout << 0.5 << "\n";                      // output: 0.500000000000000* 
    cout << std::setprecision(5);             // 5 decimal places 
    cout << 0.25 << "\n";                     // output: 0.25000 
    cout << format("{:0.4f}", 0.75) << "\n";  // (C++20) output: 0.7500 
    return 0; 
}
```

----------------


## Listing 06.7: You store a fixed number of elements in an "array".

Book listing [lst-0015-book.cpp](lst-0015-book.cpp):
```cpp
// https://godbolt.org/z/cd1d1W7T1 
#include <array> 
#include <iostream> 
using std::cout; using std::array; using std::string; 
int main() {
    array<string,7> wday = { "Monday", "Tuesday",         // define 
        "Wednesday", "Thursday", "Friday", "Saturday", "Sunday" }; 
    cout << "The week starts with " << wday[0] << ".\n";  // read values
    cout << "It ends with " << wday.at(6) << ".\n";       // read safely 
    /* nothern? */
    wday[5] = "Satdee";                                   // change values 
}
```

Godbolt Listing [lst-0015-godb.cpp](lst-0015-godb.cpp), [https://godbolt.org/z/cd1d1W7T1](https://godbolt.org/z/cd1d1W7T1):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/cd1d1W7T1 
#include <array> 
#include <iostream> 
using std::cout; using std::array; using std::string; 
int main() {
    array<string,7> wday = { "Monday", "Tuesday",         // define 
        "Wednesday", "Thursday", "Friday", "Saturday", "Sunday" }; 
    cout << "The week starts with " << wday[0] << ".\n";  // read values
    cout << "It ends with " << wday.at(6) << ".\n";       // read safely 
    /* nothern? */
    wday[5] = "Satdee";                                   // change values 
}
```

----------------


## Listing 06.8 The array size must be constant.

Book listing [lst-0016-book.cpp](lst-0016-book.cpp):
```cpp
// https://godbolt.org/z/zfe7dM4ex 
#include <array> 
#include <iostream>
constexpr size_t MONTHS = 12; /* months in year */ 
int main() { 
    std::array<unsigned,MONTHS> mdays = {         // okay with a constant 
        31,28,31,30,31,30,31,31,30,31,30,30,30,31}; 
    unsigned age = 0; 
    std::cout << "How old are you? "; std::cin >> age; 
    std::array<int,age> years of life;           // (ERR) array size not by variable 
}
```

Godbolt Listing [lst-0016-godb.cpp](lst-0016-godb.cpp), [https://godbolt.org/z/zfe7dM4ex](https://godbolt.org/z/zfe7dM4ex):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/zfe7dM4ex 
#include <array> 
#include <iostream>
constexpr size_t MONTHS = 12; /* months in year */ 
int main() { 
    std::array<unsigned,MONTHS> mdays = {         // okay with a constant 
        31,28,31,30,31,30,31,31,30,31,30,30,30,31}; 
    unsigned age = 0; 
    std::cout << "How old are you? "; std::cin >> age; 
    std::array<int,age> years of life;           // (ERR) array size not by variable 
}
```

----------------


## Listing 06.9 If you need to use an array definition more than once, use "using".

Book listing [lst-0017-book.cpp](lst-0017-book.cpp):
```cpp
// https://godbolt.org/z/a4zxbd99z 
#include <array> 
#include <algorithm>                     // accumulate 
#include <numeric>                       // iota
using January = std::array<int,31>;      // alias for repeated use

void initJanuary(January& jan) {         // the exact array as parameter 
    std::iota(begin(jan), end(jan), 1);  // fills with 1, 2, 3 ... 31 
} 
int sumJanuary(const January& jan) {     // the exact array as parameter 
    return std::accumulate(begin(jan), end(jan), 0); // helper function for sum 
} 
int main() {
    January jan;                         // declares an array<int,31> 
    initJanuary( jan ); 
    int sum = sumJanuary( jan ); 
}
```

Godbolt Listing [lst-0017-godb.cpp](lst-0017-godb.cpp), [https://godbolt.org/z/a4zxbd99z](https://godbolt.org/z/a4zxbd99z):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/a4zxbd99z 
#include <array> 
#include <algorithm>                     // accumulate 
#include <numeric>                       // iota
using January = std::array<int,31>;      // alias for repeated use

void initJanuary(January& jan) {         // the exact array as parameter 
    std::iota(begin(jan), end(jan), 1);  // fills with 1, 2, 3 ... 31 
} 
int sumJanuary(const January& jan) {     // the exact array as parameter 
    return std::accumulate(begin(jan), end(jan), 0); // helper function for sum 
} 
int main() {
    January jan;                         // declares an array<int,31> 
    initJanuary( jan ); 
    int sum = sumJanuary( jan ); 
}
```

----------------


## GodboltId:T9jsEoMj4

Book listing [lst-0018-book.cpp](lst-0018-book.cpp):
```cpp
// https://godbolt.org/z/T9jsEoMj4 
#include <vector>                       // You need this header 
int main() {
    std::vector<int> squares{};         // initialize empty 
    for(int idx = 0; idx<100; ++idx) {
        squares.push_back(idx*idx);      // append an element 
    }
}
```

Godbolt Listing [lst-0018-godb.cpp](lst-0018-godb.cpp), [https://godbolt.org/z/T9jsEoMj4](https://godbolt.org/z/T9jsEoMj4):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/T9jsEoMj4 
#include <vector>                       // You need this header 
int main() {
    std::vector<int> squares{};         // initialize empty 
    for(int idx = 0; idx<100; ++idx) {
        squares.push_back(idx*idx);      // append an element 
    }
}
```

----------------


## Listing 06.10: The simplest iteration uses a range based "for" loop.

Book listing [lst-0019-book.cpp](lst-0019-book.cpp):
```cpp
// https://godbolt.org/z/hhhavG5he 
#include <vector> 
#include <iostream>                    // cout, endl 
int main() {
    std::vector squares{1,4,9,16,25};  // initialize filled
    for(int number : squares)          // number is one square at a time 
        std::cout << number << " "; 
    std::cout << std::endl; 
}
```

Godbolt Listing [lst-0019-godb.cpp](lst-0019-godb.cpp), [https://godbolt.org/z/hhhavG5he](https://godbolt.org/z/hhhavG5he):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/hhhavG5he 
#include <vector> 
#include <iostream>                    // cout, endl 
int main() {
    std::vector squares{1,4,9,16,25};  // initialize filled
    for(int number : squares)          // number is one square at a time 
        std::cout << number << " "; 
    std::cout << std::endl; 
}
```

----------------


## Listing 06.11 Accessing the elements by index.

Book listing [lst-0020-book.cpp](lst-0020-book.cpp):
```cpp
// https://godbolt.org/z/xcKM7v1Eo 
#include <vector> 
#include <iostream>                          // cout, endl 
int main() { 
  std::vector qus{1,4,9,16,25}; 
  for(unsigned idx=0; idx<qus.size(); ++idx) // size returns the count 
    std::cout << qus[idx] << " ";            // [idx] or at(idx) gets an element
  std::cout << std::endl; 
}
```

Godbolt Listing [lst-0020-godb.cpp](lst-0020-godb.cpp), [https://godbolt.org/z/xcKM7v1Eo](https://godbolt.org/z/xcKM7v1Eo):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/xcKM7v1Eo 
#include <vector> 
#include <iostream>                          // cout, endl 
int main() { 
  std::vector qus{1,4,9,16,25}; 
  for(unsigned idx=0; idx<qus.size(); ++idx) // size returns the count 
    std::cout << qus[idx] << " ";            // [idx] or at(idx) gets an element
  std::cout << std::endl; 
}
```

----------------


## Listing 06.12: The use of iterators for a loop.

Book listing [lst-0021-book.cpp](lst-0021-book.cpp):
```cpp
// https://godbolt.org/z/1WG8zvhYe 
#include <vector> 
#include <iostream>                               // cout, endl 
int main() { 
  std::vector qus{1,4,9,16,25}; 
  for(auto it = qus.begin(); it!=qus.end(); ++it) // between begin() and end() 
    std::cout << *it << " ";  // with *it you get from the iterator to the element 
  std::cout << std::endl; 
}
```

Godbolt Listing [lst-0021-godb.cpp](lst-0021-godb.cpp), [https://godbolt.org/z/1WG8zvhYe](https://godbolt.org/z/1WG8zvhYe):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/1WG8zvhYe 
#include <vector> 
#include <iostream>                               // cout, endl 
int main() { 
  std::vector qus{1,4,9,16,25}; 
  for(auto it = qus.begin(); it!=qus.end(); ++it) // between begin() and end() 
    std::cout << *it << " ";  // with *it you get from the iterator to the element 
  std::cout << std::endl; 
}
```

----------------


## Listing 06.13 Counting with an algorithm.

Book listing [lst-0022-book.cpp](lst-0022-book.cpp):
```cpp
// https://godbolt.org/z/4G67rso8T 
#include <vector> 
#include <algorithm>                           // count_if 
#include <numeric>                             // iota 
#include <iostream> 
bool even(int n) { return n%2==0; }            // test for even 
int main() { 
    std::vector<int> data(100);                // 100 x zero
    std::iota(data.begin(), data.end(), 0);    // 0, 1, 2, ... 99 
    // counts even numbers 
    std::cout << std::count_if(data.begin(), data.end(), even); 
}
```

Godbolt Listing [lst-0022-godb.cpp](lst-0022-godb.cpp), [https://godbolt.org/z/4G67rso8T](https://godbolt.org/z/4G67rso8T):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/4G67rso8T 
#include <vector> 
#include <algorithm>                           // count_if 
#include <numeric>                             // iota 
#include <iostream> 
bool even(int n) { return n%2==0; }            // test for even 
int main() { 
    std::vector<int> data(100);                // 100 x zero
    std::iota(data.begin(), data.end(), 0);    // 0, 1, 2, ... 99 
    // counts even numbers 
    std::cout << std::count_if(data.begin(), data.end(), even); 
}
```

----------------

