//    #!cpp filename=28chronoExceptions.CXX
#include <chrono>
#include <iostream>
using namespace std::chrono;
int main() {
    auto spring = local_days{31d/March/2024} + 2h + 1min;
    try {
        auto zt = zoned_time{"Europe/Berlin", spring};
    } catch (const nonexistent_local_time& e) {
        std::cout << e.what() << '\n'; // (+Exception thrown:+) does not exist
    }
// 2024-03-31 02:01:00 is in a gap between
// 2024-03-31 02:00:00 CET and
// 2024-03-31 03:00:00 CEST which are both equivalent to
// 2024-03-31 01:00:00 UTC

    //
    auto autumn = local_days{27d/October/2024} + 2h + 1min;
    try {
        auto zt = zoned_time{"Europe/Berlin", autumn};
    } catch (const ambiguous_local_time& e) {
        std::cout << e.what() << '\n'; // (+Exception thrown:+) exists twice
    }
// 2024-10-27 02:01:00 is ambiguous.  It could be
// 2024-10-27 02:01:00 CEST == 2024-10-27 00:01:00 UTC or
// 2024-10-27 02:01:00 CET == 2024-10-27 01:01:00 UTC

    std::cout << zoned_time{"Europe/Berlin", autumn, choose::earliest} << '\n';
// 2024-10-27 02:01:00 CEST

    std::cout << zoned_time{"Europe/Berlin", autumn, choose::latest} << '\n';
// 2024-10-27 02:01:00 CET

}