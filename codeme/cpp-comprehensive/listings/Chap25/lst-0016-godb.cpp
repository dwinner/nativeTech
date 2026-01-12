//#(compile) c++; compiler:g132; options:"-std=c++23"; libs:-
// https://godbolt.org/z/j9ozax8qE 
#include <ranges> // zip_transform
// …
auto res = std::views::zip_transform(f, a, b);  // Range adapter
for(auto s: res) { std::cout << s << " "; };
std::cout << '\n';         // Output: N1 C7 C0 -1