
// https://godbolt.org/z/j6EarszMT
// intentionally array& as arguments
void all(const array<int,4> &a, const array<int,4> &b) { 
     cout << "{"<<a<<"} compared with {"<<b<<"} is "
       << (a < b ? "<, " : "")
       << (a <= b ? "<=, " : "")
       << (a > b ? ">, " : "")
       << (a >= b ? ">=, " : "")
       << (a == b ? "==, " : "")
       << (a != b ? "!=, " : "")
       << '\n';
}
int main() {
    array a{10,10,10,10};
    array b{20, 5, 5, 5};
    array c{10,10,5,21};
    array d{10,10,10,10};
    cout << (a < b ? "smaller\n" : "not smaller\n"); // "smaller", because 10 < 20
    cout << (a < c ? "smaller\n" : "not smaller\n"); // "not smaller", because not 10 < 5
    cout << (a == d ? "equal\n" : "not equal\n");    // "equal", because all are 10
    for(auto &x : {a,b,c}) {
        for(auto &y : {a,b,c}) {
            all(x,y);
        }
    }
}