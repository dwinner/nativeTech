
// https://godbolt.org/z/EjT7nPPv7 
std::vector v{0,1,3,5,7,9,2,4,6,8};
std::sort(v.begin(), v.end());
for(int i:v) std::cout << i << ' ';
std::cout << '\n';
// or with a range:
std::ranges::sort(v);
for(int i:v) std::cout << i << ' ';
std::cout << '\n';