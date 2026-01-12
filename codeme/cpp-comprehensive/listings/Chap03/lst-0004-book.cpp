
// https://godbolt.org/z/Moe5M7sK8 
Data const * data = new Data(5);
data->value = 7;          // this const protects Data 
data = new Data(6);       // reassigning pointer is okay 
Data * const more = new Data(8); 
more->value = 9;          // now okay
more = new Data(10);      // reference is protected