
// https://godbolt.org/z/ohd5Pr8fb 
#include <iostream>
int main() {
    int idx = 4;
    goto more;                      // jump to label more
  print:                            // label for the next statement
    std::cout << idx << std::endl;
    idx = idx * 2;
  more:
    idx = idx + 3;
    if(idx < 20)
        goto print;                 // goto can also be used in an if statement
  end:                              // this is a label, but it is not used
    return 0;
}