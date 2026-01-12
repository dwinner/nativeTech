//#(compile) c++; compiler:gsnapshot; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/h8Pze74ME
#include <deque>
#include <ranges>                       // C++20
// …
std::deque in{1,2,33,34,35,99};
vector thirty(in.begin()+2, in.begin()+5);
for(auto &e : thirty) {
    cout << e << ' ';
}
cout << '\n';
namespace vs = std::ranges::views;      // C++20
auto v = in | vs::drop(2) | vs::take(3);
vector otuz(v.begin(), v.end());
vector trente(std::from_range, in);     // C++23