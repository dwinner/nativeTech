
#include <functional> // function
int calculate(int a, int b, std::function<int(int,int)> binop) {
    return binop(a,b);
}