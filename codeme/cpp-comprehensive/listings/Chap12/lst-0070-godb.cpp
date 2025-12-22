//#(execute) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/EPz4Kd6jP 
class Year {
    int value_;
public:
    explicit Year(int v) : value_{v} {}
    std::ostream& print(std::ostream& os) const;
    Year& advance(const Year& other);
    bool equals(const Year& other) const;
    bool less_than(const Year& other) const;
};
std::ostream& Year::print(std::ostream& os) const {
    return os << value_;
}
std::ostream& operator<<(std::ostream& os, const Year& year) {
    return year.print(os);
}
Year& Year::advance(const Year& other) {
    value_ += other.value_;
    return *this;
}
bool Year::equals(const Year& other) const {
    return value_ == other.value_;
}
bool Year::less_than(const Year& other) const {
    return value_ < other.value_;
}