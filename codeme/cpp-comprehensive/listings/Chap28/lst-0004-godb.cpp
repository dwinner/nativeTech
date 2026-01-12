//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/zG7zYGf3K
// … as before …
int main() {
  using std::tie; using std::ignore;
  string lastName {};
  int birthYear {};
  tie(ignore, lastName, ignore, birthYear) = president(2015);
  cout << lastName << ' ' << birthYear << '\n'; // Output: Obama 1961
}