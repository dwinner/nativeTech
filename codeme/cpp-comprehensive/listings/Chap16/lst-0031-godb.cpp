//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/Wvr3aK75G 
#include <compare>               // for <=>
class Value {
    int value_; 
public:
    explicit Value(int value) : value_{value} {}

    // free functions, but declared as friend
    friend Value operator+(const Value& li, const Value& re);
    friend Value operator-(const Value& li, const Value& re)
        { return Value{li.value_ - re.value_};  // also defined inside
    }
    // operator<=> does not need to be friend.
    // for all comparisons: <, >, <=, >= as well as == and !=
    auto operator<=>(const Value& re) const = default; 
};

// Definition of the previously declared friend function:
Value operator+(const Value& li, const Value& re) {
    return Value{li.value_ + re.value_}; // access to private members allowed
}

int main() {
    Value seven{7}; Value three{3}; Value two{2};
    if((three+two) < seven) {    // operator+, then operator< via <=>
        return 0;                // okay
    } else {
        return 1;                // something went wrong
    }
}