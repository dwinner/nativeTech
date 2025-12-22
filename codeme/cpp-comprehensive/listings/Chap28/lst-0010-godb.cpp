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