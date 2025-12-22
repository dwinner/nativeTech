
// https://godbolt.org/z/n6En49dsh
#include <vector>
#include <string>
struct President {
  std::string name_; int year_;
  President(std::string name,  int year)    // name is by Value
    : name_{std::move(name)}, year_{year}   // move saves an additional copy here
    { }
};
int main() {
    using namespace std;    // enable string literals
    vector<President> presidents{};
    presidents.emplace_back("Washington", 1789);
    presidents.emplace_back("Lincoln", 1861);
    presidents.emplace_back("Kennedy", 1961);
    // …
    vector<int> v{};
    v.reserve(100);
    for(int idx = 0; idx < 100; ++idx)
        v.push_back(idx);
    // …
}