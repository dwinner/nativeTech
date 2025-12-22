
// https://godbolt.org/z/Yde6azqoc
#include <iostream>
#include <array>
#include <vector>
#include <string>
using std::array; using std::move; using std::forward;

// == enlarging array ==
template<typename T, size_t S, std::size_t... Idx>
constexpr array<T, S+1>
help_append(array<T, S>&& data, T&& elem, std::index_sequence<Idx...>) {
  return { std::get<Idx>(forward<array<T, S>>(data))..., forward<T>(elem) };
}
template<typename T, size_t S>
constexpr auto
append(array<T, S> data, T elem) {
  return help_append(move(data), move(elem),
                  std::make_index_sequence<S>{});
}

// == Example ==
class Picture {            // rule of zero; movable
  std::vector<char> data_; // lots of data
  std::string name_;
public:
  explicit Picture(const std::string& name) : data_(1000,0), name_{name}
    { /* ... load picture here ... */ }
  auto name() const { return name_; }
};
int main() {
  // before
  array pics{Picture{"Mona"}, Picture{"Scream"}, Picture{"Vincent"}};
  std::cout << pics[0].name() << '\n'; // Output: Mona
  // enlarge
  Picture new_pic { "Clocks" };
  auto more = append(move(pics), move(new_pic));
  // after
  std::cout << pics[0].name() << '\n'; // Output:
  std::cout << more[0].name() << '\n'; // Output: Mona
  std::cout << more[3].name() << '\n'; // Output: Clocks
}