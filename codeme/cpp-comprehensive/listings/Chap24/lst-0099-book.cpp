
// https://godbolt.org/z/rWGrhbYa7
#include <string>
#include <iterator> // distance
#include <ranges>   // subrange
struct Person {
  std::string name;
  friend bool operator<(const Person &a, const Person &b) {  
    // only first letter
    return a.name.size()==0 ? true
      : (b.name.size()==0 ? false : a.name[0] < b.name[0]);
  }
};
// …
multiset data{ 1, 4,4, 2,2,2, 7, 9 };
auto [from1, to1] = data.equal_range(2);
cout << "Number of 2s: "
  << std::distance(from1, to1) << '\n';     // Output: Number of 2s: 3
auto [from2, to2] = data.equal_range(5);
cout << "Number of 5s: "
  << std::distance(from2, to2) << '\n';     // Output: Number of 5s: 0
multiset<Person> room{
  {"Karl"}, {"Kurt"}, {"Peter"}, {"Karl"}, {"Ken"}};
auto [p, q] = room.equal_range(Person{"K"});
for(auto& p : std::ranges::subrange(p,q)) { // C++20 range or simple loop
  cout << p.name << ' ';
}
cout << '\n'; // Output: Karl Kurt Karl Ken