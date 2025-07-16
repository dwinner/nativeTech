#include <windows.h>
#include <iostream>
using namespace std;

HANDLE hInEvent;
CHAR lpEventName[]="InEventName";

int main()
{
  char c;

  hInEvent = OpenEvent(EVENT_MODIFY_STATE, FALSE, lpEventName);
  if (hInEvent == NULL)
  {
    cout << "Open event failed." << endl;
    cout << "Input any char to exit." << endl;
    cin.get();

    return GetLastError();
  }

  cout << "Input any char: ";
  cin >> c;
  // устанавливаем событие о вводе символа  
  SetEvent(hInEvent);
  // закрываем дескриптор события в текущем процессе
  CloseHandle(hInEvent);

  cin.get();
  cout << "Press any key to exit." << endl;
  cin.get();

  return 0;
}