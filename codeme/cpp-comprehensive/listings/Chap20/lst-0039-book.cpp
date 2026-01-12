
// https://godbolt.org/z/oKsz7d55T 
#include <string>
#include <iostream>                    // cout
using std::string; using std::cout;
string greet(string name) {
    return name + "!";                 // string operator+(string, const char*)
}
int main() {
    string name{ "Havalock Vetinari" };// explicit: string(const char*)
    cout << "Angua";                   // ostream& operator<<(ostream&, const char*)
    cout <<                            // ostream& operator<<(ostream, string)
      greet("Carrot Ironfoundersson"); // implicitly: string(const char*)
}