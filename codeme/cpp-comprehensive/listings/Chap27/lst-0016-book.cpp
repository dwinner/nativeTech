
#include <iostream>
using std::cout; using std::endl;
int main() {
    auto ddash = [](auto &os) -> std::ostream& { return os << "--"; };
    cout << "Text1" << ddash << "Text2" << endl; // Output: Text1--Text2
}