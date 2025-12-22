# Listings of Chap08.docx

This is the list of listings on one page.
You can also view a [linked summary](README.md).


## Listing 08.1: Calculates all prime numbers in a user-defined range.

Book listing [lst-0001-book.cpp](lst-0001-book.cpp):
```cpp
// https://godbolt.org/z/Y33Tzcf5T 
#include <iostream>                   // cout
#include <vector>                     // container vector
#include <string>                     // stoi
int inputUpTo(int argc, const char* argv[]) {
    /* Determine number */
    int upTo = 0;                     // introduce new variable
    if(argc<=1) {                     // if statement with then and else block
        std::cout << "Up to what number do you want to calculate prime numbers? ";
        if(!(std::cin >> upTo)) {     // check return value
            return -1;                // error in user input
        }
    } else {                          // else part of the if statement
        upTo = std::stoi(argv[1]);
    }
    return upTo;                      // return input
}
std::vector primes{2};                // new vector<int> with initialization
bool isPrime(int n) {
    /* primes must be sorted in ascending order */
    for(int factor : primes) {        // range-based for loop
        if(factor*factor > n)         // too large to be a divisor at all?
            return true;              // … then terminate inner loop early
        if(n%factor==0)               // is divisor?
            return false;             // … then exit
    }
    return true;                      // no divisor found
}
void calculatePrimesUpTo(int upTo) {
    /* Primes calculation */
    /* vector must contain {2} at this point */
    for(int n=3; n<upTo; n=n+2) {     // standard for loop
        if(isPrime(n)) {
            primes.push_back(n);      // is prime – mark as divisor and result
        }
    }
}
void outputPrimes() {
    for(int prime : primes) {         // range based, over all elements
        std::cout << prime << " ";
    }
    std::cout << " ";
}
int main(int argc, const char* argv[]) {
    int upTo = inputUpTo(argc, argv); // declares variable
    if(upTo < 2) { return 1; }        // exit main with nonokay value.
    calculatePrimesUpTo(upTo);
    outputPrimes();
    return 0;
}
```

