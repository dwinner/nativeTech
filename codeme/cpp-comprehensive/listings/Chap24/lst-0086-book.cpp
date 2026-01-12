
// https://godbolt.org/z/dh8Yqx6c4
set data{ 10, 20, 30, 40, 50, 60, 70 };
auto lo = data.lower_bound(35);
auto up = data.upper_bound(55);
data.erase(lo, up);       // deletes all numbers between 35 and 55
cout <<= data;            // Output: 10 20 30 60 70
lo = data.lower_bound(20);
up = data.upper_bound(60);
data.erase(lo, up);      // deletes including 60, because up points to 70
cout <<= data;           // Output: 10 70
auto n = data.erase(69); // deletes nothing
cout << "Number of removed elements: "<< n << '\n'; // Output: Number … 0
n = data.erase(70);      // deletes one element
cout << "Number of removed elements: "<< n << '\n';  // Output: Number … 1
cout <<= data;           // Output: 10