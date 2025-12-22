
// https://godbolt.org/z/rrq1v5qPs 
std::list lst{1, 2, 3, 4, 5, 6, 7, 8, 9};
auto take5 = rs::take_view{lst, 5};     // View via type
auto take6 = lst | vs::take(6);         // View via adapter