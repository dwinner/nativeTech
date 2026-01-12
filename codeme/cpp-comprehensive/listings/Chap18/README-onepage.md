# Listings of Chap18.docx

This is the list of listings on one page.
You can also view a [linked summary](README.md).


## Listing 18.1: With "friend", you can allow another class access to private members.

Book listing [lst-0001-book.cpp](lst-0001-book.cpp):
```cpp
// https://godbolt.org/z/sTosKfocx 
#include <iostream>
class Mechanic;
class Thing {
    int value_; // private
public:
    explicit Thing(int value) : value_{value} {}
    void increment() { ++value_; }
    std::ostream& print(std::ostream& os) const { return os << value_; }
    friend class Mechanic;
};
class Mechanic {
    const Thing &thing_;
public:
    explicit Mechanic(const Thing &thing) : thing_{thing} {}
    auto getThingValue() const {
        return thing_.value_;        // access to private member of Thing
    }
};
int main() {
    Thing thing{45};
    thing.print(std::cout) << '\n';                // output: 45
    Mechanic mechanic{thing};
    thing.increment();                             // change internal value
    std::cout << mechanic.getThingValue() << '\n'; // output: 46
}
```

Godbolt Listing [lst-0001-godb.cpp](lst-0001-godb.cpp), [https://godbolt.org/z/sTosKfocx](https://godbolt.org/z/sTosKfocx):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/sTosKfocx 
#include <iostream>
class Mechanic;
class Thing {
    int value_; // private
public:
    explicit Thing(int value) : value_{value} {}
    void increment() { ++value_; }
    std::ostream& print(std::ostream& os) const { return os << value_; }
    friend class Mechanic;
};
class Mechanic {
    const Thing &thing_;
public:
    explicit Mechanic(const Thing &thing) : thing_{thing} {}
    auto getThingValue() const {
        return thing_.value_;        // access to private member of Thing
    }
};
int main() {
    Thing thing{45};
    thing.print(std::cout) << '\n';                // output: 45
    Mechanic mechanic{thing};
    thing.increment();                             // change internal value
    std::cout << mechanic.getThingValue() << '\n'; // output: 46
}
```

----------------


## Listing 18.2: The Tree class has access to private members of Node.

Book listing [lst-0002-book.cpp](lst-0002-book.cpp):
```cpp
// https://godbolt.org/z/oxbcE18TM
#include <memory> // unique_ptr
#include <iostream>
#include <string>
using std::unique_ptr; using std::cout;
template <typename K, typename D> class Tree; // forward declaration
template <typename K, typename D>
class Node {
    friend class Tree<K,D>; // allow access to private members
    K key;
    D data;
    unique_ptr<Node> left, right;
public:
    Node(const K& k, const D& d) : key(k), data(d) { }
};
template <typename K, typename D>
class Tree {
public:
    void insert(const K &key, const D& data);
    D* find(const K &key) { return findRec(key, root); }
private:
    D* findRec(const K &key, unique_ptr<Node<K,D>> &node);
    unique_ptr<Node<K,D>> root;
};
template <typename K, typename D>
void Tree<K,D>::insert(const K& key, const D& data) {
    auto *current = &root;
    while(*current) { // as long as unique_ptr contains something
        auto &node = *(current->get());
        if (key < node.key) {
            current = &node.left;
        } else if (node.key < key) {
            current = &node.right;
        }
    }
    *current = std::make_unique<Node<K,D>>(key,data);
};
template <typename K, typename D>
D* Tree<K,D>::findRec(const K& key, unique_ptr<Node<K,D>> &where) {
    if(!where)
        return nullptr;
    auto &node = *(where.get());
    if(key < node.key)
        return findRec(key, node.left);
    if(node.key < key)
        return findRec(key, node.right);
    return &node.data; // key == node.key
};
int main() {
    Tree<int,std::string> bt {};
    bt.insert(3, "three");
    bt.insert(2, "two");
    bt.insert(4, "four");
    auto where = bt.find(7);
    if(where==nullptr) cout<<"no 7\n";      // output: no 7
    where = bt.find(3);
    if(where!=nullptr) cout<<*where<<"\n";  // output: three
}
```

Godbolt Listing [lst-0002-godb.cpp](lst-0002-godb.cpp), [https://godbolt.org/z/oxbcE18TM](https://godbolt.org/z/oxbcE18TM):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/oxbcE18TM
#include <memory> // unique_ptr
#include <iostream>
#include <string>
using std::unique_ptr; using std::cout;
template <typename K, typename D> class Tree; // forward declaration
template <typename K, typename D>
class Node {
    friend class Tree<K,D>; // allow access to private members
    K key;
    D data;
    unique_ptr<Node> left, right;
public:
    Node(const K& k, const D& d) : key(k), data(d) { }
};
template <typename K, typename D>
class Tree {
public:
    void insert(const K &key, const D& data);
    D* find(const K &key) { return findRec(key, root); }
private:
    D* findRec(const K &key, unique_ptr<Node<K,D>> &node);
    unique_ptr<Node<K,D>> root;
};
template <typename K, typename D>
void Tree<K,D>::insert(const K& key, const D& data) {
    auto *current = &root;
    while(*current) { // as long as unique_ptr contains something
        auto &node = *(current->get());
        if (key < node.key) {
            current = &node.left;
        } else if (node.key < key) {
            current = &node.right;
        }
    }
    *current = std::make_unique<Node<K,D>>(key,data);
};
template <typename K, typename D>
D* Tree<K,D>::findRec(const K& key, unique_ptr<Node<K,D>> &where) {
    if(!where)
        return nullptr;
    auto &node = *(where.get());
    if(key < node.key)
        return findRec(key, node.left);
    if(node.key < key)
        return findRec(key, node.right);
    return &node.data; // key == node.key
};
int main() {
    Tree<int,std::string> bt {};
    bt.insert(3, "three");
    bt.insert(2, "two");
    bt.insert(4, "four");
    auto where = bt.find(7);
    if(where==nullptr) cout<<"no 7\n";      // output: no 7
    where = bt.find(3);
    if(where!=nullptr) cout<<*where<<"\n";  // output: three
}
```

----------------


## Listing 18.3: When inheriting, you can specify what should become visible.

Book listing [lst-0004-book.cpp](lst-0004-book.cpp):
```cpp
// https://godbolt.org/z/eKbK39nG1 
class Base {
public:
    int xPublic = 1;
protected:
    int xProtected = 2;
private:
    int xPrivate = 3;
};
class DerivedPublic : public Base {
    // xPublic becomes 'public'
    // xProtected becomes 'protected'
    // xPrivate is not visible here
};
class DerivedProtected : protected Base {
    // xPublic becomes 'protected'
    // xProtected becomes 'protected'
    // xPrivate is not visible here
};
class DerivedPrivate : private Base { // or if nothing is specified
    // xPublic becomes 'private'
    // xProtected becomes 'private'
    // xPrivate is not visible here
};
```

Godbolt Listing [lst-0004-godb.cpp](lst-0004-godb.cpp), [https://godbolt.org/z/eKbK39nG1](https://godbolt.org/z/eKbK39nG1):
```cpp
//#(execute) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/eKbK39nG1 
class Base {
public:
    int xPublic = 1;
protected:
    int xProtected = 2;
private:
    int xPrivate = 3;
};
class DerivedPublic : public Base {
    // xPublic becomes 'public'
    // xProtected becomes 'protected'
    // xPrivate is not visible here
};
class DerivedProtected : protected Base {
    // xPublic becomes 'protected'
    // xProtected becomes 'protected'
    // xPrivate is not visible here
};
class DerivedPrivate : private Base { // or if nothing is specified
    // xPublic becomes 'private'
    // xProtected becomes 'private'
    // xPrivate is not visible here
};
```

----------------


## Listing 18.4: Inherited visibilities, viewed from the outside.

Book listing [lst-0005-book.cpp](lst-0005-book.cpp):
```cpp
// https://godbolt.org/z/61EdcjPn4
#include <iostream>
using std::cout; using std::ostream;
// ... as before ...
int main() {
    // public inheritance
    DerivedPublic dpu{};
    cout << dpu.xPublic << '\n';    // output: 1
    cout << dpu.xProtected << '\n'; // no access from outside
    // protected inheritance
    DerivedProtected dpt{};
    cout << dpt.xPublic << '\n';    // no access from outside
    cout << dpt.xProtected << '\n'; // no access from outside
    // private inheritance
    DerivedPrivate dpv{};
    cout << dpv.xPublic << '\n';    // no access from outside
    cout << dpv.xProtected << '\n'; // no access from outside
}
```

Godbolt Listing [lst-0005-godb.cpp](lst-0005-godb.cpp), [https://godbolt.org/z/61EdcjPn4](https://godbolt.org/z/61EdcjPn4):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/61EdcjPn4
#include <iostream>
using std::cout; using std::ostream;
// ... as before ...
int main() {
    // public inheritance
    DerivedPublic dpu{};
    cout << dpu.xPublic << '\n';    // output: 1
    cout << dpu.xProtected << '\n'; // no access from outside
    // protected inheritance
    DerivedProtected dpt{};
    cout << dpt.xPublic << '\n';    // no access from outside
    cout << dpt.xProtected << '\n'; // no access from outside
    // private inheritance
    DerivedPrivate dpv{};
    cout << dpv.xPublic << '\n';    // no access from outside
    cout << dpv.xProtected << '\n'; // no access from outside
}
```

----------------


## Listing 18.5: Inherited visibilities for further derivations.

Book listing [lst-0006-book.cpp](lst-0006-book.cpp):
```cpp
// https://godbolt.org/z/Pq5aWq8xr
#include <iostream>
using std::cout; using std::ostream;
// ... as before ...
struct NormalCase : public DerivedPublic {
    void print() {
        cout << xPublic;
        cout << xProtected;
    }
};
struct SpecialCase : public DerivedPrivate {
    void print() {
        cout << xPublic;                   // (ERR) no access
        cout << xProtected;                // (ERR) no access
    }
};
int main() {
    NormalCase n {};
    n.print();                // output: 12
    SpecialCase s {};
    s.print();
}
```

Godbolt Listing [lst-0006-godb.cpp](lst-0006-godb.cpp), [https://godbolt.org/z/Pq5aWq8xr](https://godbolt.org/z/Pq5aWq8xr):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/Pq5aWq8xr
#include <iostream>
using std::cout; using std::ostream;
// ... as before ...
struct NormalCase : public DerivedPublic {
    void print() {
        cout << xPublic;
        cout << xProtected;
    }
};
struct SpecialCase : public DerivedPrivate {
    void print() {
        cout << xPublic;                   // (ERR) no access
        cout << xProtected;                // (ERR) no access
    }
};
int main() {
    NormalCase n {};
    n.print();                // output: 12
    SpecialCase s {};
    s.print();
}
```

----------------


## Listing 18.6: In practice, child classes use privately inherited members indirectly.

Book listing [lst-0007-book.cpp](lst-0007-book.cpp):
```cpp
// https://godbolt.org/z/WYqYe7PoY
#include <iostream>
using std::cout; using std::ostream;
class Base {
    public: int data = 5;
};
class Middle : private Base {
protected: void print() {
        cout << data;   // 'data' is inherited privately here
    }
};
class Ultimately : public Middle {
    public: void go() {
        // 'data' is not visible
        print();        // 'print' is protected visible
    }
};
int main() {
    Ultimately u {};
    u.go();             // output: 5
}
```

Godbolt Listing [lst-0007-godb.cpp](lst-0007-godb.cpp), [https://godbolt.org/z/WYqYe7PoY](https://godbolt.org/z/WYqYe7PoY):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/WYqYe7PoY
#include <iostream>
using std::cout; using std::ostream;
class Base {
    public: int data = 5;
};
class Middle : private Base {
protected: void print() {
        cout << data;   // 'data' is inherited privately here
    }
};
class Ultimately : public Middle {
    public: void go() {
        // 'data' is not visible
        print();        // 'print' is protected visible
    }
};
int main() {
    Ultimately u {};
    u.go();             // output: 5
}
```

----------------


## Listing 18.7: A has-a relationship via private inheritance.

Book listing [lst-0008-book.cpp](lst-0008-book.cpp):
```cpp
// https://godbolt.org/z/ETr73T5a9
class Engine {
public:
  Engine(int numCylinders);
  void start();                   // start engine
};
class Car : private Engine {      // Car has a engine
public:
  Car() : Engine{8} { }           // initializes a car with 8 cylinders
  using Engine::start;            // starts car by starting the engine
};
```

Godbolt Listing [lst-0008-godb.cpp](lst-0008-godb.cpp), [https://godbolt.org/z/ETr73T5a9](https://godbolt.org/z/ETr73T5a9):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/ETr73T5a9
class Engine {
public:
  Engine(int numCylinders);
  void start();                   // start engine
};
class Car : private Engine {      // Car has a engine
public:
  Car() : Engine{8} { }           // initializes a car with 8 cylinders
  using Engine::start;            // starts car by starting the engine
};
```

----------------


## Listing 18.8: Has-a relationship using a member.

Book listing [lst-0009-book.cpp](lst-0009-book.cpp):
```cpp
// https://godbolt.org/z/67EjhMf54
class Car {
public:
  Car() : engine_{8} { }            // initializes a car with 8 cylinders
  void start() { engine_.start(); } // starts the car by starting the engine
private:
  Engine engine_;                   // Car has-a engine
};
```

Godbolt Listing [lst-0009-godb.cpp](lst-0009-godb.cpp), [https://godbolt.org/z/67EjhMf54](https://godbolt.org/z/67EjhMf54):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/67EjhMf54
class Car {
public:
  Car() : engine_{8} { }            // initializes a car with 8 cylinders
  void start() { engine_.start(); } // starts the car by starting the engine
private:
  Engine engine_;                   // Car has-a engine
};
```

----------------


## Listing 18.9: A signature class has only pure virtual methods.

Book listing [lst-0010-book.cpp](lst-0010-book.cpp):
```cpp
// https://godbolt.org/z/rW4oTPhrG 
struct Driver {
    virtual void init() = 0;
    virtual void done() = 0;
    virtual bool send(const char* data, unsigned len) = 0;
};
```

Godbolt Listing [lst-0010-godb.cpp](lst-0010-godb.cpp), [https://godbolt.org/z/rW4oTPhrG](https://godbolt.org/z/rW4oTPhrG):
```cpp
//#(execute) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/rW4oTPhrG 
struct Driver {
    virtual void init() = 0;
    virtual void done() = 0;
    virtual bool send(const char* data, unsigned len) = 0;
};
```

----------------


## Listing 18.10: A signature class makes a good base class.

Book listing [lst-0011-book.cpp](lst-0011-book.cpp):
```cpp
// https://godbolt.org/z/cevM7nq53
#include <iostream>
using std::cout;
struct KeyboardDriver : public Driver {
    void init() override { cout << "Init Keyboard\n"; }
    void done() override { cout << "Done Keyboard\n"; }
    bool send(const char* data, unsigned int len) override {
        cout << "sending " << len << " bytes\n";
        return true;
    }
};
struct Computer {
    Driver &driver_;
    explicit Computer(Driver &driver) : driver_{driver} {
        driver_.init();
    }
    void run() {
        driver_.send("Hello", 5);
    }
    ~Computer() {
        driver_.done();
    }
    Computer(const Computer&) = delete;
};
int main() {
    KeyboardDriver keyboard {};
    Computer computer(keyboard); // Output: Init Keyboard
    computer.run();              // Output: sending 5 bytes
}                                // Output: Done Keyboard
```

Godbolt Listing [lst-0011-godb.cpp](lst-0011-godb.cpp), [https://godbolt.org/z/cevM7nq53](https://godbolt.org/z/cevM7nq53):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/cevM7nq53
#include <iostream>
using std::cout;
struct KeyboardDriver : public Driver {
    void init() override { cout << "Init Keyboard\n"; }
    void done() override { cout << "Done Keyboard\n"; }
    bool send(const char* data, unsigned int len) override {
        cout << "sending " << len << " bytes\n";
        return true;
    }
};
struct Computer {
    Driver &driver_;
    explicit Computer(Driver &driver) : driver_{driver} {
        driver_.init();
    }
    void run() {
        driver_.send("Hello", 5);
    }
    ~Computer() {
        driver_.done();
    }
    Computer(const Computer&) = delete;
};
int main() {
    KeyboardDriver keyboard {};
    Computer computer(keyboard); // Output: Init Keyboard
    computer.run();              // Output: sending 5 bytes
}                                // Output: Done Keyboard
```

----------------


## Listing 18.11: A virtual method with implementation "= 0" is abstract.

Book listing [lst-0012-book.cpp](lst-0012-book.cpp):
```cpp
// https://godbolt.org/z/xWr7axonx
#include <iostream>           // cout
#include <memory>             // unique_ptr
using std::cout;
class Driver {                // abstract base class
public:
    virtual void init() = 0;
    virtual void done() = 0;
    virtual void send(const std::string &data) = 0;
};
class ProductionDriver : public Driver {
public:
    void init() override { }
    void done() override { }
    void send(const std::string &data) override { cout << data << "\n"; }
};
class DebuggingDriver : public Driver {
   size_t countSend_ = 0;
public:
    void init() override {
        countSend_= 0; cout << "Ok, I'm initialized.\n";
    }
    void done() override {
        cout << "send used:" << countSend_ << " times\n";
    }
    void send(const std::string &data) override {
        cout << "send("<<countSend_<<"):"<< data << "\n";
        ++countSend_;
    }
};
struct DriverWrapper {        // RAII wrapper for init() and done()
    Driver &driver_;
    explicit DriverWrapper(Driver& driver) : driver_(driver) {
        driver_.init(); }
    ~DriverWrapper() { driver_.done(); }
    DriverWrapper(const DriverWrapper&) = delete; // no copying
};

void doWork(Driver &driver) { // someone who flexibly uses any driver
    DriverWrapper wrapper(driver);      // call init() and done()
    driver.send("An Unexpected Journey");
    driver.send("The Desolation Of Smaug");
}

int main() {
    // same doWork, once with production and once with debugging driver
    ProductionDriver production{};
    doWork( production );
    DebuggingDriver debugging{};
    doWork( debugging );

    // more common variant of a dynamically created driver
    std::unique_ptr<Driver> driver{ new ProductionDriver{} };
    doWork( *driver );
}
```

Godbolt Listing [lst-0012-godb.cpp](lst-0012-godb.cpp), [https://godbolt.org/z/xWr7axonx](https://godbolt.org/z/xWr7axonx):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/xWr7axonx
#include <iostream>           // cout
#include <memory>             // unique_ptr
using std::cout;
class Driver {                // abstract base class
public:
    virtual void init() = 0;
    virtual void done() = 0;
    virtual void send(const std::string &data) = 0;
};
class ProductionDriver : public Driver {
public:
    void init() override { }
    void done() override { }
    void send(const std::string &data) override { cout << data << "\n"; }
};
class DebuggingDriver : public Driver {
   size_t countSend_ = 0;
public:
    void init() override {
        countSend_= 0; cout << "Ok, I'm initialized.\n";
    }
    void done() override {
        cout << "send used:" << countSend_ << " times\n";
    }
    void send(const std::string &data) override {
        cout << "send("<<countSend_<<"):"<< data << "\n";
        ++countSend_;
    }
};
struct DriverWrapper {        // RAII wrapper for init() and done()
    Driver &driver_;
    explicit DriverWrapper(Driver& driver) : driver_(driver) {
        driver_.init(); }
    ~DriverWrapper() { driver_.done(); }
    DriverWrapper(const DriverWrapper&) = delete; // no copying
};

void doWork(Driver &driver) { // someone who flexibly uses any driver
    DriverWrapper wrapper(driver);      // call init() and done()
    driver.send("An Unexpected Journey");
    driver.send("The Desolation Of Smaug");
}

int main() {
    // same doWork, once with production and once with debugging driver
    ProductionDriver production{};
    doWork( production );
    DebuggingDriver debugging{};
    doWork( debugging );

    // more common variant of a dynamically created driver
    std::unique_ptr<Driver> driver{ new ProductionDriver{} };
    doWork( *driver );
}
```

----------------


## Listing 18.12: Multiple inheritance means having multiple base classes.

Book listing [lst-0014-book.cpp](lst-0014-book.cpp):
```cpp
// https://godbolt.org/z/xnGTM351G
#include <iostream>
using std::cout;
class Mammal {
public:
    void giveBirth() { cout << "Birth!\n"; }
};
class Flying {
public:
    void fly() { cout << "Flight!\n"; }
};
class Bat: public Mammal, public Flying {
public:
    void call() { cout << "Ultrasound!\n"; }
};
int main() {
    Bat bruce{};
    bruce.giveBirth();    // Output: Birth!
    bruce.fly();          // Output: Flight!
    bruce.call();         // Output: Ultrasound!
}
```

Godbolt Listing [lst-0014-godb.cpp](lst-0014-godb.cpp), [https://godbolt.org/z/xnGTM351G](https://godbolt.org/z/xnGTM351G):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/xnGTM351G
#include <iostream>
using std::cout;
class Mammal {
public:
    void giveBirth() { cout << "Birth!\n"; }
};
class Flying {
public:
    void fly() { cout << "Flight!\n"; }
};
class Bat: public Mammal, public Flying {
public:
    void call() { cout << "Ultrasound!\n"; }
};
int main() {
    Bat bruce{};
    bruce.giveBirth();    // Output: Birth!
    bruce.fly();          // Output: Flight!
    bruce.call();         // Output: Ultrasound!
}
```

----------------


## Listing 18.13: The clock and calendar combination results in a clock with a calendar.

Book listing [lst-0015-book.cpp](lst-0015-book.cpp):
```cpp
// https://godbolt.org/z/oxoef1nTs
// Parameters must be valid values of the respective unit.
#include <iostream> // ostream
#include <iomanip>  // setw, setfill
#include <format>
using std::ostream; using std::format;
class Clock {
protected:
    int h_, m_, s_;
public:
    Clock(int hours, int minutes, int seconds)
    : h_{hours}, m_{minutes}, s_{seconds} {}
    void setClock(int hours, int minutes, int seconds) {
        h_ = hours; m_ = minutes; s_ = seconds;
    }
    friend ostream& operator<<(ostream&os, const Clock& c) {
        return os << format("{:02}:{:02}:{:02}", c.h_, c.m_, c.s_);
    }
    void tick() { // +1 second
        if(s_ >= 59) {
            s_ = 0;
            if(m_ >= 59) {
                m_ = 0;
                if(h_ >= 23) h_ = 0;
                else { h_ += 1; }
            } else { m_ += 1; }
        } else { s_ += 1; }
    }
};
```

Godbolt Listing [lst-0015-godb.cpp](lst-0015-godb.cpp), [https://godbolt.org/z/oxoef1nTs](https://godbolt.org/z/oxoef1nTs):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/oxoef1nTs
// Parameters must be valid values of the respective unit.
#include <iostream> // ostream
#include <iomanip>  // setw, setfill
#include <format>
using std::ostream; using std::format;
class Clock {
protected:
    int h_, m_, s_;
public:
    Clock(int hours, int minutes, int seconds)
    : h_{hours}, m_{minutes}, s_{seconds} {}
    void setClock(int hours, int minutes, int seconds) {
        h_ = hours; m_ = minutes; s_ = seconds;
    }
    friend ostream& operator<<(ostream&os, const Clock& c) {
        return os << format("{:02}:{:02}:{:02}", c.h_, c.m_, c.s_);
    }
    void tick() { // +1 second
        if(s_ >= 59) {
            s_ = 0;
            if(m_ >= 59) {
                m_ = 0;
                if(h_ >= 23) h_ = 0;
                else { h_ += 1; }
            } else { m_ += 1; }
        } else { s_ += 1; }
    }
};
```

----------------


## Listing 18.14: A simple calendar class.

Book listing [lst-0017-book.cpp](lst-0017-book.cpp):
```cpp
// https://godbolt.org/z/56Ec357Tf
// … includes and usings …
struct Calendar {
    int y_, m_, d_;
    static const std::vector<int> mlens_;
    bool leapyear() const {
        if(y_ % 4 != 0) return false;
        if(y_ % 100 != 0) return true;
        if(y_ % 400 != 0) return false;
        return true;
    }
public:
    Calendar(int year, int month, int day)
    : y_{year}, m_{month}, d_{day} {}
    void setCalendar(int year, int month, int day) {
        y_ = year; m_ = month; d_ = day;
    }
    friend ostream& operator<<(ostream& os, const Calendar& c) {
        return os << format("{:04}-{:02}-{:02}", c.y_, c.m_, c.d_);
    }
    void advance() { // +1 day
        auto maxd = mlens_[m_-1];  // 0-based vector
        if(m_==2 && leapyear())
            maxd += 1;             // February in a leap year
        if(d_ >= maxd) {
            d_ = 1;
            if(m_ >= 12) { m_ = 1; y_ += 1; }
            else { m_ += 1; }
        } else { d_ += 1; }
    }
};
const std::vector<int> Calendar::mlens_ = {31,28,31,30,31,30,31,31,30,31,30,31};
```

Godbolt Listing [lst-0017-godb.cpp](lst-0017-godb.cpp), [https://godbolt.org/z/56Ec357Tf](https://godbolt.org/z/56Ec357Tf):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/56Ec357Tf
// … includes and usings …
struct Calendar {
    int y_, m_, d_;
    static const std::vector<int> mlens_;
    bool leapyear() const {
        if(y_ % 4 != 0) return false;
        if(y_ % 100 != 0) return true;
        if(y_ % 400 != 0) return false;
        return true;
    }
public:
    Calendar(int year, int month, int day)
    : y_{year}, m_{month}, d_{day} {}
    void setCalendar(int year, int month, int day) {
        y_ = year; m_ = month; d_ = day;
    }
    friend ostream& operator<<(ostream& os, const Calendar& c) {
        return os << format("{:04}-{:02}-{:02}", c.y_, c.m_, c.d_);
    }
    void advance() { // +1 day
        auto maxd = mlens_[m_-1];  // 0-based vector
        if(m_==2 && leapyear())
            maxd += 1;             // February in a leap year
        if(d_ >= maxd) {
            d_ = 1;
            if(m_ >= 12) { m_ = 1; y_ += 1; }
            else { m_ += 1; }
        } else { d_ += 1; }
    }
};
const std::vector<int> Calendar::mlens_ = {31,28,31,30,31,30,31,31,30,31,30,31};
```

----------------


## Listing 18.15: The calendar clock is a calendar and a clock.

Book listing [lst-0019-book.cpp](lst-0019-book.cpp):
```cpp
// https://godbolt.org/z/zh5cPG7TK
class CalendarClock : public Clock, public Calendar {
public:
    CalendarClock(int y, int m, int d, int hh, int mm, int ss)
    : Calendar{y,m,d}, Clock{hh,mm,ss} {}
    void tick() {         // +1 second
        auto prev_h = h_;
        Clock::tick();    // Call base class method
        if(h_ < prev_h) { // if new day
            advance();    // … advance calendar
        }
    }
    friend ostream& operator<<(ostream&os, const CalendarClock& cc) {
        operator<<(os, (Calendar&)cc) << " "; // Call free function
        operator<<(os, (Clock&)cc);           // Call free function
        return os;
    }
};
```

Godbolt Listing [lst-0019-godb.cpp](lst-0019-godb.cpp), [https://godbolt.org/z/zh5cPG7TK](https://godbolt.org/z/zh5cPG7TK):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/zh5cPG7TK
class CalendarClock : public Clock, public Calendar {
public:
    CalendarClock(int y, int m, int d, int hh, int mm, int ss)
    : Calendar{y,m,d}, Clock{hh,mm,ss} {}
    void tick() {         // +1 second
        auto prev_h = h_;
        Clock::tick();    // Call base class method
        if(h_ < prev_h) { // if new day
            advance();    // … advance calendar
        }
    }
    friend ostream& operator<<(ostream&os, const CalendarClock& cc) {
        operator<<(os, (Calendar&)cc) << " "; // Call free function
        operator<<(os, (Clock&)cc);           // Call free function
        return os;
    }
};
```

----------------


## Listing 18.16: With multiple inheritance, the value of a pointer can change.

Book listing [lst-0023-book.cpp](lst-0023-book.cpp):
```cpp
// https://godbolt.org/z/oc7jsbrEK 
#include <iostream>
using std::cout;
struct Base1 {
    virtual void f1() {}
};
struct Base2 {
    virtual void f2() {}
};
struct Derived : public Base1, public Base2 {
    virtual void g() {};
};
void compare(void* a, void* b) {
    cout << (a==b ? "identical\n" : "different\n");
}
int main() {
    Derived d{};
    auto *pd = &d;
    cout << pd << '\n';           // for example 0x1000
    auto pb1 = static_cast<Base1*>(pd);
    cout << pb1 << '\n';          // still 0x1000
    auto pb2 = static_cast<Base2*>(pd);
    cout << pb2 << '\n';          // now 0x1008!
    cout << (pd==pb1 ? "same\n" : "different\n"); // Output: same
    cout << (pd==pb2 ? "same\n" : "different\n"); // Output: same
    compare(pb1, pd);             // Output: identical
    compare(pb2, pd);             // Output: different
}
```

Godbolt Listing [lst-0023-godb.cpp](lst-0023-godb.cpp), [https://godbolt.org/z/oc7jsbrEK](https://godbolt.org/z/oc7jsbrEK):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/oc7jsbrEK 
#include <iostream>
using std::cout;
struct Base1 {
    virtual void f1() {}
};
struct Base2 {
    virtual void f2() {}
};
struct Derived : public Base1, public Base2 {
    virtual void g() {};
};
void compare(void* a, void* b) {
    cout << (a==b ? "identical\n" : "different\n");
}
int main() {
    Derived d{};
    auto *pd = &d;
    cout << pd << '\n';           // for example 0x1000
    auto pb1 = static_cast<Base1*>(pd);
    cout << pb1 << '\n';          // still 0x1000
    auto pb2 = static_cast<Base2*>(pd);
    cout << pb2 << '\n';          // now 0x1008!
    cout << (pd==pb1 ? "same\n" : "different\n"); // Output: same
    cout << (pd==pb2 ? "same\n" : "different\n"); // Output: same
    compare(pb1, pd);             // Output: identical
    compare(pb2, pd);             // Output: different
}
```

----------------


## Listing 18.17: The observer pattern with multiple inheritance.

Book listing [lst-0024-book.cpp](lst-0024-book.cpp):
```cpp
// https://godbolt.org/z/EnE7Y886M 
#include <iostream>
#include <vector>
using std::cout;
// == Observer Design Pattern ==
struct Observer {
    virtual void update() = 0;
};
class Subject {
    std::vector<Observer*> observers_; // not owning pointers
protected:
    void notify() {
        for (auto o : observers_)
            o->update();
    }
public:
    void addObserver(Observer* o) {
        observers_.push_back(o);
    }
};
// == Concrete Class ==
struct MyThing {
    int calc() { return 1+1; }
};
// == bring together ==
struct MyObservableThing : public MyThing, public Subject {
    int calc() {
        notify();
        return MyThing::calc();
    }
};
// == observe something ==
struct MyObserver : public Observer {
    void update() override {
        cout << "observed\n";
    }
};
int main() {
    MyObserver myObserver{};
    MyObservableThing myObservableThing{};
    myObservableThing.addObserver(&myObserver);
    auto result = myObservableThing.calc(); // Output: observed
}
```

Godbolt Listing [lst-0024-godb.cpp](lst-0024-godb.cpp), [https://godbolt.org/z/EnE7Y886M](https://godbolt.org/z/EnE7Y886M):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/EnE7Y886M 
#include <iostream>
#include <vector>
using std::cout;
// == Observer Design Pattern ==
struct Observer {
    virtual void update() = 0;
};
class Subject {
    std::vector<Observer*> observers_; // not owning pointers
protected:
    void notify() {
        for (auto o : observers_)
            o->update();
    }
public:
    void addObserver(Observer* o) {
        observers_.push_back(o);
    }
};
// == Concrete Class ==
struct MyThing {
    int calc() { return 1+1; }
};
// == bring together ==
struct MyObservableThing : public MyThing, public Subject {
    int calc() {
        notify();
        return MyThing::calc();
    }
};
// == observe something ==
struct MyObserver : public Observer {
    void update() override {
        cout << "observed\n";
    }
};
int main() {
    MyObserver myObserver{};
    MyObservableThing myObservableThing{};
    myObservableThing.addObserver(&myObserver);
    auto result = myObservableThing.calc(); // Output: observed
}
```

----------------


## Listing 18.18: A simple example with virtual inheritance.

Book listing [lst-0025-book.cpp](lst-0025-book.cpp):
```cpp
// https://godbolt.org/z/8jYeYPs7W 
#include <iostream>
using std::cout;
class Base {
public:
  int data_ = 0;
};
class Derived1 : public virtual Base {
};
class Derived2 : public virtual Base {
};
class DerivedDerived : public Derived1, public Derived2 {
public:
  void method() {
     data_ = 1;  // unambiguous, because there is only one data_
  }
};
void output(const DerivedDerived &dd) {
    cout << dd.data_
        << (((Derived1&)dd).data_)
        << (((Derived2&)dd).data_)
        << (((Base&)dd).data_) << '\n';
}
int main() {
  DerivedDerived dd{};
  output(dd);    // Output: 0000
  dd.method();   // sets data_ to 1
  output(dd);    // Output: 1111
}
```

Godbolt Listing [lst-0025-godb.cpp](lst-0025-godb.cpp), [https://godbolt.org/z/8jYeYPs7W](https://godbolt.org/z/8jYeYPs7W):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/8jYeYPs7W 
#include <iostream>
using std::cout;
class Base {
public:
  int data_ = 0;
};
class Derived1 : public virtual Base {
};
class Derived2 : public virtual Base {
};
class DerivedDerived : public Derived1, public Derived2 {
public:
  void method() {
     data_ = 1;  // unambiguous, because there is only one data_
  }
};
void output(const DerivedDerived &dd) {
    cout << dd.data_
        << (((Derived1&)dd).data_)
        << (((Derived2&)dd).data_)
        << (((Base&)dd).data_) << '\n';
}
int main() {
  DerivedDerived dd{};
  output(dd);    // Output: 0000
  dd.method();   // sets data_ to 1
  output(dd);    // Output: 1111
}
```

----------------


## Listing 18.19: Effectively, user() calls a sister method here.

Book listing [lst-0026-book.cpp](lst-0026-book.cpp):
```cpp
// https://godbolt.org/z/s9xnn7c5j 
#include <iostream>
using std::cout;
struct Base {                           // abstract class
  virtual void provider() = 0;
  virtual void user() = 0;
};
struct Derived1 : public virtual Base { // still abstract
  virtual void user() override { provider(); }
};
struct Derived2 : public virtual Base { // still abstract
  virtual void provider() override { cout << "Derived2::provider!\n"; }
};
struct DerivedDerived : public Derived1, public Derived2 { // concrete
};
int main() {
  DerivedDerived dd{};
  DerivedDerived *pdd = &dd;
  Derived1* pd1 = pdd; // Cast within the hierarchy
  Derived2* pd2 = pdd; // Cast within the hierarchy
  pdd->user();         // Output: Derived2::provider()!
  pd1->user();         // Output: Derived2::provider()!
  pd2->user();         // Output: Derived2::provider()!
}
```

Godbolt Listing [lst-0026-godb.cpp](lst-0026-godb.cpp), [https://godbolt.org/z/s9xnn7c5j](https://godbolt.org/z/s9xnn7c5j):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/s9xnn7c5j 
#include <iostream>
using std::cout;
struct Base {                           // abstract class
  virtual void provider() = 0;
  virtual void user() = 0;
};
struct Derived1 : public virtual Base { // still abstract
  virtual void user() override { provider(); }
};
struct Derived2 : public virtual Base { // still abstract
  virtual void provider() override { cout << "Derived2::provider!\n"; }
};
struct DerivedDerived : public Derived1, public Derived2 { // concrete
};
int main() {
  DerivedDerived dd{};
  DerivedDerived *pdd = &dd;
  Derived1* pd1 = pdd; // Cast within the hierarchy
  Derived2* pd2 = pdd; // Cast within the hierarchy
  pdd->user();         // Output: Derived2::provider()!
  pd1->user();         // Output: Derived2::provider()!
  pd2->user();         // Output: Derived2::provider()!
}
```

----------------


## Listing 18.20: You create a literal data type with a constexpr constructor.

Book listing [lst-0029-book.cpp](lst-0029-book.cpp):
```cpp
// https://godbolt.org/z/Kvfx9Yeox 
#include <array>
class Value {
    int val_;
public:
    constexpr Value(int val) : val_{val} {};
    constexpr int get() const { return val_; }
};

constexpr Value five{5}; // works, Value is a literal data type

std::array<int,five.get()> data; // works, get is constexpr
```

Godbolt Listing [lst-0029-godb.cpp](lst-0029-godb.cpp), [https://godbolt.org/z/Kvfx9Yeox](https://godbolt.org/z/Kvfx9Yeox):
```cpp
//#(execute) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/Kvfx9Yeox 
#include <array>
class Value {
    int val_;
public:
    constexpr Value(int val) : val_{val} {};
    constexpr int get() const { return val_; }
};

constexpr Value five{5}; // works, Value is a literal data type

std::array<int,five.get()> data; // works, get is constexpr
```

----------------


## Listing 18.21: User-defined literals are especially useful with literal data types.

Book listing [lst-0030-book.cpp](lst-0030-book.cpp):
```cpp
// https://godbolt.org/z/r1vYc7r5j 
#include <array>
#include <iostream>
#include <type_traits> // is_literal_type
class Value {
    int val_;
public:
    constexpr Value(int val) : val_{val} {};
    constexpr operator int() const { return val_; }
};
namespace lit {
    constexpr Value operator"" _val(const char*, size_t sz) { 
      return Value(sz); }
}
struct Nope {
    constexpr Nope() {};
    virtual ~Nope() {};                 // non-constexpr destructor
};
int main() {
    using namespace lit;
    constexpr Value five{5};
    std::array<int,"11111"_val> data;  // use user-defined literal
    std::cout << data.size() << '\n';                        // Output: 5
    std::cout << std::boolalpha;
    std::cout << std::is_literal_type<Value>::value << '\n'; // Output: true
    std::cout << std::is_literal_type<Nope>::value << '\n';  // Output: false
}
```

Godbolt Listing [lst-0030-godb.cpp](lst-0030-godb.cpp), [https://godbolt.org/z/r1vYc7r5j](https://godbolt.org/z/r1vYc7r5j):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/r1vYc7r5j 
#include <array>
#include <iostream>
#include <type_traits> // is_literal_type
class Value {
    int val_;
public:
    constexpr Value(int val) : val_{val} {};
    constexpr operator int() const { return val_; }
};
namespace lit {
    constexpr Value operator"" _val(const char*, size_t sz) { 
      return Value(sz); }
}
struct Nope {
    constexpr Nope() {};
    virtual ~Nope() {};                 // non-constexpr destructor
};
int main() {
    using namespace lit;
    constexpr Value five{5};
    std::array<int,"11111"_val> data;  // use user-defined literal
    std::cout << data.size() << '\n';                        // Output: 5
    std::cout << std::boolalpha;
    std::cout << std::is_literal_type<Value>::value << '\n'; // Output: true
    std::cout << std::is_literal_type<Nope>::value << '\n';  // Output: false
}
```

----------------

