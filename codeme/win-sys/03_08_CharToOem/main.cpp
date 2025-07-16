#include <windows.h>
#include <iostream>
using namespace std;

int main()
{

  char big[] = "ÀÁÂÃÄÅ¨ÆÇÈÉÊËÌÍÎÏĞÑÒÓÔÕÖ×ØÙÚÛÜİŞß";
  char sml[] = "àáâãäå¸æçèéêëìíîïğñòóôõö÷øùúûüışÿ";

  CharToOem(big,big);
  CharToOem(sml,sml);

  cout << big << endl;
  cout << sml << endl;

  return 0;
}