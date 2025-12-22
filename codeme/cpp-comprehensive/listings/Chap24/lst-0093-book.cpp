
// https://godbolt.org/z/7h3vsWvYG 
map<string,int> dwarves;
dwarves.emplace("Fili",  2859);
cout << dwarves["Fili"] << '\n'; // Output: 2859
cout << dwarves["Dori"] << '\n'; // newly created. Output: 0
dwarves["Kili"] = 2846;          // newly created and immediately overwritten
cout << dwarves["Kili"] << '\n'; // Output: 2846
cout <<= dwarves;                // Output: Dori:0 Fili:2859 Kili:2846