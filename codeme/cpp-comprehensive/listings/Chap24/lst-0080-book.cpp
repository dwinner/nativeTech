
// https://godbolt.org/z/qzaj79bor 
set data{ 10, 20, 30, 40, 50, 60, 70 };
set<int> target;
auto hint = target.begin();
for(auto &e : data) {
    hint =                      // Use insertion position in the next round
        target.insert(hint, e); // Hint helps because data is sorted
}