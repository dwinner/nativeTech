
// https://godbolt.org/z/qnadYPWh1 
#include <iostream>
#include <thread>
using std::cout; using std::endl;

long fib(long n) { return n<=1 ? n : fib(n-1)+fib(n-2); }
void task1() { auto r = fib(40); cout << "fib(40)=" << r << endl; }
void task2() { auto r = fib(41); cout << "fib(41)=" << r << endl; }
void task3() { auto r = fib(42); cout << "fib(42)=" << r << endl; }

struct BackgroundTask {
    void operator()() const {
        task1();
        task2();
        task3();
    }
};

int main() {
    BackgroundTask backgroundTask{}; // Initialization, does not compute anything yet
    std::jthread myThread{ backgroundTask }; // Computation starts
}