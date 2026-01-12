//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/zEsxd8e7G
#include <string>
#include <set>
#include <iostream>
#include <tuple> // tuple, tie
using std::string; using std::set; using std::cout; using std::tie;
struct Hobbit {
  string firstName;
  string lastName;
  Hobbit(const string v, const string n) : firstName{v}, lastName{n} {}
};
struct CompLastName {
  bool operator()(const Hobbit& x, const Hobbit& y) const { // normal <
    return tie(x.lastName, x.firstName) < tie(y.lastName, y.firstName);
  }
  using is_transparent = std::true_type; // allowed for find
  bool operator()(const Hobbit& x, const string& y) const { // for find etc.
    return x.lastName < y;
  }
  bool operator()(const string& x, const Hobbit& y) const { // for find etc.
    return x < y.lastName;
  }
};
int main() {
    using namespace std::literals; // allow "…"s
    set<Hobbit,CompLastName> hobbits;
    hobbits.emplace( "Frodo", "Baggins" );
    hobbits.emplace( "Sam", "Gamgee" );
    auto f1 = hobbits.find( Hobbit{"Frodo", "Baggins"} ); // whole key
    if(f1 != hobbits.end()) {
        cout << "found: " << f1->firstName << '\n';       // Frodo
    }
    auto f2 = hobbits.find( "Gamgee"s );                  // equivalent key
    if(f2 != hobbits.end()) {
        cout << "found: " << f2->firstName << '\n';       // Sam
    }
}