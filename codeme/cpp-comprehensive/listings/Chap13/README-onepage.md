# Listings of Chap13.docx

This is the list of listings on one page.
You can also view a [linked summary](README.md).


## Listing 13.1: You define a namespace with "namespace".

Book listing [lst-0001-book.cpp](lst-0001-book.cpp):
```cpp
// https://godbolt.org/z/qo6W3bh5d 
#include <string>
#include <iostream>                   // ostream, cout
namespace plant {
    class Tree {
        std::string name_;
    public:
        explicit Tree(const std::string_view name) : name_{name} {}
        void print(std::ostream& os) const { os << name_; }
    };
    std::ostream& operator<<(std::ostream& os, const Tree& arg)
        { arg.print(os); return os; }
    using ConiferTree = Tree;         // for future extensions …
    using BroadleafTree = Tree;       // … provide for
    namespace exampleNames {          // embedded namespace
        std::string oakName = "Oak";
        std::string beechName = "Beech";
        std::string firName = "Fir";
    } // end namespace exampleNames
} // end namespace plant

int main() {  // main must not be in a namespace
    using namespace plant::exampleNames; // make all example names available
    plant::ConiferTree fir{ firName };
    plant::BroadleafTree oak{ oakName };
    fir.print(std::cout); std::cout << "\n";
    using plant::operator<<;          // without it 'cout << oak' won't work
    std::cout << oak << "\n";
}
```

