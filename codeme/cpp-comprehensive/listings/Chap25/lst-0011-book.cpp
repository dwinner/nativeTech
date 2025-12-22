
// https://godbolt.org/z/W9n7sTovz
#include <algorithm>
#include <iostream>
#include <vector>
#include <string>
#include <string_view>
#include <ranges>
using std::vector; using std::string; using std::string_view;
using namespace std::literals; using std::find_if;
vector<string> demo_split(string_view s) {
  vector<string> result{};
  auto it = s.begin();
  while(it != s.end()) {
    // until normal character:
    it = find_if(it, s.end(), [](char c) { return c!=' '; });
    // until whitespace:
    auto jt = find_if(it, s.end(), [](char c) { return c==' '; });
    if(it!=s.end())
      result.push_back(string(it, jt)); // Copy to result
    it = jt;
  }
  return result;
}
int main() {
  auto text = "The text is short"sv;
  auto res = demo_split(text);
  std::ranges::for_each(res, [](const string &e) {
      std::cout << "[" << e << "] "; });
  std::cout << '\n'; // Output: [The] [text] [is] [short]
  // or directly with views::split:
  for(auto word : text | std::views::split(" "sv)) {
    std::cout << "[";
    for(auto c : word) std::cout << c;
    std::cout << "] ";
  } // Output: [The] [text] [is] [short]
}