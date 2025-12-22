
int func();                        // returns an int
std::string func();                // a string from the standard library
void func();                       // no return value
std::pair<int,std::string> func(); // composite type from the stdlib
vector<int> func();                // returns a new container
vector<int>& func();               // reference to some container
const vector<int>& func();         // same, but you cannot change it