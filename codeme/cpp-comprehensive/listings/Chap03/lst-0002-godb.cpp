//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/WM11Gs6sz 
auto data = make_shared<Data>(5); 
auto more = make_shared<Data>(6); 
data = more;
more->value = 7; 
cout << data->value << '\n'; // now also 7