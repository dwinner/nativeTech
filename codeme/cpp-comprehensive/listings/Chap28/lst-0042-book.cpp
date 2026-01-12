
// https://godbolt.org/z/vTx4cdqzq 
#include <chrono>
#include <iostream>
void showDuration(std::chrono::milliseconds dur) {
    std::cout << dur.count() << "ms\n";
}
int main() {
    using namespace std::chrono;
    auto x = 2s;
    auto y = 3ms;
    showDuration(x + y); // Output: 2003ms
    showDuration(x - y); // Output: 1997ms
}