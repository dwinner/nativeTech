
// https://godbolt.org/z/Yb4jEdaKj 
vector<int> from{ 2,3,4 };
vector<int> to{};
to = from;                  // Assignment with operator=, now both are the same

vector<int> drain{};
sink = std::move(from);           // Move, now 'from' is empty
vector<int> v;
v.assign(4, 100);                 // v is now {100, 100, 100, 100}
v.assign(to.begin(), to.end());   // v is now {2,3,4}
int z[] = { 10, 20, 30, 40 };
v.assign(z+1, z+4);               // v is now {20, 30, 40}