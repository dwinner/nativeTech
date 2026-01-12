
// https://godbolt.org/z/hhGo46z5a 
#include <iostream>  // ostream
#include <format>    // format, vformat, make_format_args
using std::ostream;
class Value {
protected: // not public, only for own and derived use
    int value_;
    const std::string fmt_;   // e.g. "{:02}" or "{:04}"
    Value(int v, unsigned w)  // constructor with two arguments
      : value_{v}, fmt_{std::format("{{:0{}}}", w)} {}
public:
    ostream& print(ostream& os) const;
};
ostream& operator<<(ostream& os, const Value& right) {
    return right.print(os);
}
ostream& Value::print(ostream& os) const {
    return os << std::vformat(fmt_, std::make_format_args(value_));
}