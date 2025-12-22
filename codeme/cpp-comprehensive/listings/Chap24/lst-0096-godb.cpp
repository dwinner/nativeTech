//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/Eqsjex4Ta
string search7(const map<int,string> &data) {
    return data[7];               // (ERR) non-const method on const parameter
}
string search5(const map<int,string> &data) {
    auto it = data.find(5);       // not automatically inserting
    return it==data.end() ? string{} : it->second;
}
// …
map<int,string> dwarfs{ {1,"one"}, {3,"three"}, {5,"five"}, {7,"seven"} };
cout << search7(dwarfs) << '\n';
cout << search5(dwarfs) << '\n';  // Output: five