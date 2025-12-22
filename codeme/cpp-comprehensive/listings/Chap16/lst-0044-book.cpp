
// https://godbolt.org/z/xhd7s1YhE 
#include <string>
using std::string;
enum class StoplightColor {
    RED, YELLOW, REDYELLOW, GREEN
};
struct Stoplight {
    StoplightColor color_;
    Stoplight(StoplightColor color, string name) : color_{color} {}
};

Stoplight createStoplight(StoplightColor color, string name) {
    return Stoplight{color, name};
}

int main() {
    Stoplight stoplight = createStoplight(StoplightColor::RED, "AX-001");
}