Godbolt Listing [lst-0001-godb.cpp](lst-0001-godb.cpp), [https://godbolt.org/z/qo6W3bh5d](https://godbolt.org/z/qo6W3bh5d):
```cpp
//#(compile) c++; compiler:g141; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/qo6W3bh5d 
#include <string>
#include <iostream>                   // ostream, cout
namespace plant {
    class Tree {
        std::string name_;
    public:
        explicit Tree(const std::string_view name) : name_{name} {}
        void print(std::ostream& os) const { os << name_; }
    };
    std::ostream& operator<<(std::ostream& os, const Tree& arg)
        { arg.print(os); return os; }
    using ConiferTree = Tree;         // for future extensions …
    using BroadleafTree = Tree;       // … provide for
    namespace exampleNames {          // embedded namespace
        std::string oakName = "Oak";
        std::string beechName = "Beech";
        std::string firName = "Fir";
    } // end namespace exampleNames
} // end namespace plant

int main() {  // main must not be in a namespace
    using namespace plant::exampleNames; // make all example names available
    plant::ConiferTree fir{ firName };
    plant::BroadleafTree oak{ oakName };
    fir.print(std::cout); std::cout << "\n";
    using plant::operator<<;          // without it 'cout << oak' won't work
    std::cout << oak << "\n";
}
```

----------------


## Listing 13.2: In separate namespaces, you can define the same operators.

Book listing [lst-0005-book.cpp](lst-0005-book.cpp):
```cpp
namespace plant {
    // … as before …
    std::ostream& operator<<(std::ostream&, const Tree&) {…};
    namespace debug {
        std::ostream& operator<<(std::ostream&, const Tree&) {…};
    }
}
plant::Tree tree{"MyTree"};
void run() {
    using namespace plant;
    cout << tree << "\n";
}
void diagnostic() {
    using namespace plant::debug;
    cout << tree << "\n";
}
int main() {
    run();
    diagnostic();
}
```

----------------


## Listing 13.3: An anonymous namespace makes definitions local to the current file.

Book listing [lst-0006-book.cpp](lst-0006-book.cpp):
```cpp
// https://godbolt.org/z/e3d55zvT8 
// modul.hpp
#include <string>
#include <iostream>
namespace plant {
    class Tree {
        std::string name_;
    public:
        explicit Tree(const std::string_view name);
        void print(std::ostream& os) const;
    };
    std::ostream& operator<<(std::ostream& os, const Tree& arg);
}
// modul.cpp
#include "modul.hpp"
namespace {  // anonymous namespace
    std::string PREFIX = "TREE:";
    void printInfo(std::ostream& os) {
        os << "Author: Torsten T. Will\n";
    }
}
bool debug = false;  // global, no namespace
namespace plant {
    Tree::Tree(const std::string_view name)
        : name_{name} {}
    void Tree::print(std::ostream& os) const {
        os << PREFIX << name_;
    }
    std::ostream& operator<<(std::ostream& os, const Tree& arg) {
        if(debug) printInfo(os);
        arg.print(os); return os;
    }
}
// main.cpp
#include "modul.hpp"
int main() {
    plant::Tree x{"x"};
    x.print(std::cout); std::cout << "\n";
}
```

Godbolt Listing [lst-0006-godb.cpp](lst-0006-godb.cpp), [https://godbolt.org/z/e3d55zvT8](https://godbolt.org/z/e3d55zvT8):
```cpp
//#(compile) c++; compiler:g141; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/e3d55zvT8 
// modul.hpp
#include <string>
#include <iostream>
namespace plant {
    class Tree {
        std::string name_;
    public:
        explicit Tree(const std::string_view name);
        void print(std::ostream& os) const;
    };
    std::ostream& operator<<(std::ostream& os, const Tree& arg);
}
// modul.cpp
#include "modul.hpp"
namespace {  // anonymous namespace
    std::string PREFIX = "TREE:";
    void printInfo(std::ostream& os) {
        os << "Author: Torsten T. Will\n";
    }
}
bool debug = false;  // global, no namespace
namespace plant {
    Tree::Tree(const std::string_view name)
        : name_{name} {}
    void Tree::print(std::ostream& os) const {
        os << PREFIX << name_;
    }
    std::ostream& operator<<(std::ostream& os, const Tree& arg) {
        if(debug) printInfo(os);
        arg.print(os); return os;
    }
}
// main.cpp
#include "modul.hpp"
int main() {
    plant::Tree x{"x"};
    x.print(std::cout); std::cout << "\n";
}
```

----------------


## GodboltId:zfYzPjqrW

Book listing [lst-0007-book.cpp](lst-0007-book.cpp):
```cpp
// https://godbolt.org/z/zfYzPjqrW
// modul.cpp
#include "modul.hpp"
static std::string PREFIX = "TREE:";
static void printInfo(std::ostream& os) {
    os << "Author: Torsten T. Will\n";
}
bool debug = false;
// rest as before
```

Godbolt Listing [lst-0007-godb.cpp](lst-0007-godb.cpp), [https://godbolt.org/z/zfYzPjqrW](https://godbolt.org/z/zfYzPjqrW):
```cpp
//#(compile) c++; compiler:g141; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/zfYzPjqrW
// modul.cpp
#include "modul.hpp"
static std::string PREFIX = "TREE:";
static void printInfo(std::ostream& os) {
    os << "Author: Torsten T. Will\n";
}
bool debug = false;
// rest as before
```

----------------


## Listing 13.4: All instances share their "static" data fields and methods.

Book listing [lst-0009-book.cpp](lst-0009-book.cpp):
```cpp
// https://godbolt.org/z/n4e8xdEGj 
#include <iostream> // cout
#include <string>
using std::string;
class Tree {
    static size_t countConstructed_;
    static size_t countDestructed_;
    string kind_;
    Tree(string kind) : kind_{kind}      // private constructor
        { ++countConstructed_; }
public:
    Tree(const Tree& o) : kind_{o.kind_}
        { ++countConstructed_; }
    string getKind() const { return kind_; }
    ~Tree() { ++countDestructed_; }
    static Tree create(string kind) { return Tree{kind}; }
    static void stats(std::ostream& os) {
        os << "Constructed:+" << countConstructed_
           << " Destructed:-" << countDestructed_ << "\n";
    }
};
size_t Tree::countConstructed_ = 0;
size_t Tree::countDestructed_ = 0;
int main() {
    Tree birch = Tree::create("Birch");
    for(auto kind : {"Ash", "Yew", "Oak"}) {
        Tree temp = Tree::create(kind);
        std::cout << temp.getKind() << "\n";
    }
    Tree::stats(std::cout);
}
```

Godbolt Listing [lst-0009-godb.cpp](lst-0009-godb.cpp), [https://godbolt.org/z/n4e8xdEGj](https://godbolt.org/z/n4e8xdEGj):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/n4e8xdEGj 
#include <iostream> // cout
#include <string>
using std::string;
class Tree {
    static size_t countConstructed_;
    static size_t countDestructed_;
    string kind_;
    Tree(string kind) : kind_{kind}      // private constructor
        { ++countConstructed_; }
public:
    Tree(const Tree& o) : kind_{o.kind_}
        { ++countConstructed_; }
    string getKind() const { return kind_; }
    ~Tree() { ++countDestructed_; }
    static Tree create(string kind) { return Tree{kind}; }
    static void stats(std::ostream& os) {
        os << "Constructed:+" << countConstructed_
           << " Destructed:-" << countDestructed_ << "\n";
    }
};
size_t Tree::countConstructed_ = 0;
size_t Tree::countDestructed_ = 0;
int main() {
    Tree birch = Tree::create("Birch");
    for(auto kind : {"Ash", "Yew", "Oak"}) {
        Tree temp = Tree::create(kind);
        std::cout << temp.getKind() << "\n";
    }
    Tree::stats(std::cout);
}
```

----------------


## GodboltId:b9arcEMM1

Book listing [lst-0011-book.cpp](lst-0011-book.cpp):
```cpp
// https://godbolt.org/z/b9arcEMM1 
constinit auto SZ = 10*1000-1;                  // global variable
size_t autoincrement() {
    static constinit size_t i = 0;              // local static variable
    return i++;
}
class BraitenbergVehicle {
    inline static constinit size_t count_ = 0; // class variable
public:
    size_t id_;
    BraitenbergVehicle() : id_{++count_} {}
    ~BraitenbergVehicle() { --count_; }
};
```

Godbolt Listing [lst-0011-godb.cpp](lst-0011-godb.cpp), [https://godbolt.org/z/b9arcEMM1](https://godbolt.org/z/b9arcEMM1):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/b9arcEMM1 
constinit auto SZ = 10*1000-1;                  // global variable
size_t autoincrement() {
    static constinit size_t i = 0;              // local static variable
    return i++;
}
class BraitenbergVehicle {
    inline static constinit size_t count_ = 0; // class variable
public:
    size_t id_;
    BraitenbergVehicle() : id_{++count_} {}
    ~BraitenbergVehicle() { --count_; }
};
```

----------------


## Listing 13.5: A local static variable is initialized once and reused thereafter.

Book listing [lst-0012-book.cpp](lst-0012-book.cpp):
```cpp
// https://godbolt.org/z/h5qond6db 
#include <iostream>                     // cout
class Keyboard {
    Keyboard(const Keyboard&) = delete; // no copy
    const size_t nr_;                   // current number
public:
    static inline size_t count_ = 0;    // counts created instances
    explicit Keyboard() : nr_{count_++} {
        std::cout << "  Keyboard().nr:"<<nr_<<"\n";
    }
};
Keyboard& getKeyboard() {
    std::cout << "  getKeyboard()\n";
    static Keyboard keyboard{};         // static local variable
    return keyboard;
}
void func() {
    std::cout << "kbFunc...\n";
    Keyboard& kbFunc = getKeyboard();
}
int main() {
    std::cout << "kbA...\n";
    Keyboard& kbA = getKeyboard();
    func();
    std::cout << "kbB...\n";
    Keyboard& kbB = getKeyboard();
    std::cout << "count:" << Keyboard::count_ << "\n";
}
```

Godbolt Listing [lst-0012-godb.cpp](lst-0012-godb.cpp), [https://godbolt.org/z/h5qond6db](https://godbolt.org/z/h5qond6db):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/h5qond6db 
#include <iostream>                     // cout
class Keyboard {
    Keyboard(const Keyboard&) = delete; // no copy
    const size_t nr_;                   // current number
public:
    static inline size_t count_ = 0;    // counts created instances
    explicit Keyboard() : nr_{count_++} {
        std::cout << "  Keyboard().nr:"<<nr_<<"\n";
    }
};
Keyboard& getKeyboard() {
    std::cout << "  getKeyboard()\n";
    static Keyboard keyboard{};         // static local variable
    return keyboard;
}
void func() {
    std::cout << "kbFunc...\n";
    Keyboard& kbFunc = getKeyboard();
}
int main() {
    std::cout << "kbA...\n";
    Keyboard& kbA = getKeyboard();
    func();
    std::cout << "kbB...\n";
    Keyboard& kbB = getKeyboard();
    std::cout << "count:" << Keyboard::count_ << "\n";
}
```

----------------


## Listing 13.6: The Meyers Singleton.

Book listing [lst-0013-book.cpp](lst-0013-book.cpp):
```cpp
Keyboard& getKeyboard() {
    cout << "  getKeyboard()\n”;
    static Keyboard keyboard{}; // static local variable
    return keyboard;
}
```

----------------


## Listing 13.7: The identifiers of an inline namespace also go into its surrounding namespace.

Book listing [lst-0014-book.cpp](lst-0014-book.cpp):
```cpp
// https://godbolt.org/z/115M1csEE 
#include <iostream>
namespace mylib {
    namespace v1 {
        int version() { return 1; }
    }
    inline namespace v2 { // current version
        int version() { return 2; }
    }
}

int main() {
    std::cout << "Version " << mylib::version() << "\n”;     // Output: 2
    std::cout << "Version " << mylib::v1::version() << "\n”; // Output: 1
    std::cout << "Version " << mylib::v2::version() << "\n”; // Output: 2
}
```

Godbolt Listing [lst-0014-godb.cpp](lst-0014-godb.cpp), [https://godbolt.org/z/115M1csEE](https://godbolt.org/z/115M1csEE):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/115M1csEE 
#include <iostream>
namespace mylib {
    namespace v1 {
        int version() { return 1; }
    }
    inline namespace v2 { // current version
        int version() { return 2; }
    }
}

int main() {
    std::cout << "Version " << mylib::version() << "\n”;     // Output: 2
    std::cout << "Version " << mylib::v1::version() << "\n”; // Output: 1
    std::cout << "Version " << mylib::v2::version() << "\n”; // Output: 2
}
```

----------------


## GodboltId:WTWxYa3rd

Book listing [lst-0024-book.cpp](lst-0024-book.cpp):
```cpp
// https://godbolt.org/z/WTWxYa3rd 
class Widget {
    unsigned x = 0, y = 0, w = 0, h = 0; // for example
public:
    unsigned getLeft() const;
    unsigned getTop() const;
    unsigned getRight() const;
    unsigned getBottom() const;
    void setWidth(unsigned w);
    void setHeight(unsigned h);
};
```

Godbolt Listing [lst-0024-godb.cpp](lst-0024-godb.cpp), [https://godbolt.org/z/WTWxYa3rd](https://godbolt.org/z/WTWxYa3rd):
```cpp
//#(execute) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/WTWxYa3rd 
class Widget {
    unsigned x = 0, y = 0, w = 0, h = 0; // for example
public:
    unsigned getLeft() const;
    unsigned getTop() const;
    unsigned getRight() const;
    unsigned getBottom() const;
    void setWidth(unsigned w);
    void setHeight(unsigned h);
};
```

----------------


## Listing 13.8: Local constants of a file fit well into an anonymous namespace.

Book listing [lst-0029-book.cpp](lst-0029-book.cpp):
```cpp
// https://godbolt.org/z/bxPaz9aW5
#include <vector>
namespace {                         // anonymous namespace for constants
    const unsigned DATA_SIZE = 100; /* number of elements in data */
    const double LIMIT = 999.999;   /* max value during initialization */
};
std::vector<int> createData() {
    std::vector<int> result(DATA_SIZE);
    double currVal = 1.0;
    for(auto &elem : result) {
        elem = currVal;
        currVal *= 2;          // next value is larger
        if(currVal > LIMIT) {
            currVal = LIMIT;   // no value should be larger
        }
    }
    return result;
}
```

Godbolt Listing [lst-0029-godb.cpp](lst-0029-godb.cpp), [https://godbolt.org/z/bxPaz9aW5](https://godbolt.org/z/bxPaz9aW5):
```cpp
//#(compile) c++; compiler:g141; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/bxPaz9aW5
#include <vector>
namespace {                         // anonymous namespace for constants
    const unsigned DATA_SIZE = 100; /* number of elements in data */
    const double LIMIT = 999.999;   /* max value during initialization */
};
std::vector<int> createData() {
    std::vector<int> result(DATA_SIZE);
    double currVal = 1.0;
    for(auto &elem : result) {
        elem = currVal;
        currVal *= 2;          // next value is larger
        if(currVal > LIMIT) {
            currVal = LIMIT;   // no value should be larger
        }
    }
    return result;
}
```

----------------


## GodboltId:fM4nsx9GP

Book listing [lst-0030-book.cpp](lst-0030-book.cpp):
```cpp
// https://godbolt.org/z/fM4nsx9GP 
struct Widget {
    int num_ = 0;
    void setNumber(int x) {    // a non-const method
        num_=x;
    }
};
Widget createWidget() {        // Return by value
    Widget result{};            // Create
    return result;
}
int main() {
    Widget w = createWidget(); // Return by value creates a copy
    w.setNumber(100);          // changing is naturally okay, w is non-const
}
```

Godbolt Listing [lst-0030-godb.cpp](lst-0030-godb.cpp), [https://godbolt.org/z/fM4nsx9GP](https://godbolt.org/z/fM4nsx9GP):
```cpp
//#(compile) c++; compiler:g141; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/fM4nsx9GP 
struct Widget {
    int num_ = 0;
    void setNumber(int x) {    // a non-const method
        num_=x;
    }
};
Widget createWidget() {        // Return by value
    Widget result{};            // Create
    return result;
}
int main() {
    Widget w = createWidget(); // Return by value creates a copy
    w.setNumber(100);          // changing is naturally okay, w is non-const
}
```

----------------


## Listing 13.9: Although the return type is marked with 'const' here, it has no effect because it is always copied.

Book listing [lst-0031-book.cpp](lst-0031-book.cpp):
```cpp
// https://godbolt.org/z/98x3YsnKc
const Widget createWidget() {  // return as const value
    Widget result{};
    return result;
}
int main() {
    Widget w = createWidget();  // copied into new non-const w
    w.setNumber(100);           // w is non-const, changing is okay
}
```

Godbolt Listing [lst-0031-godb.cpp](lst-0031-godb.cpp), [https://godbolt.org/z/98x3YsnKc](https://godbolt.org/z/98x3YsnKc):
```cpp
//#(compile) c++; compiler:g141; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/98x3YsnKc
const Widget createWidget() {  // return as const value
    Widget result{};
    return result;
}
int main() {
    Widget w = createWidget();  // copied into new non-const w
    w.setNumber(100);           // w is non-const, changing is okay
}
```

----------------


## Listing 13.10: Constant references in returns.

Book listing [lst-0032-book.cpp](lst-0032-book.cpp):
```cpp
// https://godbolt.org/z/3EG1ovcev
#include <string>
#include <string_view>
using std::string; using std::string_view;
class Widget {
    string name_ = "";
public:
    void setName(string_view newName) {
        name_ = newName;
    }
    const string& getName() const {    // const& return
        return name_;
    }
};
int main() {
    Widget w{};
    w.setName("Title");
    string name1 = w.getName();        // new string, thus a copy
    name1.clear();                     // you are allowed to modify the copy again
    const string& name2 = w.getName(); // const reference to internal string name_
    /* name2.clear(); */               // name2 is const, so it doesn't work
    string& name3 = w.getName();       // (ERR) Function returns const&, not &.
    auto name4 = w.getName();          // identical to name1
    const auto& name5 = w.getName();   // identical to name2
}
```

Godbolt Listing [lst-0032-godb.cpp](lst-0032-godb.cpp), [https://godbolt.org/z/3EG1ovcev](https://godbolt.org/z/3EG1ovcev):
```cpp
//#(compile) c++; compiler:g141; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/3EG1ovcev
#include <string>
#include <string_view>
using std::string; using std::string_view;
class Widget {
    string name_ = "";
public:
    void setName(string_view newName) {
        name_ = newName;
    }
    const string& getName() const {    // const& return
        return name_;
    }
};
int main() {
    Widget w{};
    w.setName("Title");
    string name1 = w.getName();        // new string, thus a copy
    name1.clear();                     // you are allowed to modify the copy again
    const string& name2 = w.getName(); // const reference to internal string name_
    /* name2.clear(); */               // name2 is const, so it doesn't work
    string& name3 = w.getName();       // (ERR) Function returns const&, not &.
    auto name4 = w.getName();          // identical to name1
    const auto& name5 = w.getName();   // identical to name2
}
```

----------------


## Listing 13.11: References can be returned as constant and nonconstant.

Book listing [lst-0033-book.cpp](lst-0033-book.cpp):
```cpp
// https://godbolt.org/z/9MTj3rP5d 
#include <string>
#include <iostream>
using std::string; using std::cout;

class Widget {
    string name_{};
public:
    const string& readName() const;         // const&-return, const-method
    string& getName();                      // &-return
};

const string& Widget::readName() const { return name_; }
string& Widget::getName() { return name_; }

int main() {
    Widget w{};
    const string& readonly = w.readName(); // const&, immutable
    cout << "Name: " << readonly << "\n";  // still "" empty.
    string& readwrite = w.getName();       // &, mutable
    readwrite.append("attached");          // also changes name_ and readonly
    cout << "Name via readwrite: " << readwrite << "\n"; // "attached"
    cout << "Name via readonly: " << readonly << "\n";   // also "attached"
}
```

Godbolt Listing [lst-0033-godb.cpp](lst-0033-godb.cpp), [https://godbolt.org/z/9MTj3rP5d](https://godbolt.org/z/9MTj3rP5d):
```cpp
//#(compile) c++; compiler:g141; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/9MTj3rP5d 
#include <string>
#include <iostream>
using std::string; using std::cout;

class Widget {
    string name_{};
public:
    const string& readName() const;         // const&-return, const-method
    string& getName();                      // &-return
};

const string& Widget::readName() const { return name_; }
string& Widget::getName() { return name_; }

int main() {
    Widget w{};
    const string& readonly = w.readName(); // const&, immutable
    cout << "Name: " << readonly << "\n";  // still "" empty.
    string& readwrite = w.getName();       // &, mutable
    readwrite.append("attached");          // also changes name_ and readonly
    cout << "Name via readwrite: " << readwrite << "\n"; // "attached"
    cout << "Name via readonly: " << readonly << "\n";   // also "attached"
}
```

----------------


## GodboltId:4dnTs4vPx

Book listing [lst-0036-book.cpp](lst-0036-book.cpp):
```cpp
// https://godbolt.org/z/4dnTs4vPx
namespace {
    const int MAX_A = 12;       // the same as MAX_B, but no static needed
}
static const int MAX_B = 10;    // in the global namespace
struct Data {
    static const int SIZE = 14; // as a data field in a class
};

void func() {
    static const int LIMIT = 16; // as a local constant
}
```

Godbolt Listing [lst-0036-godb.cpp](lst-0036-godb.cpp), [https://godbolt.org/z/4dnTs4vPx](https://godbolt.org/z/4dnTs4vPx):
```cpp
//#(compile) c++; compiler:g141; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/4dnTs4vPx
namespace {
    const int MAX_A = 12;       // the same as MAX_B, but no static needed
}
static const int MAX_B = 10;    // in the global namespace
struct Data {
    static const int SIZE = 14; // as a data field in a class
};

void func() {
    static const int LIMIT = 16; // as a local constant
}
```

----------------


## Listing 13.12: Some expressions must be known at compile time.

Book listing [lst-0037-book.cpp](lst-0037-book.cpp):
```cpp
// https://godbolt.org/z/E6MWxY1f7
#include <array>
int main() {

    std::array<int, 5> arr5{};     // literal and thus a constant expression
    std::array<int, 2+3> arr23{};  // 2+3 can be evaluated by the compiler

    const size_t SIZE = 5;         // defines a constant
    std::array<int, SIZE> arrSC{}; // can often be used by the compiler

    size_t size = 7;
    std::array<int,size> arrVar{}; // you cannot use a variable
}
```

Godbolt Listing [lst-0037-godb.cpp](lst-0037-godb.cpp), [https://godbolt.org/z/E6MWxY1f7](https://godbolt.org/z/E6MWxY1f7):
```cpp
//#(compile) c++; compiler:g141; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/E6MWxY1f7
#include <array>
int main() {

    std::array<int, 5> arr5{};     // literal and thus a constant expression
    std::array<int, 2+3> arr23{};  // 2+3 can be evaluated by the compiler

    const size_t SIZE = 5;         // defines a constant
    std::array<int, SIZE> arrSC{}; // can often be used by the compiler

    size_t size = 7;
    std::array<int,size> arrVar{}; // you cannot use a variable
}
```

----------------


## Listing 13.13: Whether the compiler can use a constant in a constant expression is not always immediately apparent.

Book listing [lst-0038-book.cpp](lst-0038-book.cpp):
```cpp
// https://godbolt.org/z/xPzcWPdMa 
#include <array>
struct Data {
    static const size_t LATE;             // declare constant
    static const size_t EARLY;            // declare constant
};

void func() {
    int x = Data::LATE;                   // use constant
}

const size_t Data::EARLY = 10;            // define constant

std::array<int, Data::EARLY> arrEARLY {}; // use constant
std::array<int, Data::LATE>  arrLATE {};  // (ERR) use constant
const size_t Data::LATE = 10;             // define constant

int main() {
    func();
}
```

Godbolt Listing [lst-0038-godb.cpp](lst-0038-godb.cpp), [https://godbolt.org/z/xPzcWPdMa](https://godbolt.org/z/xPzcWPdMa):
```cpp
//#(compile) c++; compiler:g141; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/xPzcWPdMa 
#include <array>
struct Data {
    static const size_t LATE;             // declare constant
    static const size_t EARLY;            // declare constant
};

void func() {
    int x = Data::LATE;                   // use constant
}

const size_t Data::EARLY = 10;            // define constant

std::array<int, Data::EARLY> arrEARLY {}; // use constant
std::array<int, Data::LATE>  arrLATE {};  // (ERR) use constant
const size_t Data::LATE = 10;             // define constant

int main() {
    func();
}
```

----------------


## Listing 13.14: With "constexpr", the compiler sees when an expression is not computable early.

Book listing [lst-0040-book.cpp](lst-0040-book.cpp):
```cpp
struct Data {
    static constexpr size_t LATE; // (ERR) does not work without direct initialization
    static constexpr size_t EARLY = 10;
};
constexpr size_t Data::LATE = 10; // (ERR) with constexpr, definition is different
```

----------------


## GodboltId:Yv85s9T6v

Book listing [lst-0041-book.cpp](lst-0041-book.cpp):
```cpp
// https://godbolt.org/z/Yv85s9T6v 
constexpr size_t doubleIfTooSmall1(size_t value) {
   return value < 100 ? value*2 : value; // returns double if less than 100
}
std::array<int, doubleIfTooSmall1(50)> arr {};
```

Godbolt Listing [lst-0041-godb.cpp](lst-0041-godb.cpp), [https://godbolt.org/z/Yv85s9T6v](https://godbolt.org/z/Yv85s9T6v):
```cpp
//#(compile) c++; compiler:g141; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/Yv85s9T6v 
constexpr size_t doubleIfTooSmall1(size_t value) {
   return value < 100 ? value*2 : value; // returns double if less than 100
}
std::array<int, doubleIfTooSmall1(50)> arr {};
```

----------------


## Listing 13.15: With "if constexpr", you can decide at compile time what code will be executed.

Book listing [lst-0044-book.cpp](lst-0044-book.cpp):
```cpp
// https://godbolt.org/z/zMbnYzo17 
template<typename T>
auto deref(T t) {
    if constexpr (std::is_pointer_v<T>) {
        return *t;
    } else {
        return t;
    }
}
int main() {
    int i = 42;
    std::cout << deref(i) << '\n';         // directly the value
    auto p = std::make_unique<int>(73);
    std::cout << deref(p.get()) << '\n';   // dereferenced pointer
}
```

Godbolt Listing [lst-0044-godb.cpp](lst-0044-godb.cpp), [https://godbolt.org/z/zMbnYzo17](https://godbolt.org/z/zMbnYzo17):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/zMbnYzo17 
template<typename T>
auto deref(T t) {
    if constexpr (std::is_pointer_v<T>) {
        return *t;
    } else {
        return t;
    }
}
int main() {
    int i = 42;
    std::cout << deref(i) << '\n';         // directly the value
    auto p = std::make_unique<int>(73);
    std::cout << deref(p.get()) << '\n';   // dereferenced pointer
}
```

----------------


## Listing 13.16: "if constexpr" also works with "else".

Book listing [lst-0045-book.cpp](lst-0045-book.cpp):
```cpp
// https://godbolt.org/z/eo468MvMo 
#include <iostream>
#include <string>
struct S {
   int n;
   std::string s;
   float d;
};
template <std::size_t N> auto& get(S& s) {
    if constexpr (N == 0) return s.n;
    else if constexpr (N == 1) return s.s;
    else if constexpr (N == 2) return s.d;
}
int main() {
    S obj { 0, "hello", 10.0f };
    std::cout << get<0>(obj) << ", " << get<1>(obj) << "\n”; // Output: 0, hello
}
```

Godbolt Listing [lst-0045-godb.cpp](lst-0045-godb.cpp), [https://godbolt.org/z/eo468MvMo](https://godbolt.org/z/eo468MvMo):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/eo468MvMo 
#include <iostream>
#include <string>
struct S {
   int n;
   std::string s;
   float d;
};
template <std::size_t N> auto& get(S& s) {
    if constexpr (N == 0) return s.n;
    else if constexpr (N == 1) return s.s;
    else if constexpr (N == 2) return s.d;
}
int main() {
    S obj { 0, "hello", 10.0f };
    std::cout << get<0>(obj) << ", " << get<1>(obj) << "\n”; // Output: 0, hello
}
```

----------------


## GodboltId:44KMf1fWr

Book listing [lst-0046-book.cpp](lst-0046-book.cpp):
```cpp
// https://godbolt.org/z/44KMf1fWr 
template<auto N>
constexpr auto fibonacci()    {
    if constexpr (N>=2) {
        return fibonacci<N-1>() + fibonacci<N-2>();
    } else {
        return N;
    }
}

int main() {
    std::cout << fibonacci<10>() << '\n';  // Output: 55
    std::cout << fibonacci<20>() << '\n';  // Output: 6765
}
```

Godbolt Listing [lst-0046-godb.cpp](lst-0046-godb.cpp), [https://godbolt.org/z/44KMf1fWr](https://godbolt.org/z/44KMf1fWr):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/44KMf1fWr 
template<auto N>
constexpr auto fibonacci()    {
    if constexpr (N>=2) {
        return fibonacci<N-1>() + fibonacci<N-2>();
    } else {
        return N;
    }
}

int main() {
    std::cout << fibonacci<10>() << '\n';  // Output: 55
    std::cout << fibonacci<20>() << '\n';  // Output: 6765
}
```

----------------


## Listing 13.17: With "consteval", you can execute functions at compile time.

Book listing [lst-0047-book.cpp](lst-0047-book.cpp):
```cpp
// https://godbolt.org/z/v3x7nGs3f 
int get_input() {
    return 50;  // or read something from a file or so
}
constexpr auto calculate_1(int input) {
    return input * 2;
}
consteval auto calculate_2(int input) {
    return input * 2;
}
int main() {
    int input = get_input();
    auto a = calculate_1(77);     // at compile time … maybe computable
    auto b = calculate_1(input);  // … computable, but valid
    auto c = calculate_2(77);     // … computable
    auto d = calculate_2(input);  // (ERR) … not computable, invalid
}
```

Godbolt Listing [lst-0047-godb.cpp](lst-0047-godb.cpp), [https://godbolt.org/z/v3x7nGs3f](https://godbolt.org/z/v3x7nGs3f):
```cpp
//#(compile) c++; compiler:g141; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/v3x7nGs3f 
int get_input() {
    return 50;  // or read something from a file or so
}
constexpr auto calculate_1(int input) {
    return input * 2;
}
consteval auto calculate_2(int input) {
    return input * 2;
}
int main() {
    int input = get_input();
    auto a = calculate_1(77);     // at compile time … maybe computable
    auto b = calculate_1(input);  // … computable, but valid
    auto c = calculate_2(77);     // … computable
    auto d = calculate_2(input);  // (ERR) … not computable, invalid
}
```

----------------


## Listing 13.18: Even more complex functions can be computed at compile time with "consteval".

Book listing [lst-0048-book.cpp](lst-0048-book.cpp):
```cpp
// https://godbolt.org/z/Wbr7n8M4q 
#include <iostream>
#include <array>
constexpr bool isPrime(int n) { // computable at compile-time
  if(n < 2) return false; // 0, 1 are not prime
  for (int i = 2; i*i <= n; i += i>2 ? 2 : 1) { // 2,3,5,7,9,11,13,15…
    if (n % i == 0) return false;
  }
  return n > 1; // for 0 and 1
}
template<int Num>
consteval std::array<int, Num> primeNumbers() { // only at compile-time
  std::array<int, Num> primes{};
  int idx = 0;
  for (int val = 1; idx < Num; ++val) {
    if (isPrime(val)) primes[idx++] = val;
  }
  return primes;
}
int main() {
  // initialize with prime numbers
  auto primes = primeNumbers<100>();  // 1000000 doesn't work
  for (auto v : primes) {
    std::cout << v << ' ';
  }
  std::cout << '\n';
}
```

Godbolt Listing [lst-0048-godb.cpp](lst-0048-godb.cpp), [https://godbolt.org/z/Wbr7n8M4q](https://godbolt.org/z/Wbr7n8M4q):
```cpp
//#(compile) c++; compiler:g141; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/Wbr7n8M4q 
#include <iostream>
#include <array>
constexpr bool isPrime(int n) { // computable at compile-time
  if(n < 2) return false; // 0, 1 are not prime
  for (int i = 2; i*i <= n; i += i>2 ? 2 : 1) { // 2,3,5,7,9,11,13,15…
    if (n % i == 0) return false;
  }
  return n > 1; // for 0 and 1
}
template<int Num>
consteval std::array<int, Num> primeNumbers() { // only at compile-time
  std::array<int, Num> primes{};
  int idx = 0;
  for (int val = 1; idx < Num; ++val) {
    if (isPrime(val)) primes[idx++] = val;
  }
  return primes;
}
int main() {
  // initialize with prime numbers
  auto primes = primeNumbers<100>();  // 1000000 doesn't work
  for (auto v : primes) {
    std::cout << v << ' ';
  }
  std::cout << '\n';
}
```

----------------


## Listing 13.19: The compiler determines whether the call can be evaluated at compile time in the given context.

Book listing [lst-0049-book.cpp](lst-0049-book.cpp):
```cpp
// https://godbolt.org/z/7KWqTPo1K 
#include <iostream> // cout
constexpr int get_value() {
    if consteval {
       return 42;
    } else {
       return 668;
    }
}
int main() {
  auto a = get_value();
  std::cout << a << '\n';        // Output: 668
  constexpr auto b = get_value();
  std::cout << b << '\n';        // Output: 42
}
```

Godbolt Listing [lst-0049-godb.cpp](lst-0049-godb.cpp), [https://godbolt.org/z/7KWqTPo1K](https://godbolt.org/z/7KWqTPo1K):
```cpp
//#(compile) c++; compiler:g132; options:"-std=c++23"; libs:-
// https://godbolt.org/z/7KWqTPo1K 
#include <iostream> // cout
constexpr int get_value() {
    if consteval {
       return 42;
    } else {
       return 668;
    }
}
int main() {
  auto a = get_value();
  std::cout << a << '\n';        // Output: 668
  constexpr auto b = get_value();
  std::cout << b << '\n';        // Output: 42
}
```

----------------


## Listing 13.20: "mutable" makes a data field in const methods mutable.

Book listing [lst-0051-book.cpp](lst-0051-book.cpp):
```cpp
// https://godbolt.org/z/sPorKnEbd 
#include <iostream>
class Data {
    int value_;
    mutable size_t getCount_{0};
  public:
    explicit Data(int v) : value_{v} { }
    ~Data() {
        std::cout << "get was used " << getCount_ << " times\n";
    }
    int get() const {
        ++getCount_;
        return value_;
    }
};
int main() {
    Data d{42};
    for(int i=0; i<10; ++i) { d.get(); }
} // Output: get was used 10 times
```

Godbolt Listing [lst-0051-godb.cpp](lst-0051-godb.cpp), [https://godbolt.org/z/sPorKnEbd](https://godbolt.org/z/sPorKnEbd):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/sPorKnEbd 
#include <iostream>
class Data {
    int value_;
    mutable size_t getCount_{0};
  public:
    explicit Data(int v) : value_{v} { }
    ~Data() {
        std::cout << "get was used " << getCount_ << " times\n";
    }
    int get() const {
        ++getCount_;
        return value_;
    }
};
int main() {
    Data d{42};
    for(int i=0; i<10; ++i) { d.get(); }
} // Output: get was used 10 times
```

----------------


## Listing 13.21: "const" with containers.

Book listing [lst-0052-book.cpp](lst-0052-book.cpp):
```cpp
// https://godbolt.org/z/GE5v6Ea8x 
#include <map>
#include <string>
using std::map; using std::string;
struct MyClass {
  bool isFound(const map<int,string> &dict,  // immutable input parameter.
               const int &key,               // likewise
               string &result                // output parameter: not const
               ) const                       // instance of MyClass const
  {
    const map<int,string>::const_iterator where  // reference and value fixed
      = dict.find(key);
    if(where == end(dict)) {
      return false;
    } else {
      result = where->second;
      return true;
    }
  }
};
```

Godbolt Listing [lst-0052-godb.cpp](lst-0052-godb.cpp), [https://godbolt.org/z/GE5v6Ea8x](https://godbolt.org/z/GE5v6Ea8x):
```cpp
//#(execute) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/GE5v6Ea8x 
#include <map>
#include <string>
using std::map; using std::string;
struct MyClass {
  bool isFound(const map<int,string> &dict,  // immutable input parameter.
               const int &key,               // likewise
               string &result                // output parameter: not const
               ) const                       // instance of MyClass const
  {
    const map<int,string>::const_iterator where  // reference and value fixed
      = dict.find(key);
    if(where == end(dict)) {
      return false;
    } else {
      result = where->second;
      return true;
    }
  }
};
```

----------------


## Listing 13.22: x and y can somehow be changed from outside.

Book listing [lst-0053-book.cpp](lst-0053-book.cpp):
```cpp
#include <iostream>
#include <chrono> // milliseconds
#include <thread> // this_thread::sleep

struct Pos {
  volatile int x;
  volatile int y;
};
Pos pos{};

// defined somewhere else:
int installMouseDriver(Pos *p);

constexpr auto DELAY = std::chrono::milliseconds(100);
constexpr auto LOOPS = 30; // 30*100ms = 3s
int main() {
    installMouseDriver( &pos );  // MouseDriver updates x and y
    for(int i=0; i<LOOPS; ++i) {
        std::cout << "mouse at ("<<pos.x<<","<<pos.y<<")\n";
        std::this_thread::sleep_for(DELAY);
    }
}
```

----------------

