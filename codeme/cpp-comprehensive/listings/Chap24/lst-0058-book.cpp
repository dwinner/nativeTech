
// https://godbolt.org/z/KrbTrx1aG 
array<int,5> data{ 10, 11, 12, 13, 14};
cout << std::get<2>(data) << '\n'; // 12