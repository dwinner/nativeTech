
// https://godbolt.org/z/9MTj3rP5d 
#include <string>
#include <iostream>
using std::string; using std::cout;

class Widget {
    string name_{};
public:
    const string& readName() const;         // const&-return, const-method
    string& getName();                      // &-return
};

const string& Widget::readName() const { return name_; }
string& Widget::getName() { return name_; }

int main() {
    Widget w{};
    const string& readonly = w.readName(); // const&, immutable
    cout << "Name: " << readonly << "\n";  // still "" empty.
    string& readwrite = w.getName();       // &, mutable
    readwrite.append("attached");          // also changes name_ and readonly
    cout << "Name via readwrite: " << readwrite << "\n"; // "attached"
    cout << "Name via readonly: " << readonly << "\n";   // also "attached"
}