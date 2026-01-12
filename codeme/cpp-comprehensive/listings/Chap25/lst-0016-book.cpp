
// https://godbolt.org/z/j9ozax8qE 
#include <ranges> // zip_transform
// …
auto res = std::views::zip_transform(f, a, b);  // Range adapter
for(auto s: res) { std::cout << s << " "; };
std::cout << '\n';         // Output: N1 C7 C0 -1