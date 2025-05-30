; window.asm
; ����᪮� win32-�ਫ������, �����������饥 ������ �뢮� ����
;
; ��������� MASM
;  ml /c /coff /Cp window.asm
;  link window.obj /subsystem:windows
;
; ��������� TASM
;  tasm /m /ml /D_TASM_ window.asm
;  tlink32 /Tpe /aa /c /x window.obj
;
; ��������� WASM
;  wasm window.asm
;  wlink form window.obj form windows nt
;
include def32.inc
include kernel32.inc
include user32.inc
   .386
   .model flat
   .data
class_name     db 'window class 1',0
window_name    db 'win32 assembly example',0
; �������, ����뢠��� ����� ����.
wc   WNDCLASSEX <4*12,CS_HREDRAW or CS_VREDRAW,offset win_proc,0,0,?,?,?,COLOR_WINDOW+1,0,offset class_name,0>
   ; ����� ��室���� ᫥���騥 ����
   ; wc.cbSize = 4*12 - ࠧ��� �⮩ ��������
   ; wc.style - �⨫� ���� (����ᮢ뢠�� �� ��������� ࠧ���)
   ; wc.lpfnWndProc - ��ࠡ��稪 ᮡ�⨩ ���� (win_proc)
   ; wc.cbClsExtra - �᫮ �������⥫��� ���⮢ ��᫥ �������� (0)
   ; wc.cbWndExtra - �᫮ �������⥫��� ���⮢ ��᫥ ���� (0)
   ; wc.hInstance - �����䨪��� ��襣� ����� (?)
   ; wc.hIcon - �����䨪��� ������ (?)
   ; wc.hCursor - �����䨪��� ����� (?)
   ; wc.hbrBackground - �����䨪��� ���� ��� 梥� 䮭�+1 (COLOR_WINDOW+1)
   ; wc.lpszMenuName - ����� � �᭮��� ���� (� �⮬ �ਬ�� - 0)
   ; wc.lpszClassName - ��� ����� (��ப� class_name)
   ; wc.hIconSm - �����䨪��� �����쪮� ������ (⮫쪮 � windows 95, ��� NT ������ ���� 0)

   .data?
msg_   MSG   <?,?,?,?,?,?>   ; � �� - �������, � ���ன �����頥���
            ; ᮮ�饭�� ��᫥ GetMessage
   .code
_start:
   xor   ebx, ebx      ; � EBX �㤥� 0 ��� ������ push 0 (���� � 2 ࠧ�)
; ��।���� �����䨪��� ��襩 �ணࠬ��
   push  ebx
   call  GetModuleHandle
   mov   esi, eax      ; � ��࠭�� ��� � ESI
; �������� � ��ॣ�����㥬 �����
   mov   dword ptr wc.hInstance, eax ; �����䨪��� �।��
  ; �롥६ ������
   push  IDI_APPLICATION   ; �⠭���⭠� ������ �ਫ������
   push  ebx      ; �����䨪��� ����� � �������
   call  LoadIcon
   mov   wc.hIcon, eax   ; �����䨪��� ������ ��� ��襣� �����
  ; �롥६ ��� �����
   push  IDC_ARROW   ; �⠭���⭠� ��५��
   push  ebx      ; �����䨪��� ����� � ����஬
   call  LoadCursor
   mov   wc.hCursor, eax   ; �����䨪��� ����� ��� ��襣� �����
   push  offset wc
   call  RegisterClassEx   ; ��ॣ�����㥬 �����
; ᮧ����� ����
   mov   ecx,CW_USEDEFAULT   ; push ecx ���� push N � ���� ࠧ
   push  ebx      ; ���� �������� CREATESTRUCT (����� NULL)
   push  esi      ; �����䨪��� �����, ����� �㤥� �������
            ; ᮮ�饭�� �� ���� (� ����, ���)
   push  ebx      ; �����䨪��� ���� ��� ����-��⮬��
   push  ebx      ; �����䨪��� ����-�।��
   push  ecx      ; ���� (CW_USEDEFAULT - �� 㬮�砭��)
   push  ecx      ; �ਭ� (�� 㬮�砭��)
   push  ecx      ; y-���न��� (�� 㬮�砭��)
   push  ecx      ; x-���न��� (�� 㬮�砭��)
   push  WS_OVERLAPPEDWINDOW   ; �⨫� ����
   push  offset window_name   ; ��������� ����
   push  offset class_name   ; �� ��ॣ����஢���� �����
   push  ebx      ; �������⥫�� �⨫�
   call  CreateWindowEx   ; ᮧ���� ���� (eax - �����䨪��� ����)
   push  eax      ; �����䨪��� ��� UpdateWindow
   push  SW_SHOWNORMAL   ; ⨯ ������ ��� ��� ShowWindow
   push  eax      ; �����䨪��� ��� ShowWindow
