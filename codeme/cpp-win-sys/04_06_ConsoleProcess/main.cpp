#include <windows.h>
#include <iostream>
using namespace std;

int count;

void main()
{
  for ( ; ; )
  {
    ::count++;
    Sleep(1000);
    cout << "count = " << ::count << endl;
  }
}