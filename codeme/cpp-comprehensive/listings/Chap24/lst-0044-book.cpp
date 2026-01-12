
// https://godbolt.org/z/a3qqr1YPM 
int car[5]  {1,2,3,4,5};
span span_1 = car;           // directly from a C-array
array arr   {1,2,3,4,5};
span span_2 {arr};           // directly from a std::array
vector vec  {1,2,3,4,5};
span<int,3> span_3 {vec};    // with 'Extent' from a std::vector