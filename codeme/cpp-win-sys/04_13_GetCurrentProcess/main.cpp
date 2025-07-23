#include <windows.h>
#include <iostream>
using namespace std;

int main()
{
  HANDLE  hProcess;

  // получаем псевдодескриптор текущего процесса
  hProcess = GetCurrentProcess();
  // выводим псевдодескриптор на консоль
  cout << hProcess << endl;

  cin.get();

  return 0;
}