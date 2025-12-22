
// https://godbolt.org/z/W6Mfnxj1W
#include <cstdio> // toupper, tolower
// …
auto comp = [](char a, char b) { return toupper(a) < toupper(b); };
map<char,int,decltype(comp)> lets(comp); // as template parameter and argument
lets['a'] = 1;
lets['B'] = 2;
lets['c'] = 3;
lets['A'] = 4; // overwrites position 'a'
cout <<= lets; // Output: a:4 B:2 c:3
struct Comp {  // Functor
    bool operator()(char a, char b) const { return toupper(a) < toupper(b); }
};
map<char,int,Comp> lets2; // here the template parameter is sufficient