//#(compile) c++; compiler:g141; options:-O1 -std=c++23 -latomic; libs:boost@184
// https://godbolt.org/z/dG318o9ez
#include <iostream>
#include <atomic>

struct CArray { int a[100]; };
struct Simple { int x, y; };

int main() {
    std::atomic<CArray> carray{};
    std::cout << (carray.is_lock_free() ? "lock-free" : "locks")
        << '\n';                               // Output: locks
    std::atomic<Simple> simple{};
    std::cout << (simple.is_lock_free() ? "lock-free" : "locks")
        << '\n';                               // Output: lock-free
}