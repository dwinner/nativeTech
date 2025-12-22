
// https://godbolt.org/z/s1GjfWxv6 
void print(ranges::view auto range) { // Value parameter, restricted to Views
  for (auto const& e : range) { cout << e; } cout << '\n';
}
vector vec{1, 2, 3, 4, 5, 6, 7, 8, 9, 10};
list   lst{1, 2, 3, 4, 5, 6, 7, 8, 9, 10};
print(vec);                       // (ERR) Forbidden for containers, very good!
print(vs::all(vec));              // Convert container to view
print(vs::all(lst));              // Convert container to view
print(vec | vs::take(3));         // take with vector works
print(lst | vs::take(3));         // take with list works
print(vec | vs::drop(5));         // drop with vector works
print(lst | vs::drop(5));         // as a value parameter, drop with list works