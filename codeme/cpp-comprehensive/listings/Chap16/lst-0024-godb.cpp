//#(compile) c++; compiler:g141; options:-O2 -std=c++23; libs:-
// https://godbolt.org/z/aoqW86vs8 
#include <vector>
class Image {
    std::vector<std::byte> data_;              // byte has been available since C++17
public:
    explicit Image(const char *fn) { /*...*/ }
    Image(Image&& other) noexcept              // move constructor
        : data_{} // create empty
    {
        using std::swap;
        swap(data_, other.data_);
    }
    Image& operator=(Image&& other) noexcept { // move operator
        using std::swap;
        swap(data_, other.data_);
        return *this;
    }
};