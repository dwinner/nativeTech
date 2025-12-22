
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