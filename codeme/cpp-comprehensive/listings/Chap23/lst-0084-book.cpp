
// https://godbolt.org/z/E89bav3rE
#include <iostream>
template <typename T, size_t n=1>    // Nontype parameter with default value
class FixedArray {
    T data_[n] {0};                  // Using nontype parameter
public:
    T& operator[](size_t index) { return data_[index]; }
    static constexpr size_t size() { return n; }
    void print(std::ostream &os) const {
        for(auto it : data_)
            os << it << ' ';
        os << '\n';
    }
};
int main() {
    FixedArray<int,10> vals {};         // n = 10
    for(size_t idx=0; idx < vals.size(); ++idx) {
        vals[idx] = idx+idx;
    }
    vals.print(std::cout);              // Output: 0 2 4 6 8 10 12 14 16 18
    FixedArray<double> dvals;           // Default parameter for n
    std::cout << dvals.size() << '\n';  // Output: 1
}