
// https://godbolt.org/z/hee331WGe 
unordered_set nums{ 1,2,3,4,5,6,7,8,9,10 };
cout <<= nums;               // Output similar to: 9 1 2 3 4 5 6 7 8 10
auto it = nums.begin();
while(it != nums.end()) {
    if(*it % 2 == 0) {       // even number?
        it = nums.erase(it); // Remaining elements do not change order
    } else {
        ++it;
    }
}
cout <<= nums;                // Output similar to: 9 1 3 5 7