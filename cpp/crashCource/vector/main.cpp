#include <iostream>
#include <vector>
#include <format>

using namespace std;

int main()
{
   // Create a vector of integers
   vector<int> myVector{11, 22};
   // vector myVector { 11, 22 };  // Using CTAD

   // Add some more integers to the vector using push_back()
   myVector.push_back(33);
   myVector.push_back(44);

   // Access elements
   cout << std::format("1st element: {}", myVector[0]) << endl;
   //cout << std::format("invalid element: {}", myVector[50]) << endl;

   return 0;
}
