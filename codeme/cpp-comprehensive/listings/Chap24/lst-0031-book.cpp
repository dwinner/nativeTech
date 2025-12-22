
// https://godbolt.org/z/freaK1Pea
vector vowels { 'A', 'e', 'i', 'o', 'u' };
const vector even { '0', '2', '4', '6', '8' };
auto it1 = vowels.begin();           // vector<char>::iterator
*it1 = 'a';                          // '*it1' returns 'char&'
auto it2 = even.begin();             // vector<char>::const_iterator
auto it3 = vowels.cbegin();          // enforces const_iterator
*i2 = '9'; *i3 = 'x';                // (ERR) 'const char&' is not modifiable
for(auto it=vowels.cbegin()+1; it!=vowels.cend(); ++it)
    { cout << *it; } cout << '\n';   // Output: eiou
for(auto it=vowels.crbegin()+1; it!=vowels.crend(); ++it) // ++ despite reverse!
    { cout << *it; } cout << '\n';   // Output: oiea