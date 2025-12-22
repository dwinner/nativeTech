
// https://godbolt.org/z/rTvovhr5M 
#include <vector>
set data{ 10, 20, 30, };
data.insert( { 40, 50, 60, 70 }); // Initializer list
std::vector new_vec{ 5, 25, 35, 15, 25, 75, 95 };
data.insert( new_vec.cbegin()+1, new_vec.cend()-1 ); // area
cout <<= data; // Output: 10 15 20 25 30 35 40 50 60 70 75