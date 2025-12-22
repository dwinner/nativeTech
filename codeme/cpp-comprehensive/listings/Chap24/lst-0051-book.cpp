
// https://godbolt.org/z/6836bdczd 
std::vector v{ 1, 2, 3, 4, 5, 6 };
for(auto it=v.begin(); it!=v.end(); ++it) {
  it = v.erase(it);
}
// Here v is: { 2, 4, 6 }