//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/rqz1s6h11
#include <iostream>
#include <vector>
#include <list>
#include <ranges>
namespace rs = std::ranges;

template<rs::range R>
void alg(R&& range) {
  if constexpr(rs::random_access_range<R>)
    std::cout << "random access.\n";
  else if constexpr(rs::bidirectional_range<R>)
    std::cout << "bidirectional, but not random access\n";
  else static_assert(false, "unsupported range type");
}
int main() {
    std::vector<int> vec {};        // vector is random access
    alg(vec);
    std::list<int> lst;             // list is only bidirectional
    alg(lst);
    std::istreambuf_iterator<char> i1{std::cin}, i2{}; // not even bidirectional
    auto fwd = rs::subrange{i1, i2};
    alg(fwd);                       // (ERR) Error: no matching implementation
}