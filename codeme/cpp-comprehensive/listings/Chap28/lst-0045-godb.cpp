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