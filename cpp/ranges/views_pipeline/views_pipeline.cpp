import std;

using namespace std;

void printRange(string_view message, auto&& range)
{
   for (const auto& item : range)
   {
      print("{} ", item);
   }

   println("{}", message);
}

int main()
{
   vector values{ 1, 2, 3, 4, 5, 6, 7, 8, 9, 10 };
   printRange("Original sequence: ", values);

   auto result{ values
      | views::filter([](const auto& value) { return value % 2 == 0; })
      | views::transform([](const auto& value) { return value * 2.0; })
      | views::drop(2)
      | views::reverse };
   printRange("Final sequence: ", result);
}
