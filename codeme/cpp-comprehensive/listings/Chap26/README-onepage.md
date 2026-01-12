# Listings of Chap26.docx

This is the list of listings on one page.
You can also view a [linked summary](README.md).


## Listing 26.1: With a "vector", you can simulate a "set".

Book listing [lst-0001-book.cpp](lst-0001-book.cpp):
```cpp
// https://godbolt.org/z/T4d4jdqox
#include <vector>
#include <iostream>
#include <algorithm>
using std::vector; using std::ostream; using std::cout;
int main() {
    vector<int> data{};
    data.reserve(400);                     // Space for 400 elements
    // Phase 1: fill
    for(int idx = 1; idx <= 20; ++idx) {
        for(int val = 0; val < 20; ++val) {
            data.push_back(val % idx);    // something between 0 and 19
        }
    }
    cout << data.size() << '\n';          // 400 elements between 0 and 19
    // Post-processing Phase 1: create set-equivalent
    std::sort(data.begin(), data.end());  // preparation for unique
    auto wo = std::unique(data.begin(), data.end()); // duplicates to the end
    data.erase(wo, data.end());           // remove duplicates
    data.shrink_to_fit();
    cout << data.size() << '\n';          // only 20 elements left
    // Phase 2: use
    for(auto &e:data)
        cout << e << ' ';                 // Output: 0 1 2 .. 18 19
    cout << '\n';
    auto it = std::lower_bound(data.begin(), data.end(), 16); // search value
    if(it!=data.end() && *it == 16)
        cout << "found!\n";
    if(std::binary_search(data.begin(), data.end(), 7))   // yes or no
        cout << "also found!\n";
}
```

