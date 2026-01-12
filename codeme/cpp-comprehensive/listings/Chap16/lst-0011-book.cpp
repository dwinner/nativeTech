
#include "my_string.hpp"
static const my_string ZEBRA { "zebra" };
int main() {
    my_string animal{ "horse" };
    if(ZEBRA.equals(animal)) return 0;
    else return 1;
}