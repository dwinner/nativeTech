//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/KKE6nWT49 
#include <iostream>
using std::cout; using std::endl;
class dendl { // Dots followed by newline
    int dendl_;
public:
    dendl(int n=1)
      : dendl_{n} {}
    std::ostream& operator()(std::ostream& os) const { // Functor
        for(int i=0; i<dendl_; ++i) os << '.';
        return os << '\n';
    }
};
std::ostream& operator<<( std::ostream& os, const dendl& elem) {
    return elem(os);
}
int main() {
    cout << "Text1" << dendl(4); // Output: Text1....
    cout << "Text2" << dendl(2); // Output: Text2..
    cout << "Text3" << dendl();  // Output: Text3.
}