Godbolt Listing [lst-0001-godb.cpp](lst-0001-godb.cpp), [https://godbolt.org/z/T4d4jdqox](https://godbolt.org/z/T4d4jdqox):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/T4d4jdqox
#include <vector>
#include <iostream>
#include <algorithm>
using std::vector; using std::ostream; using std::cout;
int main() {
    vector<int> data{};
    data.reserve(400);                     // Space for 400 elements
    // Phase 1: fill
    for(int idx = 1; idx <= 20; ++idx) {
        for(int val = 0; val < 20; ++val) {
            data.push_back(val % idx);    // something between 0 and 19
        }
    }
    cout << data.size() << '\n';          // 400 elements between 0 and 19
    // Post-processing Phase 1: create set-equivalent
    std::sort(data.begin(), data.end());  // preparation for unique
    auto wo = std::unique(data.begin(), data.end()); // duplicates to the end
    data.erase(wo, data.end());           // remove duplicates
    data.shrink_to_fit();
    cout << data.size() << '\n';          // only 20 elements left
    // Phase 2: use
    for(auto &e:data)
        cout << e << ' ';                 // Output: 0 1 2 .. 18 19
    cout << '\n';
    auto it = std::lower_bound(data.begin(), data.end(), 16); // search value
    if(it!=data.end() && *it == 16)
        cout << "found!\n";
    if(std::binary_search(data.begin(), data.end(), 7))   // yes or no
        cout << "also found!\n";
}
```

----------------


## GodboltId:fGKsfs1Y4

Book listing [lst-0002-book.cpp](lst-0002-book.cpp):
```cpp
// https://godbolt.org/z/fGKsfs1Y4 
#include <iostream>  // cout
#include <algorithm> // copy
#include <iterator>  // ostream_iterator
#include <vector>
int main() {
  std::vector<char> path{};
  for (char ch = 'a'; ch <= 'z'; ++ch) {
    path.push_back(ch);
  }
  std::ranges::copy(path, // here everything, but it also works with other ranges
    std::ostream_iterator<char>(std::cout, " ") // copy to cout, separator " "
  );
}
```

Godbolt Listing [lst-0002-godb.cpp](lst-0002-godb.cpp), [https://godbolt.org/z/fGKsfs1Y4](https://godbolt.org/z/fGKsfs1Y4):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/fGKsfs1Y4 
#include <iostream>  // cout
#include <algorithm> // copy
#include <iterator>  // ostream_iterator
#include <vector>
int main() {
  std::vector<char> path{};
  for (char ch = 'a'; ch <= 'z'; ++ch) {
    path.push_back(ch);
  }
  std::ranges::copy(path, // here everything, but it also works with other ranges
    std::ostream_iterator<char>(std::cout, " ") // copy to cout, separator " "
  );
}
```

----------------


## Listing 26.2: You get back an array enlarged by one element.

Book listing [lst-0003-book.cpp](lst-0003-book.cpp):
```cpp
// https://godbolt.org/z/Yde6azqoc
#include <iostream>
#include <array>
#include <vector>
#include <string>
using std::array; using std::move; using std::forward;

// == enlarging array ==
template<typename T, size_t S, std::size_t... Idx>
constexpr array<T, S+1>
help_append(array<T, S>&& data, T&& elem, std::index_sequence<Idx...>) {
  return { std::get<Idx>(forward<array<T, S>>(data))..., forward<T>(elem) };
}
template<typename T, size_t S>
constexpr auto
append(array<T, S> data, T elem) {
  return help_append(move(data), move(elem),
                  std::make_index_sequence<S>{});
}

// == Example ==
class Picture {            // rule of zero; movable
  std::vector<char> data_; // lots of data
  std::string name_;
public:
  explicit Picture(const std::string& name) : data_(1000,0), name_{name}
    { /* ... load picture here ... */ }
  auto name() const { return name_; }
};
int main() {
  // before
  array pics{Picture{"Mona"}, Picture{"Scream"}, Picture{"Vincent"}};
  std::cout << pics[0].name() << '\n'; // Output: Mona
  // enlarge
  Picture new_pic { "Clocks" };
  auto more = append(move(pics), move(new_pic));
  // after
  std::cout << pics[0].name() << '\n'; // Output:
  std::cout << more[0].name() << '\n'; // Output: Mona
  std::cout << more[3].name() << '\n'; // Output: Clocks
}
```

Godbolt Listing [lst-0003-godb.cpp](lst-0003-godb.cpp), [https://godbolt.org/z/Yde6azqoc](https://godbolt.org/z/Yde6azqoc):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/Yde6azqoc
#include <iostream>
#include <array>
#include <vector>
#include <string>
using std::array; using std::move; using std::forward;

// == enlarging array ==
template<typename T, size_t S, std::size_t... Idx>
constexpr array<T, S+1>
help_append(array<T, S>&& data, T&& elem, std::index_sequence<Idx...>) {
  return { std::get<Idx>(forward<array<T, S>>(data))..., forward<T>(elem) };
}
template<typename T, size_t S>
constexpr auto
append(array<T, S> data, T elem) {
  return help_append(move(data), move(elem),
                  std::make_index_sequence<S>{});
}

// == Example ==
class Picture {            // rule of zero; movable
  std::vector<char> data_; // lots of data
  std::string name_;
public:
  explicit Picture(const std::string& name) : data_(1000,0), name_{name}
    { /* ... load picture here ... */ }
  auto name() const { return name_; }
};
int main() {
  // before
  array pics{Picture{"Mona"}, Picture{"Scream"}, Picture{"Vincent"}};
  std::cout << pics[0].name() << '\n'; // Output: Mona
  // enlarge
  Picture new_pic { "Clocks" };
  auto more = append(move(pics), move(new_pic));
  // after
  std::cout << pics[0].name() << '\n'; // Output:
  std::cout << more[0].name() << '\n'; // Output: Mona
  std::cout << more[3].name() << '\n'; // Output: Clocks
}
```

----------------


## Listing 26.3: "append" at runtime.

Book listing [lst-0007-book.cpp](lst-0007-book.cpp):
```cpp
// https://godbolt.org/z/MMP16Mj3W 
#include <iostream>
#include <array>
#include <string>
using std::array;
template<typename T, size_t S>
auto append(const array<T, S>& data, T elem) {
    array<T, S+1> result {};
    for(auto i=0u; i < data.size(); ++i)
        result[i] = data[i];
    result[S] =elem;
    return result;
}
int main() {
    // before
    array pics { 3, 4, 5 };
    std::cout << pics[0] << '\n'; // Output: 3
    // enlarge
    auto more = append(pics, 77);
    // after
    std::cout << more[3] << '\n'; // Output: 77
}
```

Godbolt Listing [lst-0007-godb.cpp](lst-0007-godb.cpp), [https://godbolt.org/z/MMP16Mj3W](https://godbolt.org/z/MMP16Mj3W):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/MMP16Mj3W 
#include <iostream>
#include <array>
#include <string>
using std::array;
template<typename T, size_t S>
auto append(const array<T, S>& data, T elem) {
    array<T, S+1> result {};
    for(auto i=0u; i < data.size(); ++i)
        result[i] = data[i];
    result[S] =elem;
    return result;
}
int main() {
    // before
    array pics { 3, 4, 5 };
    std::cout << pics[0] << '\n'; // Output: 3
    // enlarge
    auto more = append(pics, 77);
    // after
    std::cout << more[3] << '\n'; // Output: 77
}
```

----------------


## Listing 26.4: Different implementations of an algorithm, distinguished by concepts.

Book listing [lst-0008-book.cpp](lst-0008-book.cpp):
```cpp
// https://godbolt.org/z/rqz1s6h11
#include <iostream>
#include <vector>
#include <list>
#include <ranges>
namespace rs = std::ranges;

template<rs::range R>
void alg(R&& range) {
  if constexpr(rs::random_access_range<R>)
    std::cout << "random access.\n";
  else if constexpr(rs::bidirectional_range<R>)
    std::cout << "bidirectional, but not random access\n";
  else static_assert(false, "unsupported range type");
}
int main() {
    std::vector<int> vec {};        // vector is random access
    alg(vec);
    std::list<int> lst;             // list is only bidirectional
    alg(lst);
    std::istreambuf_iterator<char> i1{std::cin}, i2{}; // not even bidirectional
    auto fwd = rs::subrange{i1, i2};
    alg(fwd);                       // (ERR) Error: no matching implementation
}
```

Godbolt Listing [lst-0008-godb.cpp](lst-0008-godb.cpp), [https://godbolt.org/z/rqz1s6h11](https://godbolt.org/z/rqz1s6h11):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/rqz1s6h11
#include <iostream>
#include <vector>
#include <list>
#include <ranges>
namespace rs = std::ranges;

template<rs::range R>
void alg(R&& range) {
  if constexpr(rs::random_access_range<R>)
    std::cout << "random access.\n";
  else if constexpr(rs::bidirectional_range<R>)
    std::cout << "bidirectional, but not random access\n";
  else static_assert(false, "unsupported range type");
}
int main() {
    std::vector<int> vec {};        // vector is random access
    alg(vec);
    std::list<int> lst;             // list is only bidirectional
    alg(lst);
    std::istreambuf_iterator<char> i1{std::cin}, i2{}; // not even bidirectional
    auto fwd = rs::subrange{i1, i2};
    alg(fwd);                       // (ERR) Error: no matching implementation
}
```

----------------

