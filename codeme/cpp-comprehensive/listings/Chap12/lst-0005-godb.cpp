//#(compile) c++; compiler:g141; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/K8WePajvM 
Person john1 {"John", 45, "Boston" }; // correct
Person john2 {"Boston", 45, "John" }; // oops, swapped, and no one notices
Person jack { .name_ = "Jack", .age_ = 23, .city_ = "Dallas" }; // okay
Person jimmi { .name_ = "Jimmi", .age_ = 48  }; // okay, not all specified
Person carl { "Carl", .age_ = 53  };  // (ERR) all designated or none
Person paul { .age = 34, .name_ = "Paul", .city = "Reno" }; // (ERR) swapped
Person jim(.name_="Jim", .age_=34, .city_="NYC"); // (ERR) not parenthesis