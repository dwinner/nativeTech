//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/P7zbrh71d
#include <vector>
#include <string>
#include <iostream>
#include <string_view>
using std::string; using std::string_view; using std::vector; using std::cout;
int get_len(string_view str) { return str.size(); } // string_view as parameter
int main() {
    string s1 = "Hello";                  // simply with string literal
    string s2{'H','e','l','l','o'};       // or with list of char
    using namespace std::literals;        // for ""s-suffix and ""sv-suffix
    auto s3 = "Hello"s;  // even simpler with real string literal
    vector<char> v1{"Hello"};             // (ERR) no vector with string literal
    vector<char> v2{'H','e','l','l','o'}; // list of char is okay
    cout << s1 << s2 << s3 << '\n';       // Output of string works
    cout << v1 << v2 << '\n';             // (ERR) vector has no output
    const auto str = "String"s;           // Stringliteral
    const auto strv = "String-View"sv;    // String-View literal
    cout << "Length of 'str' is " << get_len(str) << '\n';   // Output: … 6
    cout << "Length of 'strv' is " << get_len(strv) << '\n'; // Output: … 11
}