#include <limits>
#include <iostream>

#include <stdfloat>
#include <format>

#include <chrono>

using namespace std;

void title() {
    cout << "| Name | " ;
    cout << " suf | ";
    cout << " bits | ";
    cout << " d2/d10 | ";
    cout << " max e2 | ";
    cout << " max() | ";
    cout << " eps() | ";
    cout << " Zeit | ";
    cout << endl;
    cout << "|----|" ; // name
    cout << "----|"; // suf
    cout << "---:|"; // bits
    cout << "---:|"; // d2/d10
    cout << "---:|"; // max e2
    cout << "---:|"; // max()
    cout << "---:|"; // eps()
    cout << "---:|"; // Zeit
    cout << endl;
}


double total_result = 0;

 template <typename Float>
 void show(const char* name, const char* suffix) {
    cout << "| " << name << " | " ;
    cout << suffix << " | ";
    cout << sizeof(Float)*8 << " | ";
    cout << numeric_limits<Float>::digits << "/";
    cout << numeric_limits<Float>::digits10 << " | ";
    cout << numeric_limits<Float>::max_exponent << " | ";
    cout << std::format("{:.4g}", numeric_limits<Float>::max()) << " | ";
    cout << std::format("{:.4g}", numeric_limits<Float>::epsilon()) << " | ";

    using namespace std::chrono;
    auto t0 = steady_clock::now();
    constexpr Float r{3.9}; // experiment here
    Float x{0.55}; // any start
    for (int i=0; i<10000; ++i) {
        x = r*(x-x*x); // chaos!
    }
    auto t1 = steady_clock::now();
    std::cout << nanoseconds{t1-t0}.count()/1000 << "ms |";
    x += (double)x; // dont let the opmitizer remove the loop

    cout << endl;
}
#define SHOW(ftype, suffix) show<ftype>(#ftype, suffix)

int main() {
    title();
    SHOW(std::float16_t, "f16");
    SHOW(std::bfloat16_t, "bf16");
    SHOW(float, "f");
    SHOW(std::float32_t, "f32");
    SHOW(double, "");
    SHOW(std::float64_t, "f64");
    SHOW(long double, "l");
    SHOW(std::float128_t, "f128");
    cout << total_result << endl;
}
