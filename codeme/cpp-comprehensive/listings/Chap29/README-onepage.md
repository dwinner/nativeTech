# Listings of Chap29.docx

This is the list of listings on one page.
You can also view a [linked summary](README.md).


## Listing 29.1: How to start a thread.

Book listing [lst-0001-book.cpp](lst-0001-book.cpp):
```cpp
// https://godbolt.org/z/qnadYPWh1 
#include <iostream>
#include <thread>
using std::cout; using std::endl;

long fib(long n) { return n<=1 ? n : fib(n-1)+fib(n-2); }
void task1() { auto r = fib(40); cout << "fib(40)=" << r << endl; }
void task2() { auto r = fib(41); cout << "fib(41)=" << r << endl; }
void task3() { auto r = fib(42); cout << "fib(42)=" << r << endl; }

struct BackgroundTask {
    void operator()() const {
        task1();
        task2();
        task3();
    }
};

int main() {
    BackgroundTask backgroundTask{}; // Initialization, does not compute anything yet
    std::jthread myThread{ backgroundTask }; // Computation starts
}
```

Godbolt Listing [lst-0001-godb.cpp](lst-0001-godb.cpp), [https://godbolt.org/z/qnadYPWh1](https://godbolt.org/z/qnadYPWh1):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/qnadYPWh1 
#include <iostream>
#include <thread>
using std::cout; using std::endl;

long fib(long n) { return n<=1 ? n : fib(n-1)+fib(n-2); }
void task1() { auto r = fib(40); cout << "fib(40)=" << r << endl; }
void task2() { auto r = fib(41); cout << "fib(41)=" << r << endl; }
void task3() { auto r = fib(42); cout << "fib(42)=" << r << endl; }

struct BackgroundTask {
    void operator()() const {
        task1();
        task2();
        task3();
    }
};

int main() {
    BackgroundTask backgroundTask{}; // Initialization, does not compute anything yet
    std::jthread myThread{ backgroundTask }; // Computation starts
}
```

----------------


## GodboltId:nPGY47r4K

Book listing [lst-0003-book.cpp](lst-0003-book.cpp):
```cpp
// https://godbolt.org/z/nPGY47r4K 
std::jthread myThread{ [] {
    task1();
    task2();
    task3();
} };
```

Godbolt Listing [lst-0003-godb.cpp](lst-0003-godb.cpp), [https://godbolt.org/z/nPGY47r4K](https://godbolt.org/z/nPGY47r4K):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/nPGY47r4K 
std::jthread myThread{ [] {
    task1();
    task2();
    task3();
} };
```

----------------


## Listing 29.2: With "stop_token", the outside world communicates into the thread.

Book listing [lst-0004-book.cpp](lst-0004-book.cpp):
```cpp
// https://godbolt.org/z/6cqd7Tsae 
struct BackgroundTask {
    void operator()(std::stop_token st) const { // Token for communication
        task1();
        if(st.stop_requested()) return;
        task2();
        if(st.stop_requested()) return;
        task3();
    }
};
int main() {
    BackgroundTask backgroundTask{};
    std::jthread myThread{ backgroundTask };
    std::this_thread::sleep_for(std::chrono::milliseconds(100)); // wait 100ms
    myThread.request_stop(); // request the thread to stop
}
```

Godbolt Listing [lst-0004-godb.cpp](lst-0004-godb.cpp), [https://godbolt.org/z/6cqd7Tsae](https://godbolt.org/z/6cqd7Tsae):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/6cqd7Tsae 
struct BackgroundTask {
    void operator()(std::stop_token st) const { // Token for communication
        task1();
        if(st.stop_requested()) return;
        task2();
        if(st.stop_requested()) return;
        task3();
    }
};
int main() {
    BackgroundTask backgroundTask{};
    std::jthread myThread{ backgroundTask };
    std::this_thread::sleep_for(std::chrono::milliseconds(100)); // wait 100ms
    myThread.request_stop(); // request the thread to stop
}
```

----------------


## GodboltId:Tzs93MzfK

Book listing [lst-0005-book.cpp](lst-0005-book.cpp):
```cpp
// https://godbolt.org/z/Tzs93MzfK 
std::thread myThread{ [] {  // pure thread
    task1();
    task2();
    task3();
} };
myThread.join();   // waits for the thread to finish
```

Godbolt Listing [lst-0005-godb.cpp](lst-0005-godb.cpp), [https://godbolt.org/z/Tzs93MzfK](https://godbolt.org/z/Tzs93MzfK):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/Tzs93MzfK 
std::thread myThread{ [] {  // pure thread
    task1();
    task2();
    task3();
} };
myThread.join();   // waits for the thread to finish
```

----------------


## GodboltId:E49on6zn3

Book listing [lst-0006-book.cpp](lst-0006-book.cpp):
```cpp
// https://godbolt.org/z/E49on6zn3
#include <iostream>
#include <thread>
#include <vector>
#include <exception>
using std::cout; using std::endl;

long fib(long n) { return n<=1 ? n : fib(n-1)+fib(n-2); }
void task1() { auto r = fib(40); cout << "fib(40)=" << r << endl; }

void main_program() {
    try {
        std::thread th{ &task1 };
        std::vector data{ 0,1,2 };
        data.at(666);                     // (ERR) triggers out_of_range
        th.join();                        // would wait
    } catch(std::runtime_error &ex) {     // (ERR) does not match out_of_range
         /*...*/
    }
}

int main() {
  try {
    main_program();
  } catch( ... ) {                        // so far, so good, looks safe
    std::cout << "An error has occurred\n"; // you won't see this
  }
}
```

Godbolt Listing [lst-0006-godb.cpp](lst-0006-godb.cpp), [https://godbolt.org/z/E49on6zn3](https://godbolt.org/z/E49on6zn3):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23 -Wno-unused-result; libs:boost@184
// https://godbolt.org/z/E49on6zn3
#include <iostream>
#include <thread>
#include <vector>
#include <exception>
using std::cout; using std::endl;

long fib(long n) { return n<=1 ? n : fib(n-1)+fib(n-2); }
void task1() { auto r = fib(40); cout << "fib(40)=" << r << endl; }

void main_program() {
    try {
        std::thread th{ &task1 };
        std::vector data{ 0,1,2 };
        data.at(666);                     // (ERR) triggers out_of_range
        th.join();                        // would wait
    } catch(std::runtime_error &ex) {     // (ERR) does not match out_of_range
         /*...*/
    }
}

int main() {
  try {
    main_program();
  } catch( ... ) {                        // so far, so good, looks safe
    std::cout << "An error has occurred\n"; // you won't see this
  }
}
```

----------------


## GodboltId:T4c1GnjY4

Book listing [lst-0007-book.cpp](lst-0007-book.cpp):
```cpp
// https://godbolt.org/z/T4c1GnjY4
#include <iostream>
#include <thread>
#include <vector>
#include <exception>
using std::cout; using std::endl;

long fib(long n) { return n<=1 ? n : fib(n-1)+fib(n-2); }
void task1() { auto r = fib(40); cout << "fib(40)=" << r << endl; }

void main_program() {
    std::thread th{ &task1 };
    try {
        std::vector data{ 0,1,2 };
        data.at(666);                 // (ERR) triggers out_of_range
    } catch(std::runtime_error &ex) { // does not match out_of_range
         /* ... */                    // handle specific error here
    } catch( ... ) {
        th.join();
        throw;                        // continue error handling outside
    }
    th.join();                        // waits after Okay or specific error
}

int main() {
    try {
        main_program();
    } catch( ... ) {
        std::cout << "An error has occurred\n";
    }
}
```

Godbolt Listing [lst-0007-godb.cpp](lst-0007-godb.cpp), [https://godbolt.org/z/T4c1GnjY4](https://godbolt.org/z/T4c1GnjY4):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23 -Wno-unused-result; libs:boost@184
// https://godbolt.org/z/T4c1GnjY4
#include <iostream>
#include <thread>
#include <vector>
#include <exception>
using std::cout; using std::endl;

long fib(long n) { return n<=1 ? n : fib(n-1)+fib(n-2); }
void task1() { auto r = fib(40); cout << "fib(40)=" << r << endl; }

void main_program() {
    std::thread th{ &task1 };
    try {
        std::vector data{ 0,1,2 };
        data.at(666);                 // (ERR) triggers out_of_range
    } catch(std::runtime_error &ex) { // does not match out_of_range
         /* ... */                    // handle specific error here
    } catch( ... ) {
        th.join();
        throw;                        // continue error handling outside
    }
    th.join();                        // waits after Okay or specific error
}

int main() {
    try {
        main_program();
    } catch( ... ) {
        std::cout << "An error has occurred\n";
    }
}
```

----------------


## GodboltId:8TdzEGf3d

Book listing [lst-0008-book.cpp](lst-0008-book.cpp):
```cpp
// https://godbolt.org/z/8TdzEGf3d 
void mainProgram() {
    std::jthread th{ &task1 };
    std::vector data{ 0,1,2 };
    data.at(666);                  // (ERR) triggers out_of_range
}
```

Godbolt Listing [lst-0008-godb.cpp](lst-0008-godb.cpp), [https://godbolt.org/z/8TdzEGf3d](https://godbolt.org/z/8TdzEGf3d):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/8TdzEGf3d 
void mainProgram() {
    std::jthread th{ &task1 };
    std::vector data{ 0,1,2 };
    data.at(666);                  // (ERR) triggers out_of_range
}
```

----------------


## Listing 29.3: Add parameters to the thread function in the constructor.

Book listing [lst-0009-book.cpp](lst-0009-book.cpp):
```cpp
// https://godbolt.org/z/bMdrWrd4c
#include <iostream>
#include <thread>
using std::cout; using std::endl;

long fib(long n) { return n<=1 ? n : fib(n-1)+fib(n-2); }
void runFib(long n) {
    auto r = fib(n);
    cout << "fib("<<n<<")=" << r << endl;
}
long ack(long m, long n) { // Ackermann function
    if(m==0) return n+1;
    if(n==0) return ack(m-1, 1);
    return ack(m - 1, ack(m, n-1));
}
void runAck(long m, long n) {
    auto r = ack(m, n);
    cout << "ack("<<m<<','<<n<<")=" << r << endl;
}

int main() {
    std::jthread f40{ runFib, 40 };
    std::jthread f41{ runFib, 41 };
    std::jthread f42{ runFib, 42 };

    f40.join(); f41.join(); f42.join();

    std::thread a1{ runAck, 4, 0 };
    std::thread a2{ runAck, 4, 1 };
    std::thread a3{ runAck, 2, 700 };
    std::thread a4{ runAck, 3, 10 };
}
```

Godbolt Listing [lst-0009-godb.cpp](lst-0009-godb.cpp), [https://godbolt.org/z/bMdrWrd4c](https://godbolt.org/z/bMdrWrd4c):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/bMdrWrd4c
#include <iostream>
#include <thread>
using std::cout; using std::endl;

long fib(long n) { return n<=1 ? n : fib(n-1)+fib(n-2); }
void runFib(long n) {
    auto r = fib(n);
    cout << "fib("<<n<<")=" << r << endl;
}
long ack(long m, long n) { // Ackermann function
    if(m==0) return n+1;
    if(n==0) return ack(m-1, 1);
    return ack(m - 1, ack(m, n-1));
}
void runAck(long m, long n) {
    auto r = ack(m, n);
    cout << "ack("<<m<<','<<n<<")=" << r << endl;
}

int main() {
    std::jthread f40{ runFib, 40 };
    std::jthread f41{ runFib, 41 };
    std::jthread f42{ runFib, 42 };

    f40.join(); f41.join(); f42.join();

    std::thread a1{ runAck, 4, 0 };
    std::thread a2{ runAck, 4, 1 };
    std::thread a3{ runAck, 2, 700 };
    std::thread a4{ runAck, 3, 10 };
}
```

----------------


## Listing 29.4: Parameters are copied into the thread.

Book listing [lst-0010-book.cpp](lst-0010-book.cpp):
```cpp
// https://godbolt.org/z/fbnbKzrs1 
#include <iostream>
#include <thread>
#include <chrono>
using namespace std::chrono; // seconds, suffix s

void delayPrint(seconds s, const std::string& msg) {
    std::this_thread::sleep_for(s);
    std::cout << msg << std::endl;
}
int main() {
    std::jthread m1{ delayPrint, 1s, "On your marks" };
    std::jthread m2{ delayPrint, 2s, std::string{"set"} };
    std::string go = "go";
    std::jthread m3{ delayPrint, 3s, go };
}
```

Godbolt Listing [lst-0010-godb.cpp](lst-0010-godb.cpp), [https://godbolt.org/z/fbnbKzrs1](https://godbolt.org/z/fbnbKzrs1):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/fbnbKzrs1 
#include <iostream>
#include <thread>
#include <chrono>
using namespace std::chrono; // seconds, suffix s

void delayPrint(seconds s, const std::string& msg) {
    std::this_thread::sleep_for(s);
    std::cout << msg << std::endl;
}
int main() {
    std::jthread m1{ delayPrint, 1s, "On your marks" };
    std::jthread m2{ delayPrint, 2s, std::string{"set"} };
    std::string go = "go";
    std::jthread m3{ delayPrint, 3s, go };
}
```

----------------


## Listing 29.5: Use caution with raw pointers as parameters.

Book listing [lst-0011-book.cpp](lst-0011-book.cpp):
```cpp
// https://godbolt.org/z/hvq5dbzde 
#include <iostream>
#include <thread>
#include <chrono>
using namespace std::chrono; // seconds, suffix s

void delayPrint(seconds s, const char* msg) {  // (ERR) raw pointer
    std::this_thread::sleep_for(s);
    std::cout << msg << std::endl;             // (ERR) this won't work
}

void run() {
    const char risk[] = "This won't end well...";
    std::jthread t{ delayPrint, 1s, risk };    // (ERR) raw pointer
    t.detach();
    // here the scope of 'risk' is left
}
int main() {
    run();
    std::this_thread::sleep_for(2s);           // wait another 2 seconds
}
```

Godbolt Listing [lst-0011-godb.cpp](lst-0011-godb.cpp), [https://godbolt.org/z/hvq5dbzde](https://godbolt.org/z/hvq5dbzde):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/hvq5dbzde 
#include <iostream>
#include <thread>
#include <chrono>
using namespace std::chrono; // seconds, suffix s

void delayPrint(seconds s, const char* msg) {  // (ERR) raw pointer
    std::this_thread::sleep_for(s);
    std::cout << msg << std::endl;             // (ERR) this won't work
}

void run() {
    const char risk[] = "This won't end well...";
    std::jthread t{ delayPrint, 1s, risk };    // (ERR) raw pointer
    t.detach();
    // here the scope of 'risk' is left
}
int main() {
    run();
    std::this_thread::sleep_for(2s);           // wait another 2 seconds
}
```

----------------


## Listing 29.6: Forcing a reference with ref.

Book listing [lst-0012-book.cpp](lst-0012-book.cpp):
```cpp
// https://godbolt.org/z/GnsWYvecv 
#include <iostream>
#include <thread>
#include <chrono>
using namespace std::chrono; // seconds, suffix s
struct State {
    int counter;
};
void showState(const State& state) {
    for(auto i : { 5,4,3,2,1 }) {
        std::cout << "counter: " << state.counter << std::endl;
        std::this_thread::sleep_for(1s);
    }
}
int main() {
    State state { 4 };
    std::jthread th{showState, std::ref(state)}; // remains reference to state
    std::this_thread::sleep_for(1s);
    state.counter = 501;
    std::this_thread::sleep_for(1s);
    state.counter = 87;
    std::this_thread::sleep_for(1s);
    state.counter = 2;
}
```

Godbolt Listing [lst-0012-godb.cpp](lst-0012-godb.cpp), [https://godbolt.org/z/GnsWYvecv](https://godbolt.org/z/GnsWYvecv):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/GnsWYvecv 
#include <iostream>
#include <thread>
#include <chrono>
using namespace std::chrono; // seconds, suffix s
struct State {
    int counter;
};
void showState(const State& state) {
    for(auto i : { 5,4,3,2,1 }) {
        std::cout << "counter: " << state.counter << std::endl;
        std::this_thread::sleep_for(1s);
    }
}
int main() {
    State state { 4 };
    std::jthread th{showState, std::ref(state)}; // remains reference to state
    std::this_thread::sleep_for(1s);
    state.counter = 501;
    std::this_thread::sleep_for(1s);
    state.counter = 87;
    std::this_thread::sleep_for(1s);
    state.counter = 2;
}
```

----------------


## Listing 29.7: Using "move" to transfer inputs to the thread.

Book listing [lst-0013-book.cpp](lst-0013-book.cpp):
```cpp
// https://godbolt.org/z/Pjzqsv9nz
// … includes …
#include <thread>
using namespace std::chrono; // seconds, suffix s
struct Image {
    std::vector<char> data_; // Copying expensive
    explicit Image() : data_(1'000'000) {}
};
void showImage(Image img) {
    std::cout << img.data_.size() << '\n';
}
void showIptr(std::unique_ptr<int> iptr) {
    std::cout << *iptr << '\n';
}
int main() {
    // expensive to copy, but good to move:
    Image image{};
    std::cout << image.data_.size() << std::endl;    // Output: 1000000
    std::jthread th1{ showImage, std::move(image) }; // Output: 1000000
    std::this_thread::sleep_for(1s);
    std::cout << image.data_.size() << std::endl;    // Output: 0
    th1.join();  // explicitly wait until the thread is done
    // impossible to copy, but good to move:
    auto iptr = std::make_unique<int>( 657 );
    std::cout << (bool)iptr << std::endl;            // Output: 1 for true
    std::jthread th2{ showIptr, std::move(iptr) };   // Output: 657
    std::this_thread::sleep_for(1s);
    std::cout << (bool)iptr.get() << std::endl;      // Output: 0 for false
}
```

Godbolt Listing [lst-0013-godb.cpp](lst-0013-godb.cpp), [https://godbolt.org/z/Pjzqsv9nz](https://godbolt.org/z/Pjzqsv9nz):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23 -Wno-unused-result; libs:boost@184
// https://godbolt.org/z/Pjzqsv9nz
// … includes …
#include <thread>
using namespace std::chrono; // seconds, suffix s
struct Image {
    std::vector<char> data_; // Copying expensive
    explicit Image() : data_(1'000'000) {}
};
void showImage(Image img) {
    std::cout << img.data_.size() << '\n';
}
void showIptr(std::unique_ptr<int> iptr) {
    std::cout << *iptr << '\n';
}
int main() {
    // expensive to copy, but good to move:
    Image image{};
    std::cout << image.data_.size() << std::endl;    // Output: 1000000
    std::jthread th1{ showImage, std::move(image) }; // Output: 1000000
    std::this_thread::sleep_for(1s);
    std::cout << image.data_.size() << std::endl;    // Output: 0
    th1.join();  // explicitly wait until the thread is done
    // impossible to copy, but good to move:
    auto iptr = std::make_unique<int>( 657 );
    std::cout << (bool)iptr << std::endl;            // Output: 1 for true
    std::jthread th2{ showIptr, std::move(iptr) };   // Output: 657
    std::this_thread::sleep_for(1s);
    std::cout << (bool)iptr.get() << std::endl;      // Output: 0 for false
}
```

----------------


## Listing 29.8: Returning a thread.

Book listing [lst-0014-book.cpp](lst-0014-book.cpp):
```cpp
// https://godbolt.org/z/Kv84eb96E 
#include <iostream>
#include <thread>
#include <chrono>
using namespace std::chrono; // seconds, suffix s
auto makeThread(std::string who) {
    return std::jthread{ [who] {
        std::this_thread::sleep_for(1s);
        std::cout << "Good luck, " << who << std::endl;
    } };
}
int main() {
    auto th = makeThread("Jim"); // Output: Good luck, Jim
    th.join();
}
```

Godbolt Listing [lst-0014-godb.cpp](lst-0014-godb.cpp), [https://godbolt.org/z/Kv84eb96E](https://godbolt.org/z/Kv84eb96E):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/Kv84eb96E 
#include <iostream>
#include <thread>
#include <chrono>
using namespace std::chrono; // seconds, suffix s
auto makeThread(std::string who) {
    return std::jthread{ [who] {
        std::this_thread::sleep_for(1s);
        std::cout << "Good luck, " << who << std::endl;
    } };
}
int main() {
    auto th = makeThread("Jim"); // Output: Good luck, Jim
    th.join();
}
```

----------------


## Listing 29.9: Moving a thread.

Book listing [lst-0015-book.cpp](lst-0015-book.cpp):
```cpp
// https://godbolt.org/z/8cMTrEWz6
#include <thread>
#include <chrono>
using namespace std::chrono; // seconds, suffix s
void yourMission(std::jthread job) {
    job.join();
}
int main() {
     std::jthread th{ [] {
        std::this_thread::sleep_for(1s);
        std::cout << "should you choose to accept it" << std::endl;
    } };
    yourMission( std::move(th) );  // Transfer responsibility
}
```

Godbolt Listing [lst-0015-godb.cpp](lst-0015-godb.cpp), [https://godbolt.org/z/8cMTrEWz6](https://godbolt.org/z/8cMTrEWz6):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23 -Wno-unused-result; libs:boost@184
// https://godbolt.org/z/8cMTrEWz6
#include <thread>
#include <chrono>
using namespace std::chrono; // seconds, suffix s
void yourMission(std::jthread job) {
    job.join();
}
int main() {
     std::jthread th{ [] {
        std::this_thread::sleep_for(1s);
        std::cout << "should you choose to accept it" << std::endl;
    } };
    yourMission( std::move(th) );  // Transfer responsibility
}
```

----------------


## Listing 29.10: Threads in containers.

Book listing [lst-0016-book.cpp](lst-0016-book.cpp):
```cpp
// https://godbolt.org/z/YrEPnYMaP 
#include <iostream>
#include <thread>
#include <vector>
using std::cout; using std::endl;
long fib(long n) { return n<=1 ? n : fib(n-1)+fib(n-2); }
void runFib(long n) { auto r = fib(n); cout << "fib("<<n<<")=" << r << endl; }
int main() {
    std::vector<std::jthread> threads;
    // starten
    for( auto n : { 38, 39, 40, 41, 42, 43, }) {
        threads.emplace_back( runFib, n );
    }
}
```

Godbolt Listing [lst-0016-godb.cpp](lst-0016-godb.cpp), [https://godbolt.org/z/YrEPnYMaP](https://godbolt.org/z/YrEPnYMaP):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/YrEPnYMaP 
#include <iostream>
#include <thread>
#include <vector>
using std::cout; using std::endl;
long fib(long n) { return n<=1 ? n : fib(n-1)+fib(n-2); }
void runFib(long n) { auto r = fib(n); cout << "fib("<<n<<")=" << r << endl; }
int main() {
    std::vector<std::jthread> threads;
    // starten
    for( auto n : { 38, 39, 40, 41, 42, 43, }) {
        threads.emplace_back( runFib, n );
    }
}
```

----------------


## Listing 29.11: Finding out the hardware concurrency.

Book listing [lst-0019-book.cpp](lst-0019-book.cpp):
```cpp
// https://godbolt.org/z/zojG68dWq
#include <thread>
#include <iostream>
#include <vector>
#include <chrono>  // steady_clock
using std::cout; using std::endl; using namespace std::chrono;
long fib(long n) { return n<=1 ? n : fib(n-1)+fib(n-2); }

int main() {
  cout << std::thread::hardware_concurrency() << '\n';
  for(int nthreads : { 1,2,3,4,5,6 }) {
    cout << "Threads: ";
    const auto start = steady_clock::now();

    std::vector<std::jthread> threads;
    for(int ti = 1; ti <= nthreads; ++ti) {
      threads.emplace_back( std::jthread{fib, 40});
      cout << ti << "... "; cout.flush();
    }
    for(auto &th : threads) th.join(); // explicitly join before timing

    const auto now = steady_clock::now();
    cout << "  Time:  " << duration_cast<milliseconds>(
        now-start).count()<<"ms\n";
  }
}
```

Godbolt Listing [lst-0019-godb.cpp](lst-0019-godb.cpp), [https://godbolt.org/z/zojG68dWq](https://godbolt.org/z/zojG68dWq):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23 -Wno-unused-result; libs:boost@184
// https://godbolt.org/z/zojG68dWq
#include <thread>
#include <iostream>
#include <vector>
#include <chrono>  // steady_clock
using std::cout; using std::endl; using namespace std::chrono;
long fib(long n) { return n<=1 ? n : fib(n-1)+fib(n-2); }

int main() {
  cout << std::thread::hardware_concurrency() << '\n';
  for(int nthreads : { 1,2,3,4,5,6 }) {
    cout << "Threads: ";
    const auto start = steady_clock::now();

    std::vector<std::jthread> threads;
    for(int ti = 1; ti <= nthreads; ++ti) {
      threads.emplace_back( std::jthread{fib, 40});
      cout << ti << "... "; cout.flush();
    }
    for(auto &th : threads) th.join(); // explicitly join before timing

    const auto now = steady_clock::now();
    cout << "  Time:  " << duration_cast<milliseconds>(
        now-start).count()<<"ms\n";
  }
}
```

----------------


## Listing 29.12: Each thread has an identifier.

Book listing [lst-0020-book.cpp](lst-0020-book.cpp):
```cpp
// https://godbolt.org/z/TM6c7cdss 
#include <thread>
#include <iostream>
int main() {
    std::cout << "Main: " << std::this_thread::get_id() << '\n';
    std::jthread th{ []{
        std::cout << "Thread: " << std::this_thread::get_id() << '\n';
    }};
}
```

Godbolt Listing [lst-0020-godb.cpp](lst-0020-godb.cpp), [https://godbolt.org/z/TM6c7cdss](https://godbolt.org/z/TM6c7cdss):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/TM6c7cdss 
#include <thread>
#include <iostream>
int main() {
    std::cout << "Main: " << std::this_thread::get_id() << '\n';
    std::jthread th{ []{
        std::cout << "Thread: " << std::this_thread::get_id() << '\n';
    }};
}
```

----------------


## Listing 29.13: A classic data race.

Book listing [lst-0021-book.cpp](lst-0021-book.cpp):
```cpp
// https://godbolt.org/z/7GEs9a4n6 
#include <thread>
#include <iostream>

int count = 0; // is simultaneously modified

void run() {
    for(int i=0; i<1'000'000; ++i) {
        count += 1;   // unprotected
    }
}

int main() {
    std::cout << "Start: " << count << '\n'; // Output: Start: 0
    std::thread th1{ run };
    std::thread th2{ run };
    std::thread th3{ run };
    th1.join(); th2.join(); th3.join();
    std::cout << "End: " << count << '\n';   // Output certainly not: 3000000
}
```

Godbolt Listing [lst-0021-godb.cpp](lst-0021-godb.cpp), [https://godbolt.org/z/7GEs9a4n6](https://godbolt.org/z/7GEs9a4n6):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/7GEs9a4n6 
#include <thread>
#include <iostream>

int count = 0; // is simultaneously modified

void run() {
    for(int i=0; i<1'000'000; ++i) {
        count += 1;   // unprotected
    }
}

int main() {
    std::cout << "Start: " << count << '\n'; // Output: Start: 0
    std::thread th1{ run };
    std::thread th2{ run };
    std::thread th3{ run };
    th1.join(); th2.join(); th3.join();
    std::cout << "End: " << count << '\n';   // Output certainly not: 3000000
}
```

----------------


## Listing 29.14: Benign data races are also undefined.

Book listing [lst-0022-book.cpp](lst-0022-book.cpp):
```cpp
// https://godbolt.org/z/v3b6sxEab 
#include <thread>
/* exact count not so important */
int count = 0; // is concurrently modified
void run() {
    for(int i=0; i<1'000; ++i) {
        count += 1; // unprotected
        if(count > 1000) return;  // (ERR) Termination condition
        for(int j=0; j<1'000; ++j)
            ;
    }
}

int main() {
    std::thread th1{ run };
    std::thread th2{ run };
    std::thread th3{ run };
    th1.join(); th2.join(); th3.join();
}
```

Godbolt Listing [lst-0022-godb.cpp](lst-0022-godb.cpp), [https://godbolt.org/z/v3b6sxEab](https://godbolt.org/z/v3b6sxEab):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/v3b6sxEab 
#include <thread>
/* exact count not so important */
int count = 0; // is concurrently modified
void run() {
    for(int i=0; i<1'000; ++i) {
        count += 1; // unprotected
        if(count > 1000) return;  // (ERR) Termination condition
        for(int j=0; j<1'000; ++j)
            ;
    }
}

int main() {
    std::thread th1{ run };
    std::thread th2{ run };
    std::thread th3{ run };
    th1.join(); th2.join(); th3.join();
}
```

----------------


## Listing 29.15: Latches can count down and wait.

Book listing [lst-0023-book.cpp](lst-0023-book.cpp):
```cpp
// https://godbolt.org/z/rrfn4d5WP 
#include <thread>
#include <latch>
#include <iostream>
long fib(long n) { return n<=1 ? n : fib(n-1)+fib(n-2); }

int main() {
    std::latch la{ 3 };                             // we expect 3 threads
    std::jthread th1{ [&la] { fib(39); la.count_down(); } };
    std::jthread th2{ [&la] { fib(38); la.count_down(); } };
    std::jthread th3{ [&la] { fib(40); la.count_down(); } };
    fib(37); // main thread
    std::cout << "Main thread: done\n";
    la.wait();                                      // waits until la == 0
    std::cout << "Rest done\n";
}
```

Godbolt Listing [lst-0023-godb.cpp](lst-0023-godb.cpp), [https://godbolt.org/z/rrfn4d5WP](https://godbolt.org/z/rrfn4d5WP):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/rrfn4d5WP 
#include <thread>
#include <latch>
#include <iostream>
long fib(long n) { return n<=1 ? n : fib(n-1)+fib(n-2); }

int main() {
    std::latch la{ 3 };                             // we expect 3 threads
    std::jthread th1{ [&la] { fib(39); la.count_down(); } };
    std::jthread th2{ [&la] { fib(38); la.count_down(); } };
    std::jthread th3{ [&la] { fib(40); la.count_down(); } };
    fib(37); // main thread
    std::cout << "Main thread: done\n";
    la.wait();                                      // waits until la == 0
    std::cout << "Rest done\n";
}
```

----------------


## Listing 29.16: A barrier waits for a specific number of threads.

Book listing [lst-0024-book.cpp](lst-0024-book.cpp):
```cpp
// https://godbolt.org/z/515dMo9Kx
#include <thread>
#include <barrier>
#include <iostream>
#include <vector>
long fib(long n) { return n<=1 ? n : fib(n-1)+fib(n-2); }

constexpr int anz = 8;          // 8 workers
constexpr int max_n = 32;       // calculate up to 32
std::vector<long> results(anz); // Buffer: space for 8 results

void output() {                 // Signaling function prints buffer
    for (auto n : results) std::cout << n << ' ';
    std::cout << '\n';
}

std::barrier ba{anz, output};   // always output after 8

void worker(std::stop_token st, int idx) {
    // n = 0, 9, 17, 25, … ; 1, 10, 18, 26, …
    for(int n = idx; n<max_n; n += anz) {
        if(st.stop_requested()) return;
        results[idx] = fib(n);  // write result to buffer
        ba.arrive_and_wait();   // wait until 8 threads are here
    }
}

int main() {
    std::vector<std::jthread> threads;     // 8 Threads
    for (int idx=0; idx<anz; ++idx) {
        threads.emplace_back(worker, idx); // create thread with index
    }
    for (auto& t : threads) t.join();      // wait until all are done
}
```

Godbolt Listing [lst-0024-godb.cpp](lst-0024-godb.cpp), [https://godbolt.org/z/515dMo9Kx](https://godbolt.org/z/515dMo9Kx):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23 -Wno-unused-result; libs:boost@184
// https://godbolt.org/z/515dMo9Kx
#include <thread>
#include <barrier>
#include <iostream>
#include <vector>
long fib(long n) { return n<=1 ? n : fib(n-1)+fib(n-2); }

constexpr int anz = 8;          // 8 workers
constexpr int max_n = 32;       // calculate up to 32
std::vector<long> results(anz); // Buffer: space for 8 results

void output() {                 // Signaling function prints buffer
    for (auto n : results) std::cout << n << ' ';
    std::cout << '\n';
}

std::barrier ba{anz, output};   // always output after 8

void worker(std::stop_token st, int idx) {
    // n = 0, 9, 17, 25, … ; 1, 10, 18, 26, …
    for(int n = idx; n<max_n; n += anz) {
        if(st.stop_requested()) return;
        results[idx] = fib(n);  // write result to buffer
        ba.arrive_and_wait();   // wait until 8 threads are here
    }
}

int main() {
    std::vector<std::jthread> threads;     // 8 Threads
    for (int idx=0; idx<anz; ++idx) {
        threads.emplace_back(worker, idx); // create thread with index
    }
    for (auto& t : threads) t.join();      // wait until all are done
}
```

----------------


## Listing 29.17: A mutex together with a simple lock.

Book listing [lst-0025-book.cpp](lst-0025-book.cpp):
```cpp
// https://godbolt.org/z/3fz5a4vYv 
#include <mutex> // mutex, lock_guard
#include <list>
#include <algorithm> // find
using std::lock_guard; using std::mutex; using std::find;
class MxIntList {
    std::list <int> data_;
    mutable mutex mx_;
public:
    void add(int value) {
        lock_guard guard{mx_};  // protects until the end of the method
        data_.push_back(value);
    }
    bool contains(int searchVal) const {
        lock_guard guard{mx_};  // protects until the end of the method
        return find(data_.begin(), data_.end(), searchVal) != data_.end();
    }
};
```

Godbolt Listing [lst-0025-godb.cpp](lst-0025-godb.cpp), [https://godbolt.org/z/3fz5a4vYv](https://godbolt.org/z/3fz5a4vYv):
```cpp
//#(execute) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/3fz5a4vYv 
#include <mutex> // mutex, lock_guard
#include <list>
#include <algorithm> // find
using std::lock_guard; using std::mutex; using std::find;
class MxIntList {
    std::list <int> data_;
    mutable mutex mx_;
public:
    void add(int value) {
        lock_guard guard{mx_};  // protects until the end of the method
        data_.push_back(value);
    }
    bool contains(int searchVal) const {
        lock_guard guard{mx_};  // protects until the end of the method
        return find(data_.begin(), data_.end(), searchVal) != data_.end();
    }
};
```

----------------


## Listing 29.18: The interface to a multithreaded stack.

Book listing [lst-0026-book.cpp](lst-0026-book.cpp):
```cpp
// https://godbolt.org/z/4j7njd763 
#include <vector>

template<class T>
class MxStack {
public:
    bool isEmpty() const;
    void push(const T&);
    void pop();
    const T& top() const;
};
```

Godbolt Listing [lst-0026-godb.cpp](lst-0026-godb.cpp), [https://godbolt.org/z/4j7njd763](https://godbolt.org/z/4j7njd763):
```cpp
//#(execute) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/4j7njd763 
#include <vector>

template<class T>
class MxStack {
public:
    bool isEmpty() const;
    void push(const T&);
    void pop();
    const T& top() const;
};
```

----------------


## Listing 29.19: Problematic code for the "MxStack".

Book listing [lst-0027-book.cpp](lst-0027-book.cpp):
```cpp
// https://godbolt.org/z/1boeTMMvf 
MxStack<int> mxs{};
// …
// more code
// …
if( ! mxs.isEmpty()) {            // (ERR) not safe
    const auto value = mxs.top(); // (ERR) not safe
    mxs.pop();                    // (ERR) not safe
    // …
    // more code
    // …
}
```

Godbolt Listing [lst-0027-godb.cpp](lst-0027-godb.cpp), [https://godbolt.org/z/1boeTMMvf](https://godbolt.org/z/1boeTMMvf):
```cpp
//#(execute) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/1boeTMMvf 
MxStack<int> mxs{};
// …
// more code
// …
if( ! mxs.isEmpty()) {            // (ERR) not safe
    const auto value = mxs.top(); // (ERR) not safe
    mxs.pop();                    // (ERR) not safe
    // …
    // more code
    // …
}
```

----------------


## Listing 29.20: A possible way two threads could execute the previous code.

Book listing [lst-0028-book.cpp](lst-0028-book.cpp):
```cpp
/* Thread 1 */                         /* Thread 2 */
if( ! mxs.isEmpty()) {
                                    if( ! mxs.isEmpty()) {
    const auto value = mxs.top();
                                        const auto value = mxs.top();
    mxs.pop();
    // … more code …
                                        mxs.pop();
                                        // … more code …
```

----------------


## Listing 29.21: A very simple thread-safe stack.

Book listing [lst-0029-book.cpp](lst-0029-book.cpp):
```cpp
// https://godbolt.org/z/vcb9qP8jK
#include <vector>
#include <thread>
#include <mutex>
#include <iostream>
#include <numeric>  // iota
#include <concepts> // copyable 

/* T: noexcept copyable and assignable */
template<std::copyable T>
class MxStack {
    std::vector<T> data_;
    std::mutex mx_;

public:
    MxStack() : data_{} {}

    bool isEmpty() const { return data_.empty(); }

    void push(const T& val) {
        std::lock_guard<std::mutex> g{mx_};
        data_.push_back(val);
    }

    T pop() {
        std::lock_guard g{mx_};
        if(data_.empty())
            throw std::length_error{"empty stack"};
        T tmp{std::move(data_.back())};
        data_.pop_back();
        return tmp;
    }
};

int main() {
    // Prepare stack
    MxStack<int> mxs{};
    for(int i=1; i<=1'000'000; ++i) mxs.push(i);
    // Define computation
    auto sumIt = [&mxs](long &sum) {
        int val{};
        try {
            while( ! mxs.isEmpty()) {
                sum += mxs.pop(); // might still throw
            }
        } catch(std::length_error &ex) {}
    };
    // Compute
    long sum1 = 0;           // for partial result
    std::jthread th1{sumIt, std::ref(sum1)};
    long sum2 = 0;           // for partial result
    std::thread th2{sumIt, std::ref(sum2)};
    th1.join(); th2.join();
    long sum = sum1 + sum2; // Total result
    // Result
    std::cout << "Expected result: "
        << (1'000'000L*1'000'001)/2 << '\n'; // Output: 500000500000
    std::cout << "Actual: "
        << sum << '\n';                      // Output: 500000500000
}
```

Godbolt Listing [lst-0029-godb.cpp](lst-0029-godb.cpp), [https://godbolt.org/z/vcb9qP8jK](https://godbolt.org/z/vcb9qP8jK):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23 -Wno-unused-result; libs:boost@184
// https://godbolt.org/z/vcb9qP8jK
#include <vector>
#include <thread>
#include <mutex>
#include <iostream>
#include <numeric>  // iota
#include <concepts> // copyable 

/* T: noexcept copyable and assignable */
template<std::copyable T>
class MxStack {
    std::vector<T> data_;
    std::mutex mx_;

public:
    MxStack() : data_{} {}

    bool isEmpty() const { return data_.empty(); }

    void push(const T& val) {
        std::lock_guard<std::mutex> g{mx_};
        data_.push_back(val);
    }

    T pop() {
        std::lock_guard g{mx_};
        if(data_.empty())
            throw std::length_error{"empty stack"};
        T tmp{std::move(data_.back())};
        data_.pop_back();
        return tmp;
    }
};

int main() {
    // Prepare stack
    MxStack<int> mxs{};
    for(int i=1; i<=1'000'000; ++i) mxs.push(i);
    // Define computation
    auto sumIt = [&mxs](long &sum) {
        int val{};
        try {
            while( ! mxs.isEmpty()) {
                sum += mxs.pop(); // might still throw
            }
        } catch(std::length_error &ex) {}
    };
    // Compute
    long sum1 = 0;           // for partial result
    std::jthread th1{sumIt, std::ref(sum1)};
    long sum2 = 0;           // for partial result
    std::thread th2{sumIt, std::ref(sum2)};
    th1.join(); th2.join();
    long sum = sum1 + sum2; // Total result
    // Result
    std::cout << "Expected result: "
        << (1'000'000L*1'000'001)/2 << '\n'; // Output: 500000500000
    std::cout << "Actual: "
        << sum << '\n';                      // Output: 500000500000
}
```

----------------


## Listing 29.22: "swap" for "MxStack".

Book listing [lst-0032-book.cpp](lst-0032-book.cpp):
```cpp
// https://godbolt.org/z/9PMcz1cqP 
friend void swap(MxStack& re, MxStack& li) {
    if(&re==&li) return; // Same address? Swapping with itself is unnecessary
    std::lock( re.mx_, li.mx_ );   // multiple locks simultaneously
    std::lock_guard lkre{re.mx_, std::adopt_lock}; // already locked
    std::lock_guard lkli{li.mx_, std::adopt_lock}; // already locked
    std::swap(li.data_, re.data_); // perform swap
}
```

Godbolt Listing [lst-0032-godb.cpp](lst-0032-godb.cpp), [https://godbolt.org/z/9PMcz1cqP](https://godbolt.org/z/9PMcz1cqP):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/9PMcz1cqP 
friend void swap(MxStack& re, MxStack& li) {
    if(&re==&li) return; // Same address? Swapping with itself is unnecessary
    std::lock( re.mx_, li.mx_ );   // multiple locks simultaneously
    std::lock_guard lkre{re.mx_, std::adopt_lock}; // already locked
    std::lock_guard lkli{li.mx_, std::adopt_lock}; // already locked
    std::swap(li.data_, re.data_); // perform swap
}
```

----------------


## Listing 29.23: Mutexes can be transferred.

Book listing [lst-0033-book.cpp](lst-0033-book.cpp):
```cpp
// https://godbolt.org/z/e7rraPGx4 
#include <thread>
#include <mutex>
#include <vector>
#include <numeric> // accumulate, iota
using std::mutex; using std::unique_lock;

std::vector<int> myData;              // shared data
mutex myMutex;                        // Mutex for the data
unique_lock<mutex> prepareData() {
    unique_lock lk1{myMutex};        // lock
    myData.resize(1000);
    std::iota(myData.begin(), myData.end(), 1); // 1..1000
    return lk1;                                 // Transfer lock
}
int processData() {
    unique_lock lk2 = prepareData();         // Lock transferred
    return std::accumulate(myData.begin(), myData.end(), 0);
}
```

Godbolt Listing [lst-0033-godb.cpp](lst-0033-godb.cpp), [https://godbolt.org/z/e7rraPGx4](https://godbolt.org/z/e7rraPGx4):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/e7rraPGx4 
#include <thread>
#include <mutex>
#include <vector>
#include <numeric> // accumulate, iota
using std::mutex; using std::unique_lock;

std::vector<int> myData;              // shared data
mutex myMutex;                        // Mutex for the data
unique_lock<mutex> prepareData() {
    unique_lock lk1{myMutex};        // lock
    myData.resize(1000);
    std::iota(myData.begin(), myData.end(), 1); // 1..1000
    return lk1;                                 // Transfer lock
}
int processData() {
    unique_lock lk2 = prepareData();         // Lock transferred
    return std::accumulate(myData.begin(), myData.end(), 0);
}
```

----------------


## Listing 29.24: Late initialization with a single thread.

Book listing [lst-0034-book.cpp](lst-0034-book.cpp):
```cpp
// https://godbolt.org/z/T7vehq58b 
std::shared_ptr<BigData> bigData{};
BigData& getBigData() {
    if(!bigData) bigData.reset(new BigData{});
    return *bigData;
}
int useBigData() {
    auto bigData = getBigData();
    // bigData->…
}
```

Godbolt Listing [lst-0034-godb.cpp](lst-0034-godb.cpp), [https://godbolt.org/z/T7vehq58b](https://godbolt.org/z/T7vehq58b):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/T7vehq58b 
std::shared_ptr<BigData> bigData{};
BigData& getBigData() {
    if(!bigData) bigData.reset(new BigData{});
    return *bigData;
}
int useBigData() {
    auto bigData = getBigData();
    // bigData->…
}
```

----------------


## Listing 29.25: Late initialization with multiple threads.

Book listing [lst-0035-book.cpp](lst-0035-book.cpp):
```cpp
// https://godbolt.org/z/3vcnzfqfM 
#include <mutex> // once_flag, call_once
std::shared_ptr<BigData> bigData{};
std::once_flag bigDataInitFlag;
void initBigData() {
    bigData = std::make_shared<BigData>();
}
int useBigData() {
    std::call_once(bigDataInitFlag, initBigData);
    // bigData->…
}
```

Godbolt Listing [lst-0035-godb.cpp](lst-0035-godb.cpp), [https://godbolt.org/z/3vcnzfqfM](https://godbolt.org/z/3vcnzfqfM):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/3vcnzfqfM 
#include <mutex> // once_flag, call_once
std::shared_ptr<BigData> bigData{};
std::once_flag bigDataInitFlag;
void initBigData() {
    bigData = std::make_shared<BigData>();
}
int useBigData() {
    std::call_once(bigDataInitFlag, initBigData);
    // bigData->…
}
```

----------------


## Listing 29.26: Late initialization can also be useful within a thread.

Book listing [lst-0036-book.cpp](lst-0036-book.cpp):
```cpp
// https://godbolt.org/z/TcsrefEPh 
#include <mutex> // once_flag, call_once
#include <memory>
struct Connection {
    void csend(const char *data) {} // dummy
    const char* crecv() {} // dummy
};
class Sender {
    std::shared_ptr<Connection> conn_;
    std::once_flag connInitFlag_;
    void open() {
      conn_.reset( new Connection{} );
    }
public:
    void send(const char* data) {
      std::call_once(connInitFlag_, &Sender::open, this); // method pointer
      conn_->csend(data);
    }
    const char* recv() {
      std::call_once(connInitFlag_, [this] {this->open();} ); // lambda
      return conn_->crecv();
    }
};
```

Godbolt Listing [lst-0036-godb.cpp](lst-0036-godb.cpp), [https://godbolt.org/z/TcsrefEPh](https://godbolt.org/z/TcsrefEPh):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/TcsrefEPh 
#include <mutex> // once_flag, call_once
#include <memory>
struct Connection {
    void csend(const char *data) {} // dummy
    const char* crecv() {} // dummy
};
class Sender {
    std::shared_ptr<Connection> conn_;
    std::once_flag connInitFlag_;
    void open() {
      conn_.reset( new Connection{} );
    }
public:
    void send(const char* data) {
      std::call_once(connInitFlag_, &Sender::open, this); // method pointer
      conn_->csend(data);
    }
    const char* recv() {
      std::call_once(connInitFlag_, [this] {this->open();} ); // lambda
      return conn_->crecv();
    }
};
```

----------------


## Listing 29.27: Precautions are necessary when a mutex needs to be locked multiple times.

Book listing [lst-0037-book.cpp](lst-0037-book.cpp):
```cpp
// https://godbolt.org/z/KPaPoh4Gn
#include <mutex> // recursive_mutex
#include <iostream>
struct MulDiv {
    std::recursive_mutex mx_;
    int value_;
    explicit MulDiv(int value) : value_{value} {}
    void mul(int x) {
        std::lock_guard lk1(mx_);  // inner
        value_ *= x;
    }
    void div(int x) {
        std::lock_guard lk2(mx_);  // inner
        value_ /= x;
    }
    void muldiv(int x, int y){
        std::lock_guard lk3(mx_);  // outer
        mul(x);
        div(y);
    }
};
int main() {
   MulDiv m{42}; // 3*7*2 *5
   m.muldiv(5, 15);
   std::cout << m.value_ << '\n';  // Output: 14
}
```

Godbolt Listing [lst-0037-godb.cpp](lst-0037-godb.cpp), [https://godbolt.org/z/KPaPoh4Gn](https://godbolt.org/z/KPaPoh4Gn):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23 -Wno-unused-result; libs:boost@184
// https://godbolt.org/z/KPaPoh4Gn
#include <mutex> // recursive_mutex
#include <iostream>
struct MulDiv {
    std::recursive_mutex mx_;
    int value_;
    explicit MulDiv(int value) : value_{value} {}
    void mul(int x) {
        std::lock_guard lk1(mx_);  // inner
        value_ *= x;
    }
    void div(int x) {
        std::lock_guard lk2(mx_);  // inner
        value_ /= x;
    }
    void muldiv(int x, int y){
        std::lock_guard lk3(mx_);  // outer
        mul(x);
        div(y);
    }
};
int main() {
   MulDiv m{42}; // 3*7*2 *5
   m.muldiv(5, 15);
   std::cout << m.value_ << '\n';  // Output: 14
}
```

----------------


## Listing 29.28: Memory only for the current thread.

Book listing [lst-0038-book.cpp](lst-0038-book.cpp):
```cpp
// https://godbolt.org/z/1j8W7GjTf 
#include <iostream>
#include <string>
#include <thread>
#include <mutex>
thread_local unsigned int usage = 0;
static std::mutex cout_mutex;
void use(const std::string thread_name) {
    ++usage;
    std::lock_guard lock(cout_mutex); // Protect output
    std::cout << thread_name << ": " << usage << '\n';
}
int main() {
    std::jthread a{use, "a"}, b{use, "b"};
    use("main");
}
```

Godbolt Listing [lst-0038-godb.cpp](lst-0038-godb.cpp), [https://godbolt.org/z/1j8W7GjTf](https://godbolt.org/z/1j8W7GjTf):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/1j8W7GjTf 
#include <iostream>
#include <string>
#include <thread>
#include <mutex>
thread_local unsigned int usage = 0;
static std::mutex cout_mutex;
void use(const std::string thread_name) {
    ++usage;
    std::lock_guard lock(cout_mutex); // Protect output
    std::cout << thread_name << ": " << usage << '\n';
}
int main() {
    std::jthread a{use, "a"}, b{use, "b"};
    use("main");
}
```

----------------


## Listing 29.29: Two threads communicate via a condition variable.

Book listing [lst-0040-book.cpp](lst-0040-book.cpp):
```cpp
// https://godbolt.org/z/c9x6xT8dr
#include <thread>
#include <mutex>
#include <condition_variable>
#include <vector>
#include <deque>
#include <iostream>
#include <syncstream>   // osyncstream
std::deque<int> g_data{};          // Data exchange between threads
std::condition_variable g_condvar; // notify
std::mutex g_mx;                   // protects g_data during changes
void produce(int limit) {
  std::vector primes{2};           // prior primes as test-divisors
  for(int candidate=3; candidate < limit; candidate+=2) {
    for(int divisor : primes) {
      if(divisor*divisor > candidate) {     // candidate is prime
        std::lock_guard lk{g_mx};           // protect data
        g_data.push_back(candidate);        // fill
        g_condvar.notify_one();             // notify
        primes.push_back(candidate);        // for internal calculations
        break; // next prime candidate
      } else if(candidate % divisor == 0) { // not prime
        break;                              // next prime candidate
      } else {
        // next divisor to check
      }
    }
  }
  // notify all work done
  std::lock_guard lk{g_mx};            // protect data
  g_data.push_back(0);                 // fill with end marker
  g_condvar.notify_all();              // notify
}
void consume(char l, char r) {
  while(true) {                        // forever
    std::unique_lock lk{g_mx};
    g_condvar.wait(lk, []{ return !g_data.empty();});
    int prim = g_data.front();         // fetch data
    if(prim == 0) return;              // done; leave 0 for other consumers
    g_data.pop_front();
    lk.unlock();                       // release lock
    std::osyncstream osync{std::cout}; // synchronize output
    osync << l << prim << r <<' ';
  }
}
int main() {
  // a producer:
  std::jthread thProd{produce, 1'000};
  // three consumers
  std::jthread thCon1{consume, '[', ']' };
  std::jthread thCon2{consume, '<', '>' };
  std::jthread thCon3{consume, '{', '}' };
  // wait and finish
  thProd.join();
  thCon1.join(); thCon2.join(); thCon3.join();
  std::cout << '\n';
}
```

Godbolt Listing [lst-0040-godb.cpp](lst-0040-godb.cpp), [https://godbolt.org/z/c9x6xT8dr](https://godbolt.org/z/c9x6xT8dr):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23 -Wno-unused-result; libs:boost@184
// https://godbolt.org/z/c9x6xT8dr
#include <thread>
#include <mutex>
#include <condition_variable>
#include <vector>
#include <deque>
#include <iostream>
#include <syncstream>   // osyncstream
std::deque<int> g_data{};          // Data exchange between threads
std::condition_variable g_condvar; // notify
std::mutex g_mx;                   // protects g_data during changes
void produce(int limit) {
  std::vector primes{2};           // prior primes as test-divisors
  for(int candidate=3; candidate < limit; candidate+=2) {
    for(int divisor : primes) {
      if(divisor*divisor > candidate) {     // candidate is prime
        std::lock_guard lk{g_mx};           // protect data
        g_data.push_back(candidate);        // fill
        g_condvar.notify_one();             // notify
        primes.push_back(candidate);        // for internal calculations
        break; // next prime candidate
      } else if(candidate % divisor == 0) { // not prime
        break;                              // next prime candidate
      } else {
        // next divisor to check
      }
    }
  }
  // notify all work done
  std::lock_guard lk{g_mx};            // protect data
  g_data.push_back(0);                 // fill with end marker
  g_condvar.notify_all();              // notify
}
void consume(char l, char r) {
  while(true) {                        // forever
    std::unique_lock lk{g_mx};
    g_condvar.wait(lk, []{ return !g_data.empty();});
    int prim = g_data.front();         // fetch data
    if(prim == 0) return;              // done; leave 0 for other consumers
    g_data.pop_front();
    lk.unlock();                       // release lock
    std::osyncstream osync{std::cout}; // synchronize output
    osync << l << prim << r <<' ';
  }
}
int main() {
  // a producer:
  std::jthread thProd{produce, 1'000};
  // three consumers
  std::jthread thCon1{consume, '[', ']' };
  std::jthread thCon2{consume, '<', '>' };
  std::jthread thCon3{consume, '{', '}' };
  // wait and finish
  thProd.join();
  thCon1.join(); thCon2.join(); thCon3.join();
  std::cout << '\n';
}
```

----------------


## Listing 29.30: "async" can easily initiate asynchronous computations.

Book listing [lst-0045-book.cpp](lst-0045-book.cpp):
```cpp
// https://godbolt.org/z/Gh9Y5srd7 
#include <iostream>
#include <future>  // async
using std::cout; using std::endl;
long fib(long n) { return n<=1 ? n : fib(n-1)+fib(n-2); }

int main() {
    auto f40 = std::async(fib, 40);
    auto f41 = std::async(fib, 41);
    auto f42 = std::async(fib, 42);
    auto f43 = std::async(fib, 43);
    /* ... at this point, further calculations can be made ... */
    cout << "fib(40): " << f40.get() << endl; // Output: fib(40): 102334155
    cout << "fib(41): " << f41.get() << endl; // Output: fib(41): 165580141
    cout << "fib(42): " << f42.get() << endl; // Output: fib(42): 267914296
    cout << "fib(43): " << f43.get() << endl; // Output: fib(43): 433494437
}
```

Godbolt Listing [lst-0045-godb.cpp](lst-0045-godb.cpp), [https://godbolt.org/z/Gh9Y5srd7](https://godbolt.org/z/Gh9Y5srd7):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/Gh9Y5srd7 
#include <iostream>
#include <future>  // async
using std::cout; using std::endl;
long fib(long n) { return n<=1 ? n : fib(n-1)+fib(n-2); }

int main() {
    auto f40 = std::async(fib, 40);
    auto f41 = std::async(fib, 41);
    auto f42 = std::async(fib, 42);
    auto f43 = std::async(fib, 43);
    /* ... at this point, further calculations can be made ... */
    cout << "fib(40): " << f40.get() << endl; // Output: fib(40): 102334155
    cout << "fib(41): " << f41.get() << endl; // Output: fib(41): 165580141
    cout << "fib(42): " << f42.get() << endl; // Output: fib(42): 267914296
    cout << "fib(43): " << f43.get() << endl; // Output: fib(43): 433494437
}
```

----------------


## Listing 29.31: It is okay not to retrieve a result.

Book listing [lst-0046-book.cpp](lst-0046-book.cpp):
```cpp
// https://godbolt.org/z/7Mea1Y3Mn 
#include <iostream>
#include <future>  // async
using std::cout; using std::endl;
long fib(long n) { return n<=1 ? n : fib(n-1)+fib(n-2); }

int main() {
    auto f40 = std::async(fib, 40);
    auto f41 = std::async(fib, 41);
    auto f42 = std::async(fib, 42);
    auto f43 = std::async(fib, 43);
    cout << "fib(40): " << f40.get() << endl; // Output: fib(40): 102334155
} // also waits for f41, f42, and f43.
```

Godbolt Listing [lst-0046-godb.cpp](lst-0046-godb.cpp), [https://godbolt.org/z/7Mea1Y3Mn](https://godbolt.org/z/7Mea1Y3Mn):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/7Mea1Y3Mn 
#include <iostream>
#include <future>  // async
using std::cout; using std::endl;
long fib(long n) { return n<=1 ? n : fib(n-1)+fib(n-2); }

int main() {
    auto f40 = std::async(fib, 40);
    auto f41 = std::async(fib, 41);
    auto f42 = std::async(fib, 42);
    auto f43 = std::async(fib, 43);
    cout << "fib(40): " << f40.get() << endl; // Output: fib(40): 102334155
} // also waits for f41, f42, and f43.
```

----------------


## Listing 29.32: How to enforce deferred execution.

Book listing [lst-0047-book.cpp](lst-0047-book.cpp):
```cpp
// https://godbolt.org/z/P9MjGe6Mn 
#include <iostream>
#include <future>  // async
#include <vector>
using std::cout; using std::endl;
long fib(long n) { return n<=1 ? n : fib(n-1)+fib(n-2); }
int main() {
    // Prepare tasks
    std::vector< std::future<long> > fibs;
    for(int n=0; n<50; ++n) {
        auto fut = std::async(std::launch::deferred, fib, n);
        fibs.push_back( std::move(fut) );
    }
    // only retrieve the required result
    cout << "fib(42): " << fibs[42].get() << endl; // Output: fib(42): 267914296
}
```

Godbolt Listing [lst-0047-godb.cpp](lst-0047-godb.cpp), [https://godbolt.org/z/P9MjGe6Mn](https://godbolt.org/z/P9MjGe6Mn):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/P9MjGe6Mn 
#include <iostream>
#include <future>  // async
#include <vector>
using std::cout; using std::endl;
long fib(long n) { return n<=1 ? n : fib(n-1)+fib(n-2); }
int main() {
    // Prepare tasks
    std::vector< std::future<long> > fibs;
    for(int n=0; n<50; ++n) {
        auto fut = std::async(std::launch::deferred, fib, n);
        fibs.push_back( std::move(fut) );
    }
    // only retrieve the required result
    cout << "fib(42): " << fibs[42].get() << endl; // Output: fib(42): 267914296
}
```

----------------


## Listing 29.33: Waiting for a certain time with "async".

Book listing [lst-0048-book.cpp](lst-0048-book.cpp):
```cpp
// https://godbolt.org/z/o9oYrvMcj 
#include <iostream>
#include <future>  // async
#include <chrono>
using std::cout; using std::endl; using namespace std::chrono;
long fib(long n) { return n<=1 ? n : fib(n-1)+fib(n-2); }
int main() {
    auto f43 = std::async(fib, 43);
    while(true) {
        auto ready = f43.wait_for(500ms);
        if(ready==std::future_status::timeout) {
            std::cout << "not yet..." << std::endl;
        } else {
            break;
        }
    }
    // pick up, is immediately there
    cout << "fib(43): " << f43.get() << endl; // Output: fib(43): 701408733
}
```

Godbolt Listing [lst-0048-godb.cpp](lst-0048-godb.cpp), [https://godbolt.org/z/o9oYrvMcj](https://godbolt.org/z/o9oYrvMcj):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/o9oYrvMcj 
#include <iostream>
#include <future>  // async
#include <chrono>
using std::cout; using std::endl; using namespace std::chrono;
long fib(long n) { return n<=1 ? n : fib(n-1)+fib(n-2); }
int main() {
    auto f43 = std::async(fib, 43);
    while(true) {
        auto ready = f43.wait_for(500ms);
        if(ready==std::future_status::timeout) {
            std::cout << "not yet..." << std::endl;
        } else {
            break;
        }
    }
    // pick up, is immediately there
    cout << "fib(43): " << f43.get() << endl; // Output: fib(43): 701408733
}
```

----------------


## Listing 29.34: Waiting for a lock with a "timed_mutex".

Book listing [lst-0049-book.cpp](lst-0049-book.cpp):
```cpp
// https://godbolt.org/z/xzY8hj9oP 
#include <chrono>
#include <future>
#include <mutex>
#include <vector>
#include <iostream>
std::timed_mutex mtx;
long fibX(long n) { return n < 2L ? 1L : fibX(n-1L) + fibX(n-2L); }
long fibCall(long n) {
    using namespace std::chrono; // Suffixes
    if(mtx.try_lock_for(1000ms)) {
        auto res = fibX(n);
        mtx.unlock();
        return res;
    } else {
        return 0L;
    }
}

int main() {
    std::vector< std::future<long> > fs;
    for(long n=1; n<= 42; ++n) {
        fs.emplace_back( std::async(std::launch::async, fibCall, n) );
    }
    for(auto &f : fs) {
        std::cout << f.get() << " ";
    }
    std::cout << std::endl;
}
```

Godbolt Listing [lst-0049-godb.cpp](lst-0049-godb.cpp), [https://godbolt.org/z/xzY8hj9oP](https://godbolt.org/z/xzY8hj9oP):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/xzY8hj9oP 
#include <chrono>
#include <future>
#include <mutex>
#include <vector>
#include <iostream>
std::timed_mutex mtx;
long fibX(long n) { return n < 2L ? 1L : fibX(n-1L) + fibX(n-2L); }
long fibCall(long n) {
    using namespace std::chrono; // Suffixes
    if(mtx.try_lock_for(1000ms)) {
        auto res = fibX(n);
        mtx.unlock();
        return res;
    } else {
        return 0L;
    }
}

int main() {
    std::vector< std::future<long> > fs;
    for(long n=1; n<= 42; ++n) {
        fs.emplace_back( std::async(std::launch::async, fibCall, n) );
    }
    for(auto &f : fs) {
        std::cout << f.get() << " ";
    }
    std::cout << std::endl;
}
```

----------------


## Listing 29.35: Exceptions only arrive at "get" in the outer thread.

Book listing [lst-0050-book.cpp](lst-0050-book.cpp):
```cpp
// https://godbolt.org/z/rYM9xqsGa
#include <future> // async
#include <vector>
#include <algorithm> // max
#include <iostream>

int calculateHeight(int count, int maxCount, int scale) {
  if(maxCount == 0)
      throw std::logic_error("All heights 0");
  return (count * scale) / maxCount;
}

void bar(const std::vector<int> &counts) {
  // Start calculation
  auto maxCount = *std::max_element(counts.begin(), counts.end());
  std::vector< std::future<int> > futs;
  for(int count : counts) {
    futs.push_back(
          std::async(std::launch::async,
              calculateHeight, count, maxCount, 200) );
  }

  // Collect results
  for(auto &fut : futs) {
    std::cout << fut.get() << ' ';                 // triggers exception
  }
  std::cout << '\n';
}

int main() {
  try {
    bar(std::vector {10,23,13,0,33,4 });         // Output: 60 139 78 0 200 24
    bar(std::vector { 0, 0, 0, 0 });             // triggers exception
  } catch(std::exception &ex) {
    std::cout << "Error: " << ex.what() << '\n'; // Output: Error: All heights 0
  }
}
```

Godbolt Listing [lst-0050-godb.cpp](lst-0050-godb.cpp), [https://godbolt.org/z/rYM9xqsGa](https://godbolt.org/z/rYM9xqsGa):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23 -Wno-unused-result; libs:boost@184
// https://godbolt.org/z/rYM9xqsGa
#include <future> // async
#include <vector>
#include <algorithm> // max
#include <iostream>

int calculateHeight(int count, int maxCount, int scale) {
  if(maxCount == 0)
      throw std::logic_error("All heights 0");
  return (count * scale) / maxCount;
}

void bar(const std::vector<int> &counts) {
  // Start calculation
  auto maxCount = *std::max_element(counts.begin(), counts.end());
  std::vector< std::future<int> > futs;
  for(int count : counts) {
    futs.push_back(
          std::async(std::launch::async,
              calculateHeight, count, maxCount, 200) );
  }

  // Collect results
  for(auto &fut : futs) {
    std::cout << fut.get() << ' ';                 // triggers exception
  }
  std::cout << '\n';
}

int main() {
  try {
    bar(std::vector {10,23,13,0,33,4 });         // Output: 60 139 78 0 200 24
    bar(std::vector { 0, 0, 0, 0 });             // triggers exception
  } catch(std::exception &ex) {
    std::cout << "Error: " << ex.what() << '\n'; // Output: Error: All heights 0
  }
}
```

----------------


## Listing 29.36: Only the "get" needs to be encapsulated in "try".

Book listing [lst-0051-book.cpp](lst-0051-book.cpp):
```cpp
// https://godbolt.org/z/1WG5arvWT
#include <future> // async
#include <iostream>
using namespace std;
int calcHeight(int count, int maxCount, int scale) {
  if(maxCount == 0)
      throw logic_error("maxCount is 0");
  return (count * scale) / maxCount;
}

int main() {
  auto fut = async(launch::async, calcHeight,0,0,200); // (ERR) throws
  try {
    cout << fut.get() << '\n';              // triggers exception
  } catch(exception &ex) {
    cout << "Error: " << ex.what() << '\n'; // Output: Error: maxCount is 0
  }
}
```

Godbolt Listing [lst-0051-godb.cpp](lst-0051-godb.cpp), [https://godbolt.org/z/1WG5arvWT](https://godbolt.org/z/1WG5arvWT):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/1WG5arvWT
#include <future> // async
#include <iostream>
using namespace std;
int calcHeight(int count, int maxCount, int scale) {
  if(maxCount == 0)
      throw logic_error("maxCount is 0");
  return (count * scale) / maxCount;
}

int main() {
  auto fut = async(launch::async, calcHeight,0,0,200); // (ERR) throws
  try {
    cout << fut.get() << '\n';              // triggers exception
  } catch(exception &ex) {
    cout << "Error: " << ex.what() << '\n'; // Output: Error: maxCount is 0
  }
}
```

----------------


## Listing 29.37: A "future" and "promise" working together.

Book listing [lst-0053-book.cpp](lst-0053-book.cpp):
```cpp
// https://godbolt.org/z/7E1E6aezE
#include <future>
#include <thread>
#include <iostream>
#include <exception>
int ack(int m, int n); // Ackermann function
void longComputation(std::promise<int> intPromise) {
  try {
    int result = ack(3,12);
    intPromise.set_value(result);                       // report result
  } catch (std::exception &e) {
    intPromise.set_exception(make_exception_ptr(e));    // report exception
  } catch ( ... ) {
    intPromise.set_exception(std::current_exception()); // Exc. w/o name
  }
}
int main () {
  std::promise<int> intPromise;                         // Create promise
  std::future<int> intFuture = intPromise.get_future(); // Request future
  std::jthread th{ longComputation,                     // start
             std::move(intPromise) };                   // Pass promise
  th.detach();                                          // let it run
  // might throw an exception:
  int result = intFuture.get();                         // Request result
  std::cout << result << std::endl;
}
```

Godbolt Listing [lst-0053-godb.cpp](lst-0053-godb.cpp), [https://godbolt.org/z/7E1E6aezE](https://godbolt.org/z/7E1E6aezE):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23 -Wno-unused-result; libs:boost@184
// https://godbolt.org/z/7E1E6aezE
#include <future>
#include <thread>
#include <iostream>
#include <exception>
int ack(int m, int n); // Ackermann function
void longComputation(std::promise<int> intPromise) {
  try {
    int result = ack(3,12);
    intPromise.set_value(result);                       // report result
  } catch (std::exception &e) {
    intPromise.set_exception(make_exception_ptr(e));    // report exception
  } catch ( ... ) {
    intPromise.set_exception(std::current_exception()); // Exc. w/o name
  }
}
int main () {
  std::promise<int> intPromise;                         // Create promise
  std::future<int> intFuture = intPromise.get_future(); // Request future
  std::jthread th{ longComputation,                     // start
             std::move(intPromise) };                   // Pass promise
  th.detach();                                          // let it run
  // might throw an exception:
  int result = intFuture.get();                         // Request result
  std::cout << result << std::endl;
}
```

----------------


## Listing 29.38: Preparing a packaged task for later execution.

Book listing [lst-0054-book.cpp](lst-0054-book.cpp):
```cpp
// https://godbolt.org/z/9s94nbrx3 
#include <future>
#include <thread>
#include <iostream>
int ack(int m, int n); // Ackermann function
int main () {
    std::packaged_task<int(void)> task1 {  // Signature of the remaining function
    []{ return ack(3,11);}  };             // prepare ack(3,11)
  auto f1 = task1.get_future();            // Communication channel
  std::jthread th1 { move(task1) };        // in new thread
  std::cout << "  ack(3,11):" << f1.get()  // Retrieve result
      << '\n';                             // Output: ack(3,11):16381
}
```

Godbolt Listing [lst-0054-godb.cpp](lst-0054-godb.cpp), [https://godbolt.org/z/9s94nbrx3](https://godbolt.org/z/9s94nbrx3):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/9s94nbrx3 
#include <future>
#include <thread>
#include <iostream>
int ack(int m, int n); // Ackermann function
int main () {
    std::packaged_task<int(void)> task1 {  // Signature of the remaining function
    []{ return ack(3,11);}  };             // prepare ack(3,11)
  auto f1 = task1.get_future();            // Communication channel
  std::jthread th1 { move(task1) };        // in new thread
  std::cout << "  ack(3,11):" << f1.get()  // Retrieve result
      << '\n';                             // Output: ack(3,11):16381
}
```

----------------


## Listing 29.39: Provide a packaged task with arguments later.

Book listing [lst-0055-book.cpp](lst-0055-book.cpp):
```cpp
// https://godbolt.org/z/bq1hn5rjo 
#include <future>
#include <thread>
#include <iostream>
int ack(int m, int n); // Ackermann function
int main () {
  std::packaged_task<int(int,int)> task2 { &ack }; // different signature
  auto f2 = task2.get_future();
  std::jthread th2 { move(task2), 3, 12 };         // Parameters here
  std::cout << "  ack(3,12):" << f2.get() << '\n'; // Output: ack(3,12):32765
  th2.join();
}
```

Godbolt Listing [lst-0055-godb.cpp](lst-0055-godb.cpp), [https://godbolt.org/z/bq1hn5rjo](https://godbolt.org/z/bq1hn5rjo):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/bq1hn5rjo 
#include <future>
#include <thread>
#include <iostream>
int ack(int m, int n); // Ackermann function
int main () {
  std::packaged_task<int(int,int)> task2 { &ack }; // different signature
  auto f2 = task2.get_future();
  std::jthread th2 { move(task2), 3, 12 };         // Parameters here
  std::cout << "  ack(3,12):" << f2.get() << '\n'; // Output: ack(3,12):32765
  th2.join();
}
```

----------------


## Listing 29.40: How to determine if the operations are lock-free.

Book listing [lst-0056-book.cpp](lst-0056-book.cpp):
```cpp
// https://godbolt.org/z/dG318o9ez
#include <iostream>
#include <atomic>

struct CArray { int a[100]; };
struct Simple { int x, y; };

int main() {
    std::atomic<CArray> carray{};
    std::cout << (carray.is_lock_free() ? "lock-free" : "locks")
        << '\n';                               // Output: locks
    std::atomic<Simple> simple{};
    std::cout << (simple.is_lock_free() ? "lock-free" : "locks")
        << '\n';                               // Output: lock-free
}
```

Godbolt Listing [lst-0056-godb.cpp](lst-0056-godb.cpp), [https://godbolt.org/z/dG318o9ez](https://godbolt.org/z/dG318o9ez):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23 -latomic; libs:boost@184
// https://godbolt.org/z/dG318o9ez
#include <iostream>
#include <atomic>

struct CArray { int a[100]; };
struct Simple { int x, y; };

int main() {
    std::atomic<CArray> carray{};
    std::cout << (carray.is_lock_free() ? "lock-free" : "locks")
        << '\n';                               // Output: locks
    std::atomic<Simple> simple{};
    std::cout << (simple.is_lock_free() ? "lock-free" : "locks")
        << '\n';                               // Output: lock-free
}
```

----------------


## Listing 29.41: The "SpinlockMutex" class prevents sleeping while waiting.

Book listing [lst-0057-book.cpp](lst-0057-book.cpp):
```cpp
// https://godbolt.org/z/EnjPr5r76
#include <atomic>
class SpinlockMutex {
  std::atomic_flag flag_;
public:
  SpinlockMutex()
  : flag_{ATOMIC_FLAG_INIT}
  {}
  void lock() {                               // e.g., called by lock_guard
    while(flag_.test_and_set(std::memory_order_acquire)) // mostly read
      { /* nothing */ }
  }
  void unlock() {
    flag_.clear(std::memory_order_release);              // write operation
  }
};
```

Godbolt Listing [lst-0057-godb.cpp](lst-0057-godb.cpp), [https://godbolt.org/z/EnjPr5r76](https://godbolt.org/z/EnjPr5r76):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23 -latomic; libs:-
// https://godbolt.org/z/EnjPr5r76
#include <atomic>
class SpinlockMutex {
  std::atomic_flag flag_;
public:
  SpinlockMutex()
  : flag_{ATOMIC_FLAG_INIT}
  {}
  void lock() {                               // e.g., called by lock_guard
    while(flag_.test_and_set(std::memory_order_acquire)) // mostly read
      { /* nothing */ }
  }
  void unlock() {
    flag_.clear(std::memory_order_release);              // write operation
  }
};
```

----------------


## Listing 29.42: Your coroutine returns a "generator".

Book listing [lst-0058-book.cpp](lst-0058-book.cpp):
```cpp
// https://godbolt.org/z/Gfazq6GE7 
#include <generator>
#include <iostream>
#include <vector>

std::generator<int> fib(int n) { // generates int values
  int a = 0, b = 1;
  while (--n) {
    co_yield b;                  // makes this function a coroutine
    auto tmp = a;
    a = b;
    b += tmp;
  }
}
int main() {
  for (auto i : fib(10)) std::cout << i << ' ';
  std::cout << '\n';              // Output: 1 1 2 3 5 8 13 21 34 55
}
```

Godbolt Listing [lst-0058-godb.cpp](lst-0058-godb.cpp), [https://godbolt.org/z/Gfazq6GE7](https://godbolt.org/z/Gfazq6GE7):
```cpp
//#(compile) c++; compiler:gsnapshot; options:"-std=c++23"; libs:-
// https://godbolt.org/z/Gfazq6GE7 
#include <generator>
#include <iostream>
#include <vector>

std::generator<int> fib(int n) { // generates int values
  int a = 0, b = 1;
  while (--n) {
    co_yield b;                  // makes this function a coroutine
    auto tmp = a;
    a = b;
    b += tmp;
  }
}
int main() {
  for (auto i : fib(10)) std::cout << i << ' ';
  std::cout << '\n';              // Output: 1 1 2 3 5 8 13 21 34 55
}
```

----------------


## Listing 29.43: A simple example of a coroutine return type with its "promise_type".

Book listing [lst-0059-book.cpp](lst-0059-book.cpp):
```cpp
// https://godbolt.org/z/8enx43Ksb
#include <coroutine>
#include <iostream>
struct Routine { // Coroutine return type
  struct promise_type;                     // defined further ahead
  Routine(auto h) : hdl_(h) {}             // Constructor
  ~Routine() { if(hdl_) hdl_.destroy(); }  // Destructor
  Routine(const Routine&) = delete;        // no copies
  Routine(Routine&&) = delete;             // no moves
  // Interface for the caller:
  bool more() {                            // continue coroutine
    if(!hdl_ || hdl_.done()) return false; // - nothing more to do
    hdl_.resume();                         // - blocking call
    return !hdl_.done();                   // - signals done or not
  }
private:
  using handle_type_ = std::coroutine_handle<promise_type>;
  handle_type_ hdl_;     // to control the coroutine
};
struct Routine::promise_type {             // necessary promise_type
  auto get_return_object() {               // initializes the handle
    return Routine{ handle_type_::from_promise(*this) };
  }
  auto initial_suspend() { return std::suspend_never{}; } // start immediately
  auto final_suspend() noexcept { return std::suspend_always{}; } // normal case
  void unhandled_exception() {}            // no exception handling
  void return_void() {}                    // Coroutine has no return
};
Routine counter() {  // Return type is our wrapper class with the promise_type
  for (unsigned i = 0;; ++i) {
    co_await std::suspend_always{};        // Pause coroutine
    std::cout << "counter: " << i << std::endl;
  }
}

int main() {
  Routine routine = counter();             // Create and start coroutine
  for (int i = 0; i < 3; ++i) {
      std::cout << "Hello main!\n";
      routine.more();                      // continue coroutine
  }
}
```

Godbolt Listing [lst-0059-godb.cpp](lst-0059-godb.cpp), [https://godbolt.org/z/8enx43Ksb](https://godbolt.org/z/8enx43Ksb):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/8enx43Ksb
#include <coroutine>
#include <iostream>
struct Routine { // Coroutine return type
  struct promise_type;                     // defined further ahead
  Routine(auto h) : hdl_(h) {}             // Constructor
  ~Routine() { if(hdl_) hdl_.destroy(); }  // Destructor
  Routine(const Routine&) = delete;        // no copies
  Routine(Routine&&) = delete;             // no moves
  // Interface for the caller:
  bool more() {                            // continue coroutine
    if(!hdl_ || hdl_.done()) return false; // - nothing more to do
    hdl_.resume();                         // - blocking call
    return !hdl_.done();                   // - signals done or not
  }
private:
  using handle_type_ = std::coroutine_handle<promise_type>;
  handle_type_ hdl_;     // to control the coroutine
};
struct Routine::promise_type {             // necessary promise_type
  auto get_return_object() {               // initializes the handle
    return Routine{ handle_type_::from_promise(*this) };
  }
  auto initial_suspend() { return std::suspend_never{}; } // start immediately
  auto final_suspend() noexcept { return std::suspend_always{}; } // normal case
  void unhandled_exception() {}            // no exception handling
  void return_void() {}                    // Coroutine has no return
};
Routine counter() {  // Return type is our wrapper class with the promise_type
  for (unsigned i = 0;; ++i) {
    co_await std::suspend_always{};        // Pause coroutine
    std::cout << "counter: " << i << std::endl;
  }
}

int main() {
  Routine routine = counter();             // Create and start coroutine
  for (int i = 0; i < 3; ++i) {
      std::cout << "Hello main!\n";
      routine.more();                      // continue coroutine
  }
}
```

----------------

