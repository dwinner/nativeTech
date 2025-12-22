# Listings of Chap28.docx

This is the list of listings on one page.
You can also view a [linked summary](README.md).


## Listing 28.1: How to return two values simultaneously using "pair".

Book listing [lst-0001-book.cpp](lst-0001-book.cpp):
```cpp
// https://godbolt.org/z/4YM9d99xG 
#include <iostream>
#include <string>
#include <vector>
#include <utility> // pair
using std::pair; using std::cout; using std::cin; using std::string;
std::vector<string> months { "Jan", "Feb", "Mar" };
std::vector temps { 8, 12, 11 };
std::pair<string, int> monthWithTemp(size_t m) {
    auto month = months.at(m);
    auto temperature = temps.at(m);
    return std::make_pair(month, temperature);
}
int main() {
    std::pair data = monthWithTemp(1);
    cout << "Month : " << data.first << std::endl; // Output: Month : Feb
    cout << "Temperature : " << data.second << std::endl; 
    // Output: Temperature : 12
}
```

Godbolt Listing [lst-0001-godb.cpp](lst-0001-godb.cpp), [https://godbolt.org/z/4YM9d99xG](https://godbolt.org/z/4YM9d99xG):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/4YM9d99xG 
#include <iostream>
#include <string>
#include <vector>
#include <utility> // pair
using std::pair; using std::cout; using std::cin; using std::string;
std::vector<string> months { "Jan", "Feb", "Mar" };
std::vector temps { 8, 12, 11 };
std::pair<string, int> monthWithTemp(size_t m) {
    auto month = months.at(m);
    auto temperature = temps.at(m);
    return std::make_pair(month, temperature);
}
int main() {
    std::pair data = monthWithTemp(1);
    cout << "Month : " << data.first << std::endl; // Output: Month : Feb
    cout << "Temperature : " << data.second << std::endl; 
    // Output: Temperature : 12
}
```

----------------


## Listing 28.2: With "tuple", you can return any number of elements.

Book listing [lst-0002-book.cpp](lst-0002-book.cpp):
```cpp
// https://godbolt.org/z/5z4s6WMxG
#include <iostream>
#include <string>
#include <algorithm> // min, max
#include <tuple>
using std::tuple; using std::make_tuple; using std::get; using std::cout;
using std::string;
tuple<int,int,int> arrange(int a, int b, int c) {
  using std::min; using std::max;
  auto x = min(a,min(b,c));
  auto y = max(min(a,b), min(max(a,b),c));
  auto z = max(a,max(b,c));
  return make_tuple(x, y, z);
}
auto president(int year) {
  using namespace std::literals;
  if(year >= 2021)
    return std::make_tuple("Joseph"s, "Biden"s, "Democratic"s, 1942);
  if(year >= 2017)
    return std::make_tuple("Donald"s, "Trump"s, "Republican"s, 1946);
  if(year >= 2009)
    return std::make_tuple("Barack"s, "Obama"s, "Democratic"s, 1961);
  // Add more presidents as needed...
  return std::make_tuple(""s, ""s, ""s, 0); // Default case
}
int main() {
  tuple<int,int,int> zs = arrange(23, 42, 7);
  cout << get<0>(zs) <<' '<< get<1>(zs) <<' '<< get<2>(zs) <<'\n'; 
  // Output: 7 23 42
  auto ps = president(2015);
  cout << get<1>(ps) << '\n'; // Output: Obama
}
```

Godbolt Listing [lst-0002-godb.cpp](lst-0002-godb.cpp), [https://godbolt.org/z/5z4s6WMxG](https://godbolt.org/z/5z4s6WMxG):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/5z4s6WMxG
#include <iostream>
#include <string>
#include <algorithm> // min, max
#include <tuple>
using std::tuple; using std::make_tuple; using std::get; using std::cout;
using std::string;
tuple<int,int,int> arrange(int a, int b, int c) {
  using std::min; using std::max;
  auto x = min(a,min(b,c));
  auto y = max(min(a,b), min(max(a,b),c));
  auto z = max(a,max(b,c));
  return make_tuple(x, y, z);
}
auto president(int year) {
  using namespace std::literals;
  if(year >= 2021)
    return std::make_tuple("Joseph"s, "Biden"s, "Democratic"s, 1942);
  if(year >= 2017)
    return std::make_tuple("Donald"s, "Trump"s, "Republican"s, 1946);
  if(year >= 2009)
    return std::make_tuple("Barack"s, "Obama"s, "Democratic"s, 1961);
  // Add more presidents as needed...
  return std::make_tuple(""s, ""s, ""s, 0); // Default case
}
int main() {
  tuple<int,int,int> zs = arrange(23, 42, 7);
  cout << get<0>(zs) <<' '<< get<1>(zs) <<' '<< get<2>(zs) <<'\n'; 
  // Output: 7 23 42
  auto ps = president(2015);
  cout << get<1>(ps) << '\n'; // Output: Obama
}
```

----------------


## Listing 28.3: "get" works with a type as an index.

Book listing [lst-0003-book.cpp](lst-0003-book.cpp):
```cpp
// https://godbolt.org/z/TE1EcEYPv
// … as before …
int main() {
  tuple ps = president(2015);
  cout << get<int>(ps) << '\n';     // Output: 1940
  cout << get<string>(ps) << '\n';  // (ERR) not unique
}
```

Godbolt Listing [lst-0003-godb.cpp](lst-0003-godb.cpp), [https://godbolt.org/z/TE1EcEYPv](https://godbolt.org/z/TE1EcEYPv):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/TE1EcEYPv
// … as before …
int main() {
  tuple ps = president(2015);
  cout << get<int>(ps) << '\n';     // Output: 1940
  cout << get<string>(ps) << '\n';  // (ERR) not unique
}
```

----------------


## Listing 28.4: Decomposing tuples with "tie" and "ignore".

Book listing [lst-0004-book.cpp](lst-0004-book.cpp):
```cpp
// https://godbolt.org/z/zG7zYGf3K
// … as before …
int main() {
  using std::tie; using std::ignore;
  string lastName {};
  int birthYear {};
  tie(ignore, lastName, ignore, birthYear) = president(2015);
  cout << lastName << ' ' << birthYear << '\n'; // Output: Obama 1961
}
```

Godbolt Listing [lst-0004-godb.cpp](lst-0004-godb.cpp), [https://godbolt.org/z/zG7zYGf3K](https://godbolt.org/z/zG7zYGf3K):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/zG7zYGf3K
// … as before …
int main() {
  using std::tie; using std::ignore;
  string lastName {};
  int birthYear {};
  tie(ignore, lastName, ignore, birthYear) = president(2015);
  cout << lastName << ' ' << birthYear << '\n'; // Output: Obama 1961
}
```

----------------


## Listing 28.5: Tuples implement lexicographical order.

Book listing [lst-0005-book.cpp](lst-0005-book.cpp):
```cpp
// https://godbolt.org/z/sG4E66YhW
#include <iostream>
#include <string>
#include <tuple>
#include <vector>
#include <algorithm> // ranges::sort
using std::tuple; using std::get; using std::cout; using std::string;
int main() {
    std::vector<tuple<string,string,int>> armstrongs =
      { {"Armstrong", "Louis", 1901}  // Initialize using initializer list
      , {"Armstrong", "Lance", 1971}
      , {"Armstrong", "Neil", 1930} };
    std::ranges::sort(armstrongs);    // Lance < Louis < Neil
    for(const auto& a : armstrongs) {
      cout << get<0>(a) << ", " << get<1>(a) << ", " << get<2>(a) << "\n";
    }
}
```

Godbolt Listing [lst-0005-godb.cpp](lst-0005-godb.cpp), [https://godbolt.org/z/sG4E66YhW](https://godbolt.org/z/sG4E66YhW):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/sG4E66YhW
#include <iostream>
#include <string>
#include <tuple>
#include <vector>
#include <algorithm> // ranges::sort
using std::tuple; using std::get; using std::cout; using std::string;
int main() {
    std::vector<tuple<string,string,int>> armstrongs =
      { {"Armstrong", "Louis", 1901}  // Initialize using initializer list
      , {"Armstrong", "Lance", 1971}
      , {"Armstrong", "Neil", 1930} };
    std::ranges::sort(armstrongs);    // Lance < Louis < Neil
    for(const auto& a : armstrongs) {
      cout << get<0>(a) << ", " << get<1>(a) << ", " << get<2>(a) << "\n";
    }
}
```

----------------


## Listing 28.6: Tuples as keys.

Book listing [lst-0006-book.cpp](lst-0006-book.cpp):
```cpp
// https://godbolt.org/z/9qhMo6zbb 
#include <iostream>
#include <string>
#include <tuple>
#include <map>
#include <unordered_set>

using std::tuple; using std::get; using std::cout; using std::string;

int main() {
    std::map<tuple<int,string>,double> m { {{12,"x"},3.14} };
    cout << m[{12,"x"}] << "\n"; // Output: 3.14
    std::unordered_set<tuple<int,string>> s { {12,"x"} }; // (ERR) no std::hash
}
```

Godbolt Listing [lst-0006-godb.cpp](lst-0006-godb.cpp), [https://godbolt.org/z/9qhMo6zbb](https://godbolt.org/z/9qhMo6zbb):
```cpp
//#(execute) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/9qhMo6zbb 
#include <iostream>
#include <string>
#include <tuple>
#include <map>
#include <unordered_set>

using std::tuple; using std::get; using std::cout; using std::string;

int main() {
    std::map<tuple<int,string>,double> m { {{12,"x"},3.14} };
    cout << m[{12,"x"}] << "\n"; // Output: 3.14
    std::unordered_set<tuple<int,string>> s { {12,"x"} }; // (ERR) no std::hash
}
```

----------------


## Listing 28.7: Forming tuples in-place using tie.

Book listing [lst-0007-book.cpp](lst-0007-book.cpp):
```cpp
// https://godbolt.org/z/vdT7TTjWK 
#include <iostream>
#include <string>
#include <tuple>
#include <map>
#include <unordered_set>
using std::tuple; using std::tie; using std::cout; using std::string;
struct Point {
    int x,y,z;
    bool operator<(const Point &b) {
        return tie(x,y,z) < tie(b.x, b.y, b.z);
    }
};
int main() {
    Point a{ 11, 22, 33 };
    Point b{ 11, 33, 0 };
    cout << std::boolalpha << (a < b) << "\n"; // Output: true
}
```

Godbolt Listing [lst-0007-godb.cpp](lst-0007-godb.cpp), [https://godbolt.org/z/vdT7TTjWK](https://godbolt.org/z/vdT7TTjWK):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/vdT7TTjWK 
#include <iostream>
#include <string>
#include <tuple>
#include <map>
#include <unordered_set>
using std::tuple; using std::tie; using std::cout; using std::string;
struct Point {
    int x,y,z;
    bool operator<(const Point &b) {
        return tie(x,y,z) < tie(b.x, b.y, b.z);
    }
};
int main() {
    Point a{ 11, 22, 33 };
    Point b{ 11, 33, 0 };
    cout << std::boolalpha << (a < b) << "\n"; // Output: true
}
```

----------------


## Listing 28.8: Combining several small tuples into one large tuple.

Book listing [lst-0008-book.cpp](lst-0008-book.cpp):
```cpp
// https://godbolt.org/z/eq1dqWbe9 
#include <iostream>
#include <string>
#include <tuple>
using std::tuple; using std::cout; using std::string;
int main() {
    tuple<int,string> a { 12, "gnorf" };
    tuple b { 666 };
    tuple<double,double,string> c { 77.77, 33.33, "frong" };
    tuple<int,string,int,double,double,string> r = std::tuple_cat( a, b, c );
    cout << std::get<2>(r) << "\n"; // Output: 666
}
```

Godbolt Listing [lst-0008-godb.cpp](lst-0008-godb.cpp), [https://godbolt.org/z/eq1dqWbe9](https://godbolt.org/z/eq1dqWbe9):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/eq1dqWbe9 
#include <iostream>
#include <string>
#include <tuple>
using std::tuple; using std::cout; using std::string;
int main() {
    tuple<int,string> a { 12, "gnorf" };
    tuple b { 666 };
    tuple<double,double,string> c { 77.77, 33.33, "frong" };
    tuple<int,string,int,double,double,string> r = std::tuple_cat( a, b, c );
    cout << std::get<2>(r) << "\n"; // Output: 666
}
```

----------------


## Listing 28.9: Useful tuple type traits.

Book listing [lst-0009-book.cpp](lst-0009-book.cpp):
```cpp
// https://godbolt.org/z/Psdz9s9Gc 
#include <iostream>
#include <string>
#include <tuple>
using namespace std;
template<typename Tuple>
auto back(Tuple &&tuple) {
    using Noref = typename remove_reference<Tuple>::type; // in case of Tuple&
    constexpr auto sz = tuple_size<Noref>::value;
    return get<sz-1>(forward<Tuple>(tuple));
}
int main() {
    tuple<string,int,string> enterprise = make_tuple("NCC", 1701, "D");
    cout << back(enterprise) << "\n"; // Output: D
}
```

Godbolt Listing [lst-0009-godb.cpp](lst-0009-godb.cpp), [https://godbolt.org/z/Psdz9s9Gc](https://godbolt.org/z/Psdz9s9Gc):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/Psdz9s9Gc 
#include <iostream>
#include <string>
#include <tuple>
using namespace std;
template<typename Tuple>
auto back(Tuple &&tuple) {
    using Noref = typename remove_reference<Tuple>::type; // in case of Tuple&
    constexpr auto sz = tuple_size<Noref>::value;
    return get<sz-1>(forward<Tuple>(tuple));
}
int main() {
    tuple<string,int,string> enterprise = make_tuple("NCC", 1701, "D");
    cout << back(enterprise) << "\n"; // Output: D
}
```

----------------


## Listing 28.10: Matching, searching, and enumerating with regular expressions.

Book listing [lst-0010-book.cpp](lst-0010-book.cpp):
```cpp
// https://godbolt.org/z/T1YMoM5as
#include <string>
#include <iostream>
#include <regex>
using std::regex; using std::sregex_iterator; using std::string;
const regex rgxMobile(R"(01[567]\d{6,10})");      // Mobile phone 0151-0179
bool isMobilephone(const string& text) {
  return std::regex_match(text, rgxMobile);       // Does the text match completely?
}
bool containsMobilephone(const string &text) {
  return std::regex_search(text, rgxMobile);      // somewhere in the text?
}
void listMobilephones(const string &text) {
  sregex_iterator begin{ text.cbegin(), text.cend(), rgxMobile };
  sregex_iterator end;
  for(auto it = begin; it != end; ++it)
    std::cout << it->str() << " ";                // Matched text
} // "xyz01709999 abc 0161887766 uvw" -> "0161887766"
```

Godbolt Listing [lst-0010-godb.cpp](lst-0010-godb.cpp), [https://godbolt.org/z/T1YMoM5as](https://godbolt.org/z/T1YMoM5as):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/T1YMoM5as
#include <string>
#include <iostream>
#include <regex>
using std::regex; using std::sregex_iterator; using std::string;
const regex rgxMobile(R"(01[567]\d{6,10})");      // Mobile phone 0151-0179
bool isMobilephone(const string& text) {
  return std::regex_match(text, rgxMobile);       // Does the text match completely?
}
bool containsMobilephone(const string &text) {
  return std::regex_search(text, rgxMobile);      // somewhere in the text?
}
void listMobilephones(const string &text) {
  sregex_iterator begin{ text.cbegin(), text.cend(), rgxMobile };
  sregex_iterator end;
  for(auto it = begin; it != end; ++it)
    std::cout << it->str() << " ";                // Matched text
} // "xyz01709999 abc 0161887766 uvw" -> "0161887766"
```

----------------


## Listing 28.11: Search and match.

Book listing [lst-0011-book.cpp](lst-0011-book.cpp):
```cpp
// https://godbolt.org/z/E9hcWGMW4 
#include <regex>
#include <string>
#include <iostream>
using std::regex; using std::regex_match; using std::regex_search;
int main() {
    std::cout << std::boolalpha;
    regex pattern {"ello"};
    std::string text = "Hello world";
    auto b1 = regex_match (text.cbegin(), text.cend(), pattern); // does’t match
    std::cout << b1 << "\n"; // Output: false
    auto b2 = regex_search(text.cbegin(), text.cend(), pattern); // found
    std::cout << b2 << "\n"; // Output: true
}
```

Godbolt Listing [lst-0011-godb.cpp](lst-0011-godb.cpp), [https://godbolt.org/z/E9hcWGMW4](https://godbolt.org/z/E9hcWGMW4):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/E9hcWGMW4 
#include <regex>
#include <string>
#include <iostream>
using std::regex; using std::regex_match; using std::regex_search;
int main() {
    std::cout << std::boolalpha;
    regex pattern {"ello"};
    std::string text = "Hello world";
    auto b1 = regex_match (text.cbegin(), text.cend(), pattern); // does’t match
    std::cout << b1 << "\n"; // Output: false
    auto b2 = regex_search(text.cbegin(), text.cend(), pattern); // found
    std::cout << b2 << "\n"; // Output: true
}
```

----------------


## Listing 28.12: Accessing the match details.

Book listing [lst-0012-book.cpp](lst-0012-book.cpp):
```cpp
// https://godbolt.org/z/Yzefv59en
#include <regex>
#include <string>
#include <iostream>
using std::regex; using std::regex_search; using std::cmatch;
int main() {
    cmatch res;                               // for detailed results
    std::string text = "<h2>Result and parts of it</h2>";
    regex pattern{"<h(.)>([^<]+)"};           // Search pattern with groups
    regex_search(text.c_str(), res, pattern); // Details to res
    std::cout << res[1] << ". "               // ()-Group 1: H-Level
         << res[2] << std::endl;              // ()-Group 2: H-Text
}
```

Godbolt Listing [lst-0012-godb.cpp](lst-0012-godb.cpp), [https://godbolt.org/z/Yzefv59en](https://godbolt.org/z/Yzefv59en):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/Yzefv59en
#include <regex>
#include <string>
#include <iostream>
using std::regex; using std::regex_search; using std::cmatch;
int main() {
    cmatch res;                               // for detailed results
    std::string text = "<h2>Result and parts of it</h2>";
    regex pattern{"<h(.)>([^<]+)"};           // Search pattern with groups
    regex_search(text.c_str(), res, pattern); // Details to res
    std::cout << res[1] << ". "               // ()-Group 1: H-Level
         << res[2] << std::endl;              // ()-Group 2: H-Text
}
```

----------------


## Listing 28.13: Replace matches with new text.

Book listing [lst-0013-book.cpp](lst-0013-book.cpp):
```cpp
// https://godbolt.org/z/rErP43G89 
#include <regex>
#include <string>
#include <iostream>
using std::string;
int main() {
    string text = "Title;Album;Artist";
    std::regex pattern{";"};
    string new_str = std::regex_replace(text, pattern, string{","});
    std::cout << new_str << "\n"; // Output: Title,Album,Artist
}
```

Godbolt Listing [lst-0013-godb.cpp](lst-0013-godb.cpp), [https://godbolt.org/z/rErP43G89](https://godbolt.org/z/rErP43G89):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/rErP43G89 
#include <regex>
#include <string>
#include <iostream>
using std::string;
int main() {
    string text = "Title;Album;Artist";
    std::regex pattern{";"};
    string new_str = std::regex_replace(text, pattern, string{","});
    std::cout << new_str << "\n"; // Output: Title,Album,Artist
}
```

----------------


## Listing 28.14: Hard to maintain regular expression.

Book listing [lst-0016-book.cpp](lst-0016-book.cpp):
```cpp
// https://godbolt.org/z/dG5vE3heM
#include <regex>
#include <string>
#include <iostream>
using std::regex; using std::regex_search; using std::cmatch;
const regex pattern{R"(^score\s+(\d+)\s+for\s+(\d+)\s+nights?\s+at\s+(.*))"};
void extract(const std::string &text) {
    cmatch res;
    regex_search(text.c_str(), res, pattern);
    std::cout << res[1] << "," << res[2] << "," << res[3] << "\n";
}
int main() {
    extract("score 400 for 2 nights at Minas Tirith Airport");
    extract("score 84 for 1 night at Prancing Pony");
}
```

Godbolt Listing [lst-0016-godb.cpp](lst-0016-godb.cpp), [https://godbolt.org/z/dG5vE3heM](https://godbolt.org/z/dG5vE3heM):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/dG5vE3heM
#include <regex>
#include <string>
#include <iostream>
using std::regex; using std::regex_search; using std::cmatch;
const regex pattern{R"(^score\s+(\d+)\s+for\s+(\d+)\s+nights?\s+at\s+(.*))"};
void extract(const std::string &text) {
    cmatch res;
    regex_search(text.c_str(), res, pattern);
    std::cout << res[1] << "," << res[2] << "," << res[3] << "\n";
}
int main() {
    extract("score 400 for 2 nights at Minas Tirith Airport");
    extract("score 84 for 1 night at Prancing Pony");
}
```

----------------


## Listing 28.15: Name parts and then combine them.

Book listing [lst-0017-book.cpp](lst-0017-book.cpp):
```cpp
// https://godbolt.org/z/TK6n4dG37 
const string scoreKeyword = R"(^score\s+)";
const string numberOfPoints = R"((\d+))";
const string forKeyword = R"(\s+for\s+)";
const string numberOfNights = R"((\d+))";
const string nightsAtKeyword = R"(\s+nights?\s+at\s+)";
const string hotelName = R"((.*))";
const regex pattern{ scoreKeyword + numberOfPoints +
    forKeyword + numberOfNights + nightsAtKeyword + hotelName };
}
```

Godbolt Listing [lst-0017-godb.cpp](lst-0017-godb.cpp), [https://godbolt.org/z/TK6n4dG37](https://godbolt.org/z/TK6n4dG37):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/TK6n4dG37 
const string scoreKeyword = R"(^score\s+)";
const string numberOfPoints = R"((\d+))";
const string forKeyword = R"(\s+for\s+)";
const string numberOfNights = R"((\d+))";
const string nightsAtKeyword = R"(\s+nights?\s+at\s+)";
const string hotelName = R"((.*))";
const regex pattern{ scoreKeyword + numberOfPoints +
    forKeyword + numberOfNights + nightsAtKeyword + hotelName };
}
```

----------------


## Listing 28.16: Commenting within the expression.

Book listing [lst-0018-book.cpp](lst-0018-book.cpp):
```cpp
// https://godbolt.org/z/MszTjE3dv 
const regex pattern{R"(^score)"
    R"(\s+)"
    R"((\d+))"          // points
    R"(\s+)"
    R"(for)"
    R"(\s+)"
    R"((\d+))"          // number of nights
    R"(\s+)"
    R"(night)"
    R"(s?)"             // optional: plural
    R"(\s+)"
    R"(at)"
    R"(\s+)"
    R"((.*))"           // hotel name
    R"()"};
```

Godbolt Listing [lst-0018-godb.cpp](lst-0018-godb.cpp), [https://godbolt.org/z/MszTjE3dv](https://godbolt.org/z/MszTjE3dv):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/MszTjE3dv 
const regex pattern{R"(^score)"
    R"(\s+)"
    R"((\d+))"          // points
    R"(\s+)"
    R"(for)"
    R"(\s+)"
    R"((\d+))"          // number of nights
    R"(\s+)"
    R"(night)"
    R"(s?)"             // optional: plural
    R"(\s+)"
    R"(at)"
    R"(\s+)"
    R"((.*))"           // hotel name
    R"()"};
```

----------------


## Listing 28.17: This is how you get a uniformly distributed random number between two bounds.

Book listing [lst-0019-book.cpp](lst-0019-book.cpp):
```cpp
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
```

Godbolt Listing [lst-0019-godb.cpp](lst-0019-godb.cpp), [https://godbolt.org/z/ocP83a7ac](https://godbolt.org/z/ocP83a7ac):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
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
```

----------------


## Listing 28.18: Speeds of the random number generators.

Book listing [lst-0023-book.cpp](lst-0023-book.cpp):
```cpp
// https://godbolt.org/z/e67356bMq
#include <random>
#include <chrono>  // system_clock
#include <format>
#include <iostream>

using namespace std;
using namespace chrono;
const size_t LOOPS = 10*1000*1000;

template<typename ENGINE>
void measure(const string &title, ENGINE &engine) {
  const auto start = steady_clock::now();

  /* work hard */
  unsigned long long checksum = 0;
  size_t loops = LOOPS;
  while(loops-- > 0)
    checksum += engine();

  const auto now = steady_clock::now();
  nanoseconds dur_ns = now - start;
  cout << std::format("  {}: {:5} ns/loop  {:12} ns\n",
      title, dur_ns.count() / LOOPS, dur_ns.count());
}

int main() {
  { default_random_engine e{}; measure("       default", e ); }
  { random_device e{};         measure("        device", e ); }
  { minstd_rand0 e{};          measure("  minstd_rand0", e ); }
  { minstd_rand e{};           measure("  minstd_rand ", e ); }
  { mt19937 e{};               measure("    mt19937   ", e ); }
  { mt19937_64 e{};            measure("    mt19937_64", e ); }
  { ranlux24_base e{};         measure(" ranlux24_base", e ); }
  { ranlux48_base e{};         measure(" ranlux48_base", e ); }
  { ranlux24 e{};              measure(" ranlux24     ", e ); }
  { ranlux48 e{};              measure(" ranlux48     ", e ); }
  { knuth_b e{};               measure("       knuth_b", e ); }
  {
    using wide_t = unsigned long long ;
    independent_bits_engine<ranlux48, sizeof(wide_t)*8, wide_t> e{};
    measure("indep<ranlux> ", e );
  }
  {
    using wide_t = unsigned long long;
    independent_bits_engine<
          default_random_engine,
          sizeof(wide_t)*8, wide_t>
      e {};
    measure("indep<default>", e );
  }
}
```

Godbolt Listing [lst-0023-godb.cpp](lst-0023-godb.cpp), [https://godbolt.org/z/e67356bMq](https://godbolt.org/z/e67356bMq):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/e67356bMq
#include <random>
#include <chrono>  // system_clock
#include <format>
#include <iostream>

using namespace std;
using namespace chrono;
const size_t LOOPS = 10*1000*1000;

template<typename ENGINE>
void measure(const string &title, ENGINE &engine) {
  const auto start = steady_clock::now();

  /* work hard */
  unsigned long long checksum = 0;
  size_t loops = LOOPS;
  while(loops-- > 0)
    checksum += engine();

  const auto now = steady_clock::now();
  nanoseconds dur_ns = now - start;
  cout << std::format("  {}: {:5} ns/loop  {:12} ns\n",
      title, dur_ns.count() / LOOPS, dur_ns.count());
}

int main() {
  { default_random_engine e{}; measure("       default", e ); }
  { random_device e{};         measure("        device", e ); }
  { minstd_rand0 e{};          measure("  minstd_rand0", e ); }
  { minstd_rand e{};           measure("  minstd_rand ", e ); }
  { mt19937 e{};               measure("    mt19937   ", e ); }
  { mt19937_64 e{};            measure("    mt19937_64", e ); }
  { ranlux24_base e{};         measure(" ranlux24_base", e ); }
  { ranlux48_base e{};         measure(" ranlux48_base", e ); }
  { ranlux24 e{};              measure(" ranlux24     ", e ); }
  { ranlux48 e{};              measure(" ranlux48     ", e ); }
  { knuth_b e{};               measure("       knuth_b", e ); }
  {
    using wide_t = unsigned long long ;
    independent_bits_engine<ranlux48, sizeof(wide_t)*8, wide_t> e{};
    measure("indep<ranlux> ", e );
  }
  {
    using wide_t = unsigned long long;
    independent_bits_engine<
          default_random_engine,
          sizeof(wide_t)*8, wide_t>
      e {};
    measure("indep<default>", e );
  }
}
```

----------------


## Listing 28.19: A binomial distribution.

Book listing [lst-0024-book.cpp](lst-0024-book.cpp):
```cpp
// https://godbolt.org/z/rv5fWP5Ya
#include <random>
#include <vector>
#include <iostream>
using namespace std;
int main() {
    static const size_t size = 10;
    default_random_engine e{};              // Random number generator
    vector<size_t> counts(size+1);
    binomial_distribution<int> coins{size}; // tosses 10 coins, 0 to 10 heads
    for(auto i=120*1000; i>0; --i)
        ++counts[coins(e)];                 // Tossing coins
    for(auto c : counts) cout<<" "<<c;
    cout << '\n';
    // Example output:
    // 109 1159 5344 14043 24806 29505 24544 13973 5252 1150 115
}
```

Godbolt Listing [lst-0024-godb.cpp](lst-0024-godb.cpp), [https://godbolt.org/z/rv5fWP5Ya](https://godbolt.org/z/rv5fWP5Ya):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/rv5fWP5Ya
#include <random>
#include <vector>
#include <iostream>
using namespace std;
int main() {
    static const size_t size = 10;
    default_random_engine e{};              // Random number generator
    vector<size_t> counts(size+1);
    binomial_distribution<int> coins{size}; // tosses 10 coins, 0 to 10 heads
    for(auto i=120*1000; i>0; --i)
        ++counts[coins(e)];                 // Tossing coins
    for(auto c : counts) cout<<" "<<c;
    cout << '\n';
    // Example output:
    // 109 1159 5344 14043 24806 29505 24544 13973 5252 1150 115
}
```

----------------


## Listing 28.20: A "double" random number.

Book listing [lst-0025-book.cpp](lst-0025-book.cpp):
```cpp
// https://godbolt.org/z/94McsP7PG 
#include <random>
#include <iostream>
int main() {
    std::default_random_engine e{};
    std::uniform_real_distribution<double> unif{3,7}; // in the half-open interval [3,7)
    double u = unif(e);                         // Generate random number
    std::cout << u << '\n';                     // Example output: 3.52615
}
```

Godbolt Listing [lst-0025-godb.cpp](lst-0025-godb.cpp), [https://godbolt.org/z/94McsP7PG](https://godbolt.org/z/94McsP7PG):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/94McsP7PG 
#include <random>
#include <iostream>
int main() {
    std::default_random_engine e{};
    std::uniform_real_distribution<double> unif{3,7}; // in the half-open interval [3,7)
    double u = unif(e);                         // Generate random number
    std::cout << u << '\n';                     // Example output: 3.52615
}
```

----------------


## Listing 28.21: Change generation parameters for each random number individually.

Book listing [lst-0026-book.cpp](lst-0026-book.cpp):
```cpp
// https://godbolt.org/z/xz64qK3e5 
#include <random>
#include <iostream>
int main() {
    std::default_random_engine e{};
    using Dstr = std::uniform_int_distribution<int>;      // uniformly distributed int
    Dstr card{};                                          // generate distribution
    for(int n=32; n>=1; --n)
      std::cout <<" "<< card(e, Dstr::param_type{1,n} );  // parameters only here
    std::cout << '\n';
    // output for example:
    // 1 5 23 14 15 6 2 17 17 22 9 11 17 1 1 10 11 1 6 1 6 8 6 9 7 4 1 4 2 3 2 1
}
```

Godbolt Listing [lst-0026-godb.cpp](lst-0026-godb.cpp), [https://godbolt.org/z/xz64qK3e5](https://godbolt.org/z/xz64qK3e5):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/xz64qK3e5 
#include <random>
#include <iostream>
int main() {
    std::default_random_engine e{};
    using Dstr = std::uniform_int_distribution<int>;      // uniformly distributed int
    Dstr card{};                                          // generate distribution
    for(int n=32; n>=1; --n)
      std::cout <<" "<< card(e, Dstr::param_type{1,n} );  // parameters only here
    std::cout << '\n';
    // output for example:
    // 1 5 23 14 15 6 2 17 17 22 9 11 17 1 1 10 11 1 6 1 6 8 6 9 7 4 1 4 2 3 2 1
}
```

----------------


## GodboltId:173nr4xxd

Book listing [lst-0031-book.cpp](lst-0031-book.cpp):
```cpp
// https://godbolt.org/z/173nr4xxd 
sleep(10min);
sleep(std::chrono::minutes{10});
sleep(std::chrono::duration<unsigned long,std::ratio<60>>{10});
```

Godbolt Listing [lst-0031-godb.cpp](lst-0031-godb.cpp), [https://godbolt.org/z/173nr4xxd](https://godbolt.org/z/173nr4xxd):
```cpp
//#(execute) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/173nr4xxd 
sleep(10min);
sleep(std::chrono::minutes{10});
sleep(std::chrono::duration<unsigned long,std::ratio<60>>{10});
```

----------------


## GodboltId:5snr1hq9b

Book listing [lst-0032-book.cpp](lst-0032-book.cpp):
```cpp
// https://godbolt.org/z/5snr1hq9b 
#include <chrono>
void sleep(std::chrono::seconds s) { // takes seconds as duration
    /* ... */
}
/* ... */
int main() {
    using namespace std::chrono;  // make literals available
    sleep(10min);   // wait for 10 minutes, i.e., 600 seconds
    sleep(10ms);    // (ERR) 10 milliseconds? Cannot be represented with seconds.
}
```

Godbolt Listing [lst-0032-godb.cpp](lst-0032-godb.cpp), [https://godbolt.org/z/5snr1hq9b](https://godbolt.org/z/5snr1hq9b):
```cpp
//#(execute) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/5snr1hq9b 
#include <chrono>
void sleep(std::chrono::seconds s) { // takes seconds as duration
    /* ... */
}
/* ... */
int main() {
    using namespace std::chrono;  // make literals available
    sleep(10min);   // wait for 10 minutes, i.e., 600 seconds
    sleep(10ms);    // (ERR) 10 milliseconds? Cannot be represented with seconds.
}
```

----------------


## GodboltId:h8vsjxsME

Book listing [lst-0034-book.cpp](lst-0034-book.cpp):
```cpp
// https://godbolt.org/z/h8vsjxsME 
#include <cinttypes> // int64_t
namespace std { namespace chrono {
class seconds {
    int64_t sec_;
public:
    seconds() = default;
    // … etc …
};
} }
```

Godbolt Listing [lst-0034-godb.cpp](lst-0034-godb.cpp), [https://godbolt.org/z/h8vsjxsME](https://godbolt.org/z/h8vsjxsME):
```cpp
//#(execute) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/h8vsjxsME 
#include <cinttypes> // int64_t
namespace std { namespace chrono {
class seconds {
    int64_t sec_;
public:
    seconds() = default;
    // … etc …
};
} }
```

----------------


## Listing 28.22: With "seconds", you can perform calculations.

Book listing [lst-0039-book.cpp](lst-0039-book.cpp):
```cpp
// https://godbolt.org/z/47j8h9e7v
#include <chrono>
#include <iostream>
void sleep(std::chrono::seconds dur) {
    std::cout << dur.count() << "s\n";
    /* ... */
}
int main() {
    using namespace std::chrono;
    sleep(3);          // (ERR) Error: no implicit conversion from int
    sleep(seconds{4}); // okay
    sleep(5s);         // okay
    seconds x{6};
    sleep(x);          // okay
    auto y = 10s;
    y += 20s;          // Incrementing with seconds
    sleep(y);          // now 30s
    y = y - 6s;        // Subtraction of seconds
    sleep(y);          // and now only 24s
    y /= 2;            // Division by a scalar
    sleep(y);          // 12s
    sleep(y + 7);      // (ERR) Error: seconds+int is not allowed
}
```

Godbolt Listing [lst-0039-godb.cpp](lst-0039-godb.cpp), [https://godbolt.org/z/47j8h9e7v](https://godbolt.org/z/47j8h9e7v):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/47j8h9e7v
#include <chrono>
#include <iostream>
void sleep(std::chrono::seconds dur) {
    std::cout << dur.count() << "s\n";
    /* ... */
}
int main() {
    using namespace std::chrono;
    sleep(3);          // (ERR) Error: no implicit conversion from int
    sleep(seconds{4}); // okay
    sleep(5s);         // okay
    seconds x{6};
    sleep(x);          // okay
    auto y = 10s;
    y += 20s;          // Incrementing with seconds
    sleep(y);          // now 30s
    y = y - 6s;        // Subtraction of seconds
    sleep(y);          // and now only 24s
    y /= 2;            // Division by a scalar
    sleep(y);          // 12s
    sleep(y + 7);      // (ERR) Error: seconds+int is not allowed
}
```

----------------


## Listing 28.23: You can compare "seconds".

Book listing [lst-0040-book.cpp](lst-0040-book.cpp):
```cpp
// https://godbolt.org/z/8c1r5xrs5 
#include <chrono>
#include <iostream>
using std::chrono::operator""s;  // only make literal suffix available
constexpr auto limit = 10s;
void action(std::chrono::seconds dur) {
    if(dur <= limit) {           // compare
       std::cout << dur.count() << "s\n";
    } else {
       std::cout << "too long!\n";
    }
}
int main() {
    action(4s);   // Output: 4s
    action(20s);  // Output: too long!
}
```

Godbolt Listing [lst-0040-godb.cpp](lst-0040-godb.cpp), [https://godbolt.org/z/8c1r5xrs5](https://godbolt.org/z/8c1r5xrs5):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/8c1r5xrs5 
#include <chrono>
#include <iostream>
using std::chrono::operator""s;  // only make literal suffix available
constexpr auto limit = 10s;
void action(std::chrono::seconds dur) {
    if(dur <= limit) {           // compare
       std::cout << dur.count() << "s\n";
    } else {
       std::cout << "too long!\n";
    }
}
int main() {
    action(4s);   // Output: 4s
    action(20s);  // Output: too long!
}
```

----------------


## Listing 28.24: Automatic conversion between time units.

Book listing [lst-0041-book.cpp](lst-0041-book.cpp):
```cpp
// https://godbolt.org/z/qrs63vdz1 
#include <chrono>
#include <iostream>
int main() {
    using namespace std::chrono;             // Allow suffixes
    seconds mySecs = 121s;                   // seconds{121}
    std::cout << mySecs.count() << "s\n";    // Output: 121s
    milliseconds myMillis = mySecs;          // automatically converted
    std::cout << myMillis.count() << "ms\n"; // Output: 121000ms
    nanoseconds myNanos = mySecs;
    std::cout << myNanos.count() << "ns\n";  // Output: 121000000000ns
    minutes myMinutesErr = mySecs;           // (ERR) Error: Conversion with loss
    minutes myMinutes = duration_cast<minutes>(mySecs); // explicit works
    std::cout<<myMinutes.count()<<"min\n";   // Output: 2min
}
```

Godbolt Listing [lst-0041-godb.cpp](lst-0041-godb.cpp), [https://godbolt.org/z/qrs63vdz1](https://godbolt.org/z/qrs63vdz1):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/qrs63vdz1 
#include <chrono>
#include <iostream>
int main() {
    using namespace std::chrono;             // Allow suffixes
    seconds mySecs = 121s;                   // seconds{121}
    std::cout << mySecs.count() << "s\n";    // Output: 121s
    milliseconds myMillis = mySecs;          // automatically converted
    std::cout << myMillis.count() << "ms\n"; // Output: 121000ms
    nanoseconds myNanos = mySecs;
    std::cout << myNanos.count() << "ns\n";  // Output: 121000000000ns
    minutes myMinutesErr = mySecs;           // (ERR) Error: Conversion with loss
    minutes myMinutes = duration_cast<minutes>(mySecs); // explicit works
    std::cout<<myMinutes.count()<<"min\n";   // Output: 2min
}
```

----------------


## Listing 28.26: You can mix units.

Book listing [lst-0042-book.cpp](lst-0042-book.cpp):
```cpp
// https://godbolt.org/z/vTx4cdqzq 
#include <chrono>
#include <iostream>
void showDuration(std::chrono::milliseconds dur) {
    std::cout << dur.count() << "ms\n";
}
int main() {
    using namespace std::chrono;
    auto x = 2s;
    auto y = 3ms;
    showDuration(x + y); // Output: 2003ms
    showDuration(x - y); // Output: 1997ms
}
```

Godbolt Listing [lst-0042-godb.cpp](lst-0042-godb.cpp), [https://godbolt.org/z/vTx4cdqzq](https://godbolt.org/z/vTx4cdqzq):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/vTx4cdqzq 
#include <chrono>
#include <iostream>
void showDuration(std::chrono::milliseconds dur) {
    std::cout << dur.count() << "ms\n";
}
int main() {
    using namespace std::chrono;
    auto x = 2s;
    auto y = 3ms;
    showDuration(x + y); // Output: 2003ms
    showDuration(x - y); // Output: 1997ms
}
```

----------------


## Listing 28.27: Create new time units or more efficient representations.

Book listing [lst-0043-book.cpp](lst-0043-book.cpp):
```cpp
// https://godbolt.org/z/xMz4s8K8K
#include <chrono>
#include <iostream>
using namespace std::chrono; using std::cout;
using seconds32 = duration<int32_t>;                // other representation
using ten_day = duration<int,std::ratio<86400*10>>; // other time unit
using fseconds = duration<double>;                  // Floating point representation
int main() {
  seconds32 s{5};
  cout << milliseconds(s).count() << "ms\n";
  ten_day z{1};
  hours h = z;                  // Conversion free
  cout << "1 ten_day has "<<h.count()<<" hours\n";                // …240…
  fseconds fs{23.75};
  cout << fs.count() << "s\n"; // Floating point output
  auto printDur = [](fseconds f) { cout << f.count() << "s\n"; }; // Function
  printDur(45ms + 63us);       // Conversion to fseconds
  // Output: 0.045063s
}
```

Godbolt Listing [lst-0043-godb.cpp](lst-0043-godb.cpp), [https://godbolt.org/z/xMz4s8K8K](https://godbolt.org/z/xMz4s8K8K):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/xMz4s8K8K
#include <chrono>
#include <iostream>
using namespace std::chrono; using std::cout;
using seconds32 = duration<int32_t>;                // other representation
using ten_day = duration<int,std::ratio<86400*10>>; // other time unit
using fseconds = duration<double>;                  // Floating point representation
int main() {
  seconds32 s{5};
  cout << milliseconds(s).count() << "ms\n";
  ten_day z{1};
  hours h = z;                  // Conversion free
  cout << "1 ten_day has "<<h.count()<<" hours\n";                // …240…
  fseconds fs{23.75};
  cout << fs.count() << "s\n"; // Floating point output
  auto printDur = [](fseconds f) { cout << f.count() << "s\n"; }; // Function
  printDur(45ms + 63us);       // Conversion to fseconds
  // Output: 0.045063s
}
```

----------------


## GodboltId:zMqjqMeav

Book listing [lst-0044-book.cpp](lst-0044-book.cpp):
```cpp
// https://godbolt.org/z/zMqjqMeav
#include <chrono>
#include <iostream>
using namespace std::chrono;
int main() {
    year y{2021};
    std::cout << y << "\n";
    month m{October};
    auto result = m + months{3};   // 'months', not 'month': 3 months later
    std::cout << result << "\n";   // overflowed to January
    weekday wd{Thursday};
    auto result = wd + days{4};    // 'days', not 'day': 4 days later
    std::cout << result << "\n";   // overflowed to Monday
    weekday sun1{0};               // 0 is Sunday
    weekday sun2{7};               // 7 is also Sunday
    std::cout << sun1 << "\t" << sun2 << "\n";
    weekday_indexed wdi{wd, 4};    // unspecified 4th Thursday
    std::cout << wdi << "\n";      // Output: Thu[4] — as given by chrono 
                                   // unspecified values
}
```

Godbolt Listing [lst-0044-godb.cpp](lst-0044-godb.cpp), [https://godbolt.org/z/zMqjqMeav](https://godbolt.org/z/zMqjqMeav):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/zMqjqMeav
#include <chrono>
#include <iostream>
using namespace std::chrono;
int main() {
    year y{2021};
    std::cout << y << "\n";
    month m{October};
    auto result = m + months{3};   // 'months', not 'month': 3 months later
    std::cout << result << "\n";   // overflowed to January
    weekday wd{Thursday};
    auto result = wd + days{4};    // 'days', not 'day': 4 days later
    std::cout << result << "\n";   // overflowed to Monday
    weekday sun1{0};               // 0 is Sunday
    weekday sun2{7};               // 7 is also Sunday
    std::cout << sun1 << "\t" << sun2 << "\n";
    weekday_indexed wdi{wd, 4};    // unspecified 4th Thursday
    std::cout << wdi << "\n";      // Output: Thu[4] — as given by chrono 
                                   // unspecified values
}
```

----------------


## GodboltId:6z48xvvvM

Book listing [lst-0045-book.cpp](lst-0045-book.cpp):
```cpp
// https://godbolt.org/z/6z48xvvvM
#include <chrono>
#include <iostream>
using namespace std::chrono;
int main() {
  year this_year{2021};
  year last_year{2020};
  year_month_day ymd{this_year, October, day{28}};
  std::cout << ymd << "\n";
  month_weekday mwd{November, Thursday[4]};  // in an unspecified year
  std::cout << mwd << "\n";                  // Output: Nov/Thu[4]
  month_day_last mdlast{February};      // last day of an unspecified February
  year_month_day_last leap{last_year, mdlast};         // Add year
  year_month_day_last noleap{this_year, mdlast};       // Add year
  std::cout << leap << "\t" << leap.day() << "\n";     // Output: 2020-02-29 29
  std::cout << noleap << "\t" << noleap.day() << "\n"; // Output: 2021-02-28 28
}
```

Godbolt Listing [lst-0045-godb.cpp](lst-0045-godb.cpp), [https://godbolt.org/z/6z48xvvvM](https://godbolt.org/z/6z48xvvvM):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/6z48xvvvM
#include <chrono>
#include <iostream>
using namespace std::chrono;
int main() {
  year this_year{2021};
  year last_year{2020};
  year_month_day ymd{this_year, October, day{28}};
  std::cout << ymd << "\n";
  month_weekday mwd{November, Thursday[4]};  // in an unspecified year
  std::cout << mwd << "\n";                  // Output: Nov/Thu[4]
  month_day_last mdlast{February};      // last day of an unspecified February
  year_month_day_last leap{last_year, mdlast};         // Add year
  year_month_day_last noleap{this_year, mdlast};       // Add year
  std::cout << leap << "\t" << leap.day() << "\n";     // Output: 2020-02-29 29
  std::cout << noleap << "\t" << noleap.day() << "\n"; // Output: 2021-02-28 28
}
```

----------------


## GodboltId:4s69W7qd3

Book listing [lst-0047-book.cpp](lst-0047-book.cpp):
```cpp
// https://godbolt.org/z/4s69W7qd3 
auto a_date = 2024y/11/14d;
auto b_date = 2024y/11/14;
auto c_date = 14d/11/2024;
auto d_date = November/14/2024;
```

Godbolt Listing [lst-0047-godb.cpp](lst-0047-godb.cpp), [https://godbolt.org/z/4s69W7qd3](https://godbolt.org/z/4s69W7qd3):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/4s69W7qd3 
auto a_date = 2024y/11/14d;
auto b_date = 2024y/11/14;
auto c_date = 14d/11/2024;
auto d_date = November/14/2024;
```

----------------


## Listing 28.28: Output of date and time with format

Book listing [lst-0049-book.cpp](lst-0049-book.cpp):
```cpp
// https://godbolt.org/z/Md38b9zvf 
#include <chrono>
#include <iostream>
#include <format>
using namespace std::chrono;
int main() {
  auto ymd = last/February/2024;   // last day in February 2024: 2024-02-29
  std::cout << ymd << "\n";        // the output with << is simple
  std::cout << std::format("{:%Y-%m-%d}\n", ymd);  // format is more flexible
  std::cout << std::format("{:%e. %B %y}\n", ymd); // much more flexible!
}
```

Godbolt Listing [lst-0049-godb.cpp](lst-0049-godb.cpp), [https://godbolt.org/z/Md38b9zvf](https://godbolt.org/z/Md38b9zvf):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/Md38b9zvf 
#include <chrono>
#include <iostream>
#include <format>
using namespace std::chrono;
int main() {
  auto ymd = last/February/2024;   // last day in February 2024: 2024-02-29
  std::cout << ymd << "\n";        // the output with << is simple
  std::cout << std::format("{:%Y-%m-%d}\n", ymd);  // format is more flexible
  std::cout << std::format("{:%e. %B %y}\n", ymd); // much more flexible!
}
```

----------------


## Listing 28.29: Always the second Tuesday of a month.

Book listing [lst-0050-book.cpp](lst-0050-book.cpp):
```cpp
// https://godbolt.org/z/19vf9jfTM 
#include <chrono>
#include <iostream>
using namespace std::chrono;
int main() {
    for (int mo = 1; mo <= 12; ++mo) {
        year_month_weekday patch_tues{mo/Tuesday[2]/2024y};
        year_month_day ymd{sys_days{patch_tues}};
        std::cout << ymd.month() << " " << ymd.day() << "\n";
    }
}
```

Godbolt Listing [lst-0050-godb.cpp](lst-0050-godb.cpp), [https://godbolt.org/z/19vf9jfTM](https://godbolt.org/z/19vf9jfTM):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/19vf9jfTM 
#include <chrono>
#include <iostream>
using namespace std::chrono;
int main() {
    for (int mo = 1; mo <= 12; ++mo) {
        year_month_weekday patch_tues{mo/Tuesday[2]/2024y};
        year_month_day ymd{sys_days{patch_tues}};
        std::cout << ymd.month() << " " << ymd.day() << "\n";
    }
}
```

----------------


## Listing 28.30: Conversion to a time with time zone and the exceptions

Book listing [lst-0052-book.cpp](lst-0052-book.cpp):
```cpp
// https://godbolt.org/z/ezn568nof
#include <chrono>
#include <iostream>
using namespace std::chrono;
int main() {
  auto spring = local_days{31d/March/2024} + 2h + 1min;
  try {
      auto zt = zoned_time{"Europe/Berlin", spring};
  } catch (const nonexistent_local_time& e) {
      std::cout << e.what() << '\n'; // Exception thrown: does not exist
  }
  auto autumn = local_days{27d/October/2024} + 2h + 1min;
  try {
      auto zt = zoned_time{"Europe/Berlin", autumn};
  } catch (const ambiguous_local_time& e) {
     std::cout << e.what() << '\n'; // Exception thrown: already exists
  }
  std::cout << zoned_time{"Europe/Berlin", autumn, choose::earliest} << '\n';
  // Output: 2024-10-27 02:01:00 CEST
  std::cout << zoned_time{"Europe/Berlin", autumn, choose::latest} << '\n';
  // Output: 2024-10-27 02:01:00 CET
}
```

Godbolt Listing [lst-0052-godb.cpp](lst-0052-godb.cpp), [https://godbolt.org/z/ezn568nof](https://godbolt.org/z/ezn568nof):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/ezn568nof
#include <chrono>
#include <iostream>
using namespace std::chrono;
int main() {
  auto spring = local_days{31d/March/2024} + 2h + 1min;
  try {
      auto zt = zoned_time{"Europe/Berlin", spring};
  } catch (const nonexistent_local_time& e) {
      std::cout << e.what() << '\n'; // Exception thrown: does not exist
  }
  auto autumn = local_days{27d/October/2024} + 2h + 1min;
  try {
      auto zt = zoned_time{"Europe/Berlin", autumn};
  } catch (const ambiguous_local_time& e) {
     std::cout << e.what() << '\n'; // Exception thrown: already exists
  }
  std::cout << zoned_time{"Europe/Berlin", autumn, choose::earliest} << '\n';
  // Output: 2024-10-27 02:01:00 CEST
  std::cout << zoned_time{"Europe/Berlin", autumn, choose::latest} << '\n';
  // Output: 2024-10-27 02:01:00 CET
}
```

----------------


## Listing 28.31: The time zone database.

Book listing [lst-0053-book.cpp](lst-0053-book.cpp):
```cpp
// https://godbolt.org/z/YTKK3vT7s 
#include <chrono>
#include <iostream>
#include <ranges> // views::transform, views::filter
#include <algorithm> // ranges::for_each
using namespace std; namespace c = std::chrono;
namespace v = std::views; namespace r = std::ranges;
int main() {
  auto show_name = [](const string_view name) { cout << name << ' '; };
  const auto& db = c::get_tzdb();
  auto names = db.zones
   | v::transform([](const c::time_zone& z) {return z.name();})
   | v::filter([](const string_view name) {
       return name.starts_with("Europe/Be");});
  r::for_each(names, show_name);
  cout << " <- Europe/Be*\n"; // Output: Europe/Belgrade Europe/Berlin
  r::for_each(
   db.links
   | v::filter([](const c::time_zone_link& l){
       return l.target()=="Europe/Berlin";})
   | v::transform([](const c::time_zone_link& l)->string_view {
       return l.name();})
   , show_name);
  cout << " <- Links to Europe/Berlin\n"; // Output: Arctic/Longyearbyen …
}
```

Godbolt Listing [lst-0053-godb.cpp](lst-0053-godb.cpp), [https://godbolt.org/z/YTKK3vT7s](https://godbolt.org/z/YTKK3vT7s):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/YTKK3vT7s 
#include <chrono>
#include <iostream>
#include <ranges> // views::transform, views::filter
#include <algorithm> // ranges::for_each
using namespace std; namespace c = std::chrono;
namespace v = std::views; namespace r = std::ranges;
int main() {
  auto show_name = [](const string_view name) { cout << name << ' '; };
  const auto& db = c::get_tzdb();
  auto names = db.zones
   | v::transform([](const c::time_zone& z) {return z.name();})
   | v::filter([](const string_view name) {
       return name.starts_with("Europe/Be");});
  r::for_each(names, show_name);
  cout << " <- Europe/Be*\n"; // Output: Europe/Belgrade Europe/Berlin
  r::for_each(
   db.links
   | v::filter([](const c::time_zone_link& l){
       return l.target()=="Europe/Berlin";})
   | v::transform([](const c::time_zone_link& l)->string_view {
       return l.name();})
   , show_name);
  cout << " <- Links to Europe/Berlin\n"; // Output: Arctic/Longyearbyen …
}
```

----------------


## GodboltId:ThM8q1zEP

Book listing [lst-0054-book.cpp](lst-0054-book.cpp):
```cpp
// https://godbolt.org/z/ThM8q1zEP
#include <iostream>
#include <ratio>
using std::cout; using std::endl;
int main() {
  using oneThird = std::ratio<1,3>;
  using twoFourths = std::ratio<2,4>;
  cout << oneThird::num << "/" << oneThird::den << endl;     // Output: 1/3
  cout << twoFourths::num << "/" << twoFourths::den << endl; // Output: 1/2
  using sum = std::ratio_add<oneThird,twoFourth>;            // add
  cout << sum::num << "/" << sum::den;                       // Output: 5/6
}
```

Godbolt Listing [lst-0054-godb.cpp](lst-0054-godb.cpp), [https://godbolt.org/z/ThM8q1zEP](https://godbolt.org/z/ThM8q1zEP):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/ThM8q1zEP
#include <iostream>
#include <ratio>
using std::cout; using std::endl;
int main() {
  using oneThird = std::ratio<1,3>;
  using twoFourths = std::ratio<2,4>;
  cout << oneThird::num << "/" << oneThird::den << endl;     // Output: 1/3
  cout << twoFourths::num << "/" << twoFourths::den << endl; // Output: 1/2
  using sum = std::ratio_add<oneThird,twoFourth>;            // add
  cout << sum::num << "/" << sum::den;                       // Output: 5/6
}
```

----------------


## GodboltId:q1znGWb85

Book listing [lst-0059-book.cpp](lst-0059-book.cpp):
```cpp
// https://godbolt.org/z/q1znGWb85 
#include <iostream>
#include <chrono>
int main() {
    using namespace std::chrono;
    time_point<system_clock, seconds> t1{10000s};
    time_point<system_clock, seconds> t2{50000s};
    auto dur = t2 - t1;
    std::cout << duration_cast<hours>(dur).count() << "h"; // Output: 11h
}
```

Godbolt Listing [lst-0059-godb.cpp](lst-0059-godb.cpp), [https://godbolt.org/z/q1znGWb85](https://godbolt.org/z/q1znGWb85):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/q1znGWb85 
#include <iostream>
#include <chrono>
int main() {
    using namespace std::chrono;
    time_point<system_clock, seconds> t1{10000s};
    time_point<system_clock, seconds> t2{50000s};
    auto dur = t2 - t1;
    std::cout << duration_cast<hours>(dur).count() << "h"; // Output: 11h
}
```

----------------


## Listing 28.32: Simple timing of a function call.

Book listing [lst-0063-book.cpp](lst-0063-book.cpp):
```cpp
// https://godbolt.org/z/3MGbYn4d5
#include <iostream>
#include <chrono>
long fib(long n) { return n<2L ? 1L : fib(n-1L)+fib(n-2L); }
int main() {
    using namespace std::chrono;
    auto t0 = steady_clock::now();  // On your marks, get set …
    auto res = fib(17);             // … go!
    auto t1 = steady_clock::now();  // Stop!
    std::cout << "Result: " << res << "\n"; // Output: Result: 2584
    std::cout << "Time: " << nanoseconds{t1-t0}.count() << "ns\n";
    // Output: Time: 50727ns (e.g.)
}
```

Godbolt Listing [lst-0063-godb.cpp](lst-0063-godb.cpp), [https://godbolt.org/z/3MGbYn4d5](https://godbolt.org/z/3MGbYn4d5):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/3MGbYn4d5
#include <iostream>
#include <chrono>
long fib(long n) { return n<2L ? 1L : fib(n-1L)+fib(n-2L); }
int main() {
    using namespace std::chrono;
    auto t0 = steady_clock::now();  // On your marks, get set …
    auto res = fib(17);             // … go!
    auto t1 = steady_clock::now();  // Stop!
    std::cout << "Result: " << res << "\n"; // Output: Result: 2584
    std::cout << "Time: " << nanoseconds{t1-t0}.count() << "ns\n";
    // Output: Time: 50727ns (e.g.)
}
```

----------------


## GodboltId:rz38hc1cx

Book listing [lst-0064-book.cpp](lst-0064-book.cpp):
```cpp
// https://godbolt.org/z/rz38hc1cx 
auto res = fib(45);
// …
std::cout << "Time: " << duration_cast<seconds>(t1-t0).count() << "s\n";
// Output: Time: 7s (e.g.)
std::cout << "Time: " << duration<double>{t1-t0}.count() << "s\n";
// Output: Time: 7.35303s (e.g.)
}
```

Godbolt Listing [lst-0064-godb.cpp](lst-0064-godb.cpp), [https://godbolt.org/z/rz38hc1cx](https://godbolt.org/z/rz38hc1cx):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/rz38hc1cx 
auto res = fib(45);
// …
std::cout << "Time: " << duration_cast<seconds>(t1-t0).count() << "s\n";
// Output: Time: 7s (e.g.)
std::cout << "Time: " << duration<double>{t1-t0}.count() << "s\n";
// Output: Time: 7.35303s (e.g.)
}
```

----------------


## GodboltId:1xbhhTdEv

Book listing [lst-0065-book.cpp](lst-0065-book.cpp):
```cpp
// https://godbolt.org/z/1xbhhTdEv 
#include <iostream>
#include <chrono>
int main() {
    using namespace std::chrono;
    auto tp = time_point_cast<seconds>(system_clock::now());
    auto d = tp.time_since_epoch();
    std::cout << d.count() << "s\n";
    std::cout << duration<double,std::ratio<86400>>{d}.count() << "days\n";
}
```

Godbolt Listing [lst-0065-godb.cpp](lst-0065-godb.cpp), [https://godbolt.org/z/1xbhhTdEv](https://godbolt.org/z/1xbhhTdEv):
```cpp
//#(execute) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/1xbhhTdEv 
#include <iostream>
#include <chrono>
int main() {
    using namespace std::chrono;
    auto tp = time_point_cast<seconds>(system_clock::now());
    auto d = tp.time_since_epoch();
    std::cout << d.count() << "s\n";
    std::cout << duration<double,std::ratio<86400>>{d}.count() << "days\n";
}
```

----------------


## GodboltId:o359TThMj

Book listing [lst-0066-book.cpp](lst-0066-book.cpp):
```cpp
// https://godbolt.org/z/o359TThMj 
#include <iostream>
#include <string>
#include <chrono>
#include <complex>
using std::cout;
int main() {
    { using namespace std;
      cout << "string"s << "\n";           // string
      cout << (1.2+3.4i) << "\n";          // complex
      }
    { using namespace std::chrono;
      cout << (35ms).count() << "ms\n";    // chrono
      }
    { using namespace std::literals;
      cout << (41s).count() << "ms\n";     // chrono seconds
      cout << "text"s << "\n";             // string
    }
    { using namespace std::chrono; 
      cout << (4h).count() << "h\n";       // chrono hours
    }
    { using namespace std::literals::chrono_literals; 
      cout << (16min).count() << "min\n";  // chrono minutes
    }
    { using std::literals::string_literals::operator""s; 
      cout << "letters"s << "\n";          // string
    }
}
```

Godbolt Listing [lst-0066-godb.cpp](lst-0066-godb.cpp), [https://godbolt.org/z/o359TThMj](https://godbolt.org/z/o359TThMj):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/o359TThMj 
#include <iostream>
#include <string>
#include <chrono>
#include <complex>
using std::cout;
int main() {
    { using namespace std;
      cout << "string"s << "\n";           // string
      cout << (1.2+3.4i) << "\n";          // complex
      }
    { using namespace std::chrono;
      cout << (35ms).count() << "ms\n";    // chrono
      }
    { using namespace std::literals;
      cout << (41s).count() << "ms\n";     // chrono seconds
      cout << "text"s << "\n";             // string
    }
    { using namespace std::chrono; 
      cout << (4h).count() << "h\n";       // chrono hours
    }
    { using namespace std::literals::chrono_literals; 
      cout << (16min).count() << "min\n";  // chrono minutes
    }
    { using std::literals::string_literals::operator""s; 
      cout << "letters"s << "\n";          // string
    }
}
```

----------------


## Listing 28.34: Very simple example of how to check the success of an operation.

Book listing [lst-0067-book.cpp](lst-0067-book.cpp):
```cpp
// https://godbolt.org/z/1xnY8vd8Y 
#include <system_error> // error_code
#include <string>
void create_dir(const std::string& pathname, std::error_code& ec);
void run() {
  std::error_code ec;
  create_dir("/some/path", ec);
  if(!ec) {  // Success …
  } else {   // Failure …
  }
}
```

Godbolt Listing [lst-0067-godb.cpp](lst-0067-godb.cpp), [https://godbolt.org/z/1xnY8vd8Y](https://godbolt.org/z/1xnY8vd8Y):
```cpp
//#(execute) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/1xnY8vd8Y 
#include <system_error> // error_code
#include <string>
void create_dir(const std::string& pathname, std::error_code& ec);
void run() {
  std::error_code ec;
  create_dir("/some/path", ec);
  if(!ec) {  // Success …
  } else {   // Failure …
  }
}
```

----------------


## Listing 28.35: Comparison of "error_code" with "error_condition".

Book listing [lst-0068-book.cpp](lst-0068-book.cpp):
```cpp
// https://godbolt.org/z/TPWc5nexq 
#include <system_error> // error_code, errc
#include <string>
void create_dir(const std::string& pathname, std::error_code& ec);
void run() {
  std::error_code ec;
  create_dir("/some/path", ec);
  if(ec == std::errc::file_exists) {   // specifically …
  } else if(!ec) {                     // Success …
  } else {                             // Failure …
  }
}
```

Godbolt Listing [lst-0068-godb.cpp](lst-0068-godb.cpp), [https://godbolt.org/z/TPWc5nexq](https://godbolt.org/z/TPWc5nexq):
```cpp
//#(execute) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/TPWc5nexq 
#include <system_error> // error_code, errc
#include <string>
void create_dir(const std::string& pathname, std::error_code& ec);
void run() {
  std::error_code ec;
  create_dir("/some/path", ec);
  if(ec == std::errc::file_exists) {   // specifically …
  } else if(!ec) {                     // Success …
  } else {                             // Failure …
  }
}
```

----------------


## Listing 28.36: Overloads of "operator==()" (abbreviated).

Book listing [lst-0069-book.cpp](lst-0069-book.cpp):
```cpp
bool operator==
 (const error_code&      lhs, const error_code&      rhs) noexcept;
…(const error_code&      lhs, const error_condition& rhs) noexcept;
…(const error_condition& lhs, const error_code&      rhs) noexcept;
…(const error_condition& lhs, const error_condition& rhs) noexcept;
```

----------------


## Listing 28.37: Definition of "enum class errc".

Book listing [lst-0070-book.cpp](lst-0070-book.cpp):
```cpp
enum class errc {
  address_family_not_supported,
  address_in_use,
  // ...
  value_too_large,
  wrong_protocol_type,
};
```

----------------


## Listing 28.38: Specialization of "is_error_condition_enum" for "errc".

Book listing [lst-0071-book.cpp](lst-0071-book.cpp):
```cpp
template <>
struct is_error_condition_enum<errc> : true_type {};
```

----------------


## Listing 28.39: Among the overloads of "make_error_condition" is also one with "errc".

Book listing [lst-0072-book.cpp](lst-0072-book.cpp):
```cpp
error_condition make_error_condition(errc c) noexcept {
  return error_condition(
      static_cast<int>(e),
      generic_category());
}
error_condition make_error_condition(io_errc c) noexcept /*...*/
error_condition make_error_condition(future_errc c) noexcept /*...*/
```

----------------


## Listing 28.40: Generating system-specific error codes in a portable program.

Book listing [lst-0073-book.cpp](lst-0073-book.cpp):
```cpp
// https://godbolt.org/z/n4v78WWod 
#include <system_error>
#include <string>

void create_dir(const std::string& pathname, std::error_code& ec) {
#if defined(_WIN32)
  // Windows implementation, with Windows error codes
#elif defined(linux)
  // Linux implementation, with Linux error codes
#else
  // general 'generic' case
  ec = std::make_error_code(std::errc::not_supported);
#endif
}
```

Godbolt Listing [lst-0073-godb.cpp](lst-0073-godb.cpp), [https://godbolt.org/z/n4v78WWod](https://godbolt.org/z/n4v78WWod):
```cpp
//#(execute) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/n4v78WWod 
#include <system_error>
#include <string>

void create_dir(const std::string& pathname, std::error_code& ec) {
#if defined(_WIN32)
  // Windows implementation, with Windows error codes
#elif defined(linux)
  // Linux implementation, with Linux error codes
#else
  // general 'generic' case
  ec = std::make_error_code(std::errc::not_supported);
#endif
}
```

----------------


## GodboltId:11Tdr6aG1

Book listing [lst-0074-book.cpp](lst-0074-book.cpp):
```cpp
// https://godbolt.org/z/11Tdr6aG1
#include <system_error>
#include <iostream>
using std::error_code; using std::system_category;
namespace mylib {
    // custom error codes
    enum class errc { LOAD_ERR = 1, UNLOAD_ERR = 2, OTHER_ERR = 3 };
    error_code make_error_code(errc ec) {
        switch(ec) {
        case errc::LOAD_ERR: return error_code((int)ec, system_category());
        case errc::UNLOAD_ERR: return error_code((int)ec, system_category());
        case errc::OTHER_ERR: return error_code((int)ec, system_category());
        }
    }
    error_code run(int arg) {
        if(arg == 667) {
            return make_error_code(errc::OTHER_ERR);
        }
        return error_code{}; // all good.
    }
}
int main() {
    std::error_code ec = mylib::run(667);
    if(!ec) {
        std::cout << "Great, it works!\n";
    } else if (ec == mylib::make_error_code(mylib::errc::OTHER_ERR)) {
        std::cout << "Another error\n";
    } else {
        std::cout << "Nothing happening here\n" << ec;
    }
}
```

Godbolt Listing [lst-0074-godb.cpp](lst-0074-godb.cpp), [https://godbolt.org/z/11Tdr6aG1](https://godbolt.org/z/11Tdr6aG1):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23 -Wno-return-type; libs:-
// https://godbolt.org/z/11Tdr6aG1
#include <system_error>
#include <iostream>
using std::error_code; using std::system_category;
namespace mylib {
    // custom error codes
    enum class errc { LOAD_ERR = 1, UNLOAD_ERR = 2, OTHER_ERR = 3 };
    error_code make_error_code(errc ec) {
        switch(ec) {
        case errc::LOAD_ERR: return error_code((int)ec, system_category());
        case errc::UNLOAD_ERR: return error_code((int)ec, system_category());
        case errc::OTHER_ERR: return error_code((int)ec, system_category());
        }
    }
    error_code run(int arg) {
        if(arg == 667) {
            return make_error_code(errc::OTHER_ERR);
        }
        return error_code{}; // all good.
    }
}
int main() {
    std::error_code ec = mylib::run(667);
    if(!ec) {
        std::cout << "Great, it works!\n";
    } else if (ec == mylib::make_error_code(mylib::errc::OTHER_ERR)) {
        std::cout << "Another error\n";
    } else {
        std::cout << "Nothing happening here\n" << ec;
    }
}
```

----------------


## GodboltId:a8zP84j1x

Book listing [lst-0076-book.cpp](lst-0076-book.cpp):
```cpp
// https://godbolt.org/z/a8zP84j1x 
#include <thread>
#include <iostream>
#include <system_error>
int main() {
    try {
        std::thread().detach(); // this will fail
    } catch(std::system_error& e) {
        std::cout
            << "system_error with code:" << e.code()
            << " message:" << e.what()
            << '\n';
    }
}
```

Godbolt Listing [lst-0076-godb.cpp](lst-0076-godb.cpp), [https://godbolt.org/z/a8zP84j1x](https://godbolt.org/z/a8zP84j1x):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/a8zP84j1x 
#include <thread>
#include <iostream>
#include <system_error>
int main() {
    try {
        std::thread().detach(); // this will fail
    } catch(std::system_error& e) {
        std::cout
            << "system_error with code:" << e.code()
            << " message:" << e.what()
            << '\n';
    }
}
```

----------------


## GodboltId:cjh93oPcr

Book listing [lst-0077-book.cpp](lst-0077-book.cpp):
```cpp
// https://godbolt.org/z/cjh93oPcr 
#include <iostream>
#include <system_error> // std::make_error_condition, std::ios_errc
int main () {
  // switch to exceptions:
  std::cin.exceptions (std::ios::failbit|std::ios::badbit);
  try {
    std::cin.rdbuf(nullptr);        // triggers an exception
  } catch (std::ios::failure& e) {  // derived from system_error
    std::cerr << "Error: ";
    if (e.code() == std::make_error_condition(std::io_errc::stream)) {
      std::cerr << "stream\n";
    } else {
      std::cerr << "other\n";
    }
  }
}
```

Godbolt Listing [lst-0077-godb.cpp](lst-0077-godb.cpp), [https://godbolt.org/z/cjh93oPcr](https://godbolt.org/z/cjh93oPcr):
```cpp
//#(execute) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/cjh93oPcr 
#include <iostream>
#include <system_error> // std::make_error_condition, std::ios_errc
int main () {
  // switch to exceptions:
  std::cin.exceptions (std::ios::failbit|std::ios::badbit);
  try {
    std::cin.rdbuf(nullptr);        // triggers an exception
  } catch (std::ios::failure& e) {  // derived from system_error
    std::cerr << "Error: ";
    if (e.code() == std::make_error_condition(std::io_errc::stream)) {
      std::cerr << "stream\n";
    } else {
      std::cerr << "other\n";
    }
  }
}
```

----------------


## Listing 28.41: Use "typeindex" for reliable type information.

Book listing [lst-0078-book.cpp](lst-0078-book.cpp):
```cpp
// https://godbolt.org/z/d1G9ez4KW
#include <iostream>
#include <typeinfo>
#include <typeindex>
#include <map>
#include <string>
struct Base {
    virtual ~Base() {}
};
struct Derived_One : public Base {};
struct Derived_Two : public Base {};
int main() {
  using std::string; using std::cout; using std::type_index;
  std::map<std::type_index, string> names {
      { type_index(typeid(int)), "int" },
      { type_index(typeid(double)), "double" },
      { type_index(typeid(Base)), "Base" },
      { type_index(typeid(Derived_One)), "Derived_One" },
      { type_index(typeid(Derived_Two)), "Derived_Two" },
      { type_index(typeid(string)), "string" },
      { type_index(typeid(string::const_iterator)), "string" },
  };
  names[type_index(typeid(names))] = "names-map";
  int integer;
  double floating;
  Base base{};
  Base *one = new Derived_One{};
  Base *two = new Derived_Two{};
  // typeid.name() is implementation- and runtime-dependent:
  cout << typeid(integer).name() << '\n';   // On my system: i
  cout << typeid(floating).name() << '\n';  // On my system: d
  cout << typeid(base).name() << '\n';      // On my system: 4Base
  cout << typeid(*one).name() << '\n';      // On my system: 11Derived_One
  cout << typeid(*two).name() << '\n';      // On my system: 11Derived_Two
  cout << typeid(string).name() << '\n';    // For me: Ss
  cout << typeid(string{"World"}.begin()).name() << '\n';
      // For me: N9__gnu_cxx17__normal_iteratorIPcSsEE
  cout << typeid(names).name() << '\n';
      // For me: St3mapISt10type_indexSsSt4lessIS0_ESaISt4pairIKS0_SsEEE
  cout << typeid(666/0).name() << '\n'; // Expression not executed! For me: i
  // type_index makes type_infos comparable:
  cout << names[type_index(typeid(integer))] << '\n';  // Output: int
  cout << names[type_index(typeid(floating))] << '\n'; // Output: double
  cout << names[type_index(typeid(base))] << '\n';     // Output: Base
  cout << names[type_index(typeid(*one))] << '\n';     // Output: Derived_One
  cout << names[type_index(typeid(*two))] << '\n';     // Output: Derived_Two
  cout << names[type_index(typeid(string))] << '\n';   // Output: string
  cout << names[type_index(typeid(names))] << '\n';    // Output: names-map
}
```

Godbolt Listing [lst-0078-godb.cpp](lst-0078-godb.cpp), [https://godbolt.org/z/d1G9ez4KW](https://godbolt.org/z/d1G9ez4KW):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23 -Wno-return-type; libs:-
// https://godbolt.org/z/d1G9ez4KW
#include <iostream>
#include <typeinfo>
#include <typeindex>
#include <map>
#include <string>
struct Base {
    virtual ~Base() {}
};
struct Derived_One : public Base {};
struct Derived_Two : public Base {};
int main() {
  using std::string; using std::cout; using std::type_index;
  std::map<std::type_index, string> names {
      { type_index(typeid(int)), "int" },
      { type_index(typeid(double)), "double" },
      { type_index(typeid(Base)), "Base" },
      { type_index(typeid(Derived_One)), "Derived_One" },
      { type_index(typeid(Derived_Two)), "Derived_Two" },
      { type_index(typeid(string)), "string" },
      { type_index(typeid(string::const_iterator)), "string" },
  };
  names[type_index(typeid(names))] = "names-map";
  int integer;
  double floating;
  Base base{};
  Base *one = new Derived_One{};
  Base *two = new Derived_Two{};
  // typeid.name() is implementation- and runtime-dependent:
  cout << typeid(integer).name() << '\n';   // On my system: i
  cout << typeid(floating).name() << '\n';  // On my system: d
  cout << typeid(base).name() << '\n';      // On my system: 4Base
  cout << typeid(*one).name() << '\n';      // On my system: 11Derived_One
  cout << typeid(*two).name() << '\n';      // On my system: 11Derived_Two
  cout << typeid(string).name() << '\n';    // For me: Ss
  cout << typeid(string{"World"}.begin()).name() << '\n';
      // For me: N9__gnu_cxx17__normal_iteratorIPcSsEE
  cout << typeid(names).name() << '\n';
      // For me: St3mapISt10type_indexSsSt4lessIS0_ESaISt4pairIKS0_SsEEE
  cout << typeid(666/0).name() << '\n'; // Expression not executed! For me: i
  // type_index makes type_infos comparable:
  cout << names[type_index(typeid(integer))] << '\n';  // Output: int
  cout << names[type_index(typeid(floating))] << '\n'; // Output: double
  cout << names[type_index(typeid(base))] << '\n';     // Output: Base
  cout << names[type_index(typeid(*one))] << '\n';     // Output: Derived_One
  cout << names[type_index(typeid(*two))] << '\n';     // Output: Derived_Two
  cout << names[type_index(typeid(string))] << '\n';   // Output: string
  cout << names[type_index(typeid(names))] << '\n';    // Output: names-map
}
```

----------------


## Listing 28.42: Boost's "demangled_name" is extremely useful for outputting type names.

Book listing [lst-0079-book.cpp](lst-0079-book.cpp):
```cpp
// https://godbolt.org/z/n54566cqT 
#include <iostream>
#include <typeinfo>
#include <string>
#include <map>
#include <boost/core/typeinfo.hpp>
int main() {
  using std::string; using std::cout;
  std::map<int, string> names;
  int i;
  double f;
  // demangled_name
  using boost::core::demangled_name;
  cout<<demangled_name(BOOST_CORE_TYPEID(i))<<'\n';      // Output: int
  cout<<demangled_name(BOOST_CORE_TYPEID(f))<<'\n';      // Output: double
  cout<<demangled_name(BOOST_CORE_TYPEID(string))<<'\n'; // Output: std::string
  cout<<demangled_name(BOOST_CORE_TYPEID(string{}.begin()))<<'\n';
  // Output: __gnu_cxx::__normal_iterator<char*, std::string>
  cout<<demangled_name(BOOST_CORE_TYPEID(namen))<<'\n';
  // Output: std::map<int, std::string, std::less<int>,
  //                                                         std::allocator<std::pair<int const, std::string> > >
  cout<<demangled_name(BOOST_CORE_TYPEID(666/0))<<'\n';  // Output: int
}
```

Godbolt Listing [lst-0079-godb.cpp](lst-0079-godb.cpp), [https://godbolt.org/z/n54566cqT](https://godbolt.org/z/n54566cqT):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/n54566cqT 
#include <iostream>
#include <typeinfo>
#include <string>
#include <map>
#include <boost/core/typeinfo.hpp>
int main() {
  using std::string; using std::cout;
  std::map<int, string> names;
  int i;
  double f;
  // demangled_name
  using boost::core::demangled_name;
  cout<<demangled_name(BOOST_CORE_TYPEID(i))<<'\n';      // Output: int
  cout<<demangled_name(BOOST_CORE_TYPEID(f))<<'\n';      // Output: double
  cout<<demangled_name(BOOST_CORE_TYPEID(string))<<'\n'; // Output: std::string
  cout<<demangled_name(BOOST_CORE_TYPEID(string{}.begin()))<<'\n';
  // Output: __gnu_cxx::__normal_iterator<char*, std::string>
  cout<<demangled_name(BOOST_CORE_TYPEID(namen))<<'\n';
  // Output: std::map<int, std::string, std::less<int>,
  //                                                         std::allocator<std::pair<int const, std::string> > >
  cout<<demangled_name(BOOST_CORE_TYPEID(666/0))<<'\n';  // Output: int
}
```

----------------


## GodboltId:Y3GsPd97d

Book listing [lst-0081-book.cpp](lst-0081-book.cpp):
```cpp
// https://godbolt.org/z/Y3GsPd97d 
#include <set>
#include <string>
struct Dragon {
    std::string name_;
};
namespace std {
    template<> struct less<Dragon> { // Template specialization
        bool operator()(const Dragon &lhs, const Dragon &rhs) const {
            return lhs.name_ < rhs.name_;
} }; }
int main() {
  std::set<Dragon> dragons {
    Dragon{"Smaug"},    Dragon{"Glaurung"},
    Dragon{"Ancalagon"},Dragon{"Scatha"}};
}
```

Godbolt Listing [lst-0081-godb.cpp](lst-0081-godb.cpp), [https://godbolt.org/z/Y3GsPd97d](https://godbolt.org/z/Y3GsPd97d):
```cpp
//#(execute) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/Y3GsPd97d 
#include <set>
#include <string>
struct Dragon {
    std::string name_;
};
namespace std {
    template<> struct less<Dragon> { // Template specialization
        bool operator()(const Dragon &lhs, const Dragon &rhs) const {
            return lhs.name_ < rhs.name_;
} }; }
int main() {
  std::set<Dragon> dragons {
    Dragon{"Smaug"},    Dragon{"Glaurung"},
    Dragon{"Ancalagon"},Dragon{"Scatha"}};
}
```

----------------


## Listing 28.43: This calculator maps keys to functors.

Book listing [lst-0082-book.cpp](lst-0082-book.cpp):
```cpp
// https://godbolt.org/z/jxvhded75
#include <string>
#include <vector>
#include <iostream>
#include <map>
#include <functional>
std::map<char,std::function<int(int,int)>> binOps { // binary operators
    {'+', std::plus<int>{} },
    {'-', std::minus<int>{} },
    {'*', std::multiplies<int>{} },
    {'/', std::divides<int>{} },
    {'%', std::modulus<int>{} },
    };
std::map<char,std::function<int(int)>> unOps { };    // unary operators
auto val = [](auto n) { return [n](){ return n; };}; // returns a lambda
std::map<char,std::function<int()>> zeroOps {        // nullary operators
  {'0', val(0)}, {'1', val(1)}, {'2', val(2)}, {'3', val(3)}, {'4', val(4)},
  {'5', val(5)}, {'6', val(6)}, {'7', val(7)}, {'8', val(8)}, {'9', val(9)},
  };
std::map<char,std::function<void(std::vector<int>&)>> stackOps { 
  { ' ', [](auto &stack) { } },           // no operation
  { 'c', [](auto &stack) { stack.clear(); } }, // Clear the entire stack
  { ':', [](auto &stack) {                // swap the top two elements
            auto top = stack.back(); stack.pop_back();
            auto second = stack.back(); stack.pop_back();
            stack.push_back(top);
            stack.push_back(second);
  } },
  { '=', [](auto &stack) {                // print the entire stack
            for(int elem : stack) { std::cout << elem; }
            std::cout << "\n";
  } },
};
void calculator(std::string input) {
  std::vector<int> stack {};
  for(char c : input) {
    int top, second;
    if(auto it = unOps.find(c); it != unOps.end()) {
      // if unary operator …
      auto func = it->second;
      top = stack.back(); stack.pop_back(); // … get top element
      stack.push_back(func(top));           // … apply func, push result onto stack
    } else if(auto it = binOps.find(c); it != binOps.end()) {
      // if binary operator …
      auto func = it->second;
      top = stack.back(); stack.pop_back(); // … get the top 2 elements
      second = stack.back(); stack.pop_back();
      stack.push_back(func(second, top));// … apply func, push result onto stack
    } else if(auto it = zeroOps.find(c); it !=zeroOps.end()) {
      // if nullary operator …
      auto func = it->second;
      stack.push_back(func());               // ... result of func on stack
    } else if(auto it = stackOps.find(c); it != stackOps.end()) {
      // if stack operator
      auto func = it->second;
      func(stack);                           // ... apply func to stack
    } else {
      std::cout << "\n'" << c << "' I don't understand.\n";
    }
  } /* for c */
}
int main(int argc, const char* argv[]) {
    if(argc > 1) {
        calculator(argv[1]);
    } else {
        // 3+4*5+6 with multiplication before addition results in 29
        calculator("345*+6+=");
    }
    calculator("93-=");                      // 9 - 3 = Output: 6
    calculator("82/=");                      // 8 / 2 = Output: 4
    calculator("92%=");                      // 9 % 2 = Output: 1
}
```

Godbolt Listing [lst-0082-godb.cpp](lst-0082-godb.cpp), [https://godbolt.org/z/jxvhded75](https://godbolt.org/z/jxvhded75):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:boost@184
// https://godbolt.org/z/jxvhded75
#include <string>
#include <vector>
#include <iostream>
#include <map>
#include <functional>
std::map<char,std::function<int(int,int)>> binOps { // binary operators
    {'+', std::plus<int>{} },
    {'-', std::minus<int>{} },
    {'*', std::multiplies<int>{} },
    {'/', std::divides<int>{} },
    {'%', std::modulus<int>{} },
    };
std::map<char,std::function<int(int)>> unOps { };    // unary operators
auto val = [](auto n) { return [n](){ return n; };}; // returns a lambda
std::map<char,std::function<int()>> zeroOps {        // nullary operators
  {'0', val(0)}, {'1', val(1)}, {'2', val(2)}, {'3', val(3)}, {'4', val(4)},
  {'5', val(5)}, {'6', val(6)}, {'7', val(7)}, {'8', val(8)}, {'9', val(9)},
  };
std::map<char,std::function<void(std::vector<int>&)>> stackOps { 
  { ' ', [](auto &stack) { } },           // no operation
  { 'c', [](auto &stack) { stack.clear(); } }, // Clear the entire stack
  { ':', [](auto &stack) {                // swap the top two elements
            auto top = stack.back(); stack.pop_back();
            auto second = stack.back(); stack.pop_back();
            stack.push_back(top);
            stack.push_back(second);
  } },
  { '=', [](auto &stack) {                // print the entire stack
            for(int elem : stack) { std::cout << elem; }
            std::cout << "\n";
  } },
};
void calculator(std::string input) {
  std::vector<int> stack {};
  for(char c : input) {
    int top, second;
    if(auto it = unOps.find(c); it != unOps.end()) {
      // if unary operator …
      auto func = it->second;
      top = stack.back(); stack.pop_back(); // … get top element
      stack.push_back(func(top));           // … apply func, push result onto stack
    } else if(auto it = binOps.find(c); it != binOps.end()) {
      // if binary operator …
      auto func = it->second;
      top = stack.back(); stack.pop_back(); // … get the top 2 elements
      second = stack.back(); stack.pop_back();
      stack.push_back(func(second, top));// … apply func, push result onto stack
    } else if(auto it = zeroOps.find(c); it !=zeroOps.end()) {
      // if nullary operator …
      auto func = it->second;
      stack.push_back(func());               // ... result of func on stack
    } else if(auto it = stackOps.find(c); it != stackOps.end()) {
      // if stack operator
      auto func = it->second;
      func(stack);                           // ... apply func to stack
    } else {
      std::cout << "\n'" << c << "' I don't understand.\n";
    }
  } /* for c */
}
int main(int argc, const char* argv[]) {
    if(argc > 1) {
        calculator(argv[1]);
    } else {
        // 3+4*5+6 with multiplication before addition results in 29
        calculator("345*+6+=");
    }
    calculator("93-=");                      // 9 - 3 = Output: 6
    calculator("82/=");                      // 8 / 2 = Output: 4
    calculator("92%=");                      // 9 % 2 = Output: 1
}
```

----------------


## GodboltId:e6xT638ch

Book listing [lst-0083-book.cpp](lst-0083-book.cpp):
```cpp
// https://godbolt.org/z/e6xT638ch 
#include <functional> // subtract, minus, bind
#include <iostream>
using std::cout;
int subtract(int a, int b) { return a - b; }
int main() {
    using namespace std::placeholders;
    cout << subtract(9, 3) << '\n'  ;  // Output: 6
    auto minus3 = std::bind(subtract, _1, 3);
    cout << minus3(9) << '\n';         // Output: 6
    auto from9 = std::bind(subtract, 9, _1);
    cout << from9(3) << '\n';          // Output: 6
    auto againMinus3 = std::bind(std::minus<int>{}, _1, 3);
    cout << againMinus3(9) << '\n';    // Output: 6
}
```

Godbolt Listing [lst-0083-godb.cpp](lst-0083-godb.cpp), [https://godbolt.org/z/e6xT638ch](https://godbolt.org/z/e6xT638ch):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/e6xT638ch 
#include <functional> // subtract, minus, bind
#include <iostream>
using std::cout;
int subtract(int a, int b) { return a - b; }
int main() {
    using namespace std::placeholders;
    cout << subtract(9, 3) << '\n'  ;  // Output: 6
    auto minus3 = std::bind(subtract, _1, 3);
    cout << minus3(9) << '\n';         // Output: 6
    auto from9 = std::bind(subtract, 9, _1);
    cout << from9(3) << '\n';          // Output: 6
    auto againMinus3 = std::bind(std::minus<int>{}, _1, 3);
    cout << againMinus3(9) << '\n';    // Output: 6
}
```

----------------


## Listing 28.44: With "bind", you can also set all parameters of a function.

Book listing [lst-0084-book.cpp](lst-0084-book.cpp):
```cpp
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
```

Godbolt Listing [lst-0084-godb.cpp](lst-0084-godb.cpp), [https://godbolt.org/z/j7sa9rexh](https://godbolt.org/z/j7sa9rexh):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:boost@184
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
```

----------------


## Listing 28.45: This is how you turn class members into free functions.

Book listing [lst-0087-book.cpp](lst-0087-book.cpp):
```cpp
// https://godbolt.org/z/GMjcjrvaT
#include <functional>
#include <iostream>
struct Numbers {
    int theNumber() {
        return 42;
    }
    int more(int n) {
        return n + data;
    }
    int data = 7;
};
int main() {
    auto func = std::mem_fn(&Numbers::theNumber);
    auto func2 = std::mem_fn(&Numbers::more);
    auto access = std::mem_fn(&Numbers::data);
    Numbers numbers;
    std::cout << func(numbers) << '\n';        // Output: 42
    std::cout << func2(numbers, 66) << '\n';   // Output: 73
    std::cout << access(numbers) << '\n';      // Output: 7
}
```

Godbolt Listing [lst-0087-godb.cpp](lst-0087-godb.cpp), [https://godbolt.org/z/GMjcjrvaT](https://godbolt.org/z/GMjcjrvaT):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:boost@184
// https://godbolt.org/z/GMjcjrvaT
#include <functional>
#include <iostream>
struct Numbers {
    int theNumber() {
        return 42;
    }
    int more(int n) {
        return n + data;
    }
    int data = 7;
};
int main() {
    auto func = std::mem_fn(&Numbers::theNumber);
    auto func2 = std::mem_fn(&Numbers::more);
    auto access = std::mem_fn(&Numbers::data);
    Numbers numbers;
    std::cout << func(numbers) << '\n';        // Output: 42
    std::cout << func2(numbers, 66) << '\n';   // Output: 73
    std::cout << access(numbers) << '\n';      // Output: 7
}
```

----------------


## Listing 28.46: Basic functionalities of "variant".

Book listing [lst-0088-book.cpp](lst-0088-book.cpp):
```cpp
// https://godbolt.org/z/baY7Tz91f
#include <variant> 
using std::get;
int main() {
    std::variant<int, float> v{};
    v = 12;                   // State changes to int
    auto i = get<int>(v);     // retrieves the int
    std::cout << i << '\n';   // Output: 12
    v = 3.456f;               // State changes to float
    std::cout << get<float>(v) << '\n';  // Output: 3.456
    get<double>(v);           // (ERR) Error
    get<3>(v);                // (ERR) Error
    std::variant<int, float> w{};
    w = get<float>(v);        // Access by type
    w = get<1>(v);            // Access is also possible via index
    w = v;                    // entire assignment is also possible
    try {
        get<int>(w);          // triggers exception
    } catch (std::bad_variant_access&) { /* ... */ }
}
```

Godbolt Listing [lst-0088-godb.cpp](lst-0088-godb.cpp), [https://godbolt.org/z/baY7Tz91f](https://godbolt.org/z/baY7Tz91f):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:boost@184
// https://godbolt.org/z/baY7Tz91f
#include <variant> 
using std::get;
int main() {
    std::variant<int, float> v{};
    v = 12;                   // State changes to int
    auto i = get<int>(v);     // retrieves the int
    std::cout << i << '\n';   // Output: 12
    v = 3.456f;               // State changes to float
    std::cout << get<float>(v) << '\n';  // Output: 3.456
    get<double>(v);           // (ERR) Error
    get<3>(v);                // (ERR) Error
    std::variant<int, float> w{};
    w = get<float>(v);        // Access by type
    w = get<1>(v);            // Access is also possible via index
    w = v;                    // entire assignment is also possible
    try {
        get<int>(w);          // triggers exception
    } catch (std::bad_variant_access&) { /* ... */ }
}
```

----------------


## Listing 28.47: Inspecting a "variant" using a visitor.

Book listing [lst-0089-book.cpp](lst-0089-book.cpp):
```cpp
// https://godbolt.org/z/G4z8Y5E6P
#include <variant>
#include <iostream>
using std::cout;
struct TypeGreeting {
  void operator()(int) const { cout << "Hello int"; }
  void operator()(float) const { cout << "Hello float"; }
};
int main() {
    std::variant<int, float> var{};
    var = 12;                                   // State int
    std::visit([](auto a) { cout << a; }, var); // generic lambda
    cout << std::endl;
    var = 3.456f;                               // State float
    std::visit(TypeGreeting{}, var);            // Functor with overloads
    cout << std::endl;
}
```

Godbolt Listing [lst-0089-godb.cpp](lst-0089-godb.cpp), [https://godbolt.org/z/G4z8Y5E6P](https://godbolt.org/z/G4z8Y5E6P):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:boost@184
// https://godbolt.org/z/G4z8Y5E6P
#include <variant>
#include <iostream>
using std::cout;
struct TypeGreeting {
  void operator()(int) const { cout << "Hello int"; }
  void operator()(float) const { cout << "Hello float"; }
};
int main() {
    std::variant<int, float> var{};
    var = 12;                                   // State int
    std::visit([](auto a) { cout << a; }, var); // generic lambda
    cout << std::endl;
    var = 3.456f;                               // State float
    std::visit(TypeGreeting{}, var);            // Functor with overloads
    cout << std::endl;
}
```

----------------


## GodboltId:1eco9Wcr3

Book listing [lst-0090-book.cpp](lst-0090-book.cpp):
```cpp
// https://godbolt.org/z/1eco9Wcr3 
#include <any>
#include <iostream>
#include <vector>
#include <string>
int main() {
    std::any a = 5;
    std::cout << std::any_cast<int>(a) << '\n';
    a = 3.456;
    std::cout << std::any_cast<double>(a) << '\n';
    using namespace std::literals;
    std::vector<std::any> data { 4, 8.976, "Geronimo"s };
    std::cout << std::any_cast<double>( data[1] ) << '\n';
    std::cout << data[1].type().name()  << '\n';
}
```

Godbolt Listing [lst-0090-godb.cpp](lst-0090-godb.cpp), [https://godbolt.org/z/1eco9Wcr3](https://godbolt.org/z/1eco9Wcr3):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/1eco9Wcr3 
#include <any>
#include <iostream>
#include <vector>
#include <string>
int main() {
    std::any a = 5;
    std::cout << std::any_cast<int>(a) << '\n';
    a = 3.456;
    std::cout << std::any_cast<double>(a) << '\n';
    using namespace std::literals;
    std::vector<std::any> data { 4, 8.976, "Geronimo"s };
    std::cout << std::any_cast<double>( data[1] ) << '\n';
    std::cout << data[1].type().name()  << '\n';
}
```

----------------


## GodboltId:115nE7xsc

Book listing [lst-0093-book.cpp](lst-0093-book.cpp):
```cpp
// https://godbolt.org/z/115nE7xsc
#include <charconv>
#include <vector>
#include <iostream>
#include <string>

std::vector<size_t> num_to_vec(const std::string& nums) {
    std::vector<size_t> result {};
    // without trailing spaces
    const auto end = nums.data() + nums.find_last_not_of( ' ' ) + 1;
    const char* st = nullptr;  // loop pointer
    auto last = nums.data();   // last untranslated character
    size_t n;                  // converted number
    do {
        for(st = last; (st<end)&&(*st==' ' ); ++st); // skip ' '
        if (last = std::from_chars(st, end, n).ptr; last != st)
            result.push_back(n);                     // store number
    } while (last != st);
    return result;
}

void errorDemo(const char* buf, size_t sz) {
    int n;
    auto [p, ec] = std::from_chars(buf, buf+sz, n);
    if (ec != std::errc{}) {
       const auto error = std::make_error_code(ec);
       std::cout << error.message() << '\n';
    }
}

int main() {
    auto result = num_to_vec("12 33 43");

    for(auto r : result) std::cout << r << " ";
    std::cout << '\n';
    // Output: 12 33 43

    errorDemo("XYZ", 4);
    // Output: Invalid argument

    errorDemo("123123123123123", 16);
    // Output: Numerical result out of range
}
```

Godbolt Listing [lst-0093-godb.cpp](lst-0093-godb.cpp), [https://godbolt.org/z/115nE7xsc](https://godbolt.org/z/115nE7xsc):
```cpp
//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:boost@184
// https://godbolt.org/z/115nE7xsc
#include <charconv>
#include <vector>
#include <iostream>
#include <string>

std::vector<size_t> num_to_vec(const std::string& nums) {
    std::vector<size_t> result {};
    // without trailing spaces
    const auto end = nums.data() + nums.find_last_not_of( ' ' ) + 1;
    const char* st = nullptr;  // loop pointer
    auto last = nums.data();   // last untranslated character
    size_t n;                  // converted number
    do {
        for(st = last; (st<end)&&(*st==' ' ); ++st); // skip ' '
        if (last = std::from_chars(st, end, n).ptr; last != st)
            result.push_back(n);                     // store number
    } while (last != st);
    return result;
}

void errorDemo(const char* buf, size_t sz) {
    int n;
    auto [p, ec] = std::from_chars(buf, buf+sz, n);
    if (ec != std::errc{}) {
       const auto error = std::make_error_code(ec);
       std::cout << error.message() << '\n';
    }
}

int main() {
    auto result = num_to_vec("12 33 43");

    for(auto r : result) std::cout << r << " ";
    std::cout << '\n';
    // Output: 12 33 43

    errorDemo("XYZ", 4);
    // Output: Invalid argument

    errorDemo("123123123123123", 16);
    // Output: Numerical result out of range
}
```

----------------


## GodboltId:dcqenEd31

Book listing [lst-0095-book.cpp](lst-0095-book.cpp):
```cpp
// https://godbolt.org/z/dcqenEd31 
#include <iostream>
#include <charconv>
#include <string_view>
#include <array>
int main() {
    std::array<char, 10> str {};
    if(auto [p, ec] = std::to_chars(str.data(), str.data() + str.size(), 42);
       ec == std::errc{} )
        std::cout << std::string_view(str.data(), p - str.data()) << "\n";
}
```

Godbolt Listing [lst-0095-godb.cpp](lst-0095-godb.cpp), [https://godbolt.org/z/dcqenEd31](https://godbolt.org/z/dcqenEd31):
```cpp
//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/dcqenEd31 
#include <iostream>
#include <charconv>
#include <string_view>
#include <array>
int main() {
    std::array<char, 10> str {};
    if(auto [p, ec] = std::to_chars(str.data(), str.data() + str.size(), 42);
       ec == std::errc{} )
        std::cout << std::string_view(str.data(), p - str.data()) << "\n";
}
```

----------------

