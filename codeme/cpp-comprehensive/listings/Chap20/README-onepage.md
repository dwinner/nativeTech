# Listings of Chap20.docx

This is the list of listings on one page.
You can also view a [linked summary](README.md).


## GodboltId:4536hxvEK

Book listing [lst-0003-book.cpp](lst-0003-book.cpp):
```cpp
// https://godbolt.org/z/4536hxvEK 
int value = 42;
int& valueRef = value;  // Reference; no address operator & needed
valueRef = 18;          // no dereference operator * needed
cout << value << "\n";  // Output: 18
```

Godbolt Listing [lst-0003-godb.cpp](lst-0003-godb.cpp), [https://godbolt.org/z/4536hxvEK](https://godbolt.org/z/4536hxvEK):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/4536hxvEK 
int value = 42;
int& valueRef = value;  // Reference; no address operator & needed
valueRef = 18;          // no dereference operator * needed
cout << value << "\n";  // Output: 18
```

----------------


## Listing 20.1: Pointers can be assigned new addresses during their lifetime.

Book listing [lst-0004-book.cpp](lst-0004-book.cpp):
```cpp
// https://godbolt.org/z/7GEWYeoc9
#include <vector>
#include <iostream>
using std::vector; using std::cout; using std::ostream;
ostream& printVector(ostream& os, const vector<int> &arg) { // Helper function
    for(int w : arg) os << w << " "; return os;
}
int main() {
    vector<int> values{ };
    values.reserve(50);                  // Guarantee space for 50 values
    int *largest = nullptr;              // Initialize with a special value
    for(int w : { 20, 2, 30, 15, 81, 104, 70, 2, }) {
        values.push_back(w);
        if(!largest || *largest < w ) {  // Dereference to value
            largest = &(values.back());  // Remember new address; hence not '*'
        }
    }
    printVector(cout, values) << "\n";   // Output: 20 2 30 15 81 104 70 2
    // largest now contains the address of 104:
    *largest = -999;                     // dereference; also overwrite value
    printVector(cout, values) << "\n";   // Output: 20 2 30 15 81 –999 70 2
}
```

Godbolt Listing [lst-0004-godb.cpp](lst-0004-godb.cpp), [https://godbolt.org/z/7GEWYeoc9](https://godbolt.org/z/7GEWYeoc9):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/7GEWYeoc9
#include <vector>
#include <iostream>
using std::vector; using std::cout; using std::ostream;
ostream& printVector(ostream& os, const vector<int> &arg) { // Helper function
    for(int w : arg) os << w << " "; return os;
}
int main() {
    vector<int> values{ };
    values.reserve(50);                  // Guarantee space for 50 values
    int *largest = nullptr;              // Initialize with a special value
    for(int w : { 20, 2, 30, 15, 81, 104, 70, 2, }) {
        values.push_back(w);
        if(!largest || *largest < w ) {  // Dereference to value
            largest = &(values.back());  // Remember new address; hence not '*'
        }
    }
    printVector(cout, values) << "\n";   // Output: 20 2 30 15 81 104 70 2
    // largest now contains the address of 104:
    *largest = -999;                     // dereference; also overwrite value
    printVector(cout, values) << "\n";   // Output: 20 2 30 15 81 –999 70 2
}
```

----------------


## Listing 20.2: Returning by value can create a copy.

Book listing [lst-0007-book.cpp](lst-0007-book.cpp):
```cpp
Planet createPlanet(const Event &evt) {  // return by value
    Planet result{"Earth"};              // stack object
    result.setLocation(evt.getPosition());
    return result;                       // return creates (potential) copy
}
```

----------------


## Listing 20.3: Returning as a pointer only copies the pointer, not the object.

Book listing [lst-0008-book.cpp](lst-0008-book.cpp):
```cpp
// return as (smart) pointer:
unique_ptr<Planet> createPlanet(const Event &evt) {
    unique_ptr<Planet> result{ new Planet{"Venus"} };  // heap object
    result->setLocation(evt.getPosition());
    return result; // passes the address
}
```

----------------


## Listing 21.4: In the following examples, I use Image as a class for which I want a pointer.

Book listing [lst-0010-book.cpp](lst-0010-book.cpp):
```cpp
// https://godbolt.org/z/ehhGPjabz 
#include <string>
#include <vector>
class Image {
    std::vector<char> imageData_;
public:
    explicit Image(const std::string& filename) { /* Load image */ }
    void draw() const { /* Paint image */ };
};
```

Godbolt Listing [lst-0010-godb.cpp](lst-0010-godb.cpp), [https://godbolt.org/z/ehhGPjabz](https://godbolt.org/z/ehhGPjabz):
```cpp
//#(execute) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/ehhGPjabz 
#include <string>
#include <vector>
class Image {
    std::vector<char> imageData_;
public:
    explicit Image(const std::string& filename) { /* Load image */ }
    void draw() const { /* Paint image */ };
};
```

----------------


## Listing 20.4: unique_ptr as data field, return value, and local variable.

Book listing [lst-0011-book.cpp](lst-0011-book.cpp):
```cpp
// https://godbolt.org/z/aTTejrnWE
#include <memory>                       // unique_ptr
#include <string>
#include <iostream>
using std::unique_ptr; using std::string;
class Component { };                    // Dummy window hierarchy
class Label : public Component { };
class Textfield : public Component { };
class Button : public Component {
public:
    int id_;                            // ID to distinguish the buttons
    explicit Button(int id) : id_{id} {}
};
class Window { };
class MyDialog : public Window {
    string title_;
    unique_ptr<Label> lblFirstName_{new Label{}};         // lots of data fields
    unique_ptr<Textfield> txtFirstName_{new Textfield{}}; // … tied to the lifetime
    unique_ptr<Label> lblLastName_{new Label{}};          // … of the class
    unique_ptr<Textfield> txtLastName_{new Textfield{}};
    unique_ptr<Button> btnOk_{new Button{1}};
    unique_ptr<Button> btnCancel_{new Button{2}};
public:
    explicit MyDialog(const string& title) : title_{title} {}
    unique_ptr<Button> showModal()
        { return std::move(btnOk_); }    // Placeholder code; OK pressed
};
unique_ptr<MyDialog> createDialog() {
    return unique_ptr<MyDialog>{ // temporary value
        new MyDialog{"Please enter name"}};
}
int showDialog() {
    unique_ptr<MyDialog> dialog = createDialog();         // local variable
    unique_ptr<Button> pressed = dialog->showModal();     // return value
    return pressed->id_;
}
int main() {
    int pressed_id = showDialog();
    if(pressed_id == 1) {
        std::cout << "Thank you for pressing OK\n";
    }
}
```

Godbolt Listing [lst-0011-godb.cpp](lst-0011-godb.cpp), [https://godbolt.org/z/aTTejrnWE](https://godbolt.org/z/aTTejrnWE):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/aTTejrnWE
#include <memory>                       // unique_ptr
#include <string>
#include <iostream>
using std::unique_ptr; using std::string;
class Component { };                    // Dummy window hierarchy
class Label : public Component { };
class Textfield : public Component { };
class Button : public Component {
public:
    int id_;                            // ID to distinguish the buttons
    explicit Button(int id) : id_{id} {}
};
class Window { };
class MyDialog : public Window {
    string title_;
    unique_ptr<Label> lblFirstName_{new Label{}};         // lots of data fields
    unique_ptr<Textfield> txtFirstName_{new Textfield{}}; // … tied to the lifetime
    unique_ptr<Label> lblLastName_{new Label{}};          // … of the class
    unique_ptr<Textfield> txtLastName_{new Textfield{}};
    unique_ptr<Button> btnOk_{new Button{1}};
    unique_ptr<Button> btnCancel_{new Button{2}};
public:
    explicit MyDialog(const string& title) : title_{title} {}
    unique_ptr<Button> showModal()
        { return std::move(btnOk_); }    // Placeholder code; OK pressed
};
unique_ptr<MyDialog> createDialog() {
    return unique_ptr<MyDialog>{ // temporary value
        new MyDialog{"Please enter name"}};
}
int showDialog() {
    unique_ptr<MyDialog> dialog = createDialog();         // local variable
    unique_ptr<Button> pressed = dialog->showModal();     // return value
    return pressed->id_;
}
int main() {
    int pressed_id = showDialog();
    if(pressed_id == 1) {
        std::cout << "Thank you for pressing OK\n";
    }
}
```

----------------


## Listing 20.5: A game board with various objects.

Book listing [lst-0013-book.cpp](lst-0013-book.cpp):
```cpp
// https://godbolt.org/z/4KP91z1bq
#include <vector>
#include <iostream>
#include <memory>                    // shared_ptr
#include <random>                    // uniform_int_distribution, random_device
namespace {                          // Beginning of the anonymous namespace
using std::shared_ptr; using std::make_shared;
using std::vector; using std::cout;
struct Asteroid {
    int points_ = 100;
    int structure_ = 10;
};
struct Ship {
    shared_ptr<Asteroid> firedLastOn_{};
    int score_ = 0;
    int firepower = 1;
    bool fireUpon(shared_ptr<Asteroid> a);
};
struct GameBoard {
    vector<shared_ptr<Asteroid>> asteroids_;
    explicit GameBoard(int nAsteroids);
    bool shipFires(Ship& ship);
};
// Implementation of Ship
bool Ship::fireUpon(shared_ptr<Asteroid> a) {
    if(!a) return false;             // invalid asteroid
    a->structure_ -= firepower;
    if(a.get() == firedLastOn_.get())
        firepower *= 2 ;             // increase damage
    else
        firepower = 1;               // reset
    firedLastOn_ = a;
    return a->structure_ <= 0;       // destroyed?
}
// Implementation of GameBoard
GameBoard::GameBoard(int nAsteroids) : asteroids_{}
{   // some standard asteroids
    for(int idx=0; idx<nAsteroids; ++idx)
        asteroids_.push_back( make_shared<Asteroid>() );
}
int rollDice(int min, int max) {
    /* static std::default_random_engine e{}; */  // Pseudo-random generator
    static std::random_device e{};   // random generator
    return std::uniform_int_distribution<int>{min, max}(e); // roll dice
}
bool GameBoard::shipFires(Ship &ship) {
    int idx = rollDice(0, asteroids_.size()-1);
    bool broken = ship.fireUpon(asteroids_[idx]);
    if(broken) {
        ship.score_ += asteroids_[idx]->points_;
        asteroids_.erase(asteroids_.begin()+idx);  // remove
    }
    return asteroids_.size() == 0;                 // everything is destroyed
}
} // End of the anonymous namespace
int main() {
    GameBoard game{10};                            // 10 asteroids
    Ship ship{};
    for(int idx = 0; idx < 85; ++idx) {            // 85 shots
        if(game.shipFires(ship)) {
            cout << "The space is empty after " << idx+1 << " shots. ";
            break;
        }
    }
    cout << "You have scored " << ship.score_ << " points.\n";
}
```

Godbolt Listing [lst-0013-godb.cpp](lst-0013-godb.cpp), [https://godbolt.org/z/4KP91z1bq](https://godbolt.org/z/4KP91z1bq):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/4KP91z1bq
#include <vector>
#include <iostream>
#include <memory>                    // shared_ptr
#include <random>                    // uniform_int_distribution, random_device
namespace {                          // Beginning of the anonymous namespace
using std::shared_ptr; using std::make_shared;
using std::vector; using std::cout;
struct Asteroid {
    int points_ = 100;
    int structure_ = 10;
};
struct Ship {
    shared_ptr<Asteroid> firedLastOn_{};
    int score_ = 0;
    int firepower = 1;
    bool fireUpon(shared_ptr<Asteroid> a);
};
struct GameBoard {
    vector<shared_ptr<Asteroid>> asteroids_;
    explicit GameBoard(int nAsteroids);
    bool shipFires(Ship& ship);
};
// Implementation of Ship
bool Ship::fireUpon(shared_ptr<Asteroid> a) {
    if(!a) return false;             // invalid asteroid
    a->structure_ -= firepower;
    if(a.get() == firedLastOn_.get())
        firepower *= 2 ;             // increase damage
    else
        firepower = 1;               // reset
    firedLastOn_ = a;
    return a->structure_ <= 0;       // destroyed?
}
// Implementation of GameBoard
GameBoard::GameBoard(int nAsteroids) : asteroids_{}
{   // some standard asteroids
    for(int idx=0; idx<nAsteroids; ++idx)
        asteroids_.push_back( make_shared<Asteroid>() );
}
int rollDice(int min, int max) {
    /* static std::default_random_engine e{}; */  // Pseudo-random generator
    static std::random_device e{};   // random generator
    return std::uniform_int_distribution<int>{min, max}(e); // roll dice
}
bool GameBoard::shipFires(Ship &ship) {
    int idx = rollDice(0, asteroids_.size()-1);
    bool broken = ship.fireUpon(asteroids_[idx]);
    if(broken) {
        ship.score_ += asteroids_[idx]->points_;
        asteroids_.erase(asteroids_.begin()+idx);  // remove
    }
    return asteroids_.size() == 0;                 // everything is destroyed
}
} // End of the anonymous namespace
int main() {
    GameBoard game{10};                            // 10 asteroids
    Ship ship{};
    for(int idx = 0; idx < 85; ++idx) {            // 85 shots
        if(game.shipFires(ship)) {
            cout << "The space is empty after " << idx+1 << " shots. ";
            break;
        }
    }
    cout << "You have scored " << ship.score_ << " points.\n";
}
```

----------------


## Listing 20.6: When a raw pointer owns an object, the potential errors are often hard to detect.

Book listing [lst-0016-book.cpp](lst-0016-book.cpp):
```cpp
// https://godbolt.org/z/Y4sMdzde8
struct StereoImage {
    Image* right_;         // (ERR) raw pointer
    Image* left_;          // (ERR) raw pointer
    StereoImage(const string& nameBase)          // construct
      : right_{new Image{nameBase+"right.jpg"}}  // okay
      , left_{new Image{nameBase+"left.jpg"}}    // dangerous
      { }
    ~StereoImage() {       // remove
        delete right_; delete left_;
    }
    StereoImage(const StereoImage&) = delete;    // no copy
    StereoImage& operator=(const StereoImage&) = delete; // no assignment
};
int main() {
    Image* image = new Image{"image.jpg"};       // (ERR) a raw pointer?
    StereoImage stereo{"3d"};
    delete image;
}
```

Godbolt Listing [lst-0016-godb.cpp](lst-0016-godb.cpp), [https://godbolt.org/z/Y4sMdzde8](https://godbolt.org/z/Y4sMdzde8):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/Y4sMdzde8
struct StereoImage {
    Image* right_;         // (ERR) raw pointer
    Image* left_;          // (ERR) raw pointer
    StereoImage(const string& nameBase)          // construct
      : right_{new Image{nameBase+"right.jpg"}}  // okay
      , left_{new Image{nameBase+"left.jpg"}}    // dangerous
      { }
    ~StereoImage() {       // remove
        delete right_; delete left_;
    }
    StereoImage(const StereoImage&) = delete;    // no copy
    StereoImage& operator=(const StereoImage&) = delete; // no assignment
};
int main() {
    Image* image = new Image{"image.jpg"};       // (ERR) a raw pointer?
    StereoImage stereo{"3d"};
    delete image;
}
```

----------------


## Listing 20.7: "primes" contains pointers to another container.

Book listing [lst-0018-book.cpp](lst-0018-book.cpp):
```cpp
// https://godbolt.org/z/W4W61h4Kv
#include <vector>
#include <numeric>     // iota
#include <iostream>
using std::vector; using std::cout;
struct Number {        // representative of a large, expensive object
    unsigned long val_;
    Number(unsigned long val) : val_{val} {}
    Number() : val_{0} {}
};
/* determines whether z is a prime number based on previous prime numbers */
bool isPrime(const Number& z, const vector<Number*> primes) {
    for(Number* p : primes) {
        if((p->val_*p->val_) > z.val_) return true;   // too large
        if(z.val_ % p->val_ == 0) return false;       // is a divisor
    }
    return true;
}
int main() {
    vector<Number> allNumbers(98);  // 98 zero-initialized elements
    std::iota(begin(allNumbers), end(allNumbers), 3); // 3..100
    /* allNumbers now contains {3..100} */
    vector<Number*> primes{};       // stores determined prime numbers
    Number two{2};
    primes.push_back(&two);         // the 2 is needed
    for(Number &z : allNumbers) {   // iterate over all numbers
        if(isPrime(z, primes)) {
            primes.push_back( &z ); // store address
        }
    }
    /* Output the rest */
    for(Number* p : primes)
        cout << p->val_ << " ";
    cout << "\n";
}
```

Godbolt Listing [lst-0018-godb.cpp](lst-0018-godb.cpp), [https://godbolt.org/z/W4W61h4Kv](https://godbolt.org/z/W4W61h4Kv):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/W4W61h4Kv
#include <vector>
#include <numeric>     // iota
#include <iostream>
using std::vector; using std::cout;
struct Number {        // representative of a large, expensive object
    unsigned long val_;
    Number(unsigned long val) : val_{val} {}
    Number() : val_{0} {}
};
/* determines whether z is a prime number based on previous prime numbers */
bool isPrime(const Number& z, const vector<Number*> primes) {
    for(Number* p : primes) {
        if((p->val_*p->val_) > z.val_) return true;   // too large
        if(z.val_ % p->val_ == 0) return false;       // is a divisor
    }
    return true;
}
int main() {
    vector<Number> allNumbers(98);  // 98 zero-initialized elements
    std::iota(begin(allNumbers), end(allNumbers), 3); // 3..100
    /* allNumbers now contains {3..100} */
    vector<Number*> primes{};       // stores determined prime numbers
    Number two{2};
    primes.push_back(&two);         // the 2 is needed
    for(Number &z : allNumbers) {   // iterate over all numbers
        if(isPrime(z, primes)) {
            primes.push_back( &z ); // store address
        }
    }
    /* Output the rest */
    for(Number* p : primes)
        cout << p->val_ << " ";
    cout << "\n";
}
```

----------------


## Listing 20.8: Do not return addresses of function-local objects.

Book listing [lst-0021-book.cpp](lst-0021-book.cpp):
```cpp
vector<Number*> primePointers(unsigned long upTo) { // (ERR) Vector of pointers is suspicious
    vector<Number> allNumbers;
    vector<Number*> primes{};
    // …
    for(Number &z : allNumbers)
        if(isPrime(z, primes))
            primes.push_back( &z ); // store address
    return primes;   // (ERR) Pointers to function-local objects
}
```

----------------


## Listing 20.9: With raw pointers pointing to a C-array, you can perform arithmetic.

Book listing [lst-0026-book.cpp](lst-0026-book.cpp):
```cpp
// https://godbolt.org/z/a5b6qMo9h
#include <iostream>
int main() {
    int carray[10] = { 1,1 };             // initialized to { 1,1,0,0,0,0,0,0,0,0 }
    int* end = carray+10;                 // pointer past the last element
    for(int* p =carray+2; p != end; ++p) {
        *p = *(p-1) + *(p-2);             // adds the previous two numbers
    }
    for(int const * p=carray; p != end; ++p)
        std::cout << *p << " ";
    std::cout << "\n";
}
```

Godbolt Listing [lst-0026-godb.cpp](lst-0026-godb.cpp), [https://godbolt.org/z/a5b6qMo9h](https://godbolt.org/z/a5b6qMo9h):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/a5b6qMo9h
#include <iostream>
int main() {
    int carray[10] = { 1,1 };             // initialized to { 1,1,0,0,0,0,0,0,0,0 }
    int* end = carray+10;                 // pointer past the last element
    for(int* p =carray+2; p != end; ++p) {
        *p = *(p-1) + *(p-2);             // adds the previous two numbers
    }
    for(int const * p=carray; p != end; ++p)
        std::cout << *p << " ";
    std::cout << "\n";
}
```

----------------


## Listing 20.10: C-arrays decay to raw pointers as parameters.

Book listing [lst-0030-book.cpp](lst-0030-book.cpp):
```cpp
// https://godbolt.org/z/1qx16Gr61 
#include <iostream>
void fibonacci(int data[], int* end) {
    for(int* p = data+2; p != end; ++p) {
        *p = *(p-1) + *(p-2);
    }
}
std::ostream& print(std::ostream &os, int data[], int* end) {
    for(int const * p=data; p != end; ++p)
        std::cout << *p << " ";
    return os;
}
int main() {
    int carray[10] = { 1,1 }; // initialized to { 1,1,0,0,0,0,0,0,0,0 }
    fibonacci(carray, carray+10);
    print(std::cout, carray, carray+10) << "\n";
}
```

Godbolt Listing [lst-0030-godb.cpp](lst-0030-godb.cpp), [https://godbolt.org/z/1qx16Gr61](https://godbolt.org/z/1qx16Gr61):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/1qx16Gr61 
#include <iostream>
void fibonacci(int data[], int* end) {
    for(int* p = data+2; p != end; ++p) {
        *p = *(p-1) + *(p-2);
    }
}
std::ostream& print(std::ostream &os, int data[], int* end) {
    for(int const * p=data; p != end; ++p)
        std::cout << *p << " ";
    return os;
}
int main() {
    int carray[10] = { 1,1 }; // initialized to { 1,1,0,0,0,0,0,0,0,0 }
    fibonacci(carray, carray+10);
    print(std::cout, carray, carray+10) << "\n";
}
```

----------------


## Listing 20.11: "unique_ptr" works with the dynamic C-array.

Book listing [lst-0034-book.cpp](lst-0034-book.cpp):
```cpp
// https://godbolt.org/z/h3q839x8r
#include <memory>      // unique_ptr
#include <iostream>    // cout
std::unique_ptr<int[]> createData(size_t sz) {
    return std::unique_ptr<int[]>(new int[sz]);
}
void fibonacci(int data[], int* end) {
    for(int* p = data+2; p != end; ++p) {
        *p = *(p-1) + *(p-2);
    }
}
std::ostream& print(std::ostream &os, int data[], int* end) {
    for(int const* p = data; p != end; ++p)
        std::cout << *p << " ";
    return os;
}
int main() {
    std::unique_ptr<int[]> data { createData(10) };
    data[0] = 1; // set values in the array through the unique_ptr
    data[1] = 1;
    fibonacci(data.get(), data.get()+10); // get the C-array pointer with get()
    print(std::cout, data.get(), data.get()+10) << "\n";
}
```

Godbolt Listing [lst-0034-godb.cpp](lst-0034-godb.cpp), [https://godbolt.org/z/h3q839x8r](https://godbolt.org/z/h3q839x8r):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/h3q839x8r
#include <memory>      // unique_ptr
#include <iostream>    // cout
std::unique_ptr<int[]> createData(size_t sz) {
    return std::unique_ptr<int[]>(new int[sz]);
}
void fibonacci(int data[], int* end) {
    for(int* p = data+2; p != end; ++p) {
        *p = *(p-1) + *(p-2);
    }
}
std::ostream& print(std::ostream &os, int data[], int* end) {
    for(int const* p = data; p != end; ++p)
        std::cout << *p << " ";
    return os;
}
int main() {
    std::unique_ptr<int[]> data { createData(10) };
    data[0] = 1; // set values in the array through the unique_ptr
    data[1] = 1;
    fibonacci(data.get(), data.get()+10); // get the C-array pointer with get()
    print(std::cout, data.get(), data.get()+10) << "\n";
}
```

----------------


## GodboltId:s6sss481E

Book listing [lst-0038-book.cpp](lst-0038-book.cpp):
```cpp
// https://godbolt.org/z/s6sss481E 
const char vimes[13] = "Samuel Vimes"; // const char[13]
const char colon[] = "Fred Colon";     // const char[11]
const char* nobby = "Nobby Nobbs";     // const char[12]
```

Godbolt Listing [lst-0038-godb.cpp](lst-0038-godb.cpp), [https://godbolt.org/z/s6sss481E](https://godbolt.org/z/s6sss481E):
```cpp
//#(execute) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/s6sss481E 
const char vimes[13] = "Samuel Vimes"; // const char[13]
const char colon[] = "Fred Colon";     // const char[11]
const char* nobby = "Nobby Nobbs";     // const char[12]
```

----------------


## Listing 20.12: With "string", you often don't even notice that text literals are "const char[]".

Book listing [lst-0039-book.cpp](lst-0039-book.cpp):
```cpp
// https://godbolt.org/z/oKsz7d55T 
#include <string>
#include <iostream>                    // cout
using std::string; using std::cout;
string greet(string name) {
    return name + "!";                 // string operator+(string, const char*)
}
int main() {
    string name{ "Havalock Vetinari" };// explicit: string(const char*)
    cout << "Angua";                   // ostream& operator<<(ostream&, const char*)
    cout <<                            // ostream& operator<<(ostream, string)
      greet("Carrot Ironfoundersson"); // implicitly: string(const char*)
}
```

Godbolt Listing [lst-0039-godb.cpp](lst-0039-godb.cpp), [https://godbolt.org/z/oKsz7d55T](https://godbolt.org/z/oKsz7d55T):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/oKsz7d55T 
#include <string>
#include <iostream>                    // cout
using std::string; using std::cout;
string greet(string name) {
    return name + "!";                 // string operator+(string, const char*)
}
int main() {
    string name{ "Havalock Vetinari" };// explicit: string(const char*)
    cout << "Angua";                   // ostream& operator<<(ostream&, const char*)
    cout <<                            // ostream& operator<<(ostream, string)
      greet("Carrot Ironfoundersson"); // implicitly: string(const char*)
}
```

----------------


## Listing 20.13: Retrieve iterators with "begin" and "end".

Book listing [lst-0040-book.cpp](lst-0040-book.cpp):
```cpp
// https://godbolt.org/z/4nqz4ed4W 
#include <vector>
#include <iostream> // cout

using std::vector;
int main() {
    vector data{ 5,4,3,2,1 };
    vector<int>::const_iterator end = data.end(); // or end(data)
    for(vector<int>::const_iterator it = data.begin(); it!=end; ++it) {
        std::cout << *it << " ";
    }
    std::cout << "\n";
}
```

Godbolt Listing [lst-0040-godb.cpp](lst-0040-godb.cpp), [https://godbolt.org/z/4nqz4ed4W](https://godbolt.org/z/4nqz4ed4W):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/4nqz4ed4W 
#include <vector>
#include <iostream> // cout

using std::vector;
int main() {
    vector data{ 5,4,3,2,1 };
    vector<int>::const_iterator end = data.end(); // or end(data)
    for(vector<int>::const_iterator it = data.begin(); it!=end; ++it) {
        std::cout << *it << " ";
    }
    std::cout << "\n";
}
```

----------------


## Listing 20.14: You can use raw pointers like iterators.

Book listing [lst-0041-book.cpp](lst-0041-book.cpp):
```cpp
// https://godbolt.org/z/zsqKz76cj 
#include <iostream>   // cout
#include <iterator>   // ostream_iterator
#include <algorithm>  // copy, ranges::copy
int main () {
  int data[6] = { 1, 2, 3, 7, 9, 10 };
  std::ostream_iterator<int> out_it (std::cout,", ");
  std::copy(data, data+6, out_it);            // Pointers as iterators
  std::cout << "\n";                          // Output: 1, 2, 3, 7, 9, 10
  std::ranges::copy(data, out_it);            // C-array pointer as range
  std::cout << "\n";                          // Output: 1, 2, 3, 7, 9, 10
}
```

Godbolt Listing [lst-0041-godb.cpp](lst-0041-godb.cpp), [https://godbolt.org/z/zsqKz76cj](https://godbolt.org/z/zsqKz76cj):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/zsqKz76cj 
#include <iostream>   // cout
#include <iterator>   // ostream_iterator
#include <algorithm>  // copy, ranges::copy
int main () {
  int data[6] = { 1, 2, 3, 7, 9, 10 };
  std::ostream_iterator<int> out_it (std::cout,", ");
  std::copy(data, data+6, out_it);            // Pointers as iterators
  std::cout << "\n";                          // Output: 1, 2, 3, 7, 9, 10
  std::ranges::copy(data, out_it);            // C-array pointer as range
  std::cout << "\n";                          // Output: 1, 2, 3, 7, 9, 10
}
```

----------------


## Listing 20.15: Is it worth saving the release of memory?

Book listing [lst-0042-book.cpp](lst-0042-book.cpp):
```cpp
// https://godbolt.org/z/xE1TWx1ve
#include <map>
#include <memory> // unique_ptr
#include <string>
#include <iostream>
#include <chrono> // time measurement
using std::map; using std::cout; using std::endl; using namespace std::chrono;
struct Node {
  std::unique_ptr<int> d_;
  Node() : Node{0}  { }
  explicit Node(int d) : d_{ new int } { *d_ = d; }     // also some memory
  friend bool operator<(const Node& a, const Node& b) {return *a.d_<*b.d_;}
  friend bool operator==(const Node& a, const Node& b) {return *a.d_==*b.d_;}
};
long long millisSince(steady_clock::time_point start) { // Helper for time measurement
  return duration_cast<milliseconds>(steady_clock::now()-start).count();
}

int main() {
  std::unique_ptr<map<int,Node>> huge{ new map<int,Node>{} };
  cout << "Building..." << endl;
  steady_clock::time_point start = steady_clock::now();
  for(int idx=0; idx < 100*1000*1000; ++idx) {   // massive amount in the map
      (*huge)[idx] = Node{idx};
  }
  cout << "Done: " << millisSince(start) << " ms" << endl; // timing here
  start = steady_clock::now();
  huge.reset();                                            // cleanup here
  cout << "End: " << millisSince(start) << " ms" << endl;  // timing here
}
```

Godbolt Listing [lst-0042-godb.cpp](lst-0042-godb.cpp), [https://godbolt.org/z/xE1TWx1ve](https://godbolt.org/z/xE1TWx1ve):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23 -Wall -Wextra; libs:-
// https://godbolt.org/z/xE1TWx1ve
#include <map>
#include <memory> // unique_ptr
#include <string>
#include <iostream>
#include <chrono> // time measurement
using std::map; using std::cout; using std::endl; using namespace std::chrono;
struct Node {
  std::unique_ptr<int> d_;
  Node() : Node{0}  { }
  explicit Node(int d) : d_{ new int } { *d_ = d; }     // also some memory
  friend bool operator<(const Node& a, const Node& b) {return *a.d_<*b.d_;}
  friend bool operator==(const Node& a, const Node& b) {return *a.d_==*b.d_;}
};
long long millisSince(steady_clock::time_point start) { // Helper for time measurement
  return duration_cast<milliseconds>(steady_clock::now()-start).count();
}

int main() {
  std::unique_ptr<map<int,Node>> huge{ new map<int,Node>{} };
  cout << "Building..." << endl;
  steady_clock::time_point start = steady_clock::now();
  for(int idx=0; idx < 100*1000*1000; ++idx) {   // massive amount in the map
      (*huge)[idx] = Node{idx};
  }
  cout << "Done: " << millisSince(start) << " ms" << endl; // timing here
  start = steady_clock::now();
  huge.reset();                                            // cleanup here
  cout << "End: " << millisSince(start) << " ms" << endl;  // timing here
}
```

----------------

