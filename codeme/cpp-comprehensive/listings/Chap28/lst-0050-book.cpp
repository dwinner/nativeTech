
// https://godbolt.org/z/19vf9jfTM 
#include <chrono>
#include <iostream>
using namespace std::chrono;
int main() {
    for (int mo = 1; mo <= 12; ++mo) {
        year_month_weekday patch_tues{mo/Tuesday[2]/2024y};
        year_month_day ymd{sys_days{patch_tues}};
        std::cout << ymd.month() << " " << ymd.day() << "\n";
    }
}