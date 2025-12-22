//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/qrs63vdz1 
#include <chrono>
#include <iostream>
int main() {
    using namespace std::chrono;             // Allow suffixes
    seconds mySecs = 121s;                   // seconds{121}
    std::cout << mySecs.count() << "s\n";    // Output: 121s
    milliseconds myMillis = mySecs;          // automatically converted
    std::cout << myMillis.count() << "ms\n"; // Output: 121000ms
    nanoseconds myNanos = mySecs;
    std::cout << myNanos.count() << "ns\n";  // Output: 121000000000ns
    minutes myMinutesErr = mySecs;           // (ERR) Error: Conversion with loss
    minutes myMinutes = duration_cast<minutes>(mySecs); // explicit works
    std::cout<<myMinutes.count()<<"min\n";   // Output: 2min
}