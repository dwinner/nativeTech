//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/8nj7vfeba
// …
template<typename IT> ostream& operator<<(ostream& os,const pair<IT,bool> wh)
  { return os << (wh.second ? "yes" : "no"); }
struct Point {
    double x_, y_;
    Point(double x, double y) : x_{x}, y_{y} {}
    auto operator<=>(const Point&) const = default;
    friend ostream& operator<<(ostream &os, const Point &a) {
        return os << "(" << a.x_ << ',' << a.y_<< ")";
    }
};
int main() {
    set data{ 10, 20, 30, 40, 50, 60, 70 };
    auto wh = data.insert(35);         // inserts between 30 and 40
    cout << "new? " << wh << '\n';     // Output: new? yes
    wh = data.insert(40);              // already exists, so does not insert
    cout << "new? " << wh << '\n';     // Output: new? no
    set<Point> points{};
    points.insert( Point{3.50,7.25} ); // temporary value
    points.emplace(1.25, 2.00);        // constructor arguments
    cout <<= points;                   // Output: (1.25,2) (3.5,7.25)
}