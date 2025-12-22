
// https://godbolt.org/z/j7sa9rexh
#include <random>
#include <vector>
#include <iostream>
#include <functional>
void rollDice() {
  std::default_random_engine engine{};
  std::vector<size_t> counts{0,0,0,0,0,0};
  std::uniform_int_distribution<int> d6{0, 5}; // uniformly distributed integers
  auto d = std::bind(d6, engine);              // d() = d6(engine)
  for(auto i=1200*1000; i>0; --i) ++counts[d()];
  for(auto c : counts) std::cout<<" "<<c;
  std::cout << '\n';
}
int main() {
    rollDice();
}