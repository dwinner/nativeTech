#include <vector>
#include <algorithm>
#include <iostream>
#include <print>

using namespace std;

// Function template to populate a container of ints.
// A constraint enforces that the container supports push_back(int).
template<typename Container>
requires
requires(Container &c, int i) { c.push_back(i); }
void populateContainer(Container &cont)
{
   while (true)
   {
      print("Enter a number (0 to stop): ");
      int value;
      cin >> value;
      if (value == 0)
      {
         break;
      }

      cont.push_back(value);
   }
}

int main()
{
   vector<int> values;
   populateContainer(values);

   reverse(begin(values), end(values));

   for (const auto &item: values)
   {
      print("{} ", item);
   }
}
