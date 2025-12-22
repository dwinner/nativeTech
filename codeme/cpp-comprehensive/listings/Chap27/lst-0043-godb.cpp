//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/sdWsnTonG
#include <format>
#include <string>
#include <vector>
#include <iostream>
struct Elf {
  std::string name;
  int birth_year;
  std::string era;
  std::string folk;
};
template<> struct std::formatter<Elf> {
  std::formatter<std::string> sub_fmt;
  constexpr auto parse(std::format_parse_context& pctx) {
    return sub_fmt.parse(pctx); // returns iterator to '}'
  }
  auto format(const Elf& elf, std::format_context& fctx) const {
    std::string s = std::format("{}/{} ({} {})",
        elf.name, elf.folk, elf.birth_year, elf.era);
    return sub_fmt.format(s, fctx); // delegate formatting
  }
};
int main() {
  std::vector<Elf> elves{
    {"Feanor", 1169, "First Age", "Noldor"},
    {"Galadriel", 1362, "EZ", "Noldor"},
    {"Legolas", 87, "DZ", "Sindar"},
    {"Elrond", 532, "EZ", "Half-elf"},
    {"Elwe", 1050, "EZ", "Sindar"},
  };
  for (const auto& e : elves) {
    std::cout << std::format("Elf: {:>20}", e) << std::endl;
  }
}