
// https://godbolt.org/z/Ee7eqq1MY 
Data* dataOwner = new Data(5); 
Data* data = dataOwner; 
Data* more = new Data(6); 
data = more;
more->value = 7; 
cout << data->value << '\n'; // now also 7 
/* tidy up */ 
delete more; 
delete dataOwner;