# Listings of Chap23.docx

This is the list of listings on one page.
You can also view a [linked summary](README.md).


## Listing 23.1: The call to a function template works so long as the function body is valid with the template parameters.

Book listing [lst-0010-book.cpp](lst-0010-book.cpp):
```cpp
// https://godbolt.org/z/aMPj3K7EE 
#include <iostream>
struct Number {
    int value_;
};
template<typename TYPE>
void print(TYPE value) {
    std::cout << value << "\n";
}
int main() {
    print(5);
    print(-10.25);
    print("Flamingo");
    Number seven { 7 };
    print(seven);    // (ERR) cout << seven does not exist
}
```

Godbolt Listing [lst-0010-godb.cpp](lst-0010-godb.cpp), [https://godbolt.org/z/aMPj3K7EE](https://godbolt.org/z/aMPj3K7EE):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/aMPj3K7EE 
#include <iostream>
struct Number {
    int value_;
};
template<typename TYPE>
void print(TYPE value) {
    std::cout << value << "\n";
}
int main() {
    print(5);
    print(-10.25);
    print("Flamingo");
    Number seven { 7 };
    print(seven);    // (ERR) cout << seven does not exist
}
```

----------------


## Listing 23.2: With and without explicit type specification during the call.

Book listing [lst-0013-book.cpp](lst-0013-book.cpp):
```cpp
// https://godbolt.org/z/816Ys17aT 
#include <iostream>
using std::cout;
template<typename TYPE>
  void func(TYP a) { cout << a <<" TYP\n"; }
void func(int a) { cout << a << " int\n"; }
int main() {
    func<int>(8); // Output: 8 TYP
    func(8);      // Output: 8 int
}
```

Godbolt Listing [lst-0013-godb.cpp](lst-0013-godb.cpp), [https://godbolt.org/z/816Ys17aT](https://godbolt.org/z/816Ys17aT):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/816Ys17aT 
#include <iostream>
using std::cout;
template<typename TYPE>
  void func(TYP a) { cout << a <<" TYP\n"; }
void func(int a) { cout << a << " int\n"; }
int main() {
    func<int>(8); // Output: 8 TYP
    func(8);      // Output: 8 int
}
```

----------------


## Listing 23.3: A template parameter can also be a constant number.

Book listing [lst-0015-book.cpp](lst-0015-book.cpp):
```cpp
// https://godbolt.org/z/c9xzM7EhP 
#include <array>
#include <iostream> // cout
using std::array; using std::cout;
template<size_t SIZE>
array<int,SIZE> createArray() {
    array<int,SIZE> result{};
    return result;
}
int main() {
    auto data = createArray<5>();
    data[3] = 33;
    for(auto e : data) cout << e << " ";
    cout << "\n";
}
```

Godbolt Listing [lst-0015-godb.cpp](lst-0015-godb.cpp), [https://godbolt.org/z/c9xzM7EhP](https://godbolt.org/z/c9xzM7EhP):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/c9xzM7EhP 
#include <array>
#include <iostream> // cout
using std::array; using std::cout;
template<size_t SIZE>
array<int,SIZE> createArray() {
    array<int,SIZE> result{};
    return result;
}
int main() {
    auto data = createArray<5>();
    data[3] = 33;
    for(auto e : data) cout << e << " ";
    cout << "\n";
}
```

----------------


## Listing 23.4: With auto as a template parameter, you can specify any simple value.

Book listing [lst-0017-book.cpp](lst-0017-book.cpp):
```cpp
template <auto value> void f() { }
f<10>();                // value gets type int
f<'a'>();               // value gets type char
```

----------------


## Listing 23.5: With "auto" and …, you can specify any values as template parameters.

Book listing [lst-0018-book.cpp](lst-0018-book.cpp):
```cpp
template<auto ... vs> struct MixedList {};
using Three = MixedList<'a', 100, 'b'>;
Three three{};
```

----------------


## Listing 23.6: For "TYPE", the compiler determines "const&".

Book listing [lst-0020-book.cpp](lst-0020-book.cpp):
```cpp
// https://godbolt.org/z/fev879zaK
#include <iostream>
const int& a_or_b(int choice) {
    static const int a = 42;
    static const int b = 73;
    if(choice==1)
        return a; // return const& to inner variable a
    else
        return b; // return const& to inner variable b
}
template<typename TYPE>
TYPE add(TYPE a, TYPE b) {
    return a + b;
}
int main() {
    auto res = add(
        a_or_b(0),   // const int&
        a_or_b(1) ); // const int&
    std::cout << res << "\n"; // Output: 115
}
```

Godbolt Listing [lst-0020-godb.cpp](lst-0020-godb.cpp), [https://godbolt.org/z/fev879zaK](https://godbolt.org/z/fev879zaK):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/fev879zaK
#include <iostream>
const int& a_or_b(int choice) {
    static const int a = 42;
    static const int b = 73;
    if(choice==1)
        return a; // return const& to inner variable a
    else
        return b; // return const& to inner variable b
}
template<typename TYPE>
TYPE add(TYPE a, TYPE b) {
    return a + b;
}
int main() {
    auto res = add(
        a_or_b(0),   // const int&
        a_or_b(1) ); // const int&
    std::cout << res << "\n"; // Output: 115
}
```

----------------


## Listing 23.7: A method can also be a template.

Book listing [lst-0022-book.cpp](lst-0022-book.cpp):
```cpp
// https://godbolt.org/z/bfofn76Ke 
#include <iostream>
class Printer {
    std::ostream& trg_;
public:
    explicit Printer(std::ostream& target)
        : trg_(target)
        {}
    template<typename TYPE>
    Printer& print(const TYP& arg) {
        trg_ << arg;
        return *this;
    }
};
int main() {
    Printer normal(std::cout);
    normal.print(7).print(" ").print(3.1415).print("\n");
    Printer error(std::cerr);
    error.print(8).print(" ").print(2.7183).print("\n");
}
```

Godbolt Listing [lst-0022-godb.cpp](lst-0022-godb.cpp), [https://godbolt.org/z/bfofn76Ke](https://godbolt.org/z/bfofn76Ke):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/bfofn76Ke 
#include <iostream>
class Printer {
    std::ostream& trg_;
public:
    explicit Printer(std::ostream& target)
        : trg_(target)
        {}
    template<typename TYPE>
    Printer& print(const TYP& arg) {
        trg_ << arg;
        return *this;
    }
};
int main() {
    Printer normal(std::cout);
    normal.print(7).print(" ").print(3.1415).print("\n");
    Printer error(std::cerr);
    error.print(8).print(" ").print(2.7183).print("\n");
}
```

----------------


## Listing 23.8: You will also find various function templates in the standard library.

Book listing [lst-0023-book.cpp](lst-0023-book.cpp):
```cpp
// https://godbolt.org/z/oTn1K4sGG 
#include <vector>
#include <iostream>  // cout, ostream
#include <algorithm> // sort, copy
#include <iterator>  // ostream_iterator
int main() {
    std::ostream_iterator<int> oit(std::cout," ");
    std::vector data { 100, 50, 1, 75, 25, 0 };       // vector<int>
    std::sort(std::begin(data), std::end(data));      // Iterator pair
    std::copy(std::begin(data), std::end(data), oit); // Iterator pair
    std::cout << '\n';                                // Output: 0 1 25 50 75 100
    std::ranges::reverse(data);                       // Range
    std::ranges::copy(data, oit);                     // Range
    std::cout << '\n';                                // Output: 100 75 50 25 1 0
}
```

Godbolt Listing [lst-0023-godb.cpp](lst-0023-godb.cpp), [https://godbolt.org/z/oTn1K4sGG](https://godbolt.org/z/oTn1K4sGG):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/oTn1K4sGG 
#include <vector>
#include <iostream>  // cout, ostream
#include <algorithm> // sort, copy
#include <iterator>  // ostream_iterator
int main() {
    std::ostream_iterator<int> oit(std::cout," ");
    std::vector data { 100, 50, 1, 75, 25, 0 };       // vector<int>
    std::sort(std::begin(data), std::end(data));      // Iterator pair
    std::copy(std::begin(data), std::end(data), oit); // Iterator pair
    std::cout << '\n';                                // Output: 0 1 25 50 75 100
    std::ranges::reverse(data);                       // Range
    std::ranges::copy(data, oit);                     // Range
    std::cout << '\n';                                // Output: 100 75 50 25 1 0
}
```

----------------


## Listing 23.9: Pass your own algorithm ranges as arguments, not the container.

Book listing [lst-0024-book.cpp](lst-0024-book.cpp):
```cpp
// https://godbolt.org/z/h3baE65od 
#include <iostream>
#include <vector>
#include <set>
#include <bitset>
#include <ranges>
namespace rng = std::ranges;
std::ostream& printBinary(std::ostream& os, rng::input_range auto&& range) {
    for(auto&& elem : range) {
        std::bitset<4> x(elem); // Copy number into bitset
        os << x << " ";
    }
    return os;
}
int main()
{
    std::vector vdata { 2, 0, 15, 12 };
    printBinary(std::cout, vdata) << "\n";
    // Output: 0010 0000 1111 1100
    std::set sdata { 2, 0, 12, 15 };
    printBinary(std::cout, sdata) << "\n";
    // Output: 0000 0010 1100 1111
    int adata[] = { 0,1,2,13,14,15 };
    printBinary(std::cout, adata) << "\n";
    // Output: 0000 0001 0010 1101 1110 1111
}
```

Godbolt Listing [lst-0024-godb.cpp](lst-0024-godb.cpp), [https://godbolt.org/z/h3baE65od](https://godbolt.org/z/h3baE65od):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/h3baE65od 
#include <iostream>
#include <vector>
#include <set>
#include <bitset>
#include <ranges>
namespace rng = std::ranges;
std::ostream& printBinary(std::ostream& os, rng::input_range auto&& range) {
    for(auto&& elem : range) {
        std::bitset<4> x(elem); // Copy number into bitset
        os << x << " ";
    }
    return os;
}
int main()
{
    std::vector vdata { 2, 0, 15, 12 };
    printBinary(std::cout, vdata) << "\n";
    // Output: 0010 0000 1111 1100
    std::set sdata { 2, 0, 12, 15 };
    printBinary(std::cout, sdata) << "\n";
    // Output: 0000 0010 1100 1111
    int adata[] = { 0,1,2,13,14,15 };
    printBinary(std::cout, adata) << "\n";
    // Output: 0000 0001 0010 1101 1110 1111
}
```

----------------


## Listing 23.10: Pass your own algorithm iterators as arguments, not the container.

Book listing [lst-0025-book.cpp](lst-0025-book.cpp):
```cpp
// https://godbolt.org/z/13eMxKTfj 
#include <iostream>
#include <vector>
#include <set>
#include <bitset>
template<typename IT>
std::ostream& printBinary(std::ostream& os, IT begin, IT end)
{
    for(IT it=begin; it != end; ++it) {
        std::bitset<4> x(*it); // Copy number into bitset
        os << x << " ";
    }
    return os;
}
int main()
{
    std::vector<int> vdata { 2, 0, 15, 12 };
    printBinary(std::cout, vdata.cbegin(), vdata.cend()) << "\n";
    // Output: 0010 0000 1111 1100
    std::set<int> sdata { 2, 0, 12, 15 };
    printBinary(std::cout, std::begin(sdata), std::end(sdata)) << "\n";
    // Output: 0000 0010 1100 1111
    int adata[] = { 0,1,2,13,14,15 };
    printBinary(std::cout, std::begin(adata), std::end(adata)) << "\n";
    // Output: 0000 0001 0010 1101 1110 1111
}
```

Godbolt Listing [lst-0025-godb.cpp](lst-0025-godb.cpp), [https://godbolt.org/z/13eMxKTfj](https://godbolt.org/z/13eMxKTfj):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/13eMxKTfj 
#include <iostream>
#include <vector>
#include <set>
#include <bitset>
template<typename IT>
std::ostream& printBinary(std::ostream& os, IT begin, IT end)
{
    for(IT it=begin; it != end; ++it) {
        std::bitset<4> x(*it); // Copy number into bitset
        os << x << " ";
    }
    return os;
}
int main()
{
    std::vector<int> vdata { 2, 0, 15, 12 };
    printBinary(std::cout, vdata.cbegin(), vdata.cend()) << "\n";
    // Output: 0010 0000 1111 1100
    std::set<int> sdata { 2, 0, 12, 15 };
    printBinary(std::cout, std::begin(sdata), std::end(sdata)) << "\n";
    // Output: 0000 0010 1100 1111
    int adata[] = { 0,1,2,13,14,15 };
    printBinary(std::cout, std::begin(adata), std::end(adata)) << "\n";
    // Output: 0000 0001 0010 1101 1110 1111
}
```

----------------


## Listing 23.11: How to find the range of integer types.

Book listing [lst-0027-book.cpp](lst-0027-book.cpp):
```cpp
// https://godbolt.org/z/r83Yeon7r
#include <iostream>                            // cout
#include <limits>                              // numeric_limits
template<typename INT_TYP>                     // Template with type argument
void infos(const char* name) {
    using L = typename std::numeric_limits<INT_TYP>; // rename for brevity
    std::cout
        << name
        << " number of bits:" << L::digits     // Bits without sign bit
        << " sign:" << L::is_signed            // stores sign?
        << " min:"<< (long long)L::min()       // smallest possible value
        << " max:"<< (long long)L::max()       // largest possible value
        << "\n";
}
int main() {
    infos<signed char>("char");                // smallest int type
    infos<short>("short");
    infos<int>("int");
    infos<long>("long");
    infos<long long>("long long");             // largest int type
}
```

Godbolt Listing [lst-0027-godb.cpp](lst-0027-godb.cpp), [https://godbolt.org/z/r83Yeon7r](https://godbolt.org/z/r83Yeon7r):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/r83Yeon7r
#include <iostream>                            // cout
#include <limits>                              // numeric_limits
template<typename INT_TYP>                     // Template with type argument
void infos(const char* name) {
    using L = typename std::numeric_limits<INT_TYP>; // rename for brevity
    std::cout
        << name
        << " number of bits:" << L::digits     // Bits without sign bit
        << " sign:" << L::is_signed            // stores sign?
        << " min:"<< (long long)L::min()       // smallest possible value
        << " max:"<< (long long)L::max()       // largest possible value
        << "\n";
}
int main() {
    infos<signed char>("char");                // smallest int type
    infos<short>("short");
    infos<int>("int");
    infos<long>("long");
    infos<long long>("long long");             // largest int type
}
```

----------------


## Listing 23.12: A function as a parameter.

Book listing [lst-0028-book.cpp](lst-0028-book.cpp):
```cpp
#include <functional> // function
int calculate(int a, int b, std::function<int(int,int)> binop) {
    return binop(a,b);
}
```

----------------


## Listing 23.13: Use the name of a suitable function as a function parameter.

Book listing [lst-0031-book.cpp](lst-0031-book.cpp):
```cpp
// https://godbolt.org/z/q719j41er
#include <functional> // function
#include <iostream>   // cout
int calculate(int a, int b, std::function<int(int,int)> binop) {
    return binop(a,b);
}
int plus(int a, int b) { return a+b; }
int times(int a, int b) { return a*b; }
int main() {
    std::cout << calculate(3, 4, plus) << "\n";   // Output: 7
    std::cout << calculate(3, 4, times) << "\n";  // Output: 12
}
```

Godbolt Listing [lst-0031-godb.cpp](lst-0031-godb.cpp), [https://godbolt.org/z/q719j41er](https://godbolt.org/z/q719j41er):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/q719j41er
#include <functional> // function
#include <iostream>   // cout
int calculate(int a, int b, std::function<int(int,int)> binop) {
    return binop(a,b);
}
int plus(int a, int b) { return a+b; }
int times(int a, int b) { return a*b; }
int main() {
    std::cout << calculate(3, 4, plus) << "\n";   // Output: 7
    std::cout << calculate(3, 4, times) << "\n";  // Output: 12
}
```

----------------


## Listing 23.14: It doesn't matter whether you use the address operator when calling or not.

Book listing [lst-0034-book.cpp](lst-0034-book.cpp):
```cpp
// https://godbolt.org/z/c9Eh346cq
#include <functional> // function
#include <iostream>   // cout
int calculate(int a, int b, std::function<int(int,int)> binop) {
    return binop(a,b);
}
int plus(int a, int b) { return a+b; }
int times(int a, int b) { return a*b; }
int main() {
    std::cout << calculate(3, 4, plus) << "\n";   // Value notation
    std::cout << calculate(3, 4, times) << "\n";  // Value notation
    std::cout << calculate(3, 4, &plus) << "\n";  // Pointer notation
    std::cout << calculate(3, 4, &times) << "\n"; // Pointer notation
}
```

Godbolt Listing [lst-0034-godb.cpp](lst-0034-godb.cpp), [https://godbolt.org/z/c9Eh346cq](https://godbolt.org/z/c9Eh346cq):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/c9Eh346cq
#include <functional> // function
#include <iostream>   // cout
int calculate(int a, int b, std::function<int(int,int)> binop) {
    return binop(a,b);
}
int plus(int a, int b) { return a+b; }
int times(int a, int b) { return a*b; }
int main() {
    std::cout << calculate(3, 4, plus) << "\n";   // Value notation
    std::cout << calculate(3, 4, times) << "\n";  // Value notation
    std::cout << calculate(3, 4, &plus) << "\n";  // Pointer notation
    std::cout << calculate(3, 4, &times) << "\n"; // Pointer notation
}
```

----------------


## Listing 23.15: A class with "operator()" creates function objects.

Book listing [lst-0035-book.cpp](lst-0035-book.cpp):
```cpp
// https://godbolt.org/z/xvfvG8hrd
#include <iostream>                       // cout
using std::cout;
class Increment {
   int amount_;
public:
    explicit Increment(int amount) : amount_{amount} {}
    int operator()(int value) const  {    // makes instances callable
        return value + amount_;
    }
    void clear() {
        amount_ = 0;
    }
};
int main() {
    Increment plusFour{4};                // create instance
    Increment plusEleven{11};             // another instance
    cout << plusFour(8) << "\n";          // Output: 12
    int result = 2 * plusEleven(5) - 7;   // result is 25
    cout << plusEleven(result/5) << "\n"; // Output: 16
    cout << 3 * Increment{1}(7) << "\n";  // Output: 24
    Increment plusZero = plusEleven;
    plusZero.clear();                     // change state
    cout << plusZero(1) << "\n";          // Output: 1
}
```

Godbolt Listing [lst-0035-godb.cpp](lst-0035-godb.cpp), [https://godbolt.org/z/xvfvG8hrd](https://godbolt.org/z/xvfvG8hrd):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/xvfvG8hrd
#include <iostream>                       // cout
using std::cout;
class Increment {
   int amount_;
public:
    explicit Increment(int amount) : amount_{amount} {}
    int operator()(int value) const  {    // makes instances callable
        return value + amount_;
    }
    void clear() {
        amount_ = 0;
    }
};
int main() {
    Increment plusFour{4};                // create instance
    Increment plusEleven{11};             // another instance
    cout << plusFour(8) << "\n";          // Output: 12
    int result = 2 * plusEleven(5) - 7;   // result is 25
    cout << plusEleven(result/5) << "\n"; // Output: 16
    cout << 3 * Increment{1}(7) << "\n";  // Output: 24
    Increment plusZero = plusEleven;
    plusZero.clear();                     // change state
    cout << plusZero(1) << "\n";          // Output: 1
}
```

----------------


## Listing 23.16: A class with a normal method.

Book listing [lst-0038-book.cpp](lst-0038-book.cpp):
```cpp
// https://godbolt.org/z/MfKzd8bxa
#include <iostream>                           // cout
using std::cout;

class Add {
   int amount_;
public:
    explicit Add(int amount) : amount_{amount} {}
    int add(int value) const  {               // instead of operator()
        return value + amount_;
    }
    void clear() {
        amount_ = 0;
    }
};
int main() {
    Add plusFour{4};                          // create instance
    Add plusEleven{11};                       // another instance
    cout << plusFour.add(8) << "\n";          // Output: 12
    int result = 2 * plusEleven.add(5) - 7;   // result is 25
    cout << plusEleven.add(result/5) << "\n"; // Output: 16
    cout << 3 * Add{1}.add(7) << "\n";        // Output: 24
    Add plusZilch = plusEleven;
    plusZilch.clear();                        // change state
    cout << plusZilch.add(1) << "\n";         // Output: 1
}
```

Godbolt Listing [lst-0038-godb.cpp](lst-0038-godb.cpp), [https://godbolt.org/z/MfKzd8bxa](https://godbolt.org/z/MfKzd8bxa):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/MfKzd8bxa
#include <iostream>                           // cout
using std::cout;

class Add {
   int amount_;
public:
    explicit Add(int amount) : amount_{amount} {}
    int add(int value) const  {               // instead of operator()
        return value + amount_;
    }
    void clear() {
        amount_ = 0;
    }
};
int main() {
    Add plusFour{4};                          // create instance
    Add plusEleven{11};                       // another instance
    cout << plusFour.add(8) << "\n";          // Output: 12
    int result = 2 * plusEleven.add(5) - 7;   // result is 25
    cout << plusEleven.add(result/5) << "\n"; // Output: 16
    cout << 3 * Add{1}.add(7) << "\n";        // Output: 24
    Add plusZilch = plusEleven;
    plusZilch.clear();                        // change state
    cout << plusZilch.add(1) << "\n";         // Output: 1
}
```

----------------


## Listing 23.17: With "operator<" as a function, you can only implement one sort order.

Book listing [lst-0039-book.cpp](lst-0039-book.cpp):
```cpp
// https://godbolt.org/z/3ch51ov87 
#include <set>
#include <string>
#include <iostream> // cout
using std::string; using std::set; using std::cout;
struct Dwarf {
    string name_;
    unsigned year_;
};
bool operator<(const Dwarf& a, const Dwarf& b) {
    return a.name_ < b.name_;
}
int main() {
    set dwarves{ Dwarf{"Balin", 2763}, Dwarf{"Dwalin", 2772},
        Dwarf{"Oin", 2774}, Dwarf{"Gloin", 2783}, Dwarf{"Thorin", 2746},
        Dwarf{"Fili", 2859}, Dwarf{"Kili", 2864} };
    for(const auto& z : dwarves) // sorted output: "Balin" to "Thorin"
        cout << z.name_  << " ";
    cout << "\n";
}
```

Godbolt Listing [lst-0039-godb.cpp](lst-0039-godb.cpp), [https://godbolt.org/z/3ch51ov87](https://godbolt.org/z/3ch51ov87):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/3ch51ov87 
#include <set>
#include <string>
#include <iostream> // cout
using std::string; using std::set; using std::cout;
struct Dwarf {
    string name_;
    unsigned year_;
};
bool operator<(const Dwarf& a, const Dwarf& b) {
    return a.name_ < b.name_;
}
int main() {
    set dwarves{ Dwarf{"Balin", 2763}, Dwarf{"Dwalin", 2772},
        Dwarf{"Oin", 2774}, Dwarf{"Gloin", 2783}, Dwarf{"Thorin", 2746},
        Dwarf{"Fili", 2859}, Dwarf{"Kili", 2864} };
    for(const auto& z : dwarves) // sorted output: "Balin" to "Thorin"
        cout << z.name_  << " ";
    cout << "\n";
}
```

----------------


## Listing 23.18: A no-frills functor with great utility.

Book listing [lst-0041-book.cpp](lst-0041-book.cpp):
```cpp
struct ByYear { // implements less-than by Dwarf::year_
    bool operator()(const Dwarf& a, const Dwarf& b) const {
        return a.year_ < b.year_;
    }
};
```

----------------


## GodboltId:PebG9bn9K

Book listing [lst-0042-book.cpp](lst-0042-book.cpp):
```cpp
// https://godbolt.org/z/PebG9bn9K
set<Dwarf,ByYear> dwarves2{begin(dwarves), end(dwarves)};
for(const auto& z : dwarves2) // differently sorted output
    cout << z.year_ << " ";
```

Godbolt Listing [lst-0042-godb.cpp](lst-0042-godb.cpp), [https://godbolt.org/z/PebG9bn9K](https://godbolt.org/z/PebG9bn9K):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/PebG9bn9K
set<Dwarf,ByYear> dwarves2{begin(dwarves), end(dwarves)};
for(const auto& z : dwarves2) // differently sorted output
    cout << z.year_ << " ";
```

----------------


## Listing 23.19: Many algorithms in the standard library also work with "operator<".

Book listing [lst-0046-book.cpp](lst-0046-book.cpp):
```cpp
// https://godbolt.org/z/YKzsW49G8 
#include <vector>
#include <algorithm>   // sort
// Definitions and further includes as before
int main() {
    vector dwarves{    // initialize as before
    /* sort */
    std::sort(begin(dwarves), end(dwarves));
    // output as before …
```

Godbolt Listing [lst-0046-godb.cpp](lst-0046-godb.cpp), [https://godbolt.org/z/YKzsW49G8](https://godbolt.org/z/YKzsW49G8):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/YKzsW49G8 
#include <vector>
#include <algorithm>   // sort
// Definitions and further includes as before
int main() {
    vector dwarves{    // initialize as before
    /* sort */
    std::sort(begin(dwarves), end(dwarves));
    // output as before …
```

----------------


## Listing 23.20: With the capture clause, you can access external variables in the lambda.

Book listing [lst-0053-book.cpp](lst-0053-book.cpp):
```cpp
// https://godbolt.org/z/eh6jnjK9z
#include <vector>
#include <string>
#include <algorithm> // sort
#include <iostream>  // cout
using std::string; using std::vector; using std::cout;
// as before
int main() {
    vector dwarves{        // as before
    /* sort */
    bool backwards = true; // or false. Variable outside the lambda
    std::ranges::sort(dwarves,
        [backwards](const Dwarf& a, const Dwarf& b) {
            if(backwards)
                return a.name_ > b.name_;
            else
                return a.name_ < b.name_;
        }
    );
    /* output */
    for(const auto& z : dwarves) // backwards: "Thorin" to "Balin"
        cout << z.name_  << " ";
    cout << "\n";
}
```

Godbolt Listing [lst-0053-godb.cpp](lst-0053-godb.cpp), [https://godbolt.org/z/eh6jnjK9z](https://godbolt.org/z/eh6jnjK9z):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/eh6jnjK9z
#include <vector>
#include <string>
#include <algorithm> // sort
#include <iostream>  // cout
using std::string; using std::vector; using std::cout;
// as before
int main() {
    vector dwarves{        // as before
    /* sort */
    bool backwards = true; // or false. Variable outside the lambda
    std::ranges::sort(dwarves,
        [backwards](const Dwarf& a, const Dwarf& b) {
            if(backwards)
                return a.name_ > b.name_;
            else
                return a.name_ < b.name_;
        }
    );
    /* output */
    for(const auto& z : dwarves) // backwards: "Thorin" to "Balin"
        cout << z.name_  << " ";
    cout << "\n";
}
```

----------------


## Listing 23.21: The capture clause can also contain references.

Book listing [lst-0054-book.cpp](lst-0054-book.cpp):
```cpp
// https://godbolt.org/z/j7qoMdG3n
#include <vector>
#include <string>
#include <algorithm> // sort
#include <iostream>  // cout
using std::string; using std::vector; using std::cout;
// as before
int main() {
    vector dwarfs{           // as before
    /* sort */
    bool backwards = true;   // or false. Variable outside the lambda
    unsigned rightway = 0;   // counts <
    unsigned wrongway = 0;   // counts >
    std::ranges::sort(dwarves,
        [backwards,&rightway,&wrongway](const Dwarf& a, const Dwarf& b) {
            bool result = backwards
                ? a.name_ > b.name_
                : a.name_ < b.name_;
            if(result==false) ++wrongway; else ++rightway;
            return result;
        }
    );
    /* output */
    cout << "Wrongway:" << wrongway << " Rightway: " << rightway << "\n";
    for(const auto& z : dwarves) // backwards: "Thorin" to "Balin"
        cout << z.name_  << " ";
    cout << "\n";
}
```

Godbolt Listing [lst-0054-godb.cpp](lst-0054-godb.cpp), [https://godbolt.org/z/j7qoMdG3n](https://godbolt.org/z/j7qoMdG3n):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/j7qoMdG3n
#include <vector>
#include <string>
#include <algorithm> // sort
#include <iostream>  // cout
using std::string; using std::vector; using std::cout;
// as before
int main() {
    vector dwarfs{           // as before
    /* sort */
    bool backwards = true;   // or false. Variable outside the lambda
    unsigned rightway = 0;   // counts <
    unsigned wrongway = 0;   // counts >
    std::ranges::sort(dwarves,
        [backwards,&rightway,&wrongway](const Dwarf& a, const Dwarf& b) {
            bool result = backwards
                ? a.name_ > b.name_
                : a.name_ < b.name_;
            if(result==false) ++wrongway; else ++rightway;
            return result;
        }
    );
    /* output */
    cout << "Wrongway:" << wrongway << " Rightway: " << rightway << "\n";
    for(const auto& z : dwarves) // backwards: "Thorin" to "Balin"
        cout << z.name_  << " ";
    cout << "\n";
}
```

----------------


## Listing 23.22: "mutable" makes value captures in lambdas mutable.

Book listing [lst-0056-book.cpp](lst-0056-book.cpp):
```cpp
// https://godbolt.org/z/zMWo69974 
#include <iostream>
int main() {
    int count = 0;
    auto plus1 = [count](int x) mutable { // count as a copy
        std::cout << ++count; return x+1;
    };
    for(int i=0; i<5; ++i) {
        plus1(i);
    }
    std::cout << "\n";
    // Output: 12345
}
```

Godbolt Listing [lst-0056-godb.cpp](lst-0056-godb.cpp), [https://godbolt.org/z/zMWo69974](https://godbolt.org/z/zMWo69974):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/zMWo69974 
#include <iostream>
int main() {
    int count = 0;
    auto plus1 = [count](int x) mutable { // count as a copy
        std::cout << ++count; return x+1;
    };
    for(int i=0; i<5; ++i) {
        plus1(i);
    }
    std::cout << "\n";
    // Output: 12345
}
```

----------------


## Listing 23.23: If possible, better without mutable.

Book listing [lst-0057-book.cpp](lst-0057-book.cpp):
```cpp
// https://godbolt.org/z/8GxPMEc5o 
#include <iostream>
int main() {
    int count = 0;
    auto plus1 = [&count](int x) { // count as reference
        ++count; return x+1;
    };
    for(int i=0; i<5; ++i) { plus1(i); }
    std::cout << "plus1 was called " << count << " times\n";
    // Output: plus1 was called 5 times
}
```

Godbolt Listing [lst-0057-godb.cpp](lst-0057-godb.cpp), [https://godbolt.org/z/8GxPMEc5o](https://godbolt.org/z/8GxPMEc5o):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/8GxPMEc5o 
#include <iostream>
int main() {
    int count = 0;
    auto plus1 = [&count](int x) { // count as reference
        ++count; return x+1;
    };
    for(int i=0; i<5; ++i) { plus1(i); }
    std::cout << "plus1 was called " << count << " times\n";
    // Output: plus1 was called 5 times
}
```

----------------


## GodboltId:xnbvEx5Kc

Book listing [lst-0060-book.cpp](lst-0060-book.cpp):
```cpp
// https://godbolt.org/z/xnbvEx5Kc 
#include <iostream>
#include <string>
using std::cout; using std::string; using namespace std::literals;
auto min2 = [](const auto &a, const auto &b) {
    return a<b ? a : b;
};
auto min3 = [](const auto &a, const auto &b, const auto &c) {
    return min2(a, min2(b,c));
};
int main() {
    cout << min3( 3, 7, 2 ) << '\n';                  // Output: 2
    cout << min3( 8.11, 113.2, -3.1 ) << '\n';        // Output: –3.1
    cout << min3( "Zoo"s, "Ape"s, "Mule"s ) << '\n';  // Output: Ape
}
```

Godbolt Listing [lst-0060-godb.cpp](lst-0060-godb.cpp), [https://godbolt.org/z/xnbvEx5Kc](https://godbolt.org/z/xnbvEx5Kc):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/xnbvEx5Kc 
#include <iostream>
#include <string>
using std::cout; using std::string; using namespace std::literals;
auto min2 = [](const auto &a, const auto &b) {
    return a<b ? a : b;
};
auto min3 = [](const auto &a, const auto &b, const auto &c) {
    return min2(a, min2(b,c));
};
int main() {
    cout << min3( 3, 7, 2 ) << '\n';                  // Output: 2
    cout << min3( 8.11, 113.2, -3.1 ) << '\n';        // Output: –3.1
    cout << min3( "Zoo"s, "Ape"s, "Mule"s ) << '\n';  // Output: Ape
}
```

----------------


## GodboltId:nenW8aM81

Book listing [lst-0061-book.cpp](lst-0061-book.cpp):
```cpp
// https://godbolt.org/z/nenW8aM81 
auto min2 = []<typename T>(const T &a, const T &b) {
    return a<b ? a : b;
};
auto min3 = []<typename T>(const T &a, const T &b, const T &c) {
    return min2(a, min2(b,c));
};
```

Godbolt Listing [lst-0061-godb.cpp](lst-0061-godb.cpp), [https://godbolt.org/z/nenW8aM81](https://godbolt.org/z/nenW8aM81):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/nenW8aM81 
auto min2 = []<typename T>(const T &a, const T &b) {
    return a<b ? a : b;
};
auto min3 = []<typename T>(const T &a, const T &b, const T &c) {
    return min2(a, min2(b,c));
};
```

----------------


## Listing 23.24: Concepts can be written in different ways.

Book listing [lst-0062-book.cpp](lst-0062-book.cpp):
```cpp
// https://godbolt.org/z/jP75nY4EW
#include <concepts>  // integral
#include <iostream>
using namespace std;
// Concept explicitly with requires
template<typename T> requires integral<T>
auto add_1(T val) { return val+1; }
// abbreviated concept
template<integral T>
auto add_2(T val) { return val+2; }
// abbreviated function template with concept
auto add_3(integral auto val) { return val+3; }
// Ad-hoc requirements for function template
auto add_4(auto val) requires integral<decltype(val)>
{ return val+4; }
int main() {
  cout << add_1(1) << '\n';               // Output: 2
  cout << add_2(1) << '\n';               // Output: 3
  cout << add_3(1) << '\n';               // Output: 4
  cout << add_4(1) << '\n';               // Output: 5
  cout << add_3("text") << '\n';          // (ERR) Error
  integral auto val = add_1(99);          // also for auto variables
  cout << val << '\n';                    // Output: 100
}
```

Godbolt Listing [lst-0062-godb.cpp](lst-0062-godb.cpp), [https://godbolt.org/z/jP75nY4EW](https://godbolt.org/z/jP75nY4EW):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/jP75nY4EW
#include <concepts>  // integral
#include <iostream>
using namespace std;
// Concept explicitly with requires
template<typename T> requires integral<T>
auto add_1(T val) { return val+1; }
// abbreviated concept
template<integral T>
auto add_2(T val) { return val+2; }
// abbreviated function template with concept
auto add_3(integral auto val) { return val+3; }
// Ad-hoc requirements for function template
auto add_4(auto val) requires integral<decltype(val)>
{ return val+4; }
int main() {
  cout << add_1(1) << '\n';               // Output: 2
  cout << add_2(1) << '\n';               // Output: 3
  cout << add_3(1) << '\n';               // Output: 4
  cout << add_4(1) << '\n';               // Output: 5
  cout << add_3("text") << '\n';          // (ERR) Error
  integral auto val = add_1(99);          // also for auto variables
  cout << val << '\n';                    // Output: 100
}
```

----------------


## Listing 23.25: With concepts, you can restrict individual overloads.

Book listing [lst-0063-book.cpp](lst-0063-book.cpp):
```cpp
// https://godbolt.org/z/Kxrz5ETh6 
#include <concepts>  // integral, floating_point
#include <iostream>
using namespace std;
int64_t trunc_to_10(integral auto val) { return (val/10)*10; }
int64_t trunc_to_10(floating_point auto val) { return int64_t(val/10)*10; }
int main() {
  cout << trunc_to_10(144) << '\n';    // Output: 140
  cout << trunc_to_10(122.2) << '\n';  // Output: 120
}
```

Godbolt Listing [lst-0063-godb.cpp](lst-0063-godb.cpp), [https://godbolt.org/z/Kxrz5ETh6](https://godbolt.org/z/Kxrz5ETh6):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/Kxrz5ETh6 
#include <concepts>  // integral, floating_point
#include <iostream>
using namespace std;
int64_t trunc_to_10(integral auto val) { return (val/10)*10; }
int64_t trunc_to_10(floating_point auto val) { return int64_t(val/10)*10; }
int main() {
  cout << trunc_to_10(144) << '\n';    // Output: 140
  cout << trunc_to_10(122.2) << '\n';  // Output: 120
}
```

----------------


## GodboltId:eaGre4bqr

Book listing [lst-0065-book.cpp](lst-0065-book.cpp):
```cpp
// https://godbolt.org/z/eaGre4bqr 
auto min2(const std::integral auto &a, const std::integral auto &b) {
    return a<b ? a : b;
};

auto min3 = []<std::integral T>(const T &a, const T &b, const T &c) {
    return min2(a, min2(b,c));
};

int main() {
    cout << min3( 3, 7, 2 ) << '\n';
    cout << min3( 8.11, 113.2, -3.1 ) << '\n'; // (ERR) Error: not an integral type
}
```

Godbolt Listing [lst-0065-godb.cpp](lst-0065-godb.cpp), [https://godbolt.org/z/eaGre4bqr](https://godbolt.org/z/eaGre4bqr):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/eaGre4bqr 
auto min2(const std::integral auto &a, const std::integral auto &b) {
    return a<b ? a : b;
};

auto min3 = []<std::integral T>(const T &a, const T &b, const T &c) {
    return min2(a, min2(b,c));
};

int main() {
    cout << min3( 3, 7, 2 ) << '\n';
    cout << min3( 8.11, 113.2, -3.1 ) << '\n'; // (ERR) Error: not an integral type
}
```

----------------


## Listing 23.26: Specify additional conditions with requires.

Book listing [lst-0066-book.cpp](lst-0066-book.cpp):
```cpp
// https://godbolt.org/z/fxoo4h9Gj 
#include <concepts>
#include <iostream>
#include <string>
using namespace std; using namespace std::literals;

string mk_string(integral auto val) { return to_string(val); }
string mk_string(string val) { return '['+val+']'; }
template<typename T>
  requires copyable<T> &&                            // Type requirement
  requires (T t) { mk_string(t) + mk_string(t); } && // simple requirement
  requires (T t) {                     // composite requirement
    {mk_string(t)} -> same_as<string>; // valid expression must meet condition
  }
string dbl_quote(const T& val) {
  T val2{val};                       // Create a copy (for demonstration purposes only)
  return '"' + mk_string(val) + mk_string(val2) + '"';
}
int main() {
  cout << dbl_quote(10) << '\n';    // Output: "1010"
  cout << dbl_quote("ab"s) << '\n'; // Output: "[ab][ab]"
  cout << dbl_quote(3.14) << '\n';  // (ERR) Error: no suitable mk_string overload
}
```

Godbolt Listing [lst-0066-godb.cpp](lst-0066-godb.cpp), [https://godbolt.org/z/fxoo4h9Gj](https://godbolt.org/z/fxoo4h9Gj):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/fxoo4h9Gj 
#include <concepts>
#include <iostream>
#include <string>
using namespace std; using namespace std::literals;

string mk_string(integral auto val) { return to_string(val); }
string mk_string(string val) { return '['+val+']'; }
template<typename T>
  requires copyable<T> &&                            // Type requirement
  requires (T t) { mk_string(t) + mk_string(t); } && // simple requirement
  requires (T t) {                     // composite requirement
    {mk_string(t)} -> same_as<string>; // valid expression must meet condition
  }
string dbl_quote(const T& val) {
  T val2{val};                       // Create a copy (for demonstration purposes only)
  return '"' + mk_string(val) + mk_string(val2) + '"';
}
int main() {
  cout << dbl_quote(10) << '\n';    // Output: "1010"
  cout << dbl_quote("ab"s) << '\n'; // Output: "[ab][ab]"
  cout << dbl_quote(3.14) << '\n';  // (ERR) Error: no suitable mk_string overload
}
```

----------------


## Listing 23.27: Define recurring requirements with concept.

Book listing [lst-0067-book.cpp](lst-0067-book.cpp):
```cpp
// https://godbolt.org/z/Tcxnr6jTM 
template<typename T>
  concept dbl_quotable = copyable<T> &&
  requires(T t) { mk_string(t) + mk_string(t); } &&
  requires(T t) {
    {mk_string(t)} -> same_as<string>;
  };
string dbl_quote(const dbl_quotable auto& val) {
  auto val2{val}; // Create a copy (for demonstration purposes only)
  return '"' + mk_string(val) + mk_string(val2) + '"';
}
```

Godbolt Listing [lst-0067-godb.cpp](lst-0067-godb.cpp), [https://godbolt.org/z/Tcxnr6jTM](https://godbolt.org/z/Tcxnr6jTM):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/Tcxnr6jTM 
template<typename T>
  concept dbl_quotable = copyable<T> &&
  requires(T t) { mk_string(t) + mk_string(t); } &&
  requires(T t) {
    {mk_string(t)} -> same_as<string>;
  };
string dbl_quote(const dbl_quotable auto& val) {
  auto val2{val}; // Create a copy (for demonstration purposes only)
  return '"' + mk_string(val) + mk_string(val2) + '"';
}
```

----------------


## GodboltId:9qsGW6M8Y

Book listing [lst-0068-book.cpp](lst-0068-book.cpp):
```cpp
// https://godbolt.org/z/9qsGW6M8Y 
template<std::ranges::range T>         // a range over type T
using ValueTypeOfRange = std::ranges::range_value_t<T>;
// or
template<typename T>
requires std::ranges::range<T>         // a range over type T
using ValueTypeOfRange = std::ranges::range_value_t<T>;
// results in:
ValueTypeOfRange<std::vector<int>> x;  // x is int
ValueTypeOfRange<std::string> y;       // y is char
ValueTypeOfRange<std::list<double>> z; // z is double
ValueTypeOfRange<int> w;               // (ERR) int is not a range
```

Godbolt Listing [lst-0068-godb.cpp](lst-0068-godb.cpp), [https://godbolt.org/z/9qsGW6M8Y](https://godbolt.org/z/9qsGW6M8Y):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/9qsGW6M8Y 
template<std::ranges::range T>         // a range over type T
using ValueTypeOfRange = std::ranges::range_value_t<T>;
// or
template<typename T>
requires std::ranges::range<T>         // a range over type T
using ValueTypeOfRange = std::ranges::range_value_t<T>;
// results in:
ValueTypeOfRange<std::vector<int>> x;  // x is int
ValueTypeOfRange<std::string> y;       // y is char
ValueTypeOfRange<std::list<double>> z; // z is double
ValueTypeOfRange<int> w;               // (ERR) int is not a range
```

----------------


## Listing 23.28: This is how you use concepts in a constexpr if.

Book listing [lst-0069-book.cpp](lst-0069-book.cpp):
```cpp
if constexpr std::integral<T> {
  // T is an integral type
} else if constexpr std::floating_point<T> {
  // T is a floating-point type
} else {
  // T is neither
}
```

----------------


## Listing 23.29: A class template takes a type as a formal parameter.

Book listing [lst-0073-book.cpp](lst-0073-book.cpp):
```cpp
// https://godbolt.org/z/jqzTMxYKv 
template <std::copyable T>      // C++20 concept
class MyContainer {
    T data_;
public:
    void setData(const T& d) { data_ = d; }
    T getData() const { return data_; }
};
```

Godbolt Listing [lst-0073-godb.cpp](lst-0073-godb.cpp), [https://godbolt.org/z/jqzTMxYKv](https://godbolt.org/z/jqzTMxYKv):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/jqzTMxYKv 
template <std::copyable T>      // C++20 concept
class MyContainer {
    T data_;
public:
    void setData(const T& d) { data_ = d; }
    T getData() const { return data_; }
};
```

----------------


## Listing 23.30: Method definitions outside the body of a class template are syntactically somewhat more complex.

Book listing [lst-0075-book.cpp](lst-0075-book.cpp):
```cpp
// https://godbolt.org/z/4fK5WdoG8 
template <std::copyable T>
class MyContainer {
    T data_;
public:
    void setData(const T& d);
    T getData() const;
};
template <std::copyable T>
void MyContainer<T>::setData(const T& d) {
    data_ = d;
}
template <std::copyable T>
T MyContainer<T>::getData() const {
    return data_;
}
```

Godbolt Listing [lst-0075-godb.cpp](lst-0075-godb.cpp), [https://godbolt.org/z/4fK5WdoG8](https://godbolt.org/z/4fK5WdoG8):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/4fK5WdoG8 
template <std::copyable T>
class MyContainer {
    T data_;
public:
    void setData(const T& d);
    T getData() const;
};
template <std::copyable T>
void MyContainer<T>::setData(const T& d) {
    data_ = d;
}
template <std::copyable T>
T MyContainer<T>::getData() const {
    return data_;
}
```

----------------


## Listing 23.31: Specializing class template methods like function templates.

Book listing [lst-0076-book.cpp](lst-0076-book.cpp):
```cpp
// https://godbolt.org/z/M6earoWMn 
#include <iostream>
#include <string>
#include <concepts>    // copyable, C++20
template <std::copyable T>
class MyContainer {
    T data_;
public:
    void setData(const T& d) { data_ = d; } // general case
    T getData() const { return data_; }     // general case
};
template<>                                  // Specialization
void MyContainer<std::string>::setData(const std::string& d) {
    data_ = "[" + d + "]";
}
int main() {
    MyContainer<std::string> mcString;
    mcString.setData("History");
    std::cout << mcString.getData() << '\n';  // Output: [History]
    MyContainer<int> mcInt;
    mcInt.setData(5);
    std::cout << mcInt.getData() << '\n';     // Output: 5
}
```

Godbolt Listing [lst-0076-godb.cpp](lst-0076-godb.cpp), [https://godbolt.org/z/M6earoWMn](https://godbolt.org/z/M6earoWMn):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/M6earoWMn 
#include <iostream>
#include <string>
#include <concepts>    // copyable, C++20
template <std::copyable T>
class MyContainer {
    T data_;
public:
    void setData(const T& d) { data_ = d; } // general case
    T getData() const { return data_; }     // general case
};
template<>                                  // Specialization
void MyContainer<std::string>::setData(const std::string& d) {
    data_ = "[" + d + "]";
}
int main() {
    MyContainer<std::string> mcString;
    mcString.setData("History");
    std::cout << mcString.getData() << '\n';  // Output: [History]
    MyContainer<int> mcInt;
    mcInt.setData(5);
    std::cout << mcInt.getData() << '\n';     // Output: 5
}
```

----------------


## Listing 23.32: This is how you create instances from a template class.

Book listing [lst-0077-book.cpp](lst-0077-book.cpp):
```cpp
// https://godbolt.org/z/9vP3o68bW 
#include <iostream>
#include <string>
#include <concepts>    // copyable, C++20
template <std::copyable T>
class MyContainer {
    T data_;
public:
    void setData(const T& d) { data_ = d; }
    T getData() const { return data_; }
};

class IntValue {
    int val_;
public:
    explicit IntValue(int val=0) : val_(val) {}
    int getInt() const { return val_; }
};
int main() {
    // C-array with three MyContainer<double> instances
    MyContainer<double> dcont[3];
    dcont[0].setData(123.123);
    dcont[1].setData(234.234);
    std::cout << dcont[0].getData() << std::endl;
    std::cout << dcont[1].getData() << std::endl;
    // custom data type as formal parameter
    IntValue ival{100'000};
    MyContainer<IntValue> scont;
    scont.setData(ival);
    std::cout << scont.getData().getInt() << std::endl;
    // string as formal parameter
    std::string str("Text");
    MyContainer<std::string> strCont;
    strCont.setData(str);
    std::cout << strCont.getData() << std::endl;
}
```

Godbolt Listing [lst-0077-godb.cpp](lst-0077-godb.cpp), [https://godbolt.org/z/9vP3o68bW](https://godbolt.org/z/9vP3o68bW):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/9vP3o68bW 
#include <iostream>
#include <string>
#include <concepts>    // copyable, C++20
template <std::copyable T>
class MyContainer {
    T data_;
public:
    void setData(const T& d) { data_ = d; }
    T getData() const { return data_; }
};

class IntValue {
    int val_;
public:
    explicit IntValue(int val=0) : val_(val) {}
    int getInt() const { return val_; }
};
int main() {
    // C-array with three MyContainer<double> instances
    MyContainer<double> dcont[3];
    dcont[0].setData(123.123);
    dcont[1].setData(234.234);
    std::cout << dcont[0].getData() << std::endl;
    std::cout << dcont[1].getData() << std::endl;
    // custom data type as formal parameter
    IntValue ival{100'000};
    MyContainer<IntValue> scont;
    scont.setData(ival);
    std::cout << scont.getData().getInt() << std::endl;
    // string as formal parameter
    std::string str("Text");
    MyContainer<std::string> strCont;
    strCont.setData(str);
    std::cout << strCont.getData() << std::endl;
}
```

----------------


## Listing 23.33: Deduce template parameters from constructor arguments.

Book listing [lst-0078-book.cpp](lst-0078-book.cpp):
```cpp
// https://godbolt.org/z/YaEW5aTra
#include <string>
#include <vector>
#include <set>
#include <tuple>
#include <memory> // shared_ptr
template <typename T>
class MyStuff {
    T data_;
public:
    MyStuff() : data_{} {}             // default constructor
    MyStuff(const T& d) : data_{d} { } // copy constructor
    T getStuff() const { return data_; }
};
class IntValue {
    int val_;
public:
    explicit IntValue(int val=0) : val_(val) {}
    int getInt() const { return val_; }
};
int main() {
    MyStuff intStuff(12);               // becomes MyStuff<int>
    std::vector vs{ 1,2,3,4 };          // becomes vector<int>
    MyStuff ivalStuff{ IntValue{33} };  // becomes MyStuff<IntValue>
    std::tuple tpl{ 23, 'a' };          // becomes tuple<int,char>
    std::set twoThree (vs.begin()+1, vs.end()-1); // becomes set<int>
    // The compiler cannot deduce:
    MyStuff<double> dblStuff;               // no constructor argument
    std::vector<char> vcs(10);              // size 10, but of what type?
    std::shared_ptr<int> ptr{new int{88}};  // no rule for int*
}
```

Godbolt Listing [lst-0078-godb.cpp](lst-0078-godb.cpp), [https://godbolt.org/z/YaEW5aTra](https://godbolt.org/z/YaEW5aTra):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/YaEW5aTra
#include <string>
#include <vector>
#include <set>
#include <tuple>
#include <memory> // shared_ptr
template <typename T>
class MyStuff {
    T data_;
public:
    MyStuff() : data_{} {}             // default constructor
    MyStuff(const T& d) : data_{d} { } // copy constructor
    T getStuff() const { return data_; }
};
class IntValue {
    int val_;
public:
    explicit IntValue(int val=0) : val_(val) {}
    int getInt() const { return val_; }
};
int main() {
    MyStuff intStuff(12);               // becomes MyStuff<int>
    std::vector vs{ 1,2,3,4 };          // becomes vector<int>
    MyStuff ivalStuff{ IntValue{33} };  // becomes MyStuff<IntValue>
    std::tuple tpl{ 23, 'a' };          // becomes tuple<int,char>
    std::set twoThree (vs.begin()+1, vs.end()-1); // becomes set<int>
    // The compiler cannot deduce:
    MyStuff<double> dblStuff;               // no constructor argument
    std::vector<char> vcs(10);              // size 10, but of what type?
    std::shared_ptr<int> ptr{new int{88}};  // no rule for int*
}
```

----------------


## Listing 23.34: You have been able to use make_ helper functions and auto for a long time.

Book listing [lst-0079-book.cpp](lst-0079-book.cpp):
```cpp
auto tpl = std::make_tuple( 5, 'x' );
auto ptr = std::make_shared( 5 );
```

----------------


## GodboltId:dMTnbT1v4

Book listing [lst-0080-book.cpp](lst-0080-book.cpp):
```cpp
// https://godbolt.org/z/dMTnbT1v4 
#include <iostream>
#include <concepts>    // copyable, C+20
template<std::copyable T, std::copyable U>
class MyPair {
    T data01_;
    U data02_;
public:
    MyPair(const T& t,const U& u) : data01_{t}, data02_{u} {}
    void print(std::ostream& os) const {
        os << data01_ << " : " << data02_ << std::endl;
    }
};
int main() {
    std::string month{"January"};
    int temp = -5;
    MyPair<std::string, int> temperature{month, temp};
    temperature.print(std::cout);
}
```

Godbolt Listing [lst-0080-godb.cpp](lst-0080-godb.cpp), [https://godbolt.org/z/dMTnbT1v4](https://godbolt.org/z/dMTnbT1v4):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/dMTnbT1v4 
#include <iostream>
#include <concepts>    // copyable, C+20
template<std::copyable T, std::copyable U>
class MyPair {
    T data01_;
    U data02_;
public:
    MyPair(const T& t,const U& u) : data01_{t}, data02_{u} {}
    void print(std::ostream& os) const {
        os << data01_ << " : " << data02_ << std::endl;
    }
};
int main() {
    std::string month{"January"};
    int temp = -5;
    MyPair<std::string, int> temperature{month, temp};
    temperature.print(std::cout);
}
```

----------------


## GodboltId:jbafeEdoW

Book listing [lst-0081-book.cpp](lst-0081-book.cpp):
```cpp
// https://godbolt.org/z/jbafeEdoW 
#include <iostream>
#include <concepts>    // copyable, C++20
template<std::copyable T, std::copyable U=T>
class MyPair {
    T data01_;
    U data02_;
public:
    MyPair(const T& t,const U& u) : data01_{t}, data02_{u} {}
    void print(std::ostream& os) const {
        os << data01_ << " : " << data02_ << std::endl;
    }
};
int main() {
    MyPair<double> numbers{11.11, 22.22};
    numbers.print(std::cout);
}
```

Godbolt Listing [lst-0081-godb.cpp](lst-0081-godb.cpp), [https://godbolt.org/z/jbafeEdoW](https://godbolt.org/z/jbafeEdoW):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/jbafeEdoW 
#include <iostream>
#include <concepts>    // copyable, C++20
template<std::copyable T, std::copyable U=T>
class MyPair {
    T data01_;
    U data02_;
public:
    MyPair(const T& t,const U& u) : data01_{t}, data02_{u} {}
    void print(std::ostream& os) const {
        os << data01_ << " : " << data02_ << std::endl;
    }
};
int main() {
    MyPair<double> numbers{11.11, 22.22};
    numbers.print(std::cout);
}
```

----------------


## GodboltId:E89bav3rE

Book listing [lst-0084-book.cpp](lst-0084-book.cpp):
```cpp
// https://godbolt.org/z/E89bav3rE
#include <iostream>
template <typename T, size_t n=1>    // Nontype parameter with default value
class FixedArray {
    T data_[n] {0};                  // Using nontype parameter
public:
    T& operator[](size_t index) { return data_[index]; }
    static constexpr size_t size() { return n; }
    void print(std::ostream &os) const {
        for(auto it : data_)
            os << it << ' ';
        os << '\n';
    }
};
int main() {
    FixedArray<int,10> vals {};         // n = 10
    for(size_t idx=0; idx < vals.size(); ++idx) {
        vals[idx] = idx+idx;
    }
    vals.print(std::cout);              // Output: 0 2 4 6 8 10 12 14 16 18
    FixedArray<double> dvals;           // Default parameter for n
    std::cout << dvals.size() << '\n';  // Output: 1
}
```

Godbolt Listing [lst-0084-godb.cpp](lst-0084-godb.cpp), [https://godbolt.org/z/E89bav3rE](https://godbolt.org/z/E89bav3rE):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/E89bav3rE
#include <iostream>
template <typename T, size_t n=1>    // Nontype parameter with default value
class FixedArray {
    T data_[n] {0};                  // Using nontype parameter
public:
    T& operator[](size_t index) { return data_[index]; }
    static constexpr size_t size() { return n; }
    void print(std::ostream &os) const {
        for(auto it : data_)
            os << it << ' ';
        os << '\n';
    }
};
int main() {
    FixedArray<int,10> vals {};         // n = 10
    for(size_t idx=0; idx < vals.size(); ++idx) {
        vals[idx] = idx+idx;
    }
    vals.print(std::cout);              // Output: 0 2 4 6 8 10 12 14 16 18
    FixedArray<double> dvals;           // Default parameter for n
    std::cout << dvals.size() << '\n';  // Output: 1
}
```

----------------


## GodboltId:9rzsh5366

Book listing [lst-0089-book.cpp](lst-0089-book.cpp):
```cpp
// https://godbolt.org/z/9rzsh5366
#include <string>
#include <tuple>
#include <concepts>             // copyable, C++20
template <std::copyable T, std::copyable U=T> // general case
class MyPair {
    T data01_;
    U data02_;
public:
    MyPair(const T& t, const U& u) : data01_{t}, data02_{u} {}
};
template <std::copyable T>      // partial specialization, T remains formal
class MyPair<T, std::string> {  // U is specialized to string
    std::tuple<T,std::string> data_;
public:
    MyPair(const T& t, const std::string& str) : data_{t, str} { }
};
template <std::copyable U>      // partial specialization, U remains formal
class MyPair<std::string, U> {  // T is specialized to string
    std::tuple<std::string, U> data_;
public:
    MyPair(const std::string& str, const U& u) : data_{str, u} { }
};
int main() {
  // uses partial specialization
  MyPair<int,std::string> intString{1, "a"};
  MyPair<std::string,int> stringInt{"b", 2};

  MyPair<int,int> intInt{3,4};                     // uses general case
  MyPair intInt2{3,4};                             // also MyPair<int,int>
  MyPair<std::string,std::string> strStr{"c","d"}; // (ERR) ambiguous
}
```

Godbolt Listing [lst-0089-godb.cpp](lst-0089-godb.cpp), [https://godbolt.org/z/9rzsh5366](https://godbolt.org/z/9rzsh5366):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/9rzsh5366
#include <string>
#include <tuple>
#include <concepts>             // copyable, C++20
template <std::copyable T, std::copyable U=T> // general case
class MyPair {
    T data01_;
    U data02_;
public:
    MyPair(const T& t, const U& u) : data01_{t}, data02_{u} {}
};
template <std::copyable T>      // partial specialization, T remains formal
class MyPair<T, std::string> {  // U is specialized to string
    std::tuple<T,std::string> data_;
public:
    MyPair(const T& t, const std::string& str) : data_{t, str} { }
};
template <std::copyable U>      // partial specialization, U remains formal
class MyPair<std::string, U> {  // T is specialized to string
    std::tuple<std::string, U> data_;
public:
    MyPair(const std::string& str, const U& u) : data_{str, u} { }
};
int main() {
  // uses partial specialization
  MyPair<int,std::string> intString{1, "a"};
  MyPair<std::string,int> stringInt{"b", 2};

  MyPair<int,int> intInt{3,4};                     // uses general case
  MyPair intInt2{3,4};                             // also MyPair<int,int>
  MyPair<std::string,std::string> strStr{"c","d"}; // (ERR) ambiguous
}
```

----------------


## GodboltId:baPeoTbM8

Book listing [lst-0090-book.cpp](lst-0090-book.cpp):
```cpp
// https://godbolt.org/z/baPeoTbM8 
template<>   // full specialization
class MyPair<std::string, std::string> {
    std::vector<std::string> data_;
public:
    MyPair(const std::string& t, const std::string& u) : data_{t, u} { }
};
int main() {
    // uses the full specialization:
    MyPair<std::string,std::string> strStr{"c","d"};
}
```

Godbolt Listing [lst-0090-godb.cpp](lst-0090-godb.cpp), [https://godbolt.org/z/baPeoTbM8](https://godbolt.org/z/baPeoTbM8):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/baPeoTbM8 
template<>   // full specialization
class MyPair<std::string, std::string> {
    std::vector<std::string> data_;
public:
    MyPair(const std::string& t, const std::string& u) : data_{t, u} { }
};
int main() {
    // uses the full specialization:
    MyPair<std::string,std::string> strStr{"c","d"};
}
```

----------------


## Listing 23.35: A template with a variable number of arguments.

Book listing [lst-0091-book.cpp](lst-0091-book.cpp):
```cpp
// https://godbolt.org/z/eenKej9sj
#include <typeinfo>               // operator typeid
#include <typeindex>              // type_index
#include <map>
#include <string>
#include <iostream>
template <typename T>
void output(std::ostream& os, T val) { // output a type name
    // static: initialized the first time
    static const std::map<std::type_index,std::string> type_names {
        { std::type_index(typeid(const char*)), "const char*"},
        { std::type_index(typeid(char)), "char"},
        { std::type_index(typeid(int)), "int"},
        { std::type_index(typeid(double)), "double"},
        { std::type_index(typeid(bool)), "bool"},
    };
    const std::string name = type_names.at(std::type_index(typeid(T)));
    os << name << ": " << val << '\n';
}
// recursive variadic function template:
template<typename First, typename ... Rest>
void output(std::ostream &os, First first, Rest ... rest) {
    output(os, first);          // single call with the foremost element
    output(os, rest ...);       // recursion with the rest of the elements
}
int main() {
  output(std::cout, 3.1415);                     // normal template
  output(std::cout, "end", 2, 3.14, 'A', false); // recursive variadic function
}
```

Godbolt Listing [lst-0091-godb.cpp](lst-0091-godb.cpp), [https://godbolt.org/z/eenKej9sj](https://godbolt.org/z/eenKej9sj):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/eenKej9sj
#include <typeinfo>               // operator typeid
#include <typeindex>              // type_index
#include <map>
#include <string>
#include <iostream>
template <typename T>
void output(std::ostream& os, T val) { // output a type name
    // static: initialized the first time
    static const std::map<std::type_index,std::string> type_names {
        { std::type_index(typeid(const char*)), "const char*"},
        { std::type_index(typeid(char)), "char"},
        { std::type_index(typeid(int)), "int"},
        { std::type_index(typeid(double)), "double"},
        { std::type_index(typeid(bool)), "bool"},
    };
    const std::string name = type_names.at(std::type_index(typeid(T)));
    os << name << ": " << val << '\n';
}
// recursive variadic function template:
template<typename First, typename ... Rest>
void output(std::ostream &os, First first, Rest ... rest) {
    output(os, first);          // single call with the foremost element
    output(os, rest ...);       // recursion with the rest of the elements
}
int main() {
  output(std::cout, 3.1415);                     // normal template
  output(std::cout, "end", 2, 3.14, 'A', false); // recursive variadic function
}
```

----------------


## GodboltId:8oYaWYnbx

Book listing [lst-0097-book.cpp](lst-0097-book.cpp):
```cpp
// https://godbolt.org/z/8oYaWYnbx 
#include <tuple>
#include <iostream>
template<typename ... Args>
auto conv2tuple(Args ... args) {
    return std::make_tuple(args...);
}
int main() {
    auto mytuple = conv2tuple("end", 2, 3.14, 'A', false);
    std::cout << std::get<2>(mytuple) << '\n'; // Output: 3.14
}
```

Godbolt Listing [lst-0097-godb.cpp](lst-0097-godb.cpp), [https://godbolt.org/z/8oYaWYnbx](https://godbolt.org/z/8oYaWYnbx):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/8oYaWYnbx 
#include <tuple>
#include <iostream>
template<typename ... Args>
auto conv2tuple(Args ... args) {
    return std::make_tuple(args...);
}
int main() {
    auto mytuple = conv2tuple("end", 2, 3.14, 'A', false);
    std::cout << std::get<2>(mytuple) << '\n'; // Output: 3.14
}
```

----------------


## Listing 23.36: Custom literal operators.

Book listing [lst-0098-book.cpp](lst-0098-book.cpp):
```cpp
// https://godbolt.org/z/cjx49ef19 
namespace my {
  Complex operator"" _i(const char*); // 0+ni
  Complex operator"" _j(long double); // n+0i
  Puzzle operator"" _puzzle(const char*, size_t);
}
using namespace my;
Complex imag4 = 4_i;      // operator"" _i(const char*)
Complex real3 = 3.331_j;  // operator"" _j(long double)

Puzzle p1 = "oXo"         // operator"" _puzzle(const char*, size_t)
            "XoX"_puzzle;
```

Godbolt Listing [lst-0098-godb.cpp](lst-0098-godb.cpp), [https://godbolt.org/z/cjx49ef19](https://godbolt.org/z/cjx49ef19):
```cpp
//#(execute) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/cjx49ef19 
namespace my {
  Complex operator"" _i(const char*); // 0+ni
  Complex operator"" _j(long double); // n+0i
  Puzzle operator"" _puzzle(const char*, size_t);
}
using namespace my;
Complex imag4 = 4_i;      // operator"" _i(const char*)
Complex real3 = 3.331_j;  // operator"" _j(long double)

Puzzle p1 = "oXo"         // operator"" _puzzle(const char*, size_t)
            "XoX"_puzzle;
```

----------------


## Listing 23.37: Which literal leads to which operator call?

Book listing [lst-0099-book.cpp](lst-0099-book.cpp):
```cpp
// https://godbolt.org/z/Mevoshb3o 
namespace lits {
  long double operator"" _w(long double);
  string      operator"" _w(const char16_t*, size_t);
  unsigned    operator"" _w(const char*);
}
int main() {
  using namespace lits;
  1.2_w;       // operator"" _w(long double), with (1.2L)
  u"one"_w;    // operator"" _w(char16_t, size_t), with (u"one", 3)
  12_w;        // operator"" _w(const char*), with "12"
  "two"_w;     // (ERR) operator"" _w(const char*, size_t) not defined
}
```

Godbolt Listing [lst-0099-godb.cpp](lst-0099-godb.cpp), [https://godbolt.org/z/Mevoshb3o](https://godbolt.org/z/Mevoshb3o):
```cpp
//#(execute) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/Mevoshb3o 
namespace lits {
  long double operator"" _w(long double);
  string      operator"" _w(const char16_t*, size_t);
  unsigned    operator"" _w(const char*);
}
int main() {
  using namespace lits;
  1.2_w;       // operator"" _w(long double), with (1.2L)
  u"one"_w;    // operator"" _w(char16_t, size_t), with (u"one", 3)
  12_w;        // operator"" _w(const char*), with "12"
  "two"_w;     // (ERR) operator"" _w(const char*, size_t) not defined
}
```

----------------


## Listing 23.38: A template for literals of exact length two.

Book listing [lst-0100-book.cpp](lst-0100-book.cpp):
```cpp
// https://godbolt.org/z/5rdzn8YP4 
namespace lits {
  // general template
  template<char...> int operator"" _bin2();
  // specializations
  template<> int operator"" _bin2<'0','0'>() { return 0; }
  template<> int operator"" _bin2<'0','1'>() { return 1; }
  template<> int operator"" _bin2<'1','0'>() { return 2; }
  template<> int operator"" _bin2<'1','1'>() { return 3; }
}
int main() {
  using namespace lits;
  int one   = 01_bin2;
  int three = 11_bin2;
}
```

Godbolt Listing [lst-0100-godb.cpp](lst-0100-godb.cpp), [https://godbolt.org/z/5rdzn8YP4](https://godbolt.org/z/5rdzn8YP4):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/5rdzn8YP4 
namespace lits {
  // general template
  template<char...> int operator"" _bin2();
  // specializations
  template<> int operator"" _bin2<'0','0'>() { return 0; }
  template<> int operator"" _bin2<'0','1'>() { return 1; }
  template<> int operator"" _bin2<'1','0'>() { return 2; }
  template<> int operator"" _bin2<'1','1'>() { return 3; }
}
int main() {
  using namespace lits;
  int one   = 01_bin2;
  int three = 11_bin2;
}
```

----------------


## Listing 23.39: Recursive definition for arbitrary literal lengths.

Book listing [lst-0101-book.cpp](lst-0101-book.cpp):
```cpp
// https://godbolt.org/z/sbxojqjqs 
namespace lits {
  // Template helper function for one argument
  template<char C> int bin();  // general case
  template<>       int bin<'1'>() { return 1; } // Specialization
  template<>       int bin<'0'>() { return 0; } // Specialization
  // Template helper function for two or more arguments
  template<char C, char D, char... ES>
  int bin() {
    return bin<C>() << (sizeof...(ES)+1) | bin<D,ES...>(); // Bit-Shift, Bit-Or
  }
  // actual operator""
  template<char...CS> int operator"" _bin()
    { return bin<CS...>(); };
}
int main() {
  using namespace lits;
  int one = 1_bin;
  int eight = 1000_bin;
  int nine = 1001_bin;
  int ten = 1010_bin;
  int eleven  = 1011_bin;
  int one_hundred_twenty_eight = 10000000_bin;
}
```

Godbolt Listing [lst-0101-godb.cpp](lst-0101-godb.cpp), [https://godbolt.org/z/sbxojqjqs](https://godbolt.org/z/sbxojqjqs):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/sbxojqjqs 
namespace lits {
  // Template helper function for one argument
  template<char C> int bin();  // general case
  template<>       int bin<'1'>() { return 1; } // Specialization
  template<>       int bin<'0'>() { return 0; } // Specialization
  // Template helper function for two or more arguments
  template<char C, char D, char... ES>
  int bin() {
    return bin<C>() << (sizeof...(ES)+1) | bin<D,ES...>(); // Bit-Shift, Bit-Or
  }
  // actual operator""
  template<char...CS> int operator"" _bin()
    { return bin<CS...>(); };
}
int main() {
  using namespace lits;
  int one = 1_bin;
  int eight = 1000_bin;
  int nine = 1001_bin;
  int ten = 1010_bin;
  int eleven  = 1011_bin;
  int one_hundred_twenty_eight = 10000000_bin;
}
```

----------------


## Listing 23.40: The new if constexpr saves function specializations.

Book listing [lst-0102-book.cpp](lst-0102-book.cpp):
```cpp
// https://godbolt.org/z/KvsGh4f1v 
namespace lits {
  // Template helper function for one argument
  template<char C> int bin() {  // general case
    if constexpr (C=='0') return 0;
    else if constexpr (C=='1') return 1;
  }
  // Template helper function for two or more arguments
  // …
}
```

Godbolt Listing [lst-0102-godb.cpp](lst-0102-godb.cpp), [https://godbolt.org/z/KvsGh4f1v](https://godbolt.org/z/KvsGh4f1v):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/KvsGh4f1v 
namespace lits {
  // Template helper function for one argument
  template<char C> int bin() {  // general case
    if constexpr (C=='0') return 0;
    else if constexpr (C=='1') return 1;
  }
  // Template helper function for two or more arguments
  // …
}
```

----------------


## Listing 23.41: Fold expressions make variadic templates simpler.

Book listing [lst-0103-book.cpp](lst-0103-book.cpp):
```cpp
// https://godbolt.org/z/cPoEjqzM7 
#include <iostream>
#include <string>
namespace lits {
    // operator"" _sx
    template<char...CS> std::string operator"" _sx() {
        return (std::string{} + ... + CS); // a fold expression
    };
}
int main() {
    using namespace lits;
    std::cout << 10000000_sx << '\n';     // Output: 10000000
    std::cout << 10'000'000_sx << '\n';   // Output: 10'000'000
    std::cout << 0x00af_sx << '\n';       // Output: 0x00af
    std::cout << 0x0'c'0'a'f_sx << '\n';  // Output: 0x0'c'0'a'f
    std::cout << 007_sx << '\n';          // Output: 007
    std::cout << 0b01_sx << '\n';         // Output: 0b01
}
```

Godbolt Listing [lst-0103-godb.cpp](lst-0103-godb.cpp), [https://godbolt.org/z/cPoEjqzM7](https://godbolt.org/z/cPoEjqzM7):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/cPoEjqzM7 
#include <iostream>
#include <string>
namespace lits {
    // operator"" _sx
    template<char...CS> std::string operator"" _sx() {
        return (std::string{} + ... + CS); // a fold expression
    };
}
int main() {
    using namespace lits;
    std::cout << 10000000_sx << '\n';     // Output: 10000000
    std::cout << 10'000'000_sx << '\n';   // Output: 10'000'000
    std::cout << 0x00af_sx << '\n';       // Output: 0x00af
    std::cout << 0x0'c'0'a'f_sx << '\n';  // Output: 0x0'c'0'a'f
    std::cout << 007_sx << '\n';          // Output: 007
    std::cout << 0b01_sx << '\n';         // Output: 0b01
}
```

----------------


## Listing 23.42: Whether preprocessed or raw is better depends on the application.

Book listing [lst-0104-book.cpp](lst-0104-book.cpp):
```cpp
// https://godbolt.org/z/Yd4sser5j 
#include <complex>
// raw form
int operator"" _dual(const char*);
int answer = 101010_dual;    // decimal 42
// preprocessed form
std::complex<long double> operator"" _i(long double d) {
  return std::complex<long double>(0, d);
}
auto val = 3.14_i; // val = std::complex<long double>(0, 3.14)
```

Godbolt Listing [lst-0104-godb.cpp](lst-0104-godb.cpp), [https://godbolt.org/z/Yd4sser5j](https://godbolt.org/z/Yd4sser5j):
```cpp
//#(execute) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/Yd4sser5j 
#include <complex>
// raw form
int operator"" _dual(const char*);
int answer = 101010_dual;    // decimal 42
// preprocessed form
std::complex<long double> operator"" _i(long double d) {
  return std::complex<long double>(0, d);
}
auto val = 3.14_i; // val = std::complex<long double>(0, 3.14)
```

----------------

