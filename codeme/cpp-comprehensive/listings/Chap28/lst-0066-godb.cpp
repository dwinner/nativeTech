//#(compile) c++; compiler:g132; options:-O3 -std=c++23; libs:-
// https://godbolt.org/z/o359TThMj 
#include <iostream>
#include <string>
#include <chrono>
#include <complex>
using std::cout;
int main() {
    { using namespace std;
      cout << "string"s << "\n";           // string
      cout << (1.2+3.4i) << "\n";          // complex
      }
    { using namespace std::chrono;
      cout << (35ms).count() << "ms\n";    // chrono
      }
    { using namespace std::literals;
      cout << (41s).count() << "ms\n";     // chrono seconds
      cout << "text"s << "\n";             // string
    }
    { using namespace std::chrono; 
      cout << (4h).count() << "h\n";       // chrono hours
    }
    { using namespace std::literals::chrono_literals; 
      cout << (16min).count() << "min\n";  // chrono minutes
    }
    { using std::literals::string_literals::operator""s; 
      cout << "letters"s << "\n";          // string
    }
}