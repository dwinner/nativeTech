
// https://godbolt.org/z/47j8h9e7v
#include <chrono>
#include <iostream>
void sleep(std::chrono::seconds dur) {
    std::cout << dur.count() << "s\n";
    /* ... */
}
int main() {
    using namespace std::chrono;
    sleep(3);          // (ERR) Error: no implicit conversion from int
    sleep(seconds{4}); // okay
    sleep(5s);         // okay
    seconds x{6};
    sleep(x);          // okay
    auto y = 10s;
    y += 20s;          // Incrementing with seconds
    sleep(y);          // now 30s
    y = y - 6s;        // Subtraction of seconds
    sleep(y);          // and now only 24s
    y /= 2;            // Division by a scalar
    sleep(y);          // 12s
    sleep(y + 7);      // (ERR) Error: seconds+int is not allowed
}