; ����� �����䨪��� ���� ��� �� ���ॡ����
   call  ShowWindow   ; �������� ����
   call  UpdateWindow   ; � ��᫠�� ��� ᮮ�饭�� WM_PAINT

; �᭮���� 横� - �஢�ઠ ᮮ�饭�� �� ���� � ��室 �� WM_QUIT
   mov   edi, offset msg_ ; push edi ���� push N � 5 ࠧ
message_loop:
   push  ebx      ; ��᫥���� ᮮ�饭��
   push  ebx      ; ��ࢮ� ᮮ�饭��
   push  ebx      ; �����䨪��� ���� (0 - �� ��� ����)
   push  edi      ; ���� �������� MSG
   call  GetMessage   ; ������� ᮮ�饭�� �� ���� � ���������
            ; - �� ���뢠�� �ᯮ�짮���� PeekMessage
            ; �᫨ �㦭� � �⮬ 横�� ��-� �믮�����
   test  eax, eax      ; �᫨ ����祭� WM_QUIT
   jz    exit_msg_loop   ; ���
   push  edi      ; ���� - �८�ࠧ����� ᮮ�饭�� ⨯�
   call  TranslateMessage ; WM_KEYUP � ᮮ�饭�� ⨯� WM_CHAR
   push  edi
   call  DispatchMessage   ; � ��᫠�� �� ��楤�� ���� (���� ��� ����
            ; ����� �㤥� �������)
   jmp   short message_loop   ; �த������ 横� 
exit_msg_loop:
; ��室 �� �ணࠬ��
   push  ebx
   call  ExitProcess

;
; ��楤�� win_proc
; ��뢠���� ����� ����� ࠧ, ����� ���� ����砥� �����-����� ᮮ�饭��
; ������ ����� ���� �ந�室��� ��� ࠡ�� �ணࠬ��
;
; ��楤�� �� ������ �������� ॣ����� EBP,EDI,ESI � EBX !
;
win_proc proc
; ⠪ ��� �� ����砥� ��ࠬ���� � �⥪�, ����ந� �⥪��� ����
   push  ebp
   mov   ebp, esp
; ��楤�� ⨯� WindowProc ��뢠���� � ᫥���騬� ��ࠬ��ࠬ�
      wp_hWnd     equ dword ptr [ebp+08h]   ; �����䨪��� ����
      wp_uMsg     equ dword ptr [ebp+0Ch]   ; ����� ᮮ�饭��
      wp_wParam   equ dword ptr [ebp+10h]   ; ���� ��ࠬ���
      wp_lParam   equ dword ptr [ebp+14h]   ; ��ன ��ࠬ���
; �᫨ �� ����稫� ᮮ�饭�� WM_DESTROY (��� ����砥� �� ���� 㦥 㤠����
; � ��࠭�, ����� alt-F4 ��� ������ � ���孥� �ࠢ�� 㣫�)
; � ���� �᭮���� �ணࠬ�� ᮮ�饭�� WM_QUIT
   cmp   wp_uMsg,WM_DESTROY
   jne   not_wm_destroy
   push  0      ; ��� ��室�
   call  PostQuitMessage   ; ��᫠�� WM_QUIT
   jmp   short end_wm_check   ; � ��� �� ��楤���
not_wm_destroy:
; �᫨ �� ����稫� ��㣮� ᮮ�饭�� - �맮��� ��� ��ࠡ��稪 �� 㬮�砭��
   leave      ; ����⠭���� ebp
   jmp   DefWindowProc   ; � �맮��� DefWindowProc � ��訬� ��ࠬ��ࠬ�
            ; � ���ᮬ ������ � �⥪�
end_wm_check:
   leave      ; ����⠭���� ebp
   ret 16      ; � ������ ᠬ�, ���⨢ �⥪ �� ��ࠬ��஢
win_proc endp
end _start
