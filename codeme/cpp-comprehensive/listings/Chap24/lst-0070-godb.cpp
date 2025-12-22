//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/jfdh3ddsz
using std::cout; using std::ostream; using std::set;
auto comp = [](auto e, auto f) { return e <= f; }; // (ERR) defined wrong!
std::set<int,decltype(comp)> data(comp);
data.insert({ 9,5,7,2,3,6,8 });
cout <<= data;                    // Output with luck: 2 3 5 6 7 8
auto it = data.find(7);
if(it != data.end()) {
    cout << "got it: " << *it << '\n';
} else {
    cout << "it's gone!" << '\n'; // you will probably end up here
}