Godbolt Listing [lst-0001-godb.cpp](lst-0001-godb.cpp), [https://godbolt.org/z/Y33Tzcf5T](https://godbolt.org/z/Y33Tzcf5T):
```cpp
//#(compile) c++; compiler:g141; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/Y33Tzcf5T 
#include <iostream>                   // cout
#include <vector>                     // container vector
#include <string>                     // stoi
int inputUpTo(int argc, const char* argv[]) {
    /* Determine number */
    int upTo = 0;                     // introduce new variable
    if(argc<=1) {                     // if statement with then and else block
        std::cout << "Up to what number do you want to calculate prime numbers? ";
        if(!(std::cin >> upTo)) {     // check return value
            return -1;                // error in user input
        }
    } else {                          // else part of the if statement
        upTo = std::stoi(argv[1]);
    }
    return upTo;                      // return input
}
std::vector primes{2};                // new vector<int> with initialization
bool isPrime(int n) {
    /* primes must be sorted in ascending order */
    for(int factor : primes) {        // range-based for loop
        if(factor*factor > n)         // too large to be a divisor at all?
            return true;              // … then terminate inner loop early
        if(n%factor==0)               // is divisor?
            return false;             // … then exit
    }
    return true;                      // no divisor found
}
void calculatePrimesUpTo(int upTo) {
    /* Primes calculation */
    /* vector must contain {2} at this point */
    for(int n=3; n<upTo; n=n+2) {     // standard for loop
        if(isPrime(n)) {
            primes.push_back(n);      // is prime – mark as divisor and result
        }
    }
}
void outputPrimes() {
    for(int prime : primes) {         // range based, over all elements
        std::cout << prime << " ";
    }
    std::cout << " ";
}
int main(int argc, const char* argv[]) {
    int upTo = inputUpTo(argc, argv); // declares variable
    if(upTo < 2) { return 1; }        // exit main with nonokay value.
    calculatePrimesUpTo(upTo);
    outputPrimes();
    return 0;
}
```

----------------


## Listing 08.2: This "for" refers to only one statement.

Book listing [lst-0002-book.cpp](lst-0002-book.cpp):
```cpp
for(int prime : primes)             // for followed by a single statement
    std::cout << prime << " ";
std::cout << " ";                   // not part of for anymore
```

----------------


## Listing 08.3: A statement block is enclosed in {  and }.

Book listing [lst-0003-book.cpp](lst-0003-book.cpp):
```cpp
for(int prime : primes) {         // Beginning of the block
    std::cout << prime;
    std::cout << " ";
}                                 // End of the block
std::cout << " ";
```

----------------


## Listing 08.4: Where statements are allowed, you can also create a block.

Book listing [lst-0004-book.cpp](lst-0004-book.cpp):
```cpp
if(number > 50) {                // outer block
    {                            // 1st inner block
        int result  = number*number;
        std::cout << "Square: " << result << std::endl;
    }
    {                            // 2nd inner block
        int result  = number+number;
        std::cout << "Doubled: " << result << std::endl;
    }
}
```

----------------


## Listing 08.5: Using "result" twice as a new variable in one block is not allowed.

Book listing [lst-0005-book.cpp](lst-0005-book.cpp):
```cpp
if(number > 50) {
    int result = number*number;        // Definition of result
    std::cout << "Square: " << result << std::endl;
    int result = number+number;        // (ERR) Error: result has already been defined
    std::cout << "Doubled: " << result << std::endl;
}
```

----------------


## Listing 08.6: Too many variables are also not good.

Book listing [lst-0006-book.cpp](lst-0006-book.cpp):
```cpp
if(number > 50) {
    int result1 = number*number;           // a result
    std::cout << "Square: " << result << std::endl;
    int result2 = number+number;           // another result
    std::cout << "Doubled: " << result << std::endl;
    int result3 = number+number+number;    // and another result
    // … many lines of code in between …
    // and here?
    // … even more lines of code …
}
```

----------------


## Listing 08.7: Empty statements everywhere.

Book listing [lst-0007-book.cpp](lst-0007-book.cpp):
```cpp
int main() {  ;;;                 // 3 empty statements
    int number = 12 ; ;           // 1 empty statement
    ; int q = number*number ;     // 1 empty statement
    if(q>50) {
        q = q - 50;
    } ;                           // 1 empty statement
    std::cout << q << std::endl;
}
```

----------------


## Listing 08.8: With auto, you can also initialize multiple variables at once.

Book listing [lst-0014-book.cpp](lst-0014-book.cpp):
```cpp
// https://godbolt.org/z/xWbqeeq3n
#include <iostream>
#include <tuple>   // make_tuple
auto mkTpl() {
    return std::make_tuple(2, 'b', 3.14);  // tuple<int,char,double>
}
struct Point {
    int x, y;
};
int main() {
    // Structured binding of a C-array
    int odd[5] = { 1,3,7,9,11 };
    auto [ one, two, three, four, five ]  = odd;
    // Structured binding of a tuple
    auto [ zwei, be, pi ]  = mkTpl();
    // Structured binding of a struct
    Point p0{  10, 15 };
    auto [ the_x, the_y ]  = p0;
}
```

Godbolt Listing [lst-0014-godb.cpp](lst-0014-godb.cpp), [https://godbolt.org/z/xWbqeeq3n](https://godbolt.org/z/xWbqeeq3n):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/xWbqeeq3n
#include <iostream>
#include <tuple>   // make_tuple
auto mkTpl() {
    return std::make_tuple(2, 'b', 3.14);  // tuple<int,char,double>
}
struct Point {
    int x, y;
};
int main() {
    // Structured binding of a C-array
    int odd[5] = { 1,3,7,9,11 };
    auto [ one, two, three, four, five ]  = odd;
    // Structured binding of a tuple
    auto [ zwei, be, pi ]  = mkTpl();
    // Structured binding of a struct
    Point p0{  10, 15 };
    auto [ the_x, the_y ]  = p0;
}
```

----------------


## GodboltId:3zo8GnbW3

Book listing [lst-0015-book.cpp](lst-0015-book.cpp):
```cpp
// https://godbolt.org/z/3zo8GnbW3
int main() {
    int odd[5] = { 1,3,7,9,11 };
    auto &[ one, two, three, four, five ] = odd;
    auto &[ zwei, be, pi ] = mkTpl();           // (ERR) no & binding to temp-values
    Point p0{  10, 15 };
    auto &[ the_x, the_y ] = p0;
}
```

Godbolt Listing [lst-0015-godb.cpp](lst-0015-godb.cpp), [https://godbolt.org/z/3zo8GnbW3](https://godbolt.org/z/3zo8GnbW3):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/3zo8GnbW3
int main() {
    int odd[5] = { 1,3,7,9,11 };
    auto &[ one, two, three, four, five ] = odd;
    auto &[ zwei, be, pi ] = mkTpl();           // (ERR) no & binding to temp-values
    Point p0{  10, 15 };
    auto &[ the_x, the_y ] = p0;
}
```

----------------


## Listing 08.9: These are all expressions used as statements.

Book listing [lst-0016-book.cpp](lst-0016-book.cpp):
```cpp
upTo = 99;                    // an assignment is an expression
upTo = upTo * 2 + 1;          // combining an assignment with a calculation
calculatePrimesUpTo(upTo);    // a function call is an expression
std::cout << "Friedrich III"; // the output operator
```

----------------


## Listing 08.10: These if statements contain initializers.

Book listing [lst-0023-book.cpp](lst-0023-book.cpp):
```cpp
// https://godbolt.org/z/d6PscKx4M 
#include <map>
#include <string>
#include <algorithm> // any_of
#include <iostream>  // cerr
std::map<int, std::string> m;
int main() {
    if(auto it = m.find(10); it != m.end()) { return it->second.size(); }
    if(char buf[10]={0}; std::fgets(buf, 10, stdin)) { m[0] += buf; }
    std::string s;
    if(auto keywords = {"if", "for", "while"};
        std::any_of(keywords.begin(), keywords.end(),
        [&s](const char* kw) { return s == kw; })) {
            std::cerr << "Error ";
   }
}
```

Godbolt Listing [lst-0023-godb.cpp](lst-0023-godb.cpp), [https://godbolt.org/z/d6PscKx4M](https://godbolt.org/z/d6PscKx4M):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/d6PscKx4M 
#include <map>
#include <string>
#include <algorithm> // any_of
#include <iostream>  // cerr
std::map<int, std::string> m;
int main() {
    if(auto it = m.find(10); it != m.end()) { return it->second.size(); }
    if(char buf[10]={0}; std::fgets(buf, 10, stdin)) { m[0] += buf; }
    std::string s;
    if(auto keywords = {"if", "for", "while"};
        std::any_of(keywords.begin(), keywords.end(),
        [&s](const char* kw) { return s == kw; })) {
            std::cerr << "Error ";
   }
}
```

----------------


## Listing 08.11: The compiler can evaluate the condition at compile time.

Book listing [lst-0024-book.cpp](lst-0024-book.cpp):
```cpp
// https://godbolt.org/z/Ycse4EWav 
#include <iostream> // cout

template<typename T>
void memory(T x) {
    if constexpr(sizeof(T) > 4) {
       std::cout << "Requires a lot of memory: " << x << "  ";
    }
}

constexpr auto DEBUG = true;
int main() {
    if constexpr(DEBUG) {
        std::cout << "Debug is on. ";
    }
    memory<long long>(44LL);
}
```

Godbolt Listing [lst-0024-godb.cpp](lst-0024-godb.cpp), [https://godbolt.org/z/Ycse4EWav](https://godbolt.org/z/Ycse4EWav):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/Ycse4EWav 
#include <iostream> // cout

template<typename T>
void memory(T x) {
    if constexpr(sizeof(T) > 4) {
       std::cout << "Requires a lot of memory: " << x << "  ";
    }
}

constexpr auto DEBUG = true;
int main() {
    if constexpr(DEBUG) {
        std::cout << "Debug is on. ";
    }
    memory<long long>(44LL);
}
```

----------------


## Listing 08.12: The loop runs 100 times.

Book listing [lst-0025-book.cpp](lst-0025-book.cpp):
```cpp
// https://godbolt.org/z/h466fYq4W 
#include <iostream>
int main() {
    /* Sum up from 1 to 100 */
    int sum = 0;
    int number = 1;
    while(number <= 100)      // condition
    {                         // block that is repeatedly executed
        sum += number;        // for the result
        number += 1;          // next number
    }                         // end of the repeated block
    std::cout << sum << std::endl;
}
```

Godbolt Listing [lst-0025-godb.cpp](lst-0025-godb.cpp), [https://godbolt.org/z/h466fYq4W](https://godbolt.org/z/h466fYq4W):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/h466fYq4W 
#include <iostream>
int main() {
    /* Sum up from 1 to 100 */
    int sum = 0;
    int number = 1;
    while(number <= 100)      // condition
    {                         // block that is repeatedly executed
        sum += number;        // for the result
        number += 1;          // next number
    }                         // end of the repeated block
    std::cout << sum << std::endl;
}
```

----------------


## Listing 08.13: The body of a "do-while" loop is executed at least once.

Book listing [lst-0027-book.cpp](lst-0027-book.cpp):
```cpp
// https://godbolt.org/z/6hh5KhExM 
#include <iostream>                // cin
#include <string>
int main() {
    std::string line;
    do {                             // execute getline at least once
       std::getline(std::cin, line);
       if(!std::cin) break;          // error or end of file
    } while(line != "quit");         // end on specific input
}
```

Godbolt Listing [lst-0027-godb.cpp](lst-0027-godb.cpp), [https://godbolt.org/z/6hh5KhExM](https://godbolt.org/z/6hh5KhExM):
```cpp
//#(execute) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/6hh5KhExM 
#include <iostream>                // cin
#include <string>
int main() {
    std::string line;
    do {                             // execute getline at least once
       std::getline(std::cin, line);
       if(!std::cin) break;          // error or end of file
    } while(line != "quit");         // end on specific input
}
```

----------------


## Listing 08.14: Sum with a "for" loop.

Book listing [lst-0028-book.cpp](lst-0028-book.cpp):
```cpp
// https://godbolt.org/z/7foYcMMaz 
#include <iostream>
int main() {
    /* Sum up from 1 to 100 */
    int sum = 0;
    for(int number=1; number <= 100; number+=1)  {  // compact form
        sum += number;                              // for the result
    }
    std::cout << sum << std::endl;
}
```

Godbolt Listing [lst-0028-godb.cpp](lst-0028-godb.cpp), [https://godbolt.org/z/7foYcMMaz](https://godbolt.org/z/7foYcMMaz):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/7foYcMMaz 
#include <iostream>
int main() {
    /* Sum up from 1 to 100 */
    int sum = 0;
    for(int number=1; number <= 100; number+=1)  {  // compact form
        sum += number;                              // for the result
    }
    std::cout << sum << std::endl;
}
```

----------------


## Listing 08.15: "for" loop with empty initialization part.

Book listing [lst-0029-book.cpp](lst-0029-book.cpp):
```cpp
// https://godbolt.org/z/185ha3q3f 
#include <iostream>
int main() {
    /* Sum up from 1 to 100 */
    int sum = 0;
    int number = 1;                            // Initialization before the loop
    for(  ; number <= 100; number=number+1) {  // empty initialization
        sum = sum + number;
    }
    std::cout << number  << std::endl;         // number now also exists outside
}
```

Godbolt Listing [lst-0029-godb.cpp](lst-0029-godb.cpp), [https://godbolt.org/z/185ha3q3f](https://godbolt.org/z/185ha3q3f):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/185ha3q3f 
#include <iostream>
int main() {
    /* Sum up from 1 to 100 */
    int sum = 0;
    int number = 1;                            // Initialization before the loop
    for(  ; number <= 100; number=number+1) {  // empty initialization
        sum = sum + number;
    }
    std::cout << number  << std::endl;         // number now also exists outside
}
```

----------------


## Listing 08.16: Loops without conditions must be terminated in some other way.

Book listing [lst-0030-book.cpp](lst-0030-book.cpp):
```cpp
int main() {
    for( ; ; )  {  // no init, no condition, no update – so forever
        /* ... User input */
        /* ... if user chooses Quit, end program */
        /* ... otherwise, perform calculation and output */
    }
}
```

----------------


## Listing 08.17: You can recognize the range-based "for" loop by the colon.

Book listing [lst-0031-book.cpp](lst-0031-book.cpp):
```cpp
// Excerpt
std::vector<int> primes{2};    // vector is a container, ready for a ranged for
    void isPrime(int n) {
    /* primes must be sorted in ascending order */
    for(int factor : prims) {  // range-based for loop
        if(factor*factor > n)
            return true;
        if(n%factor==0)
            return false;
    }
    return true;
}
// …

void outputPrimes() {
    for(int i=1; int prime : prims) {  // range-based for loop
        std::cout << i++ << ". Prime number: " << prime << " ";
    }
    std::cout << " ";
}
// …
```

----------------


## Listing 08.18: Each "case" covers one scenario, and nowhere was the "break" forgotten.

Book listing [lst-0032-book.cpp](lst-0032-book.cpp):
```cpp
// https://godbolt.org/z/Y4W11oW8b 
#include <string>
#include <vector>
#include <iostream>       // cout

void calculator(std::ostream& out, std::string input) {
    std::vector<int> stack {};
    for(char c : input) {
        if(c>='0' && c<='9') {
            stack.push_back( c-'0' ); // numeric value of the character
            continue;      // next loop iteration
        }
        int top = 0;
        int second = 0;
        switch(c)  {       // condition on character
        case '+':
            top = stack.back(); stack.pop_back();
            second = stack.back(); stack.pop_back();
            stack.push_back(second + top);
            break;
        case '-':
            top = stack.back(); stack.pop_back();
            second = stack.back(); stack.pop_back();
            stack.push_back(second - top);
            break;
        case '*':
            top = stack.back(); stack.pop_back();
            second = stack.back(); stack.pop_back();
            stack.push_back(second * top);
            break;
        case '=':
            for(int elem : stack) { out << elem; }
            out << " ";
            break;
        case ' ':
            break;
        default:
            out << " '" << c << "' I don't understand. ";
        } /* switch */
    } /* for c */
}
int main(int argc, const char* argv[]) {
    if(argc > 1) {
        calculator(std::cout, argv[1]);
    } else {
        // 3 + 4 * 5 + 6 with multiplication before addition results in 29
        calculator(std::cout, "345*+6+=");
    }
}
```

Godbolt Listing [lst-0032-godb.cpp](lst-0032-godb.cpp), [https://godbolt.org/z/Y4W11oW8b](https://godbolt.org/z/Y4W11oW8b):
```cpp
//#(compile) c++; compiler:g141; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/Y4W11oW8b 
#include <string>
#include <vector>
#include <iostream>       // cout

void calculator(std::ostream& out, std::string input) {
    std::vector<int> stack {};
    for(char c : input) {
        if(c>='0' && c<='9') {
            stack.push_back( c-'0' ); // numeric value of the character
            continue;      // next loop iteration
        }
        int top = 0;
        int second = 0;
        switch(c)  {       // condition on character
        case '+':
            top = stack.back(); stack.pop_back();
            second = stack.back(); stack.pop_back();
            stack.push_back(second + top);
            break;
        case '-':
            top = stack.back(); stack.pop_back();
            second = stack.back(); stack.pop_back();
            stack.push_back(second - top);
            break;
        case '*':
            top = stack.back(); stack.pop_back();
            second = stack.back(); stack.pop_back();
            stack.push_back(second * top);
            break;
        case '=':
            for(int elem : stack) { out << elem; }
            out << " ";
            break;
        case ' ':
            break;
        default:
            out << " '" << c << "' I don't understand. ";
        } /* switch */
    } /* for c */
}
int main(int argc, const char* argv[]) {
    if(argc > 1) {
        calculator(std::cout, argv[1]);
    } else {
        // 3 + 4 * 5 + 6 with multiplication before addition results in 29
        calculator(std::cout, "345*+6+=");
    }
}
```

----------------


## Listing 08.19: Very rarely are there "case" blocks without "break"—and even more rarely meaningful ones.

Book listing [lst-0033-book.cpp](lst-0033-book.cpp):
```cpp
// https://godbolt.org/z/jaMTcshEf 
#include <iostream>
#include <string>

using std::string; using std::cout;

void guessMonth(unsigned whatDayIsItToday) {
    switch(whatDayIsItToday) {
    /* missing break statements: fall-through intended */
    default:
        if(whatDayIsItToday>31) {
            cout << "You are cheating";
            break;
        }
    case 28:
    case 29:
        cout << "Feb ";
    case 30:
        cout << "Apr Jun Sep Nov ";
    case 31:
        cout << "Jan Mar May Jul Aug Oct Dec ";
    }

    cout << ". ";
}
int main() {
    guessMonth(31);  // if today were the 31st?
    // Output: Jan Mar May Jul Aug Oct Dec .
    guessMonth(30);  // what if it were the 30th?
    // Output: Apr Jun Sep Nov Jan Mar May Jul Aug Oct Dec .
    guessMonth(4);
    // Output: Feb Apr Jun Sep Nov Jan Mar May Jul Aug Oct Dec .
    guessMonth(77);
    // Output: You are cheating.
}
```

Godbolt Listing [lst-0033-godb.cpp](lst-0033-godb.cpp), [https://godbolt.org/z/jaMTcshEf](https://godbolt.org/z/jaMTcshEf):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/jaMTcshEf 
#include <iostream>
#include <string>

using std::string; using std::cout;

void guessMonth(unsigned whatDayIsItToday) {
    switch(whatDayIsItToday) {
    /* missing break statements: fall-through intended */
    default:
        if(whatDayIsItToday>31) {
            cout << "You are cheating";
            break;
        }
    case 28:
    case 29:
        cout << "Feb ";
    case 30:
        cout << "Apr Jun Sep Nov ";
    case 31:
        cout << "Jan Mar May Jul Aug Oct Dec ";
    }

    cout << ". ";
}
int main() {
    guessMonth(31);  // if today were the 31st?
    // Output: Jan Mar May Jul Aug Oct Dec .
    guessMonth(30);  // what if it were the 30th?
    // Output: Apr Jun Sep Nov Jan Mar May Jul Aug Oct Dec .
    guessMonth(4);
    // Output: Feb Apr Jun Sep Nov Jan Mar May Jul Aug Oct Dec .
    guessMonth(77);
    // Output: You are cheating.
}
```

----------------


## Listing 08.20: With break, you terminate a loop prematurely.

Book listing [lst-0034-book.cpp](lst-0034-book.cpp):
```cpp
// https://godbolt.org/z/75h9Y95vf 
#include <iostream>                 // cout
int main() {
    for(int x=1; x<20; x+=1) {      // outer for-loop
        for(int y=1; y<20; y+=1) {  // inner for-loop
            int prod = x*y;
            if(prod>=100) {
                break;              // exit inner y-loop
            }
            std::cout << prod << " ";
        } /* end for y */
        // destination of break
    } /* end for x */               // first actual line after break
    std::cout << " ";
}
```

Godbolt Listing [lst-0034-godb.cpp](lst-0034-godb.cpp), [https://godbolt.org/z/75h9Y95vf](https://godbolt.org/z/75h9Y95vf):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/75h9Y95vf 
#include <iostream>                 // cout
int main() {
    for(int x=1; x<20; x+=1) {      // outer for-loop
        for(int y=1; y<20; y+=1) {  // inner for-loop
            int prod = x*y;
            if(prod>=100) {
                break;              // exit inner y-loop
            }
            std::cout << prod << " ";
        } /* end for y */
        // destination of break
    } /* end for x */               // first actual line after break
    std::cout << " ";
}
```

----------------


## Listing 08.21: With "return", the current function is exited. If necessary, provide a value for the return.

Book listing [lst-0035-book.cpp](lst-0035-book.cpp):
```cpp
// https://godbolt.org/z/5ddxa9v1j 
#include <iostream>                      // cout
int min3(int x, int y, int z) {          // function returns an int
    if(x<y) {
        if(x<z) return x;
        else return z;
    } else if(y<z) {
        return y;
    }
    else return z;
}
void printMin(int x, int y, int z) {    // function returns nothing
    if(x<0 || y<0 || z<0) {
        std::cout << "Please only numbers greater than 0\n";
        return;
    }
    std::cout << min3(x,y,z) << "\n";
}                                        // no return here
int main() {
    printMin(3, -4, 8);
    printMin(6, 77, 4);
    return;                              // special return in main
}
```

Godbolt Listing [lst-0035-godb.cpp](lst-0035-godb.cpp), [https://godbolt.org/z/5ddxa9v1j](https://godbolt.org/z/5ddxa9v1j):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/5ddxa9v1j 
#include <iostream>                      // cout
int min3(int x, int y, int z) {          // function returns an int
    if(x<y) {
        if(x<z) return x;
        else return z;
    } else if(y<z) {
        return y;
    }
    else return z;
}
void printMin(int x, int y, int z) {    // function returns nothing
    if(x<0 || y<0 || z<0) {
        std::cout << "Please only numbers greater than 0\n";
        return;
    }
    std::cout << min3(x,y,z) << "\n";
}                                        // no return here
int main() {
    printMin(3, -4, 8);
    printMin(6, 77, 4);
    return;                              // special return in main
}
```

----------------


## Listing 08.22: Avoid goto statements.

Book listing [lst-0036-book.cpp](lst-0036-book.cpp):
```cpp
// https://godbolt.org/z/ohd5Pr8fb 
#include <iostream>
int main() {
    int idx = 4;
    goto more;                      // jump to label more
  print:                            // label for the next statement
    std::cout << idx << std::endl;
    idx = idx * 2;
  more:
    idx = idx + 3;
    if(idx < 20)
        goto print;                 // goto can also be used in an if statement
  end:                              // this is a label, but it is not used
    return 0;
}
```

Godbolt Listing [lst-0036-godb.cpp](lst-0036-godb.cpp), [https://godbolt.org/z/ohd5Pr8fb](https://godbolt.org/z/ohd5Pr8fb):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/ohd5Pr8fb 
#include <iostream>
int main() {
    int idx = 4;
    goto more;                      // jump to label more
  print:                            // label for the next statement
    std::cout << idx << std::endl;
    idx = idx * 2;
  more:
    idx = idx + 3;
    if(idx < 20)
        goto print;                 // goto can also be used in an if statement
  end:                              // this is a label, but it is not used
    return 0;
}
```

----------------


## Listing 08.23: One less function, but one label and two more "goto" statements.

Book listing [lst-0037-book.cpp](lst-0037-book.cpp):
```cpp
// https://godbolt.org/z/fnsWhn5T3 
// #includes, inputUpTo(), outputPrimes() and main() as before 
std::vector<int> primes{2};
void calculatePrimesUpTo(int upTo) {
    /* Primes calculation */
    /* vector must contain {2} at this point */
    for(int n=3; n<upTo; n=n+2) {
        for(int factor: primes) {
            if(factor*factor > n)
                goto prime;     // using goto, because a break ...
            if(n%factor==0)
                goto notPrime;  // ... doesn't work over two loops.
        }
      prime:  ;                 // Target of the jump before push_back
        primes.push_back(n);    // n is prime! Remember as divisor and result
      notPrime:  ;              // Target of the jump after push_back
    }
}
```

Godbolt Listing [lst-0037-godb.cpp](lst-0037-godb.cpp), [https://godbolt.org/z/fnsWhn5T3](https://godbolt.org/z/fnsWhn5T3):
```cpp
//#(compile) c++; compiler:g141; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/fnsWhn5T3 
// #includes, inputUpTo(), outputPrimes() and main() as before 
std::vector<int> primes{2};
void calculatePrimesUpTo(int upTo) {
    /* Primes calculation */
    /* vector must contain {2} at this point */
    for(int n=3; n<upTo; n=n+2) {
        for(int factor: primes) {
            if(factor*factor > n)
                goto prime;     // using goto, because a break ...
            if(n%factor==0)
                goto notPrime;  // ... doesn't work over two loops.
        }
      prime:  ;                 // Target of the jump before push_back
        primes.push_back(n);    // n is prime! Remember as divisor and result
      notPrime:  ;              // Target of the jump after push_back
    }
}
```

----------------


## Listing 08.24: As a preview, here is your first exception handling with "try" and "catch".

Book listing [lst-0038-book.cpp](lst-0038-book.cpp):
```cpp
// https://godbolt.org/z/jhPaEjh1r
#include <iostream>
int main() {
    try {                                        // beginning of the try block
        for(int n=1; ; n=n*2) {
            if (n > std::numeric_limits<int>::max()/2) { // check for coming overflow
                throw "There would be an overflow";
            }
        }
    }                                            // End of the try block
    catch(const char *error) {                   // if this error occurs, …
        std::cout << "Error: " << error << " ";  // … handle it this way
    }
}
```

Godbolt Listing [lst-0038-godb.cpp](lst-0038-godb.cpp), [https://godbolt.org/z/jhPaEjh1r](https://godbolt.org/z/jhPaEjh1r):
```cpp
//#(compile) c++; compiler:g141; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/jhPaEjh1r
#include <iostream>
int main() {
    try {                                        // beginning of the try block
        for(int n=1; ; n=n*2) {
            if (n > std::numeric_limits<int>::max()/2) { // check for coming overflow
                throw "There would be an overflow";
            }
        }
    }                                            // End of the try block
    catch(const char *error) {                   // if this error occurs, …
        std::cout << "Error: " << error << " ";  // … handle it this way
    }
}
```

----------------

