
// https://godbolt.org/z/8948besY8
map<string,string> data { {"John","Wayne"}, {"Cary","Grant" }, };
cout <<= data;                         // Cary:Grant John:Wayne
data.rbegin()->second = "Travolta";    // Overwrite target
cout <<= data;                         // Cary:Grant John:Travolta