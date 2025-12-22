//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/TE1EcEYPv
// … as before …
int main() {
  tuple ps = president(2015);
  cout << get<int>(ps) << '\n';     // Output: 1940
  cout << get<string>(ps) << '\n';  // (ERR) not unique
}