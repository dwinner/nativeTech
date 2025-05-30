; windlg.asm
; ����᪮� win32-�ਫ������, �����������饥 ࠡ��� � ��������
;
; ��������� MASM
;  ml /c /coff /Cp windlg.asm
;  rc /r windlg.rc
;  link windlg.obj windlg.res /subsystem:windows
;
; ��������� TASM
;  tasm /m /ml /D_TASM_ windlg.asm
;  brcc32 windlg.rc
;  tlink32 /Tpe /aa /c /x windlg.obj,,,,,windlg.res
;
; ��������� WASM
;  wasm windlg.asm
;  wrc /r /bt=nt windlg.rc
;  wlink file windlg.obj res windlg.res form windows nt op c

; �����䨪���� ����஫�� (������⮢ �������)
IDC_EDIT        equ  0
IDC_BUTTON      equ  1
IDC_EXIT        equ  2
; �����䨪���� ������⮢ ����
IDM_GETTEXT     equ 10
IDM_CLEAR       equ 11
IDM_EXIT        equ 12

include def32.inc
include kernel32.inc
include user32.inc

   .386
   .model flat
   .data
dialog_name    db   "ZZZ_Dialog",0 ; ��� ������� � ������
   .data?
buffer         db   512 dup(?)   ; ���� ��� ����񭭮�� ⥪��
   .code
_start:
   xor   ebx, ebx      ; � EBX �㤥� 0 ��� ������ push 0 (���� � 2 ࠧ�)
; ��।���� �����䨪��� ��襩 �ணࠬ��
   push  ebx
   call  GetModuleHandle
; �����⨬ ������
   push  ebx      ; ���祭��, ���஥ ��३��� ��� ��ࠬ��� WM_INITDIALOG
   push  offset dialog_proc ; ���� ��楤��� ⨯� DialogProc
   push  ebx      ; �����䨪��� ����-�।�� (0 - ��祩 ������)
   push  offset dialog_name ; ���� ����� ������� � ������
   push  eax      ; �����䨪��� �ணࠬ��, � ������ ���ன
            ; ��室���� ������ (��� �����䨪��� � EAX)
   call  DialogBoxParam
; ��室 �� �ணࠬ��
   push  ebx
   call  ExitProcess
;
; ��楤�� dialog_proc
; ��뢠���� �������� ����� ࠧ, ����� � �� ��-����� �ந�室��
; ������ ����� ���� �ந�室��� ��� ࠡ�� �ணࠬ��
;
; ��楤�� �� ������ �������� ॣ����� EBP,EDI,ESI � EBX !
;
dialog_proc   proc   near
; ⠪ ��� �� ����砥� ��ࠬ���� � �⥪�, ����ந� �⥪��� ����
   push  ebp
   mov   ebp, esp
; ��楤�� ⨯� DialogProc ��뢠���� � ᫥���騬� ��ࠬ��ࠬ�
   dp_hWnd     equ dword ptr [ebp+08h]   ; �����䨪��� �������
   dp_uMsg     equ dword ptr [ebp+0Ch]   ; ����� ᮮ�饭��
   dp_wParam   equ dword ptr [ebp+10h]   ; ���� ��ࠬ���
   dp_lParam   equ dword ptr [ebp+14h]   ; ��ன ��ࠬ���
   mov   ecx, dp_hWnd   ; ECX �㤥� �࠭��� �����䨪��� �������
   mov   eax, dp_uMsg   ; � EAX - ����� ᮮ�饭��
   cmp   eax, WM_INITDIALOG   ; �᫨ �� ����稫� WM_INITDIALOG
   jne   not_initdialog
   push  IDC_EDIT
   push  dp_hWnd
   call  GetDlgItem      ; ��।���� �����䨪���
   push  eax         ; ���誠 ।���஢���� ⥪��
   call  SetFocus      ; � ��।���� ��� 䮪��
not_initdialog:
   cmp   eax, WM_CLOSE      ; �᫨ �� ����稫� WM_CLOSE
   jne   not_close
   push  0
   push  ecx
   call  EndDialog      ; ������� ������
not_close:
   cmp   eax, WM_COMMAND   ; �᫨ �� ����稫� WM_COMMAND
   jne   not_command
   mov   eax, dp_wParam   ; EAX = wParam (����� ᮮ�饭��)
   cmp   dp_lParam, 0   ; �᫨ lparam ���� - ᮮ�饭�� �� ����
   jne   lParam_not_0
   cmp   ax, IDM_GETTEXT   ; �᫨ �� �㭪� ���� Get Text
   jne   not_gettext
   push  512         ; ࠧ��� ����
   push  offset buffer      ; ���� ����
   push  IDC_EDIT      ; ����� ���஫� ।���஢����
   push  ecx
   call  GetDlgItemText      ; ��⠥� ⥪�� � buffer
   push  MB_OK
   push  offset dialog_name
   push  offset buffer
   push  dp_hWnd
   call  MessageBox      ; � ������� ��� � MessageBox
not_gettext:
   cmp   eax, IDM_CLEAR   ; �᫨ �� �㭪� ���� Clear
   jne   not_clear
   push  0         ; NULL
   push  IDC_EDIT      ; ����� ����஫�
   push  ecx
   call  SetDlgItemText      ; ��⠭���� ���� ⥪��
not_clear:
   cmp   eax, IDM_EXIT   ; �᫨ �� �㭪� ���� Exit
   jne   not_exit
   push  0         ; ��� ������
   push  ecx         ; �����䨪��� �������
   call  EndDialog      ; ������� ������
lParam_not_0:         ; lParam �� ���� - ᮮ�饭�� �� ����஫�
   cmp   eax, IDC_EXIT   ; �᫨ ᮮ�饭�� �� ������ Exit
   jne   not_exit
   shr   eax, 16
   cmp   eax, BN_CLICKED   ; �᫨ �� ������
   push  0         ; ��� ������
   push  ecx         ; �����䨪��� �������
   call  EndDialog      ; ������� ������
not_exit:
   xor   eax, eax      ; ��᫥ ��ࠡ�⪨ �������
   inc   eax      ; DialogProc ������ �������� TRUE (eax=1)
   leave
   ret   16      ; ����� ��楤���
not_command:      ; � ��।����� �ࠢ����� �᫨ �� ����稫�
         ; �����-� ���������� ᮮ�饭��
   xor   eax, eax   ; ��� ������ FALSE (eax=0)
   leave
   ret   16   ; ����� ��楤���
dialog_proc endp
end _start
