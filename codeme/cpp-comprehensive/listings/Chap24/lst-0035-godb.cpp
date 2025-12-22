//#(compile) c++; compiler:g141; options:-O1 -std=c++23; libs:-
// https://godbolt.org/z/s9P1Y6jhv
vector<string> cars{ "Diesel", "Petrol", "Super", "Gas" };
cout << cars[1] << '\n';                   // Output: Petrol
cars.insert(cars.begin(), "Electricity");  // shifts everything back by one
cout << cars[1] << '\n';                   // Output: Diesel