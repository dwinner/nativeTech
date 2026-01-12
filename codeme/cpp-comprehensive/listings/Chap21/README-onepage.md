# Listings of Chap21.docx

This is the list of listings on one page.
You can also view a [linked summary](README.md).


## Listing 21.1: Lines containing preprocessor directives start with a #.

Book listing [lst-0001-book.cpp](lst-0001-book.cpp):
```cpp
// https://godbolt.org/z/j6WWxPheP
// Filename: my-macros.hpp
#ifndef MY_MACROS_HPP  // Include Guard
#define MY_MACROS_HPP
#include <iostream>    // cout, cerr
#include <vector>
#ifdef OUTPUT_TO_STANDARD
#  define OUT std::cout
#else
#  define OUT std::cerr
#endif
#define MESSAGE(text) { (OUT) << text << "\n"; }
using container_type = std::vector<int>;
static constexpr unsigned SIZE = 10;
#endif
```

Godbolt Listing [lst-0001-godb.cpp](lst-0001-godb.cpp), [https://godbolt.org/z/j6WWxPheP](https://godbolt.org/z/j6WWxPheP):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23 -Wall -Wextra; libs:-
// https://godbolt.org/z/j6WWxPheP
// Filename: my-macros.hpp
#ifndef MY_MACROS_HPP  // Include Guard
#define MY_MACROS_HPP
#include <iostream>    // cout, cerr
#include <vector>
#ifdef OUTPUT_TO_STANDARD
#  define OUT std::cout
#else
#  define OUT std::cerr
#endif
#define MESSAGE(text) { (OUT) << text << "\n"; }
using container_type = std::vector<int>;
static constexpr unsigned SIZE = 10;
#endif
```

----------------


## Listing 21.2: Your main file with "main" includes the header file via "#include".

Book listing [lst-0002-book.cpp](lst-0002-book.cpp):
```cpp
// https://godbolt.org/z/feGrM4f6j
// Filename: makros.cpp
#define OUTPUT_TO_STANDARD // Switch between cerr and cout
#include "my-macros.hpp"
#include "my-macros.hpp"  // Oops, accidentally included twice.
int main() {
    MESSAGE("Program start");
    container_type data(SIZE);
    MESSAGE("The container has " << data.size() << " elements.");
    MESSAGE("Program end");
    OUT << "That was a close call.\n";
}
```

Godbolt Listing [lst-0002-godb.cpp](lst-0002-godb.cpp), [https://godbolt.org/z/feGrM4f6j](https://godbolt.org/z/feGrM4f6j):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23 -Wall -Wextra; libs:-
// https://godbolt.org/z/feGrM4f6j
// Filename: makros.cpp
#define OUTPUT_TO_STANDARD // Switch between cerr and cout
#include "my-macros.hpp"
#include "my-macros.hpp"  // Oops, accidentally included twice.
int main() {
    MESSAGE("Program start");
    container_type data(SIZE);
    MESSAGE("The container has " << data.size() << " elements.");
    MESSAGE("Program end");
    OUT << "That was a close call.\n";
}
```

----------------


## Listing 21.3: The result of the preprocessor run.

Book listing [lst-0007-book.cpp](lst-0007-book.cpp):
```cpp
// https://godbolt.org/z/W45G5TjTz 
// …here content of <vector>…
// …here content of <iostream>…
using container_type = std::vector<int>;
static constexpr unsigned SIZE = 10;
int main() {
  { (std::cout) << "Program start" << "\n"; }
  container_type data(SIZE);
  { (std::cout) << "The container has " << data.size() << " elements." << "\n";}
  { (std::cout) << "End of program" << "\n"; }
  { std::cout << "That was a close call.\n"; }
}
```

Godbolt Listing [lst-0007-godb.cpp](lst-0007-godb.cpp), [https://godbolt.org/z/W45G5TjTz](https://godbolt.org/z/W45G5TjTz):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23 -Wall -Wextra; libs:-
// https://godbolt.org/z/W45G5TjTz 
// …here content of <vector>…
// …here content of <iostream>…
using container_type = std::vector<int>;
static constexpr unsigned SIZE = 10;
int main() {
  { (std::cout) << "Program start" << "\n"; }
  container_type data(SIZE);
  { (std::cout) << "The container has " << data.size() << " elements." << "\n";}
  { (std::cout) << "End of program" << "\n"; }
  { std::cout << "That was a close call.\n"; }
}
```

----------------


## Listing 21.5: The purely textual substitution leads to multiple executions in macros.

Book listing [lst-0015-book.cpp](lst-0015-book.cpp):
```cpp
// https://godbolt.org/z/jzKfabPh3 
#include <cmath> // sin, cos
constexpr double max2(double a, double b) { return a > b ? a : b; }
#define MAX2(a,b) ((a) > (b) ? (a) : (b))
int main() {
    double f = max2(sin(3.141592/2), cos(3.141592/2));
    double e = MAX2(sin(3.141592/2), cos(3.141592/2));
}
```

Godbolt Listing [lst-0015-godb.cpp](lst-0015-godb.cpp), [https://godbolt.org/z/jzKfabPh3](https://godbolt.org/z/jzKfabPh3):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/jzKfabPh3 
#include <cmath> // sin, cos
constexpr double max2(double a, double b) { return a > b ? a : b; }
#define MAX2(a,b) ((a) > (b) ? (a) : (b))
int main() {
    double f = max2(sin(3.141592/2), cos(3.141592/2));
    double e = MAX2(sin(3.141592/2), cos(3.141592/2));
}
```

----------------


## GodboltId:YzYz3x1zx

Book listing [lst-0017-book.cpp](lst-0017-book.cpp):
```cpp
// https://godbolt.org/z/YzYz3x1zx 
#include <iostream>
#define MAX2(a,b) ((a) > (b) ? (a) : (b))
int main() {
    int x = 0;
    int y = 0;
    int z = MAX2( ++x, ++y );  // (ERR) expands arguments multiple times
    std::cout << "x:"<< x << " y:"<< y << " z:"<<z << '\n';
}
```

Godbolt Listing [lst-0017-godb.cpp](lst-0017-godb.cpp), [https://godbolt.org/z/YzYz3x1zx](https://godbolt.org/z/YzYz3x1zx):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/YzYz3x1zx 
#include <iostream>
#define MAX2(a,b) ((a) > (b) ? (a) : (b))
int main() {
    int x = 0;
    int y = 0;
    int z = MAX2( ++x, ++y );  // (ERR) expands arguments multiple times
    std::cout << "x:"<< x << " y:"<< y << " z:"<<z << '\n';
}
```

----------------


## GodboltId:5zrfvovaW

Book listing [lst-0018-book.cpp](lst-0018-book.cpp):
```cpp
// https://godbolt.org/z/5zrfvovaW 
#include <string>
#include <cmath> // sin, cos
#define MAX2(a,b) ((a) > (b) ? (a) : (b))
int main() {
    double e = MAX2(sin(3.141592/2), cos(3.141592/2));
    int i = MAX2(10+12+45, 100/5+20);
    std::string s = MAX2(std::string("Ernie"), std::string("Bert"));
}
```

Godbolt Listing [lst-0018-godb.cpp](lst-0018-godb.cpp), [https://godbolt.org/z/5zrfvovaW](https://godbolt.org/z/5zrfvovaW):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/5zrfvovaW 
#include <string>
#include <cmath> // sin, cos
#define MAX2(a,b) ((a) > (b) ? (a) : (b))
int main() {
    double e = MAX2(sin(3.141592/2), cos(3.141592/2));
    int i = MAX2(10+12+45, 100/5+20);
    std::string s = MAX2(std::string("Ernie"), std::string("Bert"));
}
```

----------------


## GodboltId:6xzTx83zE

Book listing [lst-0019-book.cpp](lst-0019-book.cpp):
```cpp
// https://godbolt.org/z/6xzTx83zE 
#include <string>
#include <cmath> // sin, cos
constexpr double max2(double a, double b) { return  a > b ? a : b; }
constexpr int max2(int a, int b) { return  a > b ? a : b; }
std::string max2(const std::string &a, const std::string &b)
    { return  a > b ? a : b; }
int main() {
    double e = max2(sin(3.141592/2), cos(3.141592/2));
    double i = max2(10+12+45, 100/5+20);
    std::string s = max2("Ernie", "Bert");
}
```

Godbolt Listing [lst-0019-godb.cpp](lst-0019-godb.cpp), [https://godbolt.org/z/6xzTx83zE](https://godbolt.org/z/6xzTx83zE):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/6xzTx83zE 
#include <string>
#include <cmath> // sin, cos
constexpr double max2(double a, double b) { return  a > b ? a : b; }
constexpr int max2(int a, int b) { return  a > b ? a : b; }
std::string max2(const std::string &a, const std::string &b)
    { return  a > b ? a : b; }
int main() {
    double e = max2(sin(3.141592/2), cos(3.141592/2));
    double i = max2(10+12+45, 100/5+20);
    std::string s = max2("Ernie", "Bert");
}
```

----------------


## Listing 21.6: A simple function template is much better than a macro.

Book listing [lst-0020-book.cpp](lst-0020-book.cpp):
```cpp
// https://godbolt.org/z/acMjvxcM5 
#include <string>
#include <cmath> // sin, cos
template<typename TYPE>
constexpr TYPE max2(const TYPE &a, const TYPE &b)
    { return  a > b ? a : b; }
int main() {
    double e = max2(sin(3.141592/2), cos(3.141592/2));
    double i = max2(10+12+45, 100/5+20);
    std::string s = max2(std::string("Ernie"), std::string("Bert"));
}
```

Godbolt Listing [lst-0020-godb.cpp](lst-0020-godb.cpp), [https://godbolt.org/z/acMjvxcM5](https://godbolt.org/z/acMjvxcM5):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/acMjvxcM5 
#include <string>
#include <cmath> // sin, cos
template<typename TYPE>
constexpr TYPE max2(const TYPE &a, const TYPE &b)
    { return  a > b ? a : b; }
int main() {
    double e = max2(sin(3.141592/2), cos(3.141592/2));
    double i = max2(10+12+45, 100/5+20);
    std::string s = max2(std::string("Ernie"), std::string("Bert"));
}
```

----------------

