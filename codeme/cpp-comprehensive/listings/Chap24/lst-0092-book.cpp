
// https://godbolt.org/z/Y1ebrxb8T
map<int,string> zip2plc;
zip2plc.insert(std::make_pair(94103, "San Francisco"));
zip2plc.emplace(10001, "New York");
cout <<= zip2plc;    // Output: 10001:New York 94103:San Francisco
map<string,int> plc2zip;
plc2zip.emplace("New York", 10001);
plc2zip.emplace("New York", 10002);  // does not overwrite
cout <<= plc2zip;                    // Output: New York:10001