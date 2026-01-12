
// https://godbolt.org/z/9qsGW6M8Y 
template<std::ranges::range T>         // a range over type T
using ValueTypeOfRange = std::ranges::range_value_t<T>;
// or
template<typename T>
requires std::ranges::range<T>         // a range over type T
using ValueTypeOfRange = std::ranges::range_value_t<T>;
// results in:
ValueTypeOfRange<std::vector<int>> x;  // x is int
ValueTypeOfRange<std::string> y;       // y is char
ValueTypeOfRange<std::list<double>> z; // z is double
ValueTypeOfRange<int> w;               // (ERR) int is not a range