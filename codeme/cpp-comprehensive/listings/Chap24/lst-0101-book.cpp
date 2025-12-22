
// https://godbolt.org/z/6no9dW64d 
multimap int2int{ std::make_pair(3,4) };  // multimap<int,int>
using namespace std::literals; // for ""s
multimap<int,string> numlang{
    {7,"seven"s}, {6,"six"s},
    {7,"siete"s}, {6,"sechs"s},
    {7,"seven"s}, {7,"yedi"s},
    {8,"eight"s} };
cout <<= numlang; // Output: 6:six 6:sechs 7:seven 7:siete 7:seven 7:yedi 8:eight