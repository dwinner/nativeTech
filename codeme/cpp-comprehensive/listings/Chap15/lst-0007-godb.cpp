//#(execute) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/9rY7qhK89 
class Date {
    Year year_;
    Month month_ {1};
    Day day_ {1};
public:
    explicit Date(int y) : year_{y} {} // year-01-01
    Date(Year y, Month m, Day d) : year_{y}, month_{m}, day_{d} {}
    ostream& print(ostream& os) const;
};
ostream& Date::print(ostream& os) const {
    return os << year_ << "-" << month_ << "-"  << day_;
}
ostream& operator<<(ostream& os, const Date& right) {
    return right.print(os);
}
int main() {
    Date d1 { Year{2024}, Month{11}, Day{19} };
    std::cout << d1 << "\n"; // Output: 2024-11-19
}