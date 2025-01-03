#include <print>
#include <map>
#include <algorithm>

using namespace std;

int main()
{
   map<int, int> myMap
       {
           {4, 40},
           {5, 50},
           {6, 60}
       };
   for_each(cbegin(myMap), cend(myMap), [](const auto &pairItem)
   {
      println("{} -> {}", pairItem.first, pairItem.second);
   });
}
