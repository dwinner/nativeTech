#include <vector>
#include <numeric>
#include <print>

using namespace std;

int main()
{
   vector v1{1, 2, 3, 4};
   vector v2{9, 8, 7, 6};
   auto innerProduct = inner_product(cbegin(v1), cend(v1), cbegin(v2), 0);
   println("{}", innerProduct);
}