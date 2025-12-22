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