
// https://godbolt.org/z/W9sK4jnzK
#include <iostream>
#include <set>
#include <string>
struct Dwarf {
  std::string name;
  int height;
  auto operator<=>(const Dwarf& other) const {
    return height <=> other.height;
  }
};
int main() {
  std::set<Dwarf> dwarves {
    {"Thorin", 140}, {"Balin", 136}, {"Kili", 138},
    {"Dwalin", 139}, {"Oin", 135},   {"Gloin", 137},
  };
  for(auto &d: dwarves) {
    std::cout << d.name << ' ';
  } // Output: Oin Balin Kili Gloin Dwalin Thorin
}