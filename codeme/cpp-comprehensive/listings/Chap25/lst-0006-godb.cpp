//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/hYTv6f9eo 
#include <ranges>
#include <array>
#include <string>
#include <iostream>
#include <string_view>
using namespace std::literals; using namespace std;

// Function for outputting all sorts of things
template <typename OBJ>
void print(OBJ&& obj, int level = 0) {
  if constexpr(std::ranges::input_range<OBJ>) {                 // range
    cout << '[';
    for (const auto& elem : obj) print(elem, level+1);
    cout << ']';
  } else if constexpr(requires (OBJ tpl){ std::get<0>(tpl);}) { // tuple/pair
    cout << "(";
    print(get<0>(obj), level+1); print(get<1>(obj), level+1);
    cout << ")";
  } else cout << obj;                                           // element
  if (level == 0) cout << '\n';
}
int main() {
  using namespace std::views; // exceptionally for brevity
  auto const nums = array{0, 0, 1, 1, 2, 2};
  auto const animals = array{"cat"s, "dog"s};
  print(iota(0, 5) | chunk(2));                      // Output: [[01][23][4]]
  print(nums | chunk_by(equal_to{}));                // Output: [[00][11][22]]
  print(iota(0, 5) | slide(3));                      // Output: [[012][123][234]]
  print(iota(0, 10) | stride(3));                    // Output: [0369]
  print(repeat(8) |take(5));                         // Output: [88888]
  print(zip_transform(plus{}, nums, nums));          // Output: [002244]
  print(zip(iota(0, 3), iota(1, 4)));                // Output: [(01)(12)(23)]
  print(iota(0, 4) | adjacent<2>);                   // Output: [(01)(12)(23)]
  print(iota(0, 4) | pairwise);                      // Output: [(01)(12)(23)]
  print(iota(0, 4) | adjacent_transform<2>(plus{})); // Output: [135]
  print(iota(0, 4) | pairwise_transform(plus{}));    // Output: [135]
  print(animals | join_with( '+' ));                 // Output: [cat+dog]
  print(cartesian_product(iota(0, 2), "AZ"s));       // Output: [(0A)(0Z)(1A)(1Z)]
  print(enumerate("APL"s));                          // Output: [(0A)(1P)(2L)]
  return 0;
}