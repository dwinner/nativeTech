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