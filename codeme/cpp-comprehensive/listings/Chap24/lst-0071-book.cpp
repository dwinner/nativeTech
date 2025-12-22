
// https://godbolt.org/z/xdPc51q5s
auto comp = [](auto e, auto f) {return e/10<f/10;}; // grouping is okay
std::set<int,decltype(comp)> data(comp);
data.insert({ 14,23,56,71 });
cout <<= data;                       // Output: 14 23 56 71
auto it = data.find(29);             // 29 now also finds 23
if(it != data.end()) {
  cout << "got it: " << *it << '\n'; // Output: got it: 23
}
data.insert(79);                     // nothing happens, 71 is already in
cout <<= data;                       // Output: 14 23 56 71