
// https://godbolt.org/z/js3MeoccM
unordered_set<int> data;
auto res1 = data.insert( 5 );                    // Insertion by copy
if(res1.second) cout << "yes, 5 now inside\n";   // that works
auto res2 = data.emplace( 5 );                   // In-place insertion
if(res2.second) cout << "second 5 now inside\n"; // that doesn't work
auto res3 = data.insert(res1.first, 6 );         // with position hint
// res3 is just an iterator without bool
cout << *res3 << '\n';                           // definitely a 6