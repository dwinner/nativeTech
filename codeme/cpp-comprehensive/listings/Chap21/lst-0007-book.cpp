
// https://godbolt.org/z/W45G5TjTz 
// …here content of <vector>…
// …here content of <iostream>…
using container_type = std::vector<int>;
static constexpr unsigned SIZE = 10;
int main() {
  { (std::cout) << "Program start" << "\n"; }
  container_type data(SIZE);
  { (std::cout) << "The container has " << data.size() << " elements." << "\n";}
  { (std::cout) << "End of program" << "\n"; }
  { std::cout << "That was a close call.\n"; }
}