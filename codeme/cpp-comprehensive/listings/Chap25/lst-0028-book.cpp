
// https://godbolt.org/z/e54xzT9dW
#include <ranges>
#include <iostream>
#include <string_view>
#include <vector>
using namespace std::literals;
using namespace std; namespace vs = std::views; namespace rs = std::ranges;

// Example 1
class Add_1: public rs::range_adaptor_closure<Add_1> { 
  // derive from helper class
public:
  template<rs::input_range R>
  constexpr auto operator()(R&& r) const {        // universal reference
    return forward<R>(r)                          // preserve universal reference
      | vs::transform([](auto i) {return i+1;});  // Your implementation
  }
};
Add_1 add_1{};                                    // Create range adapter

// Example 2
class Dna_to_rna: public rs::range_adaptor_closure<Dna_to_rna> { // derive
public:
  template<rs::input_range R>
  constexpr auto operator()(R&& r) const {        // universal reference
    return forward<R>(r)                          // preserve universal reference
      | vs::transform([](char c)                  // Your implementation
      {
        switch(c) {
          case 'T': return 'U';
          case 't': return 'u';
          default: return c;
        }
    });
  }
};
Dna_to_rna dna_to_rna{};                          // Create Range-Adapter
// Use examples
int main() {
  vector vec{1, 2, 3, 4, 5};
  for(auto i: vec | add_1)                        // use
    cout << i << ' ';
  cout << '\n';          // Output: 2 3 4 5 6
  auto telo_rep = "TTAGGGTTAGGGTTAGGGTTAGGGT"sv;
  for(auto c: telo_rep | dna_to_rna)              // use
        cout << c;
  cout << '\n';          // Output: UUAGGGUUAGGGUUAGGGUUAGGGU
}