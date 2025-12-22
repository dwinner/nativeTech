
// https://godbolt.org/z/Ee5anM9as
#include <deque>
#include <iostream>
#include <string>
#include <cctype>   // toupper
#include <iomanip>  // boolalpha
using namespace std;
bool isPalindrome(string_view word) {
    // check from both ends simultaneously
    deque<char> deq{};
    for(char ch : word) {
        deq.push_back(toupper(ch));    // uppercase
    }
    auto ok = true;
    while(deq.size()>1 && ok) {
       if(deq.front() != deq.back()) {
           ok = false;
       }
       deq.pop_front();                // Hello deque!
       deq.pop_back();
    }
    return ok;
}
int main() {
    cout << boolalpha; // Print 'true' and 'false' instead of '1' and '0'
    cout << isPalindrome("Abrakadabra") << '\n';   // false
    cout << isPalindrome("Kajak") << '\n';         // true
    cout << isPalindrome("EvilOlive") << '\n';     // true
    cout << isPalindrome("Aibohphobia") << '\n';   // true
    cout << isPalindrome("Madam") << '\n';         // true
    cout << isPalindrome("") << '\n';              // true
}