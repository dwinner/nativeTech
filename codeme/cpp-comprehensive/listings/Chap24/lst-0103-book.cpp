
// https://godbolt.org/z/TbvKTn4Px 
multimap<char,int> vals{ {'c',1}, {'c',8}, {'g',1},
    {'c',1}, {'a',7}, {'a',1}, {'c',2}, };
cout <<= vals;             // Output: a:7 a:1 c:1 c:8 c:1 c:2 g:1
vals.erase( 'c' );         // deletes all 'c's
cout <<= vals;             // Output: a:7 a:1 g:1
vals.erase(vals.begin());  // deletes only one of the 'a's
cout <<= vals;             // Output: a:1 g:1