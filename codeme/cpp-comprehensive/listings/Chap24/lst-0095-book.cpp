
// https://godbolt.org/z/16T5dob75 
map<char,int> data { { 'a',1}, {'b',2}, {'c',3} };
for(auto it=data.rbegin(); it!=data.rend(); ++it) { // backwards
  cout << it->first << ':' << it->second << ' ';    // dereference with ->
}
cout << '\n'; // Output: c:3 b:2 a:1
for(auto &e : data) {                        // forwards, uses begin() and end()
  cout << e.first << ':' << e.second << ' '; // pair, element access with .
}
cout << '\n'; // Output: a:1 b:2 c:3