
// https://godbolt.org/z/c8a9qPoPP
#include <set>
#include <functional> // function
using std::set; using std::function; using std::initializer_list;
bool fcompTens(int a, int b) { return a%10 < b%10; }
struct Fives {
    bool operator()(int a, int b) const { return a%5 < b% 5; }
};

int main() {
    // Functor
    set<int, Fives> ff1;
    ff1.insert(5);
    set ff2({5}, Fives{}); 
    set ff3(initializer_list<int>({}), Fives{});
    // Lambda
    set<int,function<bool(int,int)>> ll1([](auto a,auto b){return a%3<b%3;});
    ll1.insert(3);
    auto lcomp = [](int a, int b) { return a%3 < b%3; };
    set<int, decltype(lcomp)> ll2(lcomp);
    ll2.insert(3);
    set ll3({3}, lcomp); 
    // Function pointer
    set<int, bool(*)(int,int)> zz1(&fcompTens);        // C-style
    zz1.insert(10);
    set<int, function<bool(int,int)>> zz2(&fcompTens); // C++ style
    zz2.insert(10);
    set<int, decltype(&fcompTens)> zz3(&fcompTens);    // C++ style
    zz3.insert(10);
}