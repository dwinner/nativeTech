#include <iostream>
#include <cmath> // nextafter
#include <iomanip> // setprecision, fixed, scientific
#include <limits> // numeric_limits

template<typename T>
void run(T step) {
    static const auto inf = std::numeric_limits<T>::infinity();
    T val = (T)1.0L;
    for(size_t idx = 0; idx<1000; ++idx) {
        auto na = std::nextafter(val,  inf);
        std::cout
            << std::setprecision(10)
            << std::fixed
            << std::scientific
            << val
            << " nextafter:" << na
            << " diff:" << (na-val)
            << '\n';
        if(na == inf) break; // check == here ok
        val *= step;
    }
}

int main() {
    std::cout << "== float ==\n";
    run<float>(64.0L);
    std::cout << "== double ==\n";
    run<double>(8192.0L * 8192.0L * 8192.0L);
}
