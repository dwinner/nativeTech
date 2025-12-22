//#(compile) c++; compiler:g141; options:-O3 -std=c++23 -DNDEBUG; libs:-
// https://godbolt.org/z/39v4aqqd1 
// Fill with 100 values
unordered_set<int> d{};
d.rehash(10);              // try to have 10 buckets
d.max_load_factor(100.0);  // 100 elements per bucket are okay
cout << "Bucket count: " << d.bucket_count() << '\n';
for(int x : std::ranges::iota_view{0,100}) { // C++20 iota(): 0,1,2,…,99
    d.insert(x);
}
// output
for(int b = d.bucket_count()-1; b>=0; --b) {
    cout << "Bucket "<<b<<":";
    for(auto it=d.begin(b); it!=d.end(b); ++it)
        cout << *it << ' ';
    cout << '\n';
}