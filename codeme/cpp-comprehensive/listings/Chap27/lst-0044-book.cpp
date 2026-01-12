
// https://godbolt.org/z/WqjK31Gn6 
// 'Elf' as before…
template<> struct std::formatter<Elf> {
  std::string attribs;                  // Sequence of '%n', '%g', '%e', '%v', and others
  constexpr auto parse(std::format_parse_context& pctx) {
    auto it = std::ranges::find(pctx.begin(), pctx.end(), '}');// search for '}'
    attribs = std::string(pctx.begin(), it);                   // save everything
    return it;                                                 // points to '}'
  }
  auto format(const Elf& elf, std::format_context& fctx) const {
    auto out = fctx.out();                                     // into here
    for(auto n=0u; n<attribs.size()-1; ++n) {
      if(attribs[n] == '%') {             // instruction to output a member
        switch(attribs[++n]) {
          case 'n': out = std::format_to(out, "{}", elf.name); break;
          case 'g': out = std::format_to(out, "{}", elf.birth_year); break;
          case 'e': out = std::format_to(out, "{}", elf.era); break;
          case 'v': out = std::format_to(out, "{}", elf.folk); break;
          case '%': out = std::format_to(out, "%"); break;     // %% becomes %
        }
      } else {
        out = std::format_to(out, "{}", attribs[n]);           // everything else
      }
    }
    return out;                                                // points to the end
  }
};
int main() {
  Elf e{"Feanor", 1169, "EZ", "Nordor"};
  std::cout << std::format("{:Elf %n}", e) << std::endl;
  // Output: Elf Feanor
  std::cout << std::format("Elf {:%n, %v, born %g in the age %e}\n", e);
  // Output: Elf Feanor, Nordor, born 1169 in the age EZ
}