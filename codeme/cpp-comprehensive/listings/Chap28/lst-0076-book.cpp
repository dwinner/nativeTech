
// https://godbolt.org/z/a8zP84j1x 
#include <thread>
#include <iostream>
#include <system_error>
int main() {
    try {
        std::thread().detach(); // this will fail
    } catch(std::system_error& e) {
        std::cout
            << "system_error with code:" << e.code()
            << " message:" << e.what()
            << '\n';
    }
}