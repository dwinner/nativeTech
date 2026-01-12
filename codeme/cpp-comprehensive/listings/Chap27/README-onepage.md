# Listings of Chap27.docx

This is the list of listings on one page.
You can also view a [linked summary](README.md).


## Listing 27.1: Returning the stream by reference allows chaining.

Book listing [lst-0003-book.cpp](lst-0003-book.cpp):
```cpp
// https://godbolt.org/z/WaEGdnWvq
#include <iostream> // cin, cout
int main() {
    int val1, val2;
    std::cout << "Please enter 2 int values: ";
    std::cin >> val1 >> val2;
    std::cout << val1 << " : " << val2 << std::endl;
}
```

Godbolt Listing [lst-0003-godb.cpp](lst-0003-godb.cpp), [https://godbolt.org/z/WaEGdnWvq](https://godbolt.org/z/WaEGdnWvq):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/WaEGdnWvq
#include <iostream> // cin, cout
int main() {
    int val1, val2;
    std::cout << "Please enter 2 int values: ";
    std::cin >> val1 >> val2;
    std::cout << val1 << " : " << val2 << std::endl;
}
```

----------------


## Listing 27.2: Unformatted input from streams.

Book listing [lst-0005-book.cpp](lst-0005-book.cpp):
```cpp
// https://godbolt.org/z/MaEzcccTG
#include <iostream>
using std::cout; using std::cin; using std::endl;
int main() {
    const unsigned int MAX = 10;
    char buffer[MAX] = {0};
    cout << "Input getline : ";
    cin.getline(buffer, MAX);
    cout << std::cin.gcount()
        << " characters were read\n";
    for(auto c : buffer) {
        if(c && c!='\0') cout.put(c);
    }
    cin.ignore(MAX, '\n');
    cout << "\nMake input (end with .) : ";
    char ch=0;
    while(cin.get(ch)) {
        if(ch == '.') break;
        cout.put(ch);
    }
    cout << "Input ended" << endl;
}
```

Godbolt Listing [lst-0005-godb.cpp](lst-0005-godb.cpp), [https://godbolt.org/z/MaEzcccTG](https://godbolt.org/z/MaEzcccTG):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/MaEzcccTG
#include <iostream>
using std::cout; using std::cin; using std::endl;
int main() {
    const unsigned int MAX = 10;
    char buffer[MAX] = {0};
    cout << "Input getline : ";
    cin.getline(buffer, MAX);
    cout << std::cin.gcount()
        << " characters were read\n";
    for(auto c : buffer) {
        if(c && c!='\0') cout.put(c);
    }
    cin.ignore(MAX, '\n');
    cout << "\nMake input (end with .) : ";
    char ch=0;
    while(cin.get(ch)) {
        if(ch == '.') break;
        cout.put(ch);
    }
    cout << "Input ended" << endl;
}
```

----------------


## Listing 27.3: State checks for streams.

Book listing [lst-0007-book.cpp](lst-0007-book.cpp):
```cpp
// https://godbolt.org/z/exKMv8646
#include <fstream>
#include <iostream>
using std::cout; using std::cin; using std::ofstream;
void checkIOstate(std::ios& stream) {
    if( stream.good() ) {
        cout << "All good\n";
    } else if( stream.bad() ) {
        cout << "Fatal error\n";
    } else if( stream.fail()) {
        cout << "I/O error\n";
        if( stream.eof()) {
            cout << "End of stream reached\n";
        }
    }
    stream.clear();
}
int main() {
    int val=0;
    cout << "Enter value: ";
    cin >> val;
    checkIOstate( cin );
    std::ifstream file;
    file.open("nonexistent.text");
    checkIOstate(file);
    std::fstream fstr;
    fstr.open("newFile.txt",
        ofstream::out | ofstream::in
        | ofstream::binary | ofstream::trunc);
    fstr << "Text in the file\n";
    fstr.seekp(std::ios_base::beg);
    char ch;
    while( fstr.good()) {
        fstr.get(ch);
        if(fstr.good()) cout.put(ch);
    }
    checkIOstate(fstr);
}
```

Godbolt Listing [lst-0007-godb.cpp](lst-0007-godb.cpp), [https://godbolt.org/z/exKMv8646](https://godbolt.org/z/exKMv8646):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/exKMv8646
#include <fstream>
#include <iostream>
using std::cout; using std::cin; using std::ofstream;
void checkIOstate(std::ios& stream) {
    if( stream.good() ) {
        cout << "All good\n";
    } else if( stream.bad() ) {
        cout << "Fatal error\n";
    } else if( stream.fail()) {
        cout << "I/O error\n";
        if( stream.eof()) {
            cout << "End of stream reached\n";
        }
    }
    stream.clear();
}
int main() {
    int val=0;
    cout << "Enter value: ";
    cin >> val;
    checkIOstate( cin );
    std::ifstream file;
    file.open("nonexistent.text");
    checkIOstate(file);
    std::fstream fstr;
    fstr.open("newFile.txt",
        ofstream::out | ofstream::in
        | ofstream::binary | ofstream::trunc);
    fstr << "Text in the file\n";
    fstr.seekp(std::ios_base::beg);
    char ch;
    while( fstr.good()) {
        fstr.get(ch);
        if(fstr.good()) cout.put(ch);
    }
    checkIOstate(fstr);
}
```

----------------


## Listing 27.4: Operator "bool" of streams.

Book listing [lst-0008-book.cpp](lst-0008-book.cpp):
```cpp
// https://godbolt.org/z/Mr6E3Ez48 
#include <iostream>
int main() {
    unsigned int val;
    std::cout << "Enter value: ";
    std::cin >> val;
    if( std::cin ) { // operator bool()
        /* ... */                               // Input correct
    } else {
        std::cout << "Error with std::cin\n";   // Error with input
    }
}
```

Godbolt Listing [lst-0008-godb.cpp](lst-0008-godb.cpp), [https://godbolt.org/z/Mr6E3Ez48](https://godbolt.org/z/Mr6E3Ez48):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/Mr6E3Ez48 
#include <iostream>
int main() {
    unsigned int val;
    std::cout << "Enter value: ";
    std::cin >> val;
    if( std::cin ) { // operator bool()
        /* ... */                               // Input correct
    } else {
        std::cout << "Error with std::cin\n";   // Error with input
    }
}
```

----------------


## Listing 27.5: Outputting a Boolean as text or a number.

Book listing [lst-0009-book.cpp](lst-0009-book.cpp):
```cpp
// https://godbolt.org/z/s1csa51z9 
#include <iostream>
#include <iomanip>
using std::cin; using std::cout; using std::endl;

void f(bool b) {
     cout << b << endl;                     // Output:  true
}

int main () {
    bool b=true;
    cout << std::boolalpha << b << endl;   // Output: true
    b=false;
    cout << b << endl;                     // Output: false
    f(true);
    cout << std::noboolalpha << b << endl; // Output: 0
    b=true;
    cout << b << endl;                     // Output: 1
}
```

Godbolt Listing [lst-0009-godb.cpp](lst-0009-godb.cpp), [https://godbolt.org/z/s1csa51z9](https://godbolt.org/z/s1csa51z9):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/s1csa51z9 
#include <iostream>
#include <iomanip>
using std::cin; using std::cout; using std::endl;

void f(bool b) {
     cout << b << endl;                     // Output:  true
}

int main () {
    bool b=true;
    cout << std::boolalpha << b << endl;   // Output: true
    b=false;
    cout << b << endl;                     // Output: false
    f(true);
    cout << std::noboolalpha << b << endl; // Output: 0
    b=true;
    cout << b << endl;                     // Output: 1
}
```

----------------


## Listing 27.6: Number formats in output.

Book listing [lst-0011-book.cpp](lst-0011-book.cpp):
```cpp
// https://godbolt.org/z/bEKaYdssE 
#include <iostream>
#include <ios>
using std::cout; using std::endl;
void f() {
    int val = 100;
    cout << val << endl;                // Output: 0x64
}
int main() {
    int val = 255;
    cout << std::showbase;
    cout << std::dec << val << endl;   // Output: 255
    cout << std::hex << val << endl;   // Output: 0xff
    f();
    cout << std::oct << val << endl;   // Output: 0377
    cout << val << std::endl;          // Output: 0377
}
```

Godbolt Listing [lst-0011-godb.cpp](lst-0011-godb.cpp), [https://godbolt.org/z/bEKaYdssE](https://godbolt.org/z/bEKaYdssE):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/bEKaYdssE 
#include <iostream>
#include <ios>
using std::cout; using std::endl;
void f() {
    int val = 100;
    cout << val << endl;                // Output: 0x64
}
int main() {
    int val = 255;
    cout << std::showbase;
    cout << std::dec << val << endl;   // Output: 255
    cout << std::hex << val << endl;   // Output: 0xff
    f();
    cout << std::oct << val << endl;   // Output: 0377
    cout << val << std::endl;          // Output: 0377
}
```

----------------


## Listing 27.7: Different ways to pad output.

Book listing [lst-0012-book.cpp](lst-0012-book.cpp):
```cpp
// https://godbolt.org/z/TMdzrjvhd 
#include <iostream>
#include <ios>        // left, right, internal
#include <iomanip>    // setw
using std::cout; using std::endl;
int main() {
    int val = -1000;
    cout << std::setw(10) << std::internal
         << val << endl;
    cout << std::setw(10) << std::left << val << endl;
    cout << std::setw(10) << std::right
         << val << endl;
}
```

Godbolt Listing [lst-0012-godb.cpp](lst-0012-godb.cpp), [https://godbolt.org/z/TMdzrjvhd](https://godbolt.org/z/TMdzrjvhd):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/TMdzrjvhd 
#include <iostream>
#include <ios>        // left, right, internal
#include <iomanip>    // setw
using std::cout; using std::endl;
int main() {
    int val = -1000;
    cout << std::setw(10) << std::internal
         << val << endl;
    cout << std::setw(10) << std::left << val << endl;
    cout << std::setw(10) << std::right
         << val << endl;
}
```

----------------


## Listing 27.8: Number formats in output.

Book listing [lst-0013-book.cpp](lst-0013-book.cpp):
```cpp
// https://godbolt.org/z/sYK8oT33x 
#include <iostream>
#include <ios>        // left, right, internal
#include <iomanip>    // setw

using std::cout; using std::endl;
int main() {
    double dval = 3.14159;
    std::ios_base::fmtflags ff(std::ios::scientific|std::ios::uppercase);
    cout << std::setiosflags(ff);
    cout << dval << endl;                           // Output: 3.141590E+00
    cout << std::resetiosflags(ff) << dval << endl; // Output: 3.14159
    cout << std::setprecision(3) << dval << endl;   // Output: 3.14
    cout << std::setw(10);
    cout << std::setfill( '*' ) << 1246 << endl;    // Output: ******1246
}
```

Godbolt Listing [lst-0013-godb.cpp](lst-0013-godb.cpp), [https://godbolt.org/z/sYK8oT33x](https://godbolt.org/z/sYK8oT33x):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/sYK8oT33x 
#include <iostream>
#include <ios>        // left, right, internal
#include <iomanip>    // setw

using std::cout; using std::endl;
int main() {
    double dval = 3.14159;
    std::ios_base::fmtflags ff(std::ios::scientific|std::ios::uppercase);
    cout << std::setiosflags(ff);
    cout << dval << endl;                           // Output: 3.141590E+00
    cout << std::resetiosflags(ff) << dval << endl; // Output: 3.14159
    cout << std::setprecision(3) << dval << endl;   // Output: 3.14
    cout << std::setw(10);
    cout << std::setfill( '*' ) << 1246 << endl;    // Output: ******1246
}
```

----------------


## Listing 27.9: Custom manipulators.

Book listing [lst-0014-book.cpp](lst-0014-book.cpp):
```cpp
// https://godbolt.org/z/z8zc3ocd5
#include <iostream>
#include <ios>        // left, right, internal
#include <iomanip>    // setw
using std::cout; using std::cin; using std::endl;
std::ostream& tabl(std::ostream& os) {
    os << '\t';
    return os;
}
std::istream& firstNum(std::istream& is) {
    char ch;
    is.get(ch);
    if( (ch >= '0') && (ch <= '9') ) {
        std::cin.putback(ch);
    }
    return is;
}
int main() {
    int val=0;
    cout << "Text1" << tabl << "Text2" << endl; // Output: Text1 (tab) Text2
    cout << "Make an input: ";
    cin >> firstNum >> val;
    cout << val << std::endl; // Output: 12345
}
```

Godbolt Listing [lst-0014-godb.cpp](lst-0014-godb.cpp), [https://godbolt.org/z/z8zc3ocd5](https://godbolt.org/z/z8zc3ocd5):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/z8zc3ocd5
#include <iostream>
#include <ios>        // left, right, internal
#include <iomanip>    // setw
using std::cout; using std::cin; using std::endl;
std::ostream& tabl(std::ostream& os) {
    os << '\t';
    return os;
}
std::istream& firstNum(std::istream& is) {
    char ch;
    is.get(ch);
    if( (ch >= '0') && (ch <= '9') ) {
        std::cin.putback(ch);
    }
    return is;
}
int main() {
    int val=0;
    cout << "Text1" << tabl << "Text2" << endl; // Output: Text1 (tab) Text2
    cout << "Make an input: ";
    cin >> firstNum >> val;
    cout << val << std::endl; // Output: 12345
}
```

----------------


## Listing 27.10: Manipulator as lambda.

Book listing [lst-0016-book.cpp](lst-0016-book.cpp):
```cpp
#include <iostream>
using std::cout; using std::endl;
int main() {
    auto ddash = [](auto &os) -> std::ostream& { return os << "--"; };
    cout << "Text1" << ddash << "Text2" << endl; // Output: Text1--Text2
}
```

----------------


## Listing 27.11: Manipulators with parameters.

Book listing [lst-0017-book.cpp](lst-0017-book.cpp):
```cpp
// https://godbolt.org/z/KKE6nWT49 
#include <iostream>
using std::cout; using std::endl;
class dendl { // Dots followed by newline
    int dendl_;
public:
    dendl(int n=1)
      : dendl_{n} {}
    std::ostream& operator()(std::ostream& os) const { // Functor
        for(int i=0; i<dendl_; ++i) os << '.';
        return os << '\n';
    }
};
std::ostream& operator<<( std::ostream& os, const dendl& elem) {
    return elem(os);
}
int main() {
    cout << "Text1" << dendl(4); // Output: Text1....
    cout << "Text2" << dendl(2); // Output: Text2..
    cout << "Text3" << dendl();  // Output: Text3.
}
```

Godbolt Listing [lst-0017-godb.cpp](lst-0017-godb.cpp), [https://godbolt.org/z/KKE6nWT49](https://godbolt.org/z/KKE6nWT49):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/KKE6nWT49 
#include <iostream>
using std::cout; using std::endl;
class dendl { // Dots followed by newline
    int dendl_;
public:
    dendl(int n=1)
      : dendl_{n} {}
    std::ostream& operator()(std::ostream& os) const { // Functor
        for(int i=0; i<dendl_; ++i) os << '.';
        return os << '\n';
    }
};
std::ostream& operator<<( std::ostream& os, const dendl& elem) {
    return elem(os);
}
int main() {
    cout << "Text1" << dendl(4); // Output: Text1....
    cout << "Text2" << dendl(2); // Output: Text2..
    cout << "Text3" << dendl();  // Output: Text3.
}
```

----------------


## Listing 27.12: Directly influencing format.

Book listing [lst-0019-book.cpp](lst-0019-book.cpp):
```cpp
// https://godbolt.org/z/csbPMYM19 
#include <iostream>
#include <ios> // hex, dec
using std::cout; using std::endl;
int main() {
    int val = 255;
    cout << std::showbase << std::hex << val << endl;   // Output: 0xff
    cout << std::noshowbase << std::dec << val << endl; // Output: 255
}
```

Godbolt Listing [lst-0019-godb.cpp](lst-0019-godb.cpp), [https://godbolt.org/z/csbPMYM19](https://godbolt.org/z/csbPMYM19):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/csbPMYM19 
#include <iostream>
#include <ios> // hex, dec
using std::cout; using std::endl;
int main() {
    int val = 255;
    cout << std::showbase << std::hex << val << endl;   // Output: 0xff
    cout << std::noshowbase << std::dec << val << endl; // Output: 255
}
```

----------------


## Listing 27.13: Influencing format with "setf" and "unsetf".

Book listing [lst-0020-book.cpp](lst-0020-book.cpp):
```cpp
// https://godbolt.org/z/1xzzMdjdM 
#include <iostream>
using std::cout; using std::endl;
int main() {
    int val = 255;
    cout.setf(std::ios_base::hex, std::ios_base::basefield);
    cout.setf(std::ios_base::showbase);
    cout << val << std::endl; // Output: 0xff
    cout.unsetf(std::ios_base::showbase);
    cout.setf(std::ios_base::dec, std::ios_base::basefield);
    cout << val << std::endl; // Output: 255
}
```

Godbolt Listing [lst-0020-godb.cpp](lst-0020-godb.cpp), [https://godbolt.org/z/1xzzMdjdM](https://godbolt.org/z/1xzzMdjdM):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/1xzzMdjdM 
#include <iostream>
using std::cout; using std::endl;
int main() {
    int val = 255;
    cout.setf(std::ios_base::hex, std::ios_base::basefield);
    cout.setf(std::ios_base::showbase);
    cout << val << std::endl; // Output: 0xff
    cout.unsetf(std::ios_base::showbase);
    cout.setf(std::ios_base::dec, std::ios_base::basefield);
    cout << val << std::endl; // Output: 255
}
```

----------------


## Listing 27.14: Saving and restoring flags.

Book listing [lst-0021-book.cpp](lst-0021-book.cpp):
```cpp
// https://godbolt.org/z/4MY1Yznv9 
#include <iostream>
using std::cout; using std::endl;
int main() {
    int val = 255;
    std::ios::fmtflags ff = std::cout.flags();
    cout.flags(std::ios::hex | std::ios::showbase);
    cout << val << endl;  // Output: 0xff
    cout.flags(ff);
    cout << val << endl;  // Output: 255
}
```

Godbolt Listing [lst-0021-godb.cpp](lst-0021-godb.cpp), [https://godbolt.org/z/4MY1Yznv9](https://godbolt.org/z/4MY1Yznv9):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/4MY1Yznv9 
#include <iostream>
using std::cout; using std::endl;
int main() {
    int val = 255;
    std::ios::fmtflags ff = std::cout.flags();
    cout.flags(std::ios::hex | std::ios::showbase);
    cout << val << endl;  // Output: 0xff
    cout.flags(ff);
    cout << val << endl;  // Output: 255
}
```

----------------


## Listing 27.15: Opening and creating files.

Book listing [lst-0022-book.cpp](lst-0022-book.cpp):
```cpp
// https://godbolt.org/z/o7YP7or8s
#include <fstream>
#include <iostream>
#include <string>
int main() {
    std::string name = "textfile.txt";
    std::ifstream file01;
    file01.open(name);
    if( file01.fail() ) {
        std::cout << "Could not open " << name << "\n";
    }
    std::ofstream file02("data.dat");
    if( file02.good() ) {
        std::cout << "data.dat opened or created\n";
    }
    std::fstream file03;
    file03.open("database.db");
    if( !file03 ) {
        std::cout << "Could not open database.db\n";
    }
}
```

Godbolt Listing [lst-0022-godb.cpp](lst-0022-godb.cpp), [https://godbolt.org/z/o7YP7or8s](https://godbolt.org/z/o7YP7or8s):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/o7YP7or8s
#include <fstream>
#include <iostream>
#include <string>
int main() {
    std::string name = "textfile.txt";
    std::ifstream file01;
    file01.open(name);
    if( file01.fail() ) {
        std::cout << "Could not open " << name << "\n";
    }
    std::ofstream file02("data.dat");
    if( file02.good() ) {
        std::cout << "data.dat opened or created\n";
    }
    std::fstream file03;
    file03.open("database.db");
    if( !file03 ) {
        std::cout << "Could not open database.db\n";
    }
}
```

----------------


## Listing 27.16: Additional flags when opening files.

Book listing [lst-0023-book.cpp](lst-0023-book.cpp):
```cpp
// https://godbolt.org/z/GoPddTWq6
#include <fstream>
#include <iostream>
using std::cout;
int main() {
    std::ofstream file01("testfile.txt", std::ios::out|std::ios::app);
    if(file01.fail()) {
        cout << "Could not open testfile.txt\n";
    } else {
        cout << "ok.\n";
    }
    std::fstream file02;
    file02.open("database.db", std::ios::out|std::ios::trunc);
    if( !file02 ) {
        cout << "Could not open database.db\n";
    } else {
        cout << "ok.\n";
    }
}
```

Godbolt Listing [lst-0023-godb.cpp](lst-0023-godb.cpp), [https://godbolt.org/z/GoPddTWq6](https://godbolt.org/z/GoPddTWq6):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/GoPddTWq6
#include <fstream>
#include <iostream>
using std::cout;
int main() {
    std::ofstream file01("testfile.txt", std::ios::out|std::ios::app);
    if(file01.fail()) {
        cout << "Could not open testfile.txt\n";
    } else {
        cout << "ok.\n";
    }
    std::fstream file02;
    file02.open("database.db", std::ios::out|std::ios::trunc);
    if( !file02 ) {
        cout << "Could not open database.db\n";
    } else {
        cout << "ok.\n";
    }
}
```

----------------


## Listing 27.17: Explicitly closing a file stream.

Book listing [lst-0024-book.cpp](lst-0024-book.cpp):
```cpp
// https://godbolt.org/z/TMzdWhE51
#include <fstream>
#include <iostream>
int main() {
    std::ofstream file01("data.db");
    if( file01.fail() ) {
        std::cout << "Could not open data.db\n";
    } else {
        std::cout << "ok.\n";
    }
    file01 << "Text for the file\n";
    if( file01.is_open()) {
        file01.close();
    }
    file01.open("data001.db");
    // Automatically:
    {
        std::ofstream file02("data002.db");
    } // from here file02 is closed
} // from here file01 is also closed
```

Godbolt Listing [lst-0024-godb.cpp](lst-0024-godb.cpp), [https://godbolt.org/z/TMzdWhE51](https://godbolt.org/z/TMzdWhE51):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/TMzdWhE51
#include <fstream>
#include <iostream>
int main() {
    std::ofstream file01("data.db");
    if( file01.fail() ) {
        std::cout << "Could not open data.db\n";
    } else {
        std::cout << "ok.\n";
    }
    file01 << "Text for the file\n";
    if( file01.is_open()) {
        file01.close();
    }
    file01.open("data001.db");
    // Automatically:
    {
        std::ofstream file02("data002.db");
    } // from here file02 is closed
} // from here file01 is also closed
```

----------------


## Listing 27.18: Reading and writing with files.

Book listing [lst-0025-book.cpp](lst-0025-book.cpp):
```cpp
// https://godbolt.org/z/3756v8rzK
#include <fstream>
#include <iomanip> // setw
#include <iostream>
int main() {
    std::ofstream file("data.dat");
    if( !file ) {
        std::cout << "Could not open data.dat\n";
        return 1;
    }
    file << std::setw(10) << std::setfill( '*' )
         << 1234 << std::endl;
}
```

Godbolt Listing [lst-0025-godb.cpp](lst-0025-godb.cpp), [https://godbolt.org/z/3756v8rzK](https://godbolt.org/z/3756v8rzK):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/3756v8rzK
#include <fstream>
#include <iomanip> // setw
#include <iostream>
int main() {
    std::ofstream file("data.dat");
    if( !file ) {
        std::cout << "Could not open data.dat\n";
        return 1;
    }
    file << std::setw(10) << std::setfill( '*' )
         << 1234 << std::endl;
}
```

----------------


## Listing 27.19: Byte-wise reading and writing.

Book listing [lst-0026-book.cpp](lst-0026-book.cpp):
```cpp
// https://godbolt.org/z/sjaGaq48v
#include <fstream>
#include <iomanip> // setw
#include <iostream>
int main() {
    std::ifstream file("data.dat");
    if( !file ) {
        std::cout << "Error opening file\n";
        return 1;
    }
    char ch;
    while(file.get(ch) ) {
        std::cout.put(ch);
    }
    if( file.eof() ) {
        file.clear();
    }
    file.close();
}
```

Godbolt Listing [lst-0026-godb.cpp](lst-0026-godb.cpp), [https://godbolt.org/z/sjaGaq48v](https://godbolt.org/z/sjaGaq48v):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/sjaGaq48v
#include <fstream>
#include <iomanip> // setw
#include <iostream>
int main() {
    std::ifstream file("data.dat");
    if( !file ) {
        std::cout << "Error opening file\n";
        return 1;
    }
    char ch;
    while(file.get(ch) ) {
        std::cout.put(ch);
    }
    if( file.eof() ) {
        file.clear();
    }
    file.close();
}
```

----------------


## Listing 27.20: Copying a file byte by byte.

Book listing [lst-0027-book.cpp](lst-0027-book.cpp):
```cpp
#include <fstream>
#include <iostream>
using std::cout;
int main() {
    std::ifstream file("data.dat");
    if( !file ) { /* Error */ cout << "ERR\n"; return 1; }
    std::ofstream filecopy("backup.dat");
    if( !filecopy ) { /* Error */ cout << "ERR\n"; return 1; }
    char ch;
    while(file.get(ch) ) {
        filecopy.put(ch);
    }
}
```

----------------


## Listing 27.21: Line-by-line reading and writing.

Book listing [lst-0028-book.cpp](lst-0028-book.cpp):
```cpp
// https://godbolt.org/z/hPaz3a3sE
#include <fstream>
#include <iostream>
using std::cout;
int main() {
    std::ifstream file("44fstream07.cpp");
    if( !file ) { /* Error */ cout << "ERR\n"; return 1; }
    std::ofstream filecopy("backup.cpp");
    if( !filecopy ) { /* Error */ cout << "ERR\n"; return 1; }
    std::string buffer;
    while( getline(file, buffer) ) {
        filecopy << buffer << std::endl;
        cout << buffer << std::endl;
    }
    if( file.eof() ) {
        file.clear();
    }
}
```

Godbolt Listing [lst-0028-godb.cpp](lst-0028-godb.cpp), [https://godbolt.org/z/hPaz3a3sE](https://godbolt.org/z/hPaz3a3sE):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/hPaz3a3sE
#include <fstream>
#include <iostream>
using std::cout;
int main() {
    std::ifstream file("44fstream07.cpp");
    if( !file ) { /* Error */ cout << "ERR\n"; return 1; }
    std::ofstream filecopy("backup.cpp");
    if( !filecopy ) { /* Error */ cout << "ERR\n"; return 1; }
    std::string buffer;
    while( getline(file, buffer) ) {
        filecopy << buffer << std::endl;
        cout << buffer << std::endl;
    }
    if( file.eof() ) {
        file.clear();
    }
}
```

----------------


## Listing 27.22: Block-wise reading and writing with "read" and "write".

Book listing [lst-0029-book.cpp](lst-0029-book.cpp):
```cpp
// https://godbolt.org/z/P6MPabsKc
#include <fstream>
#include <iostream>
#include <vector>
using std::cout;
int main() {
    std::ifstream file("testfile.txt", std::ios::binary);
    if( !file ) { /* Error */ cout <<"ERR1\n"; return 1; }
    std::ofstream filecopy("backup.dat", std::ios::binary);
    if( !filecopy ) { /* Error */ return 1; }
    file.seekg(0, std::ios::end);
    auto size = file.tellg();
    cout << "File size: " << size << " bytes\n";
    file.seekg(0, std::ios::beg); // Important!
    std::vector<char> buffer(size);
    file.read(buffer.data(), size);
    if( !file ) { cout << "Error during read...\n"; return 1;}
    cout << "Read: " << file.gcount() << " bytes\n";
    filecopy.write( buffer.data(), size );
    if( !filecopy ) { cout << "Error during write...\n"; return 1;}
}
```

Godbolt Listing [lst-0029-godb.cpp](lst-0029-godb.cpp), [https://godbolt.org/z/P6MPabsKc](https://godbolt.org/z/P6MPabsKc):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/P6MPabsKc
#include <fstream>
#include <iostream>
#include <vector>
using std::cout;
int main() {
    std::ifstream file("testfile.txt", std::ios::binary);
    if( !file ) { /* Error */ cout <<"ERR1\n"; return 1; }
    std::ofstream filecopy("backup.dat", std::ios::binary);
    if( !filecopy ) { /* Error */ return 1; }
    file.seekg(0, std::ios::end);
    auto size = file.tellg();
    cout << "File size: " << size << " bytes\n";
    file.seekg(0, std::ios::beg); // Important!
    std::vector<char> buffer(size);
    file.read(buffer.data(), size);
    if( !file ) { cout << "Error during read...\n"; return 1;}
    cout << "Read: " << file.gcount() << " bytes\n";
    filecopy.write( buffer.data(), size );
    if( !filecopy ) { cout << "Error during write...\n"; return 1;}
}
```

----------------


## Listing 27.23: Block-wise reading and writing with a helper class.

Book listing [lst-0030-book.cpp](lst-0030-book.cpp):
```cpp
// https://godbolt.org/z/b3W48s86z 
#include <fstream>
#include <iostream>
#include <string>
#include <vector>
using std::cout; using std::string;
class DataClass {
    std::string text_;
    int data_;
public:
    DataClass(string t="", int i=0)
      : text_{t}, data_{i} {}
    std::ostream& write(std::ostream& os) const {
        os << text_ << std::ends;
        os.write(reinterpret_cast<const char*>(&data_), sizeof(data_));
        return os;
    }
    std::istream& read(std::istream& is) {
        std::getline(is, text_, '\0');
        is.read(reinterpret_cast<char*>(&data_), sizeof(data_));
        return is;
    }
    std::ostream& print(std::ostream& os) {
        return os << text_ << " : " << data_ << std::endl;
    }
};
int main() {
    std::ofstream file_w("data.dat", std::ios::binary);
    if( !file_w) { cout << "Error opening file\n"; return 1; }
    std::vector<DataClass> vec_dat;
    vec_dat.push_back(DataClass("A text", 123));
    vec_dat.push_back(DataClass("More text", 321));
    vec_dat.emplace_back("Much more text", 333);
    for(const auto &elem : vec_dat){
        elem.write(file_w);
    }
    file_w.close();
    std::ifstream file_r("data.dat", std::ios::binary);
    if( !file_r) { cout << "Error opening file\n"; return 1; }
    DataClass dat_r;
    while( file_r ) {
        dat_r.read(file_r);
        if( file_r.eof()) break;
        dat_r.print(cout);
    }
}
```

Godbolt Listing [lst-0030-godb.cpp](lst-0030-godb.cpp), [https://godbolt.org/z/b3W48s86z](https://godbolt.org/z/b3W48s86z):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/b3W48s86z 
#include <fstream>
#include <iostream>
#include <string>
#include <vector>
using std::cout; using std::string;
class DataClass {
    std::string text_;
    int data_;
public:
    DataClass(string t="", int i=0)
      : text_{t}, data_{i} {}
    std::ostream& write(std::ostream& os) const {
        os << text_ << std::ends;
        os.write(reinterpret_cast<const char*>(&data_), sizeof(data_));
        return os;
    }
    std::istream& read(std::istream& is) {
        std::getline(is, text_, '\0');
        is.read(reinterpret_cast<char*>(&data_), sizeof(data_));
        return is;
    }
    std::ostream& print(std::ostream& os) {
        return os << text_ << " : " << data_ << std::endl;
    }
};
int main() {
    std::ofstream file_w("data.dat", std::ios::binary);
    if( !file_w) { cout << "Error opening file\n"; return 1; }
    std::vector<DataClass> vec_dat;
    vec_dat.push_back(DataClass("A text", 123));
    vec_dat.push_back(DataClass("More text", 321));
    vec_dat.emplace_back("Much more text", 333);
    for(const auto &elem : vec_dat){
        elem.write(file_w);
    }
    file_w.close();
    std::ifstream file_r("data.dat", std::ios::binary);
    if( !file_r) { cout << "Error opening file\n"; return 1; }
    DataClass dat_r;
    while( file_r ) {
        dat_r.read(file_r);
        if( file_r.eof()) break;
        dat_r.print(cout);
    }
}
```

----------------


## Listing 27.24: Synchronizing output between threads with "osyncstream".

Book listing [lst-0032-book.cpp](lst-0032-book.cpp):
```cpp
// https://godbolt.org/z/GefKP5sj4
#include <iostream>
#include <thread>
#include <syncstream>
long fib(long n) { return n<=1 ? n : fib(n-1)+fib(n-2); }
void runFib(long from, long step, long to) {
    for (auto n=from; n<=to; n+=step) {
        std::osyncstream osync{ std::cout }; // Sync on cout as long as osync exists
        osync << "fib("<<n<<")=" << fib(n) << '\n';
    }
}
int main() {
    std::jthread f40{ runFib, 1, 3, 40 };
    std::jthread f41{ runFib, 2, 3, 41 };
    std::jthread f42{ runFib, 3, 3, 42 };
}
```

Godbolt Listing [lst-0032-godb.cpp](lst-0032-godb.cpp), [https://godbolt.org/z/GefKP5sj4](https://godbolt.org/z/GefKP5sj4):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/GefKP5sj4
#include <iostream>
#include <thread>
#include <syncstream>
long fib(long n) { return n<=1 ? n : fib(n-1)+fib(n-2); }
void runFib(long from, long step, long to) {
    for (auto n=from; n<=to; n+=step) {
        std::osyncstream osync{ std::cout }; // Sync on cout as long as osync exists
        osync << "fib("<<n<<")=" << fib(n) << '\n';
    }
}
int main() {
    std::jthread f40{ runFib, 1, 3, 40 };
    std::jthread f41{ runFib, 2, 3, 41 };
    std::jthread f42{ runFib, 3, 3, 42 };
}
```

----------------


## Listing 27.25: Writing to a "stringstream".

Book listing [lst-0033-book.cpp](lst-0033-book.cpp):
```cpp
// https://godbolt.org/z/TT13E3sz5
#include <sstream>  // ostringstream
#include <iostream>
int main() {
    std::ostringstream ostr;
    double dval = 3.1415;
    int ival = 4321;
    ostr << dval << " : " << ival;
    const std::string sval = ostr.str();
    std::cout << sval << std::endl;    // Output: 3.1415 : 4321
}
```

Godbolt Listing [lst-0033-godb.cpp](lst-0033-godb.cpp), [https://godbolt.org/z/TT13E3sz5](https://godbolt.org/z/TT13E3sz5):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/TT13E3sz5
#include <sstream>  // ostringstream
#include <iostream>
int main() {
    std::ostringstream ostr;
    double dval = 3.1415;
    int ival = 4321;
    ostr << dval << " : " << ival;
    const std::string sval = ostr.str();
    std::cout << sval << std::endl;    // Output: 3.1415 : 4321
}
```

----------------


## Listing 27.26: Reading from a "stringstream".

Book listing [lst-0034-book.cpp](lst-0034-book.cpp):
```cpp
// https://godbolt.org/z/1EMox3hW5 
#include <sstream>  // istringstream
#include <iostream>
int main() {
    std::istringstream istr;
    std::string sval("3.1415 : 4321");
    std::string none;
    double dval=0.0;
    int ival=0;
    istr.str(sval);                 // initialize
    istr >> dval >> none >> ival;   // read
    if( ! istr.eof() ) {
        std::cout << "Error converting\n"; return 1;
    }
    std::cout << dval << " == " << none << " == " << ival << "\n";
    // Output: 3.1415 == : == 4321
}
```

Godbolt Listing [lst-0034-godb.cpp](lst-0034-godb.cpp), [https://godbolt.org/z/1EMox3hW5](https://godbolt.org/z/1EMox3hW5):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/1EMox3hW5 
#include <sstream>  // istringstream
#include <iostream>
int main() {
    std::istringstream istr;
    std::string sval("3.1415 : 4321");
    std::string none;
    double dval=0.0;
    int ival=0;
    istr.str(sval);                 // initialize
    istr >> dval >> none >> ival;   // read
    if( ! istr.eof() ) {
        std::cout << "Error converting\n"; return 1;
    }
    std::cout << dval << " == " << none << " == " << ival << "\n";
    // Output: 3.1415 == : == 4321
}
```

----------------


## Listing 27.27: Type conversion using "stringstream".

Book listing [lst-0035-book.cpp](lst-0035-book.cpp):
```cpp
// https://godbolt.org/z/vcoEcYYcx
#include <sstream> // stringstream
#include <iostream>
#include <stdexcept> // invalid_argument
template <class T1, class T2>
void myConvert(const T1& in, T2& out) {
    std::stringstream ss;
    ss << in;
    ss >> out;
    if( ! ss.eof() ) {
        throw std::invalid_argument("Error during conversion");
    }
}
int main() {
    std::string sval;
    float fval=3.1415f;
    std::string sdval("5.321");
    double dsval=0;
    std::string doesnotwork("does not work");
    try {
        myConvert(fval, sval);
        std::cout << sval << std::endl;  // Output: 3.1415
        myConvert(sdval, dsval);
        std::cout << dsval << std::endl; // Output: 5.321
        myConvert(doesnotwork, dsval);   // triggers "Error during conversion"
    }
    catch(const std::invalid_argument& e) {
        std::cout << e.what() << std::endl;
    }
}
```

Godbolt Listing [lst-0035-godb.cpp](lst-0035-godb.cpp), [https://godbolt.org/z/vcoEcYYcx](https://godbolt.org/z/vcoEcYYcx):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/vcoEcYYcx
#include <sstream> // stringstream
#include <iostream>
#include <stdexcept> // invalid_argument
template <class T1, class T2>
void myConvert(const T1& in, T2& out) {
    std::stringstream ss;
    ss << in;
    ss >> out;
    if( ! ss.eof() ) {
        throw std::invalid_argument("Error during conversion");
    }
}
int main() {
    std::string sval;
    float fval=3.1415f;
    std::string sdval("5.321");
    double dsval=0;
    std::string doesnotwork("does not work");
    try {
        myConvert(fval, sval);
        std::cout << sval << std::endl;  // Output: 3.1415
        myConvert(sdval, dsval);
        std::cout << dsval << std::endl; // Output: 5.321
        myConvert(doesnotwork, dsval);   // triggers "Error during conversion"
    }
    catch(const std::invalid_argument& e) {
        std::cout << e.what() << std::endl;
    }
}
```

----------------


## Listing 27.28: The "to_string" function.

Book listing [lst-0036-book.cpp](lst-0036-book.cpp):
```cpp
// https://godbolt.org/z/z6sYqM48e 
#include <iostream>
#include <string>
void show(double f) {
    std::cout << "os: " << f
       << "\t to_string: " << std::to_string(f) << "\n";
}
int main() {
    show(23.43);     // Output: os: 23.43    to_string: 23.430000
    show(1e-9);      // Output: os: 1e-09    to_string: 0.000000
    show(1e40);      // Output: os: 1e+40    to_string: 100…0752.000000
    show(1e-40);     // Output: os: 1e-40    to_string: 0.000000
    show(123456789); // Output: os: 1.23457e+08  to_string: 123456789.000000
}
```

Godbolt Listing [lst-0036-godb.cpp](lst-0036-godb.cpp), [https://godbolt.org/z/z6sYqM48e](https://godbolt.org/z/z6sYqM48e):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/z6sYqM48e 
#include <iostream>
#include <string>
void show(double f) {
    std::cout << "os: " << f
       << "\t to_string: " << std::to_string(f) << "\n";
}
int main() {
    show(23.43);     // Output: os: 23.43    to_string: 23.430000
    show(1e-9);      // Output: os: 1e-09    to_string: 0.000000
    show(1e40);      // Output: os: 1e+40    to_string: 100…0752.000000
    show(1e-40);     // Output: os: 1e-40    to_string: 0.000000
    show(123456789); // Output: os: 1.23457e+08  to_string: 123456789.000000
}
```

----------------


## Listing 27.29: Transferring data from the "rdbuf".

Book listing [lst-0039-book.cpp](lst-0039-book.cpp):
```cpp
// https://godbolt.org/z/8hK9Y4vhd
#include <fstream>
#include <iostream>
#include <memory> // unique_ptr
int main() {
    std::fstream file("27Streams.tex");          // Open file for reading
    auto bufptr = file.rdbuf();                  // std::streambuf*
    auto size = bufptr->pubseekoff(0, file.end); // std::streamsize
    bufptr->pubseekoff(0, file.beg);             // back to the beginning
    auto buffer = std::unique_ptr<char[]>(new char[size]); // allocate memory
    auto n = bufptr->sgetn(buffer.get(), size);  // transfer number of chars
    std::cout << "Characters read: " << n << "\n";
    std::cout.write(buffer.get(), size);         // Output char[]
}
```

Godbolt Listing [lst-0039-godb.cpp](lst-0039-godb.cpp), [https://godbolt.org/z/8hK9Y4vhd](https://godbolt.org/z/8hK9Y4vhd):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/8hK9Y4vhd
#include <fstream>
#include <iostream>
#include <memory> // unique_ptr
int main() {
    std::fstream file("27Streams.tex");          // Open file for reading
    auto bufptr = file.rdbuf();                  // std::streambuf*
    auto size = bufptr->pubseekoff(0, file.end); // std::streamsize
    bufptr->pubseekoff(0, file.beg);             // back to the beginning
    auto buffer = std::unique_ptr<char[]>(new char[size]); // allocate memory
    auto n = bufptr->sgetn(buffer.get(), size);  // transfer number of chars
    std::cout << "Characters read: " << n << "\n";
    std::cout.write(buffer.get(), size);         // Output char[]
}
```

----------------


## Listing 27.30: How to concatenate paths.

Book listing [lst-0041-book.cpp](lst-0041-book.cpp):
```cpp
// https://godbolt.org/z/jddKxj7fW
#include <filesystem>   // std::filesystem
#include <iostream>
namespace fs = std::filesystem; using std::cout; using std::endl;
int main() {
    // Path components
    fs::path root {"/"};
    fs::path dir {"var/www/"};
    fs::path index {"index.html"};
    // concatenate
    fs::path p = root / dir / index;     // operator/
    // output
    cout << "Name: " << p << endl;       // "/var/www/index.html"
    // decompose
    cout << "Parent: " << p.parent_path() << endl;   // "/var/www"
    cout << "Name: " << p.filename() << endl;        // "index.html"
    cout << "Extension: " << p.extension() << endl;  // ".html"
    // Information
    cout << std::boolalpha;
    cout << "Exists? " << fs::exists(p) << endl;
    cout << "Regular file? " << fs::is_regular_file(p) << endl;
}
```

Godbolt Listing [lst-0041-godb.cpp](lst-0041-godb.cpp), [https://godbolt.org/z/jddKxj7fW](https://godbolt.org/z/jddKxj7fW):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/jddKxj7fW
#include <filesystem>   // std::filesystem
#include <iostream>
namespace fs = std::filesystem; using std::cout; using std::endl;
int main() {
    // Path components
    fs::path root {"/"};
    fs::path dir {"var/www/"};
    fs::path index {"index.html"};
    // concatenate
    fs::path p = root / dir / index;     // operator/
    // output
    cout << "Name: " << p << endl;       // "/var/www/index.html"
    // decompose
    cout << "Parent: " << p.parent_path() << endl;   // "/var/www"
    cout << "Name: " << p.filename() << endl;        // "index.html"
    cout << "Extension: " << p.extension() << endl;  // ".html"
    // Information
    cout << std::boolalpha;
    cout << "Exists? " << fs::exists(p) << endl;
    cout << "Regular file? " << fs::is_regular_file(p) << endl;
}
```

----------------


## GodboltId:hTj94enjK

Book listing [lst-0042-book.cpp](lst-0042-book.cpp):
```cpp
// https://godbolt.org/z/hTj94enjK
#include <format>
#include <chrono>
#include <string>
#include <string_view>
#include <iostream>
using namespace std; using namespace std::literals;
void pr(string_view s) { cout << s << endl; }
double pi = 3.14159265359;

int main() {
  pr(format("Hello, {}!", "Reader"));              // simple C-string
  pr(format("Hello, {}!", "Author"s));             // simple string
  pr(format("You are {} years old.", 30));         // integers
  pr(format("That makes {:.2f} euros.", 19.9933)); // 2 decimal places
  pr(format("Scientific: {:e}", -44.876)); // results in "-4.487600e+01"
  pr(format("Binary of {} is {:b}.", 42, 42));     // binary without base
  pr(format("Hex of {} is {:#x}.", 73, 73));       // hexadecimal with base
  pr(format("Zero-padded: {:03}", 7));     // results in "007"
  pr(format("|{0:<10}|{1:^10}|{2:>10}|", "le", "mi", "ri"));
  // Alignment and index
  pr(format("{} {:.9}!", "Boa", "Constrictor")); // without index, truncate string
  using namespace std::chrono;                   // neat time specifications:
  pr(format("{}, {}", 2023y/11/5, minutes{20})); // Output: 2023-11-05, 20min
}
```

Godbolt Listing [lst-0042-godb.cpp](lst-0042-godb.cpp), [https://godbolt.org/z/hTj94enjK](https://godbolt.org/z/hTj94enjK):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/hTj94enjK
#include <format>
#include <chrono>
#include <string>
#include <string_view>
#include <iostream>
using namespace std; using namespace std::literals;
void pr(string_view s) { cout << s << endl; }
double pi = 3.14159265359;

int main() {
  pr(format("Hello, {}!", "Reader"));              // simple C-string
  pr(format("Hello, {}!", "Author"s));             // simple string
  pr(format("You are {} years old.", 30));         // integers
  pr(format("That makes {:.2f} euros.", 19.9933)); // 2 decimal places
  pr(format("Scientific: {:e}", -44.876)); // results in "-4.487600e+01"
  pr(format("Binary of {} is {:b}.", 42, 42));     // binary without base
  pr(format("Hex of {} is {:#x}.", 73, 73));       // hexadecimal with base
  pr(format("Zero-padded: {:03}", 7));     // results in "007"
  pr(format("|{0:<10}|{1:^10}|{2:>10}|", "le", "mi", "ri"));
  // Alignment and index
  pr(format("{} {:.9}!", "Boa", "Constrictor")); // without index, truncate string
  using namespace std::chrono;                   // neat time specifications:
  pr(format("{}, {}", 2023y/11/5, minutes{20})); // Output: 2023-11-05, 20min
}
```

----------------


## Listing 27.31: A formatter can delegate parsing and formatting.

Book listing [lst-0043-book.cpp](lst-0043-book.cpp):
```cpp
// https://godbolt.org/z/sdWsnTonG
#include <format>
#include <string>
#include <vector>
#include <iostream>
struct Elf {
  std::string name;
  int birth_year;
  std::string era;
  std::string folk;
};
template<> struct std::formatter<Elf> {
  std::formatter<std::string> sub_fmt;
  constexpr auto parse(std::format_parse_context& pctx) {
    return sub_fmt.parse(pctx); // returns iterator to '}'
  }
  auto format(const Elf& elf, std::format_context& fctx) const {
    std::string s = std::format("{}/{} ({} {})",
        elf.name, elf.folk, elf.birth_year, elf.era);
    return sub_fmt.format(s, fctx); // delegate formatting
  }
};
int main() {
  std::vector<Elf> elves{
    {"Feanor", 1169, "First Age", "Noldor"},
    {"Galadriel", 1362, "EZ", "Noldor"},
    {"Legolas", 87, "DZ", "Sindar"},
    {"Elrond", 532, "EZ", "Half-elf"},
    {"Elwe", 1050, "EZ", "Sindar"},
  };
  for (const auto& e : elves) {
    std::cout << std::format("Elf: {:>20}", e) << std::endl;
  }
}
```

Godbolt Listing [lst-0043-godb.cpp](lst-0043-godb.cpp), [https://godbolt.org/z/sdWsnTonG](https://godbolt.org/z/sdWsnTonG):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/sdWsnTonG
#include <format>
#include <string>
#include <vector>
#include <iostream>
struct Elf {
  std::string name;
  int birth_year;
  std::string era;
  std::string folk;
};
template<> struct std::formatter<Elf> {
  std::formatter<std::string> sub_fmt;
  constexpr auto parse(std::format_parse_context& pctx) {
    return sub_fmt.parse(pctx); // returns iterator to '}'
  }
  auto format(const Elf& elf, std::format_context& fctx) const {
    std::string s = std::format("{}/{} ({} {})",
        elf.name, elf.folk, elf.birth_year, elf.era);
    return sub_fmt.format(s, fctx); // delegate formatting
  }
};
int main() {
  std::vector<Elf> elves{
    {"Feanor", 1169, "First Age", "Noldor"},
    {"Galadriel", 1362, "EZ", "Noldor"},
    {"Legolas", 87, "DZ", "Sindar"},
    {"Elrond", 532, "EZ", "Half-elf"},
    {"Elwe", 1050, "EZ", "Sindar"},
  };
  for (const auto& e : elves) {
    std::cout << std::format("Elf: {:>20}", e) << std::endl;
  }
}
```

----------------


## Listing 27.32: A formatter can parse and output itself.

Book listing [lst-0044-book.cpp](lst-0044-book.cpp):
```cpp
// https://godbolt.org/z/WqjK31Gn6 
// 'Elf' as before…
template<> struct std::formatter<Elf> {
  std::string attribs;                  // Sequence of '%n', '%g', '%e', '%v', and others
  constexpr auto parse(std::format_parse_context& pctx) {
    auto it = std::ranges::find(pctx.begin(), pctx.end(), '}');// search for '}'
    attribs = std::string(pctx.begin(), it);                   // save everything
    return it;                                                 // points to '}'
  }
  auto format(const Elf& elf, std::format_context& fctx) const {
    auto out = fctx.out();                                     // into here
    for(auto n=0u; n<attribs.size()-1; ++n) {
      if(attribs[n] == '%') {             // instruction to output a member
        switch(attribs[++n]) {
          case 'n': out = std::format_to(out, "{}", elf.name); break;
          case 'g': out = std::format_to(out, "{}", elf.birth_year); break;
          case 'e': out = std::format_to(out, "{}", elf.era); break;
          case 'v': out = std::format_to(out, "{}", elf.folk); break;
          case '%': out = std::format_to(out, "%"); break;     // %% becomes %
        }
      } else {
        out = std::format_to(out, "{}", attribs[n]);           // everything else
      }
    }
    return out;                                                // points to the end
  }
};
int main() {
  Elf e{"Feanor", 1169, "EZ", "Nordor"};
  std::cout << std::format("{:Elf %n}", e) << std::endl;
  // Output: Elf Feanor
  std::cout << std::format("Elf {:%n, %v, born %g in the age %e}\n", e);
  // Output: Elf Feanor, Nordor, born 1169 in the age EZ
}
```

Godbolt Listing [lst-0044-godb.cpp](lst-0044-godb.cpp), [https://godbolt.org/z/WqjK31Gn6](https://godbolt.org/z/WqjK31Gn6):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/WqjK31Gn6 
// 'Elf' as before…
template<> struct std::formatter<Elf> {
  std::string attribs;                  // Sequence of '%n', '%g', '%e', '%v', and others
  constexpr auto parse(std::format_parse_context& pctx) {
    auto it = std::ranges::find(pctx.begin(), pctx.end(), '}');// search for '}'
    attribs = std::string(pctx.begin(), it);                   // save everything
    return it;                                                 // points to '}'
  }
  auto format(const Elf& elf, std::format_context& fctx) const {
    auto out = fctx.out();                                     // into here
    for(auto n=0u; n<attribs.size()-1; ++n) {
      if(attribs[n] == '%') {             // instruction to output a member
        switch(attribs[++n]) {
          case 'n': out = std::format_to(out, "{}", elf.name); break;
          case 'g': out = std::format_to(out, "{}", elf.birth_year); break;
          case 'e': out = std::format_to(out, "{}", elf.era); break;
          case 'v': out = std::format_to(out, "{}", elf.folk); break;
          case '%': out = std::format_to(out, "%"); break;     // %% becomes %
        }
      } else {
        out = std::format_to(out, "{}", attribs[n]);           // everything else
      }
    }
    return out;                                                // points to the end
  }
};
int main() {
  Elf e{"Feanor", 1169, "EZ", "Nordor"};
  std::cout << std::format("{:Elf %n}", e) << std::endl;
  // Output: Elf Feanor
  std::cout << std::format("Elf {:%n, %v, born %g in the age %e}\n", e);
  // Output: Elf Feanor, Nordor, born 1169 in the age EZ
}
```

----------------

