; winmenu.asm
; ����᪮� win32-�ਫ������, �����������饥 ࠡ��� � ����
; ��񧤮窠�� �⬥祭� ��ப�, ᪮��஢���� �� 䠩�� window.asm
;
; ��������� MASM
;  ml /c /coff /Cp winmenu.asm
;  rc /r winmenu.rc
;  link winmenu.obj winmenu.res /subsystem:windows
;
; ��������� TASM
;  tasm /m /ml /D_TASM_ winmenu.asm
;  brcc32 winmenu.rc
;  tlink32 /Tpe /aa /c /x winmenu.obj,,,,,winmenu.res
;
; ��������� WASM
;  wasm winmenu.rc
;  wrc /r /bt=nt winmenu.rc
;  wlink file winmenu.obj res winmenu.res form windows nt
;


ZZZ_TEST    equ 0      ; ᮮ�饭�� �� ��襣� ����
ZZZ_OPEN    equ 1      ; ������ ᮢ������ � ��।�����ﬨ �� winmenu.rc
ZZZ_SAVE    equ 2      ; �஬� ⮣� � ��襬 �ਬ�� �� ����� �����
ZZZ_EXIT    equ 3      ; ��⮬� �� ��� �ᯮ������� ��� ������ ��� ⠡����
         ; ���室�� � ��ࠡ��稪��

include def32.inc            ;*
include kernel32.inc            ;*
include user32.inc            ;*
   .386               ;*
   .model flat            ;*
   .data               ;*
class_name  db 'window class 1',0      ;*
window_name db 'win32 assembly example',0   ;*
menu_name   db 'ZZZ_Menu',0         ; ��� ���� � 䠩�� ����ᮢ
test_msg    db 'You selected menu item TEST',0 ; ��ப� ���
open_msg    db 'You selected menu item OPEN',0 ; ��������樨 ࠡ���
save_msg    db 'You selected menu item SAVE',0 ; ����
wc   WNDCLASSEX <4*12,CS_HREDRAW or CS_VREDRAW,offset win_proc,0,0,?,?,?,COLOR_WINDOW+1,0,offset class_name,0> ;*
   .data?               ;*
msg_   MSG   <?,?,?,?,?,?>         ;*
   .code               ;*
_start:                  ;*
   xor   ebx, ebx            ;*
   push  ebx      ;*
   call  GetModuleHandle   ;*
   mov   esi, eax      ;*
   mov   dword ptr wc.hInstance, eax ;*
   push  IDI_APPLICATION   ;*
   push  ebx      ;*
   call  LoadIcon   ;*
   mov   wc.hIcon,eax   ;*
   push  IDC_ARROW   ;*
   push  ebx      ;*
   call  LoadCursor   ;*
   mov   wc.hCursor,eax   ;*
   push  offset wc   ;*
   call  RegisterClassEx   ;*

   push  offset menu_name   ; ��� ����
   push  esi         ; ��� �����䨪���
   call  LoadMenu      ; ����㧨� ���� �� ����ᮢ
   mov   ecx, CW_USEDEFAULT   ;*
   push  ebx         ;*
   push  esi         ;*
   push  eax      ; �����䨪��� ���� ��� ����-��⮬��
   push  ebx         ;*
   push  ecx         ;*
   push  ecx         ;*
   push  ecx         ;*
   push  ecx         ;*
   push  WS_OVERLAPPEDWINDOW   ;*
   push  offset window_name   ;*
   push  offset class_name   ;*
   push  ebx         ;*
   call  CreateWindowEx      ;*
   push  eax         ;*
   push  SW_SHOWNORMAL      ;*
   push  eax         ;*
   call  ShowWindow      ;*
   call  UpdateWindow      ;*
   mov   edi, offset msg_    ;*
message_loop:            ;*
   push  ebx         ;*
   push  ebx         ;*
   push  ebx         ;*
   push  edi         ;*
   call  GetMessage      ;*
   test  eax, eax         ;*
   jz    exit_msg_loop      ;*
   push  edi         ;*
   call  TranslateMessage   ;*
   push  edi         ;*
   call  DispatchMessage      ;*
   jmp   short message_loop      ;*
exit_msg_loop:            ;*
   push  ebx         ;*
   call  ExitProcess      ;*

; ��楤�� win_proc
; ��뢠���� ����� ����� ࠧ, ����� ���� ����砥� �����-����� ᮮ�饭��
; ������ ����� ���� �ந�室��� ��� ࠡ�� �ணࠬ��
;
; ��楤�� �� ������ �������� ॣ����� EBP,EDI,ESI � EBX !
win_proc proc                  ;*
   push  ebp               ;*
   mov   ebp, esp               ;*
   wp_hWnd     equ dword ptr [ebp+08h]      ;*
   wp_uMsg     equ dword ptr [ebp+0Ch]      ;*
   wp_wParam   equ dword ptr [ebp+10h]   ;*
   wp_lParam   equ dword ptr [ebp+14h]   ;*
   cmp   wp_uMsg, WM_DESTROY         ;*
   jne   not_wm_destroy            ;*
   push  0               ;*
   call  PostQuitMessage            ;*
   jmp   short end_wm_check            ;*
not_wm_destroy:                  ;*
   cmp   wp_uMsg, WM_COMMAND   ; �᫨ �� ����稫� WM_COMMAND
   jne   not_wm_command      ; �� �� ��襣� ����
   mov   eax, wp_wParam      ; � � wParam ����� ��� ���-ᮮ�饭��
   jmp   dword ptr menu_handlers[eax*4] ; ��ᢥ��� ���室
   ; (� 32-��⭮� ०��� ����� ������ ���室 �� ��� ॣ�����)

menu_handlers   
   dd   offset menu_test,offset menu_open
   dd   offset menu_save,offset menu_exit
; ��ࠡ��稪� ᮡ�⨩ test, open � save �뢮��� MessageBox
; ��ࠡ��稪 exit ��室�� �� �ணࠬ��
menu_test:
   mov   eax, offset test_msg   ; ᮮ�饭�� ��� MessageBox
   jmp   short show_msg
menu_open:
   mov   eax, offset open_msg   ; ᮮ�饭� ��� MessageBox
   jmp   short show_msg
menu_save:
   mov   eax, offset save_msg   ; ᮮ�饭�� ��� MessageBox
show_msg:
   push  MB_OK         ; �⨫� ��� MessageBox
   push  offset menu_name   ; ���������
   push  eax         ; ᮮ�饭��
   push  wp_hWnd         ; �����䨪��� ����-�।��
   call  MessageBox      ; �맮� �㭪樨
   jmp   short end_wm_check      ; ��室 �� win_proc
menu_exit:      ; �᫨ ��ࠫ� �㭪� EXIT
   push  wp_hWnd
   call  DestroyWindow   ; 㭨�⮦�� ��� ����

end_wm_check:   ; ����� �� �����﫨 ���⠬� end_wm_check �
   leave         ;*
   xor   eax, eax   ; ���� 0 ��� १���� ࠡ��� ��楤���
   ret   16         ;*
not_wm_command:   ; not_wm_command, �⮡� ���������� �� ��譥�� jmp
   leave         ;*
   jmp   DefWindowProc   ;*
win_proc endp         ;*
end _start      ;*
