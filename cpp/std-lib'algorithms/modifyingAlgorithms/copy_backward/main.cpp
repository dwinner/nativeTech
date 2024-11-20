#include <iostream>
#include <print>
#include <vector>
#include <algorithm>

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
   vector<int> vec1, vec2;

   populateContainer(vec1);

   vec2.resize(size(vec1));

   copy_backward(cbegin(vec1), cend(vec1), end(vec2));

   for (const auto &item: vec2)
   {
      print("{} ", item);
   }
}