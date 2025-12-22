//#(execute) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/5snr1hq9b 
#include <chrono>
void sleep(std::chrono::seconds s) { // takes seconds as duration
    /* ... */
}
/* ... */
int main() {
    using namespace std::chrono;  // make literals available
    sleep(10min);   // wait for 10 minutes, i.e., 600 seconds
    sleep(10ms);    // (ERR) 10 milliseconds? Cannot be represented with seconds.
}