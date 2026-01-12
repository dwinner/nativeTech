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