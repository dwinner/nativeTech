
// https://godbolt.org/z/5zrfvovaW 
#include <string>
#include <cmath> // sin, cos
#define MAX2(a,b) ((a) > (b) ? (a) : (b))
int main() {
    double e = MAX2(sin(3.141592/2), cos(3.141592/2));
    int i = MAX2(10+12+45, 100/5+20);
    std::string s = MAX2(std::string("Ernie"), std::string("Bert"));
}