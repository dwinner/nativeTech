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