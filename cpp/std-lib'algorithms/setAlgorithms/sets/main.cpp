#include <print>
#include <iostream>
#include <vector>
#include <algorithm>

using namespace std;

template<typename T>
void printVec(const vector<T> &vector)
{
   for (const auto &item: vector)
   {
      print("'{}' ", item);
   }

   println();
}

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

template<forward_iterator Iterator>
void dumpRange(string_view message, Iterator begin, Iterator end)
{
   println("{}", message);
   for (const auto &item: ranges::subrange(begin, end))
   {
      print("'{}' ", item);
   }
}

int main()
{
   vector<int> vec1, vec2, result;
   println("Enter elements for set 1:");
   populateContainer(vec1);
   println("Enter elements for set 2:");
   populateContainer(vec2);

   // set algorithms require sorted ranges
   sort(begin(vec1), end(vec1));
   sort(begin(vec2), end(vec2));

   println("Set 1:");
   printVec(vec1);
   println("Set 2:");
   printVec(vec2);

   if (includes(cbegin(vec1), cend(vec1), cbegin(vec2), cend(vec2)))
   {
      println("The second set is a subset of the first.");
   }

   if (includes(cbegin(vec2), cend(vec2), cbegin(vec1), cend(vec1)))
   {
      println("The first set is a subset of the second");
   }

   result.resize(size(vec1) + size(vec2));
   auto newEnd{
       set_union(cbegin(vec1), cend(vec1), cbegin(vec2),
                 cend(vec2), begin(result))
   };
   dumpRange("The union is: ", begin(result), newEnd);

   newEnd = set_intersection(cbegin(vec1), cend(vec1), cbegin(vec2),
                             cend(vec2), begin(result));
   dumpRange("The intersection is: ", begin(result), newEnd);

   newEnd = set_difference(cbegin(vec1), cend(vec1), cbegin(vec2),
                           cend(vec2), begin(result));
   dumpRange("The difference between set 1 and 2 is: ", begin(result), newEnd);

   newEnd = set_symmetric_difference(cbegin(vec1), cend(vec1),
                                     cbegin(vec2), cend(vec2), begin(result));
   dumpRange("The symmetric difference is: ", begin(result), newEnd);
}
