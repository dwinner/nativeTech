
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