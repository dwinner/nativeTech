
// https://godbolt.org/z/1xbhhTdEv 
#include <iostream>
#include <chrono>
int main() {
    using namespace std::chrono;
    auto tp = time_point_cast<seconds>(system_clock::now());
    auto d = tp.time_since_epoch();
    std::cout << d.count() << "s\n";
    std::cout << duration<double,std::ratio<86400>>{d}.count() << "days\n";
}