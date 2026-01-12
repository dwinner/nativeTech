
// https://godbolt.org/z/Ycse4EWav 
#include <iostream> // cout

template<typename T>
void memory(T x) {
    if constexpr(sizeof(T) > 4) {
       std::cout << "Requires a lot of memory: " << x << "  ";
    }
}

constexpr auto DEBUG = true;
int main() {
    if constexpr(DEBUG) {
        std::cout << "Debug is on. ";
    }
    memory<long long>(44LL);
}