
// https://godbolt.org/z/3zo8GnbW3
int main() {
    int odd[5] = { 1,3,7,9,11 };
    auto &[ one, two, three, four, five ] = odd;
    auto &[ zwei, be, pi ] = mkTpl();           // (ERR) no & binding to temp-values
    Point p0{  10, 15 };
    auto &[ the_x, the_y ] = p0;
}