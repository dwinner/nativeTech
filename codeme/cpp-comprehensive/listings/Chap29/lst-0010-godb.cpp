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