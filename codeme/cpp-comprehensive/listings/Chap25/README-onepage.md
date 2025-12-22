# Listings of Chap25.docx

This is the list of listings on one page.
You can also view a [linked summary](README.md).


## Listing 25.1: Something is done with the input vector—but what?

Book listing [lst-0001-book.cpp](lst-0001-book.cpp):
```cpp
// https://godbolt.org/z/jcWvThn3a
std::vector v{0,1,3,5,7,9,2,4,6,8};
bool flag = true;
for(size_t i=1; (i < v.size()) && flag; ++i) {
    flag = false;
    for(size_t j=0; (j < v.size()-i); ++j) {
        if(v[j+1] < v[j]) {
            std::swap(v[j+1], v[j]);
            flag = true;
        }
    }
}
for(int i:v) std::cout << i << ' ';
```

Godbolt Listing [lst-0001-godb.cpp](lst-0001-godb.cpp), [https://godbolt.org/z/jcWvThn3a](https://godbolt.org/z/jcWvThn3a):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23 -Wall -Wextra -pedantic; libs:-
// https://godbolt.org/z/jcWvThn3a
std::vector v{0,1,3,5,7,9,2,4,6,8};
bool flag = true;
for(size_t i=1; (i < v.size()) && flag; ++i) {
    flag = false;
    for(size_t j=0; (j < v.size()-i); ++j) {
        if(v[j+1] < v[j]) {
            std::swap(v[j+1], v[j]);
            flag = true;
        }
    }
}
for(int i:v) std::cout << i << ' ';
```

----------------


## Listing 25.2: This input vector will be sorted!

Book listing [lst-0002-book.cpp](lst-0002-book.cpp):
```cpp
// https://godbolt.org/z/EjT7nPPv7 
std::vector v{0,1,3,5,7,9,2,4,6,8};
std::sort(v.begin(), v.end());
for(int i:v) std::cout << i << ' ';
std::cout << '\n';
// or with a range:
std::ranges::sort(v);
for(int i:v) std::cout << i << ' ';
std::cout << '\n';
```

Godbolt Listing [lst-0002-godb.cpp](lst-0002-godb.cpp), [https://godbolt.org/z/EjT7nPPv7](https://godbolt.org/z/EjT7nPPv7):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/EjT7nPPv7 
std::vector v{0,1,3,5,7,9,2,4,6,8};
std::sort(v.begin(), v.end());
for(int i:v) std::cout << i << ' ';
std::cout << '\n';
// or with a range:
std::ranges::sort(v);
for(int i:v) std::cout << i << ' ';
std::cout << '\n';
```

----------------


## Listing 25.3: Composing functions

Book listing [lst-0003-book.cpp](lst-0003-book.cpp):
```cpp
// https://godbolt.org/z/9xrP8Ehvd 
#include <algorithm>
#include <vector>
#include <iostream>
void sort_uniq(std::vector<int> &data) {
    std::ranges::sort(data);                        // sorting
    auto to_delete = std::ranges::unique(data);     // move to the back
    data.erase(to_delete.begin(), to_delete.end()); // actually delete
}
int main() {
    std::vector ns{1,5,2,3,9,2,2,2,2,1,5,2,2,3,1,1,2,2,1};
    sort_uniq(ns);
    std::ranges::for_each(ns, [](auto x) {
        std::cout << x << ' '; });
    std::cout << '\n'; // Output: 1 2 3 5 9
}
```

Godbolt Listing [lst-0003-godb.cpp](lst-0003-godb.cpp), [https://godbolt.org/z/9xrP8Ehvd](https://godbolt.org/z/9xrP8Ehvd):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/9xrP8Ehvd 
#include <algorithm>
#include <vector>
#include <iostream>
void sort_uniq(std::vector<int> &data) {
    std::ranges::sort(data);                        // sorting
    auto to_delete = std::ranges::unique(data);     // move to the back
    data.erase(to_delete.begin(), to_delete.end()); // actually delete
}
int main() {
    std::vector ns{1,5,2,3,9,2,2,2,2,1,5,2,2,3,1,1,2,2,1};
    sort_uniq(ns);
    std::ranges::for_each(ns, [](auto x) {
        std::cout << x << ' '; });
    std::cout << '\n'; // Output: 1 2 3 5 9
}
```

----------------


## Listing 25.4: Algorithms run in parallel.

Book listing [lst-0004-book.cpp](lst-0004-book.cpp):
```cpp
// https://godbolt.org/z/Koxoha5bx
#include <algorithm>  // find
#include <numeric>    // reduce, accumulate
#include <execution>  // std::execution
#include <iostream>
#include <chrono>     // time measurement
using namespace std::chrono;
long long millisSince(steady_clock::time_point start) {
    return duration_cast<milliseconds>(steady_clock::now()-start).count();
}
template <typename FUNC> void timeit(const char* title, FUNC func) {
    auto start = steady_clock::now();
    auto ret = func(); // execute
    std::cout << title << ": " << millisSince(start) << " ms" << std::endl;
}
int main() {
    using namespace std::execution;          // seq, par, par_unseq
    using std::reduce; using std::accumulate; using std::find;
    std::vector<double> v(600'000'000, 0.0); // 600 million elements
    for(auto&x:v) x = ::rand();              // fill with random values
    timeit("warm-up       ", [&v] {
        return reduce(seq, v.begin(), v.end(), 0.0);
    });
    timeit("accumulate       ", [&v] {
        return accumulate(v.begin(), v.end(), 0.0);
    });
    timeit("reduce, seq      ", [&v] {
        return reduce(seq, v.begin(), v.end(), 0.0);
    });
    timeit("reduce, par      ", [&v] {
        return reduce(par, v.begin(), v.end(), 0.0);
    });

    timeit("reduce, par_unseq", [&v] {
        return reduce(par_unseq, v.begin(), v.end(), 0.0);
    });
    timeit("find, seq        ", [&v] {
        return find(seq, v.begin(), v.end(), 1.1) == v.end() ? 0.0 : 1.0;
    });
    timeit("find, par        ", [&v] {
        return find(par, v.begin(), v.end(), 1.1) == v.end() ? 0.0 : 1.0;
    });
    return 0;
}
```

Godbolt Listing [lst-0004-godb.cpp](lst-0004-godb.cpp), [https://godbolt.org/z/Koxoha5bx](https://godbolt.org/z/Koxoha5bx):
```cpp
//#(compile) c++; compiler:g112; options:-O3 -std=c++23; libs:tbb@20214
// https://godbolt.org/z/Koxoha5bx
#include <algorithm>  // find
#include <numeric>    // reduce, accumulate
#include <execution>  // std::execution
#include <iostream>
#include <chrono>     // time measurement
using namespace std::chrono;
long long millisSince(steady_clock::time_point start) {
    return duration_cast<milliseconds>(steady_clock::now()-start).count();
}
template <typename FUNC> void timeit(const char* title, FUNC func) {
    auto start = steady_clock::now();
    auto ret = func(); // execute
    std::cout << title << ": " << millisSince(start) << " ms" << std::endl;
}
int main() {
    using namespace std::execution;          // seq, par, par_unseq
    using std::reduce; using std::accumulate; using std::find;
    std::vector<double> v(600'000'000, 0.0); // 600 million elements
    for(auto&x:v) x = ::rand();              // fill with random values
    timeit("warm-up       ", [&v] {
        return reduce(seq, v.begin(), v.end(), 0.0);
    });
    timeit("accumulate       ", [&v] {
        return accumulate(v.begin(), v.end(), 0.0);
    });
    timeit("reduce, seq      ", [&v] {
        return reduce(seq, v.begin(), v.end(), 0.0);
    });
    timeit("reduce, par      ", [&v] {
        return reduce(par, v.begin(), v.end(), 0.0);
    });

    timeit("reduce, par_unseq", [&v] {
        return reduce(par_unseq, v.begin(), v.end(), 0.0);
    });
    timeit("find, seq        ", [&v] {
        return find(seq, v.begin(), v.end(), 1.1) == v.end() ? 0.0 : 1.0;
    });
    timeit("find, par        ", [&v] {
        return find(par, v.begin(), v.end(), 1.1) == v.end() ? 0.0 : 1.0;
    });
    return 0;
}
```

----------------


## Listing 25.5: View types and their adapters.

Book listing [lst-0005-book.cpp](lst-0005-book.cpp):
```cpp
// https://godbolt.org/z/rrq1v5qPs 
std::list lst{1, 2, 3, 4, 5, 6, 7, 8, 9};
auto take5 = rs::take_view{lst, 5};     // View via type
auto take6 = lst | vs::take(6);         // View via adapter
```

Godbolt Listing [lst-0005-godb.cpp](lst-0005-godb.cpp), [https://godbolt.org/z/rrq1v5qPs](https://godbolt.org/z/rrq1v5qPs):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/rrq1v5qPs 
std::list lst{1, 2, 3, 4, 5, 6, 7, 8, 9};
auto take5 = rs::take_view{lst, 5};     // View via type
auto take6 = lst | vs::take(6);         // View via adapter
```

----------------


## Listing 25.6: Especially the C++23 views can be well combined.

Book listing [lst-0006-book.cpp](lst-0006-book.cpp):
```cpp
// https://godbolt.org/z/hYTv6f9eo 
#include <ranges>
#include <array>
#include <string>
#include <iostream>
#include <string_view>
using namespace std::literals; using namespace std;

// Function for outputting all sorts of things
template <typename OBJ>
void print(OBJ&& obj, int level = 0) {
  if constexpr(std::ranges::input_range<OBJ>) {                 // range
    cout << '[';
    for (const auto& elem : obj) print(elem, level+1);
    cout << ']';
  } else if constexpr(requires (OBJ tpl){ std::get<0>(tpl);}) { // tuple/pair
    cout << "(";
    print(get<0>(obj), level+1); print(get<1>(obj), level+1);
    cout << ")";
  } else cout << obj;                                           // element
  if (level == 0) cout << '\n';
}
int main() {
  using namespace std::views; // exceptionally for brevity
  auto const nums = array{0, 0, 1, 1, 2, 2};
  auto const animals = array{"cat"s, "dog"s};
  print(iota(0, 5) | chunk(2));                      // Output: [[01][23][4]]
  print(nums | chunk_by(equal_to{}));                // Output: [[00][11][22]]
  print(iota(0, 5) | slide(3));                      // Output: [[012][123][234]]
  print(iota(0, 10) | stride(3));                    // Output: [0369]
  print(repeat(8) |take(5));                         // Output: [88888]
  print(zip_transform(plus{}, nums, nums));          // Output: [002244]
  print(zip(iota(0, 3), iota(1, 4)));                // Output: [(01)(12)(23)]
  print(iota(0, 4) | adjacent<2>);                   // Output: [(01)(12)(23)]
  print(iota(0, 4) | pairwise);                      // Output: [(01)(12)(23)]
  print(iota(0, 4) | adjacent_transform<2>(plus{})); // Output: [135]
  print(iota(0, 4) | pairwise_transform(plus{}));    // Output: [135]
  print(animals | join_with( '+' ));                 // Output: [cat+dog]
  print(cartesian_product(iota(0, 2), "AZ"s));       // Output: [(0A)(0Z)(1A)(1Z)]
  print(enumerate("APL"s));                          // Output: [(0A)(1P)(2L)]
  return 0;
}
```

Godbolt Listing [lst-0006-godb.cpp](lst-0006-godb.cpp), [https://godbolt.org/z/hYTv6f9eo](https://godbolt.org/z/hYTv6f9eo):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/hYTv6f9eo 
#include <ranges>
#include <array>
#include <string>
#include <iostream>
#include <string_view>
using namespace std::literals; using namespace std;

// Function for outputting all sorts of things
template <typename OBJ>
void print(OBJ&& obj, int level = 0) {
  if constexpr(std::ranges::input_range<OBJ>) {                 // range
    cout << '[';
    for (const auto& elem : obj) print(elem, level+1);
    cout << ']';
  } else if constexpr(requires (OBJ tpl){ std::get<0>(tpl);}) { // tuple/pair
    cout << "(";
    print(get<0>(obj), level+1); print(get<1>(obj), level+1);
    cout << ")";
  } else cout << obj;                                           // element
  if (level == 0) cout << '\n';
}
int main() {
  using namespace std::views; // exceptionally for brevity
  auto const nums = array{0, 0, 1, 1, 2, 2};
  auto const animals = array{"cat"s, "dog"s};
  print(iota(0, 5) | chunk(2));                      // Output: [[01][23][4]]
  print(nums | chunk_by(equal_to{}));                // Output: [[00][11][22]]
  print(iota(0, 5) | slide(3));                      // Output: [[012][123][234]]
  print(iota(0, 10) | stride(3));                    // Output: [0369]
  print(repeat(8) |take(5));                         // Output: [88888]
  print(zip_transform(plus{}, nums, nums));          // Output: [002244]
  print(zip(iota(0, 3), iota(1, 4)));                // Output: [(01)(12)(23)]
  print(iota(0, 4) | adjacent<2>);                   // Output: [(01)(12)(23)]
  print(iota(0, 4) | pairwise);                      // Output: [(01)(12)(23)]
  print(iota(0, 4) | adjacent_transform<2>(plus{})); // Output: [135]
  print(iota(0, 4) | pairwise_transform(plus{}));    // Output: [135]
  print(animals | join_with( '+' ));                 // Output: [cat+dog]
  print(cartesian_product(iota(0, 2), "AZ"s));       // Output: [(0A)(0Z)(1A)(1Z)]
  print(enumerate("APL"s));                          // Output: [(0A)(1P)(2L)]
  return 0;
}
```

----------------


## Listing 25.7: Functions must receive a range parameter as a universal reference.

Book listing [lst-0009-book.cpp](lst-0009-book.cpp):
```cpp
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
```

Godbolt Listing [lst-0009-godb.cpp](lst-0009-godb.cpp), [https://godbolt.org/z/3EbsPczh1](https://godbolt.org/z/3EbsPczh1):
```cpp
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
```

----------------


## Listing 25.8: A function only for views and not for containers.

Book listing [lst-0010-book.cpp](lst-0010-book.cpp):
```cpp
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
```

Godbolt Listing [lst-0010-godb.cpp](lst-0010-godb.cpp), [https://godbolt.org/z/s1GjfWxv6](https://godbolt.org/z/s1GjfWxv6):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
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
```

----------------


## Listing 25.9: Searching with a predicate.

Book listing [lst-0011-book.cpp](lst-0011-book.cpp):
```cpp
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
```

Godbolt Listing [lst-0011-godb.cpp](lst-0011-godb.cpp), [https://godbolt.org/z/W9n7sTovz](https://godbolt.org/z/W9n7sTovz):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
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
```

----------------


## Listing 25.10: Recognizing a palindrome with a single line of code.

Book listing [lst-0012-book.cpp](lst-0012-book.cpp):
```cpp
// https://godbolt.org/z/MKxnGdrbr
#include <algorithm>
#include <iostream>
#include <vector>
#include <string>
#include <string_view>
using std::string_view; using namespace std::literals; using std::cout;
auto isPalindrome(string_view sv) {
  return std::ranges::equal(sv.begin(), sv.end(), sv.rbegin(), sv.rend()); };
int main() {
  for(auto s : {"madam"sv, "aibohphobia"sv, "abrakadabra"sv }) {
    cout << s << " is " << (isPalindrome(s)?"a":"not a") << " palindrome\n";
  }
}
```

Godbolt Listing [lst-0012-godb.cpp](lst-0012-godb.cpp), [https://godbolt.org/z/MKxnGdrbr](https://godbolt.org/z/MKxnGdrbr):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/MKxnGdrbr
#include <algorithm>
#include <iostream>
#include <vector>
#include <string>
#include <string_view>
using std::string_view; using namespace std::literals; using std::cout;
auto isPalindrome(string_view sv) {
  return std::ranges::equal(sv.begin(), sv.end(), sv.rbegin(), sv.rend()); };
int main() {
  for(auto s : {"madam"sv, "aibohphobia"sv, "abrakadabra"sv }) {
    cout << s << " is " << (isPalindrome(s)?"a":"not a") << " palindrome\n";
  }
}
```

----------------


## Listing 25.11: Even "read-only" algorithms like "for_each" can modify elements.

Book listing [lst-0013-book.cpp](lst-0013-book.cpp):
```cpp
// https://godbolt.org/z/jz1Gqo7d9 
#include <iostream>
#include <vector>
#include <algorithm>
int main() {
    std::vector<int> c{ 1,2,3,4 };
    std::ranges::for_each(c, [](auto &n) { n*=n; }); // (ERR) modifying
    std::cout << c[3] << '\n'; // Output: 16
}
```

Godbolt Listing [lst-0013-godb.cpp](lst-0013-godb.cpp), [https://godbolt.org/z/jz1Gqo7d9](https://godbolt.org/z/jz1Gqo7d9):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/jz1Gqo7d9 
#include <iostream>
#include <vector>
#include <algorithm>
int main() {
    std::vector<int> c{ 1,2,3,4 };
    std::ranges::for_each(c, [](auto &n) { n*=n; }); // (ERR) modifying
    std::cout << c[3] << '\n'; // Output: 16
}
```

----------------


## Listing 25.12: "transform" can juggle with different types.

Book listing [lst-0015-book.cpp](lst-0015-book.cpp):
```cpp
// https://godbolt.org/z/7r4b8jdGr 
#include <algorithm>
#include <iostream>
#include <vector>
#include <string>
using std::to_string; using std::string; using std::vector;
struct Squares {
    mutable int n = 1;
    int operator()() const { return n*n++; }
};
int main() {
    vector<int> sq(10);
    std::ranges::generate(sq, Squares{});
    std::ranges::for_each(sq, [](auto n) {
        std::cout << n << " "; });
    std::cout << '\n';         // Output: 1 4 9 16 25 36 49 64 81 100
    string a = "NCC-";
    vector<int> b {1,7,0,1};
    vector<string> c(4);
    auto f = [](char c, int i) -> string { return c+to_string(i); };
    std::ranges::transform(
        a,           // Input 1
        b,           // Input 2
        c.begin(),   // Output
        f);          // string f(char,int)
    std::ranges::for_each(c, [](auto s) {
        std::cout << s << " "; });
    std::cout << '\n';         // Output: N1 C7 C0 -1
}
```

Godbolt Listing [lst-0015-godb.cpp](lst-0015-godb.cpp), [https://godbolt.org/z/7r4b8jdGr](https://godbolt.org/z/7r4b8jdGr):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/7r4b8jdGr 
#include <algorithm>
#include <iostream>
#include <vector>
#include <string>
using std::to_string; using std::string; using std::vector;
struct Squares {
    mutable int n = 1;
    int operator()() const { return n*n++; }
};
int main() {
    vector<int> sq(10);
    std::ranges::generate(sq, Squares{});
    std::ranges::for_each(sq, [](auto n) {
        std::cout << n << " "; });
    std::cout << '\n';         // Output: 1 4 9 16 25 36 49 64 81 100
    string a = "NCC-";
    vector<int> b {1,7,0,1};
    vector<string> c(4);
    auto f = [](char c, int i) -> string { return c+to_string(i); };
    std::ranges::transform(
        a,           // Input 1
        b,           // Input 2
        c.begin(),   // Output
        f);          // string f(char,int)
    std::ranges::for_each(c, [](auto s) {
        std::cout << s << " "; });
    std::cout << '\n';         // Output: N1 C7 C0 -1
}
```

----------------


## Listing 25.13: Transforming with range adapters.

Book listing [lst-0016-book.cpp](lst-0016-book.cpp):
```cpp
// https://godbolt.org/z/j9ozax8qE 
#include <ranges> // zip_transform
// …
auto res = std::views::zip_transform(f, a, b);  // Range adapter
for(auto s: res) { std::cout << s << " "; };
std::cout << '\n';         // Output: N1 C7 C0 -1
```

Godbolt Listing [lst-0016-godb.cpp](lst-0016-godb.cpp), [https://godbolt.org/z/j9ozax8qE](https://godbolt.org/z/j9ozax8qE):
```cpp
//#(compile) c++; compiler:g132; options:"-std=c++23"; libs:-
// https://godbolt.org/z/j9ozax8qE 
#include <ranges> // zip_transform
// …
auto res = std::views::zip_transform(f, a, b);  // Range adapter
for(auto s: res) { std::cout << s << " "; };
std::cout << '\n';         // Output: N1 C7 C0 -1
```

----------------


## GodboltId:s53qx664K

Book listing [lst-0017-book.cpp](lst-0017-book.cpp):
```cpp
// https://godbolt.org/z/s53qx664K 
#include <iostream>
#include <random>       // default_random_engine
#include <string>
#include <iterator>     // back_inserter
#include <algorithm>    // sample

int main() {
   std::default_random_engine random{};
   const std::string in = "abcdefgh";
   for(auto idx : {0,1,2,3}) {
     std::string out;
     std::ranges::sample(in, std::back_inserter(out), 5, random);
     std::cout << out << '\n';
   }
}
```

Godbolt Listing [lst-0017-godb.cpp](lst-0017-godb.cpp), [https://godbolt.org/z/s53qx664K](https://godbolt.org/z/s53qx664K):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/s53qx664K 
#include <iostream>
#include <random>       // default_random_engine
#include <string>
#include <iterator>     // back_inserter
#include <algorithm>    // sample

int main() {
   std::default_random_engine random{};
   const std::string in = "abcdefgh";
   for(auto idx : {0,1,2,3}) {
     std::string out;
     std::ranges::sample(in, std::back_inserter(out), 5, random);
     std::cout << out << '\n';
   }
}
```

----------------


## Listing 25.14: Functionality of the set algorithms.

Book listing [lst-0018-book.cpp](lst-0018-book.cpp):
```cpp
// https://godbolt.org/z/1esod6hrT
#include <algorithm>
#include <iostream>
#include <list>
#include <string>
#include <iterator> // ostream_iterator
#include <cctype>   // toupper
using std::toupper;
int main() {
    std::list a{ 1,2,4,4,4,7,7,9 };
    std::list b{ 2,2,3,4,4,8 };
    using Os = std::ostream_iterator<int>; // Type of output iterator
    Os os{std::cout, " "};                 // Stream output iterator for int
    auto run = [&a,&b,&os](auto algo) {    // use a, b, and os
        algo(a.begin(), a.end(), b.begin(), b.end(), os); // Call algorithm
        std::cout << '\n';
    };

    // Results of the algorithms
    using It = decltype(a.begin());        // Type of input iterators

    run(std::merge<It,It,Os>);             // Output: 1 2 2 2 3 4 4 4 4 4 7 7 8 9
    run(std::set_union<It,It,Os>);         // Output: 1 2 2 3 4 4 4 7 7 8 9
    run(std::set_intersection<It,It,Os>);  // Output: 2 4 4
    run(std::set_difference<It,It,Os>);    // Output: 1 4 7 7 9
    run(std::set_symmetric_difference<It,It,Os>); // Output: 1 2 3 4 7 7 8 9

    // With letters it becomes even clearer
    std::string x = "abdddggi";
    std::string y = "BBCDDH";
    using Us = std::ostream_iterator<char>;   // Type of output iterator
    Us us{std::cout, ""};                     // Stream output iterator for char
    auto compare = [](auto c, auto d) { return toupper(c) < toupper(d); };
    auto run2 = [&x,&y,&us,&compare](auto algo) {    // use x, y, and us
        algo(x.begin(), x.end(), y.begin(), y.end(), us, compare);
        std::cout << '\n';
    };
    // Results of the algorithms
    using Jt = decltype(x.begin());           // Type of input iterators
    using Cm = decltype(compare);             // Type of comparison function

    run2(std::merge<Jt,Jt,Us,Cm>);            // Output: abBBCdddDDggHi
    run2(std::set_union<Jt,Jt,Us,Cm>);        // Output: abBCdddggHi
    run2(std::set_intersection<Jt,Jt,Us,Cm>); // Output: bdd
    run2(std::set_difference<Jt,Jt,Us,Cm>);   // Output: adggi
    run2(std::set_symmetric_difference<Jt,Jt,Us,Cm>); // Output: aBCdggHi
}
```

Godbolt Listing [lst-0018-godb.cpp](lst-0018-godb.cpp), [https://godbolt.org/z/1esod6hrT](https://godbolt.org/z/1esod6hrT):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/1esod6hrT
#include <algorithm>
#include <iostream>
#include <list>
#include <string>
#include <iterator> // ostream_iterator
#include <cctype>   // toupper
using std::toupper;
int main() {
    std::list a{ 1,2,4,4,4,7,7,9 };
    std::list b{ 2,2,3,4,4,8 };
    using Os = std::ostream_iterator<int>; // Type of output iterator
    Os os{std::cout, " "};                 // Stream output iterator for int
    auto run = [&a,&b,&os](auto algo) {    // use a, b, and os
        algo(a.begin(), a.end(), b.begin(), b.end(), os); // Call algorithm
        std::cout << '\n';
    };

    // Results of the algorithms
    using It = decltype(a.begin());        // Type of input iterators

    run(std::merge<It,It,Os>);             // Output: 1 2 2 2 3 4 4 4 4 4 7 7 8 9
    run(std::set_union<It,It,Os>);         // Output: 1 2 2 3 4 4 4 7 7 8 9
    run(std::set_intersection<It,It,Os>);  // Output: 2 4 4
    run(std::set_difference<It,It,Os>);    // Output: 1 4 7 7 9
    run(std::set_symmetric_difference<It,It,Os>); // Output: 1 2 3 4 7 7 8 9

    // With letters it becomes even clearer
    std::string x = "abdddggi";
    std::string y = "BBCDDH";
    using Us = std::ostream_iterator<char>;   // Type of output iterator
    Us us{std::cout, ""};                     // Stream output iterator for char
    auto compare = [](auto c, auto d) { return toupper(c) < toupper(d); };
    auto run2 = [&x,&y,&us,&compare](auto algo) {    // use x, y, and us
        algo(x.begin(), x.end(), y.begin(), y.end(), us, compare);
        std::cout << '\n';
    };
    // Results of the algorithms
    using Jt = decltype(x.begin());           // Type of input iterators
    using Cm = decltype(compare);             // Type of comparison function

    run2(std::merge<Jt,Jt,Us,Cm>);            // Output: abBBCdddDDggHi
    run2(std::set_union<Jt,Jt,Us,Cm>);        // Output: abBCdddggHi
    run2(std::set_intersection<Jt,Jt,Us,Cm>); // Output: bdd
    run2(std::set_difference<Jt,Jt,Us,Cm>);   // Output: adggi
    run2(std::set_symmetric_difference<Jt,Jt,Us,Cm>); // Output: aBCdggHi
}
```

----------------


## GodboltId:j6GPKc36P

Book listing [lst-0019-book.cpp](lst-0019-book.cpp):
```cpp
// https://godbolt.org/z/j6GPKc36P
#include <iostream>
#include <algorithm>
#include <string>
void one(std::string &seq) {
    std::ranges::next_permutation(seq);
    std::cout << seq << '\n';
}
int main() {
    std::string seq = "JQK";
    std::cout << seq << '\n'; // Output: JQK
    auto limit = 3*2*1;       // n!
    for(int i=0; i<limit; ++i)
        one(seq);
    // Here the sequence is back to its original state.
}
```

Godbolt Listing [lst-0019-godb.cpp](lst-0019-godb.cpp), [https://godbolt.org/z/j6GPKc36P](https://godbolt.org/z/j6GPKc36P):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/j6GPKc36P
#include <iostream>
#include <algorithm>
#include <string>
void one(std::string &seq) {
    std::ranges::next_permutation(seq);
    std::cout << seq << '\n';
}
int main() {
    std::string seq = "JQK";
    std::cout << seq << '\n'; // Output: JQK
    auto limit = 3*2*1;       // n!
    for(int i=0; i<limit; ++i)
        one(seq);
    // Here the sequence is back to its original state.
}
```

----------------


## Listing 25.15: "accumulate" example.

Book listing [lst-0020-book.cpp](lst-0020-book.cpp):
```cpp
// https://godbolt.org/z/Y7GPcqWM8
#include <numeric>    // accumulate
#include <functional> // multiplies
#include <algorithm>  // transform, fold_left
#include <iostream>
#include <vector>
using std::accumulate; using std::cout; using std::vector; using std::multiplies;
namespace rs = std::ranges;
int main() {
  vector data{ 2, 3, 5, 10, 20 };
  cout << accumulate(data.begin(),data.end(),0)<<'\n';     // +, Output: 40
  cout << rs::fold_left(data, 1, multiplies<int>{})<<'\n'; // *, 6000
  vector<bool> even( data.size() );
  std::transform( data.begin(), data.end(), even.begin(),
      [](auto n) { return n%2==0; });
  for(auto b : even) {
      cout << ( b ? "even " : "odd ");
  }
  cout << "\n";        // Output: even odd odd even even
  auto areAllEven = accumulate(even.begin(), even.end(), true,
          [](auto b, auto c) { return b&&c; });
  if(areAllEven) {
      cout << "all even numbers\n";
  } else {
      cout << "odd numbers included\n"; // this is the output
  }
}
```

Godbolt Listing [lst-0020-godb.cpp](lst-0020-godb.cpp), [https://godbolt.org/z/Y7GPcqWM8](https://godbolt.org/z/Y7GPcqWM8):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/Y7GPcqWM8
#include <numeric>    // accumulate
#include <functional> // multiplies
#include <algorithm>  // transform, fold_left
#include <iostream>
#include <vector>
using std::accumulate; using std::cout; using std::vector; using std::multiplies;
namespace rs = std::ranges;
int main() {
  vector data{ 2, 3, 5, 10, 20 };
  cout << accumulate(data.begin(),data.end(),0)<<'\n';     // +, Output: 40
  cout << rs::fold_left(data, 1, multiplies<int>{})<<'\n'; // *, 6000
  vector<bool> even( data.size() );
  std::transform( data.begin(), data.end(), even.begin(),
      [](auto n) { return n%2==0; });
  for(auto b : even) {
      cout << ( b ? "even " : "odd ");
  }
  cout << "\n";        // Output: even odd odd even even
  auto areAllEven = accumulate(even.begin(), even.end(), true,
          [](auto b, auto c) { return b&&c; });
  if(areAllEven) {
      cout << "all even numbers\n";
  } else {
      cout << "odd numbers included\n"; // this is the output
  }
}
```

----------------


## Listing 25.16: "adjacent_difference" and "pairwise_transform".

Book listing [lst-0021-book.cpp](lst-0021-book.cpp):
```cpp
// https://godbolt.org/z/5Whz59374
#include <numeric>    // adjacent_difference
#include <functional> // plus
#include <algorithm>  // copy
#include <iostream>
#include <iterator>   // ostream_iterator
#include <vector>
#include <ranges>     // pairwise_transform
using std::cout; using std::vector; namespace vs = std::views;
int main() {
    // Stream output iterator for int;
    std::ostream_iterator<int> os{std::cout, " "};
    vector data{ 1, -1, 2, -2, -4, 4, -6, 6 };
    std::copy(data.begin(), data.end (), os);
    cout << '\n'; // Output: 1 -1 2 -2 -4 4 -6 6
    vector<int> res( data.size()-1 ); // Space for result
    // Write results to res:
    adjacent_difference(data.begin(), data.end(), res.begin());
    std::copy (res.begin(), res.end (), os);
    cout << '\n'; // Output: 1 -2 3 -4 -2 8 -10
    // Write directly to os:
    adjacent_difference(data.begin(), data.end(), os, std::plus<int>{});
    cout << '\n'; // Output: 1 0 1 0 -6 0 -2 0
    // or via range adapter:
    for(auto e: vs::pairwise_transform(data, std::plus<int>{}))
        cout << e << ' ';
    cout << '\n'; // Output: 0 1 0 –6 0 –2 0
}
```

Godbolt Listing [lst-0021-godb.cpp](lst-0021-godb.cpp), [https://godbolt.org/z/5Whz59374](https://godbolt.org/z/5Whz59374):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/5Whz59374
#include <numeric>    // adjacent_difference
#include <functional> // plus
#include <algorithm>  // copy
#include <iostream>
#include <iterator>   // ostream_iterator
#include <vector>
#include <ranges>     // pairwise_transform
using std::cout; using std::vector; namespace vs = std::views;
int main() {
    // Stream output iterator for int;
    std::ostream_iterator<int> os{std::cout, " "};
    vector data{ 1, -1, 2, -2, -4, 4, -6, 6 };
    std::copy(data.begin(), data.end (), os);
    cout << '\n'; // Output: 1 -1 2 -2 -4 4 -6 6
    vector<int> res( data.size()-1 ); // Space for result
    // Write results to res:
    adjacent_difference(data.begin(), data.end(), res.begin());
    std::copy (res.begin(), res.end (), os);
    cout << '\n'; // Output: 1 -2 3 -4 -2 8 -10
    // Write directly to os:
    adjacent_difference(data.begin(), data.end(), os, std::plus<int>{});
    cout << '\n'; // Output: 1 0 1 0 -6 0 -2 0
    // or via range adapter:
    for(auto e: vs::pairwise_transform(data, std::plus<int>{}))
        cout << e << ' ';
    cout << '\n'; // Output: 0 1 0 –6 0 –2 0
}
```

----------------


## Listing 25.17: "iota", "stride", and "take"

Book listing [lst-0022-book.cpp](lst-0022-book.cpp):
```cpp
// https://godbolt.org/z/WPrP79sv6
#include <numeric>   // accumulate, iota
#include <algorithm> // copy
#include <iostream>
#include <iterator>  // ostream_iterator
#include <vector>
#include <ranges>    // iota, take, stride
using std::accumulate; using std::cout; using std::vector; 
namespace vs = std::views;
struct Generator {
    int state_;
    void operator++() { state_ += state_; }
    operator int() { return state_; }
};
int main() {
    std::ostream_iterator<int> os{std::cout, " "}; // Stream output iterator for int
    vector<int> data(7);
    std::iota(data.begin(), data.end(), 10);
    std::copy(data.begin(), data.end (), os);
    cout << '\n';           // Output: 10 11 12 13 14 15 16
    vector<int> seq(7);
    std::iota(seq.begin(), seq.end(), Generator{2});
    std::copy(seq.begin(), seq.end(), os);
    cout << '\n';           // Output: 2 4 8 16 32 64 128
    for(auto i: vs::iota(0) | vs::stride(3) | vs::take(6))
        cout << i << ' ';
    cout << '\n';           // Output: 0 3 6 9 12 15
}
```

Godbolt Listing [lst-0022-godb.cpp](lst-0022-godb.cpp), [https://godbolt.org/z/WPrP79sv6](https://godbolt.org/z/WPrP79sv6):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/WPrP79sv6
#include <numeric>   // accumulate, iota
#include <algorithm> // copy
#include <iostream>
#include <iterator>  // ostream_iterator
#include <vector>
#include <ranges>    // iota, take, stride
using std::accumulate; using std::cout; using std::vector; 
namespace vs = std::views;
struct Generator {
    int state_;
    void operator++() { state_ += state_; }
    operator int() { return state_; }
};
int main() {
    std::ostream_iterator<int> os{std::cout, " "}; // Stream output iterator for int
    vector<int> data(7);
    std::iota(data.begin(), data.end(), 10);
    std::copy(data.begin(), data.end (), os);
    cout << '\n';           // Output: 10 11 12 13 14 15 16
    vector<int> seq(7);
    std::iota(seq.begin(), seq.end(), Generator{2});
    std::copy(seq.begin(), seq.end(), os);
    cout << '\n';           // Output: 2 4 8 16 32 64 128
    for(auto i: vs::iota(0) | vs::stride(3) | vs::take(6))
        cout << i << ' ';
    cout << '\n';           // Output: 0 3 6 9 12 15
}
```

----------------


## Listing 25.18: "inclusive_scan" and "exclusive_scan".

Book listing [lst-0023-book.cpp](lst-0023-book.cpp):
```cpp
// https://godbolt.org/z/enj3znvTs
#include <numeric> // *_scan
#include <iostream>
#include <vector>
using std::inclusive_scan; using std::exclusive_scan;
std::ostream& operator<<=(std::ostream&os, const std::vector<int>&data) {
    for(auto &e : data) os << e << ' '; return os << '\n';
}
int main() {
  std::vector data{ 1, 3, 10, 18, 30, 50 };
  std::vector<int> result(6);    // 6 elements
  auto plus = [](auto a, auto b) { return a+b; };
  inclusive_scan(data.begin(),data.end(), result.begin(), plus, 100);
  std::cout <<= result;
  // Output: 101 104 114 132 162 212
  exclusive_scan(data.begin(),data.end(), result.begin(), 100);
  std::cout <<= result;
  // Output: 100 101 104 114 132 162
}
```

Godbolt Listing [lst-0023-godb.cpp](lst-0023-godb.cpp), [https://godbolt.org/z/enj3znvTs](https://godbolt.org/z/enj3znvTs):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/enj3znvTs
#include <numeric> // *_scan
#include <iostream>
#include <vector>
using std::inclusive_scan; using std::exclusive_scan;
std::ostream& operator<<=(std::ostream&os, const std::vector<int>&data) {
    for(auto &e : data) os << e << ' '; return os << '\n';
}
int main() {
  std::vector data{ 1, 3, 10, 18, 30, 50 };
  std::vector<int> result(6);    // 6 elements
  auto plus = [](auto a, auto b) { return a+b; };
  inclusive_scan(data.begin(),data.end(), result.begin(), plus, 100);
  std::cout <<= result;
  // Output: 101 104 114 132 162 212
  exclusive_scan(data.begin(),data.end(), result.begin(), 100);
  std::cout <<= result;
  // Output: 100 101 104 114 132 162
}
```

----------------


## GodboltId:dKPbjj1f8

Book listing [lst-0024-book.cpp](lst-0024-book.cpp):
```cpp
// https://godbolt.org/z/dKPbjj1f8 
#include <iostream>
#include <memory>    // uninitialized_copy
#include <alloca.h>  // alloca (Linux)
#include <list>
int main () {
  const std::list input{1,9,2,6,6,6,8};
  const auto SZ = input.size();
  // uninitialized memory area:
  int* target = (int*)alloca(sizeof(int) * SZ); // space for 7 ints
  std::uninitialized_copy(input.begin(), input.end(), target);
  // test output
  for(int idx=0; idx<SZ; ++idx) {
    std::cout << target[idx] << ' ';
  }
  std::cout << '\n'; // output: 1 9 2 6 6 6 8
}
```

Godbolt Listing [lst-0024-godb.cpp](lst-0024-godb.cpp), [https://godbolt.org/z/dKPbjj1f8](https://godbolt.org/z/dKPbjj1f8):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/dKPbjj1f8 
#include <iostream>
#include <memory>    // uninitialized_copy
#include <alloca.h>  // alloca (Linux)
#include <list>
int main () {
  const std::list input{1,9,2,6,6,6,8};
  const auto SZ = input.size();
  // uninitialized memory area:
  int* target = (int*)alloca(sizeof(int) * SZ); // space for 7 ints
  std::uninitialized_copy(input.begin(), input.end(), target);
  // test output
  for(int idx=0; idx<SZ; ++idx) {
    std::cout << target[idx] << ' ';
  }
  std::cout << '\n'; // output: 1 9 2 6 6 6 8
}
```

----------------


## Listing 26.19: A custom algorithm.

Book listing [lst-0026-book.cpp](lst-0026-book.cpp):
```cpp
// https://godbolt.org/z/n8ErWj7j6 
template<typename It, typename Func>
void adjacent_pair(It begin, It end, Func func) {
    if(begin != end) {
        It prev = begin;    // first argument
        ++begin;            // second argument
        for(; begin != end; ++begin, ++prev) {
            func(*prev, *begin);
        }
    }
}
```

Godbolt Listing [lst-0026-godb.cpp](lst-0026-godb.cpp), [https://godbolt.org/z/n8ErWj7j6](https://godbolt.org/z/n8ErWj7j6):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/n8ErWj7j6 
template<typename It, typename Func>
void adjacent_pair(It begin, It end, Func func) {
    if(begin != end) {
        It prev = begin;    // first argument
        ++begin;            // second argument
        for(; begin != end; ++begin, ++prev) {
            func(*prev, *begin);
        }
    }
}
```

----------------


## Listing 25.19: The use of adjacent_pair.

Book listing [lst-0027-book.cpp](lst-0027-book.cpp):
```cpp
// https://godbolt.org/z/91jj8Toqs 
#include <vector>
#include <iostream>
// … adjacent_pair from above here …
int main() {
    std::vector v{1,2,3,4};
    auto f = [](auto a, auto b) { std::cout << (a+b) << ' '; };
    adjacent_pair(v.begin(), v.end(), f); // 3 5 7
    std::cout << '\n';

    std::vector x{4,8};
    adjacent_pair(x.begin(), x.end(), f); // 12
    std::cout << '\n';

    std::vector w{4};
    adjacent_pair(w.begin(), w.end(), f); // nothing
    std::cout << '\n';

    std::vector<int> y{};
    adjacent_pair(y.begin(), y.end(), f); // nothing
    std::cout << '\n';
}
```

Godbolt Listing [lst-0027-godb.cpp](lst-0027-godb.cpp), [https://godbolt.org/z/91jj8Toqs](https://godbolt.org/z/91jj8Toqs):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/91jj8Toqs 
#include <vector>
#include <iostream>
// … adjacent_pair from above here …
int main() {
    std::vector v{1,2,3,4};
    auto f = [](auto a, auto b) { std::cout << (a+b) << ' '; };
    adjacent_pair(v.begin(), v.end(), f); // 3 5 7
    std::cout << '\n';

    std::vector x{4,8};
    adjacent_pair(x.begin(), x.end(), f); // 12
    std::cout << '\n';

    std::vector w{4};
    adjacent_pair(w.begin(), w.end(), f); // nothing
    std::cout << '\n';

    std::vector<int> y{};
    adjacent_pair(y.begin(), y.end(), f); // nothing
    std::cout << '\n';
}
```

----------------


## Listing 25.20: Custom range adapters for views.

Book listing [lst-0028-book.cpp](lst-0028-book.cpp):
```cpp
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
```

Godbolt Listing [lst-0028-godb.cpp](lst-0028-godb.cpp), [https://godbolt.org/z/e54xzT9dW](https://godbolt.org/z/e54xzT9dW):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
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
```

----------------

