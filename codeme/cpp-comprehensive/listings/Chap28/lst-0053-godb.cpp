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