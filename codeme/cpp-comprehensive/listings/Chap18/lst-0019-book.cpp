
// https://godbolt.org/z/zh5cPG7TK
class CalendarClock : public Clock, public Calendar {
public:
    CalendarClock(int y, int m, int d, int hh, int mm, int ss)
    : Calendar{y,m,d}, Clock{hh,mm,ss} {}
    void tick() {         // +1 second
        auto prev_h = h_;
        Clock::tick();    // Call base class method
        if(h_ < prev_h) { // if new day
            advance();    // … advance calendar
        }
    }
    friend ostream& operator<<(ostream&os, const CalendarClock& cc) {
        operator<<(os, (Calendar&)cc) << " "; // Call free function
        operator<<(os, (Clock&)cc);           // Call free function
        return os;
    }
};