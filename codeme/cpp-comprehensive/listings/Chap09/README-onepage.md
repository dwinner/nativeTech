# Listings of Chap09.docx

This is the list of listings on one page.
You can also view a [linked summary](README.md).


## Listing 09.1: Types of expressions are important because they determine what happens next.

Book listing [lst-0001-book.cpp](lst-0001-book.cpp):
```cpp
// https://godbolt.org/z/M6en1GMrE 
#include <iostream>            // cout
#include <string>
void print(int n)  {           // function print for type int
    std::cout << "Number:" << n << "\n";
}
void print(std::string s)  {   // same name, different type
    std::cout << "String:" << s << "\n";
}
int main() {
    int number = 10;
    std::string name = "Bilbo";
    print(number);             // calls print(int), number is int
    print(name);               // calls print(string), name is string
    print(11 + 22);            // expression is int
    print(name + " Baggins");  // expression is string
}
```

Godbolt Listing [lst-0001-godb.cpp](lst-0001-godb.cpp), [https://godbolt.org/z/M6en1GMrE](https://godbolt.org/z/M6en1GMrE):
```cpp
//#(compile) c++; compiler:g141; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/M6en1GMrE 
#include <iostream>            // cout
#include <string>
void print(int n)  {           // function print for type int
    std::cout << "Number:" << n << "\n";
}
void print(std::string s)  {   // same name, different type
    std::cout << "String:" << s << "\n";
}
int main() {
    int number = 10;
    std::string name = "Bilbo";
    print(number);             // calls print(int), number is int
    print(name);               // calls print(string), name is string
    print(11 + 22);            // expression is int
    print(name + " Baggins");  // expression is string
}
```

----------------


## GodboltId:EY5K6Wsr3

Book listing [lst-0008-book.cpp](lst-0008-book.cpp):
```cpp
// https://godbolt.org/z/EY5K6Wsr3 
std::vector<int> data(10);     // 10 times 0 in a vector
data.front() = 666;            // writes 666 to the front position
```

Godbolt Listing [lst-0008-godb.cpp](lst-0008-godb.cpp), [https://godbolt.org/z/EY5K6Wsr3](https://godbolt.org/z/EY5K6Wsr3):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/EY5K6Wsr3 
std::vector<int> data(10);     // 10 times 0 in a vector
data.front() = 666;            // writes 666 to the front position
```

----------------


## GodboltId:K9c913had

Book listing [lst-0009-book.cpp](lst-0009-book.cpp):
```cpp
// https://godbolt.org/z/K9c913had 
if(int result; (result = read(buffer, 100)) != 0) {
    std::cerr << "Error number "<< result << " occurred.\n";
}
```

Godbolt Listing [lst-0009-godb.cpp](lst-0009-godb.cpp), [https://godbolt.org/z/K9c913had](https://godbolt.org/z/K9c913had):
```cpp
//#(execute) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/K9c913had 
if(int result; (result = read(buffer, 100)) != 0) {
    std::cerr << "Error number "<< result << " occurred.\n";
}
```

----------------

