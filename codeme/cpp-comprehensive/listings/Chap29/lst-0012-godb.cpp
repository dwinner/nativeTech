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