//#(compile) c++; compiler:g141; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/3EG1ovcev
#include <string>
#include <string_view>
using std::string; using std::string_view;
class Widget {
    string name_ = "";
public:
    void setName(string_view newName) {
        name_ = newName;
    }
    const string& getName() const {    // const& return
        return name_;
    }
};
int main() {
    Widget w{};
    w.setName("Title");
    string name1 = w.getName();        // new string, thus a copy
    name1.clear();                     // you are allowed to modify the copy again
    const string& name2 = w.getName(); // const reference to internal string name_
    /* name2.clear(); */               // name2 is const, so it doesn't work
    string& name3 = w.getName();       // (ERR) Function returns const&, not &.
    auto name4 = w.getName();          // identical to name1
    const auto& name5 = w.getName();   // identical to name2
}