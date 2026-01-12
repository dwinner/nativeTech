
// https://godbolt.org/z/TY5x54efE
#include <unordered_set>
#include <iostream>
#include <vector>
#include <string>
using std::string; using std::unordered_set; using std::cout;
struct Word {
    string word_;
    size_t row_;
    Word(const string &word, size_t row)
        : word_{word}, row_{row} {}
    friend bool operator==(const Word& a, const Word &b)
        { return a.word_ == b.word_; } // ignores row
 };
 namespace std {
 template<> struct hash<Word> { // ignores row
        std::hash<string> stringHash;
        std::size_t operator()(const Word &w) const {
            return stringHash(w.word_);
        }
 }; }
 struct ExactWordHash { // includes row
     std::hash<string> sHash;
     std::hash<size_t> iHash;
     bool operator()(const Word& a) const {
         return sHash(a.word_) ^ iHash(a.row_);
     }
 };
 struct ExactWordEqual { // includes row
     bool operator()(const Word& a, const Word &b) const {
         return std::tie(a.word_, a.row_) == std::tie(b.word_, b.row_);
     }
 };
 int main() {
     std::vector input {
       Word{"a",0}, Word{"rose",0},
       Word{"is",1}, Word{"a",1}, Word{"rose",1},
       Word{"is",2}, Word{"a",2}, Word{"rose",2},  };
     // Use overloads
     unordered_set<Word> words( input.begin(), input.end() );
     cout << words.size() << '\n'; // Output: 3
     // Use custom functors
     unordered_set<Word,ExactWordHash,ExactWordEqual> poem(
          input.begin(), input.end() );
     cout << poem.size() << '\n';  // Output: 8
     // Hash as Lambda
     auto h = [](const auto &a) { return std::hash<string>{}(a.word_); };
     unordered_set<Word,decltype(h)> rose(input.begin(), input.end(), 10, h);
     cout << rose.size() << '\n';  // Output: 3
}