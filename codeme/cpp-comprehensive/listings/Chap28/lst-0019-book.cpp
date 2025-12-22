
// https://godbolt.org/z/ocP83a7ac
#include <random>
#include <vector>
#include <iostream>
void rollDice() {
  std::default_random_engine engine{};         // Normal quality randomness
  std::vector<size_t> counts{0,0,0,0,0,0};
  std::uniform_int_distribution<int> d6{0, 5}; // uniformly distributed integers
  for(auto i=1200*1000; i>0; --i)
    ++counts[d6(engine)];
  for(auto c : counts) std::cout<<" "<<c;
  std::cout << '\n';
}
int main() {
    rollDice();
}