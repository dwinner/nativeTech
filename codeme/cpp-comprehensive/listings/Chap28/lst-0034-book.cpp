
// https://godbolt.org/z/h8vsjxsME 
#include <cinttypes> // int64_t
namespace std { namespace chrono {
class seconds {
    int64_t sec_;
public:
    seconds() = default;
    // … etc …
};
} }