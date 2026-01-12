//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/oxoef1nTs
// Parameters must be valid values of the respective unit.
#include <iostream> // ostream
#include <iomanip>  // setw, setfill
#include <format>
using std::ostream; using std::format;
class Clock {
protected:
    int h_, m_, s_;
public:
    Clock(int hours, int minutes, int seconds)
    : h_{hours}, m_{minutes}, s_{seconds} {}
    void setClock(int hours, int minutes, int seconds) {
        h_ = hours; m_ = minutes; s_ = seconds;
    }
    friend ostream& operator<<(ostream&os, const Clock& c) {
        return os << format("{:02}:{:02}:{:02}", c.h_, c.m_, c.s_);
    }
    void tick() { // +1 second
        if(s_ >= 59) {
            s_ = 0;
            if(m_ >= 59) {
                m_ = 0;
                if(h_ >= 23) h_ = 0;
                else { h_ += 1; }
            } else { m_ += 1; }
        } else { s_ += 1; }
    }
};