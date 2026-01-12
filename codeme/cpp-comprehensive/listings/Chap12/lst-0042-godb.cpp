//#(compile) c++; compiler:g141; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/aGd56xzsT 
#include <string>       // string, stoi
#include <iostream>     // cin, cout, ostream
#include <format>
using std::ostream; using std::format;
class Year {  /* Helper types for safe date */
    int value_; // e.g., 2024
public:
    Year(int v) : value_{v} {}
    int value() { return value_; }
};
class Month {
    int value_; // 1..12
public:
    Month(int v) : value_{v} {}
    int value() { return value_; }
};
class Day {
    int value_; // 1..31
public:
    Day(int v) : value_{v} {}
    int value() { return value_; }
};
/* type-safe constructing date */
class Date {
    Year year_;
    Month month_ = 1;
    Day day_ = 1;
public:
    Date(int y) : year_{y}           // 1-argument constructor
        {}                           // sets 1st January of the specified year
    Date(Year y, Month m, Day d)     // 3-argument constructor
        : year_{y}, month_{m}, day_{d}
        {}
    ostream& print(ostream& os);     // e.g., 2024-04-20
};
ostream& Date::print(ostream& os) {  // e.g., 2024-04-20
    return os << format("{}-{:02}-{:02}",
        year_.value(), month_.value(), day_.value());
}
ostream& operator<<(ostream& os, Date d) {
    return d.print(os);
}
// http://codegolf.stackexchange.com/a/11146/1405, user Fors, 2014-02-25
Date easter(Year year) {
    const int y = year.value();
    int a = y/100*1483 - y/400*2225 + 2613;
    int b = (y%19*3510 + a/25*319)/330%29;
    b = 148 - b - (y*5/4 + a - b)%7;
    return Date{Year{y}, Month{b/31}, Day{b%31 + 1}}; // Create date
}
int main(int argc, const char *argv[] ) {
    /* Input */
    int number {};
    if(argc > 1) {
        number = std::stoi(argv[1]);
    } else {
        std::cout << "Year? "; std::cin >> number;
    }
    /* Calculation */
    Date date = easter(number);  // implicit conversion to Year
    /* Output */
    std::cout << "Easter: " << date << "\n";
}