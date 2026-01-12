//#(compile) c++; compiler:g141; options:-O3 -std=c++23 -DNDEBUG; libs:-
// https://godbolt.org/z/Ed88znPn9
#include <unordered_set> // unordered_multiset
#include <iostream>
#include <string>
using std::string; using std::unordered_multiset; using std::cout;
struct City {
    string name_;
    explicit City(const string &name) : name_{name} {}
    auto operator<=>(const City &b) const = default;
 };
 struct Entry { string city_; int zip_; };
 struct EqEntry {
     bool operator()(const Entry&a, const Entry&b) const {
         return a.city_==b.city_;
     }
 };
 struct HashEntry {
     std::hash<string> sHash;
     size_t operator()(const Entry& a) const {
         return sHash(a.city_);
     }
 };
int main() {
    unordered_multiset<Entry, HashEntry, EqEntry> directory{
        {Entry{"New York", 10001}},
        {Entry{"New York", 10002}},
        {Entry{"New York", 10003}},
        {Entry{"Chicago", 60601}},
        {Entry{"Chicago", 60602}},
    };
    const Entry search{"New York", 0}; // ZIP code does not matter in search
    cout << "New York has " << directory.count(search) << " ZIP codes.\n";
    cout << "The ZIP codes of New York are:\n";
    auto [where, until] = directory.equal_range(search);
    while (where != until) {
        cout << "  " << where->zip_ << '\n';
        ++where;
    }
}