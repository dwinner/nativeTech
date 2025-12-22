//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/3EbsPczh1
void print(const auto& range) { // (ERR) critical: constant reference
  for (auto const& e : range) { cout << e; } cout << '\n';
}
vector vec{1, 2, 3, 4, 5, 6, 7, 8, 9, 10};
list   lst{1, 2, 3, 4, 5, 6, 7, 8, 9, 10};
print(vec);                  // works on direct containers
print(lst);                  // works on direct containers
print(vec | vs::take(3));    // take with vector works
print(lst | vs::take(3));    // take with list works
print(vec | vs::drop(5));    // drop with vector works
print(lst | vs::drop(5));    // (ERR) drop with list does not work!