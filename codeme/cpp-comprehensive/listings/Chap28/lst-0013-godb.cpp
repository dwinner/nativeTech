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