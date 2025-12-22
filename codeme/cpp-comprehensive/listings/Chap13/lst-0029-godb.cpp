//#(compile) c++; compiler:g141; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/bxPaz9aW5
#include <vector>
namespace {                         // anonymous namespace for constants
    const unsigned DATA_SIZE = 100; /* number of elements in data */
    const double LIMIT = 999.999;   /* max value during initialization */
};
std::vector<int> createData() {
    std::vector<int> result(DATA_SIZE);
    double currVal = 1.0;
    for(auto &elem : result) {
        elem = currVal;
        currVal *= 2;          // next value is larger
        if(currVal > LIMIT) {
            currVal = LIMIT;   // no value should be larger
        }
    }
    return result;
}