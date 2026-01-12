//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:boost@184
// https://godbolt.org/z/115nE7xsc
#include <charconv>
#include <vector>
#include <iostream>
#include <string>

std::vector<size_t> num_to_vec(const std::string& nums) {
    std::vector<size_t> result {};
    // without trailing spaces
    const auto end = nums.data() + nums.find_last_not_of( ' ' ) + 1;
    const char* st = nullptr;  // loop pointer
    auto last = nums.data();   // last untranslated character
    size_t n;                  // converted number
    do {
        for(st = last; (st<end)&&(*st==' ' ); ++st); // skip ' '
        if (last = std::from_chars(st, end, n).ptr; last != st)
            result.push_back(n);                     // store number
    } while (last != st);
    return result;
}

void errorDemo(const char* buf, size_t sz) {
    int n;
    auto [p, ec] = std::from_chars(buf, buf+sz, n);
    if (ec != std::errc{}) {
       const auto error = std::make_error_code(ec);
       std::cout << error.message() << '\n';
    }
}

int main() {
    auto result = num_to_vec("12 33 43");

    for(auto r : result) std::cout << r << " ";
    std::cout << '\n';
    // Output: 12 33 43

    errorDemo("XYZ", 4);
    // Output: Invalid argument

    errorDemo("123123123123123", 16);
    // Output: Numerical result out of range
}