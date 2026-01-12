
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