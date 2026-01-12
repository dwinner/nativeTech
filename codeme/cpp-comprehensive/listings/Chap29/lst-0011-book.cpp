
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