#include <charconv> // from_chars
#include <iostream> // cout
#include <sstream> // istringstream
#include <cstdlib> // strtod, atof
#include <chrono> // time measurement

using namespace std::chrono;
using std::cout;

long long millisSeit(steady_clock::time_point start) {
    return duration_cast<milliseconds>(steady_clock::now()-start).count();
}

namespace {

    constexpr size_t ITERATIONS = 1000*1000;

    template<typename FUNC>
    void timeStuff(std::string name, FUNC func) {
        cout << name << "...";
        auto start = steady_clock::now();
        for(size_t idx=0; idx<ITERATIONS; ++idx) {
            double d = func(std::string("123.345"));
            (void)d;
        }
        cout << " " << millisSeit(start) << " ms" << std::endl;
    }

}

int main() {
    std::istringstream iss;
    timeStuff("istream", [&iss](const std::string &s) {
            iss.str(s);
            double val;
            iss >> val;
            return val;
        });
    timeStuff("stod", [](const std::string &s) {
            return std::stod(s);
        });
    timeStuff("strtod", [](const std::string &s) {
            return std::strtod(s.c_str(), nullptr);
        });
    timeStuff("atof", [](const std::string &s) {
            return std::atof(s.c_str());
        });
    timeStuff("sscanf", [](const std::string &s) {
            double val;
            sscanf(s.c_str(),"%lf",&val);
            return val;
        });
#if 0 // geht noch nicht scheinbar
    timeStuff("from_chars", [](const std::string &s) {
            double val;
            (void)std::from_chars(s.data(), s.data()+s.size(), val);
            return val;
        });
#endif
}


