; winpad95.asm
; ����᪮� win32-�ਫ������ - ⥪�⮢� ।����
;
; ��������� MASM
;  ml /c /coff /Cp winpad95.asm
;  rc /r winpad95.rc
;  link winpad95.obj winpad95.res /subsystem:windows
;
; ��������� TASM
;  tasm /m /ml /D_TASM_ winpad95.asm
;  brcc32 winpad95.rc
;  tlink32 /Tpe /aa /c /x winpad95.obj,,,,,winpad95.res
;
; ��������� WASM
;  wasm winpad95.rc
;  wrc /r /bt=nt winpad95.rc
;  wlink file winpad95.obj res winpad95.res form windows nt
;
include def32.inc
include user32.inc
include kernel32.inc
include comdlg32.inc

ID_MENU     equ   700h
ID_ACCEL    equ   701h
ID_ABOUT    equ   702h

MAXSIZE     equ   260      ; ���ᨬ��쭮� ��� 䠩��
MEMSIZE     equ   65535    ; ���ᨬ���� ࠧ��� �६������ ���� � �����

EditID      equ   1

   .386
   .model flat
   .const
c_w_name       db    'Asmpad95',0   ; �� � ��� ����� � ��� �᭮����� ����
edit_class     db    'edit',0   ; �।��।��񭭮� ��� ����� ��� ।����
changes_msg    db    'Save changes?',0
filter_string  db    'All Files',0,'*.*',0   ; ��᪨ ��� Get*FileName
               db    'Text Files',0,'*.txt',0,0
   .data
; �������, �ᯮ�������� Get*FileName
ofn   OPENFILENAME <SIZE ofn,?,?,offset filter_string,?,?,?,offset buffer,MAXSIZE,0,?,?,?,?,?,?,0,?,?,?>

; �������, ����뢠��� ��� �᭮���� �����
wc   WNDCLASSEX <SIZE WNDCLASSEX,CS_HREDRAW or CS_VREDRAW,offset win_proc,0,0,?,?,?,COLOR_WINDOW+1,ID_MENU,offset c_w_name,0>
flag_untitled   db   1   ; = 1 �᫨ ��� 䠩�� �� ��।����� (���� 䠩�)

   .data?
h_editwindow   dd    ?   ; �����䨪��� ���� ।����
h_accel        dd    ?   ; �����䨪��� ���ᨢ� ��ᥫ���஢
p_memory       dd    ?   ; ���� ���� � �����
SizeReadWrite  dd    ?
msg_           MSG   <>
rec            RECT  <>
buffer         db    MAXSIZE dup(?)   ; ��� 䠩��
window_title   db    MAXSIZE dup(?), 12 dup(?)

   .code
_start:
   call  GetCommandLine      ; ����稬 ���� ��������� ��ப�
   mov   edi, eax
   mov   al, ' '
   mov   ecx, MAXSIZE
   repne scasb         ; ����� ����� ����� ��襩 �ணࠬ��
   cmp   byte ptr [edi], 0
   je    cmdline_empty
   mov   esi, edi
   mov   edi, offset buffer
   rep   movsb
   mov   flag_untitled,0
cmdline_empty:
; �����⮢�� � ��ॣ�����㥬 �����
   xor   ebx,ebx
   call  GetModuleHandle      ; ��।���� ��� �����䨪���
   mov   esi, eax
   mov   wc.hInstance, eax   ; � ��࠭�� ��� � wc.hInstance
   mov   ofn._hInstance, eax
   push  IDI_APPLICATION    ; ��� IDI_ICON �᫨ ������ ���� � ������
   push  ebx      ; ��� esi �᫨ ������ ���� � ������
   call  LoadIcon
   mov   wc.hIcon,eax
   push  IDC_ARROW   ; �।��।���� ����� (��५��)
   push  ebx
   call  LoadCursor
   mov   wc.hCursor,eax
   push  offset wc
   call  RegisterClassEx
; ᮧ����� �᭮���� ����
   push  ebx
   push  esi
   push  ebx
   push  ebx
   push  200
   push  300
   push  CW_USEDEFAULT
   push  CW_USEDEFAULT
   push  WS_OVERLAPPEDWINDOW
   push  offset c_w_name
   push  offset c_w_name
   push  WS_EX_CLIENTEDGE
   call  CreateWindowEx
   push  eax         ; ��� pop esi ��। message_loop
   push  eax
   push  SW_SHOWNORMAL
   push  eax
   call  ShowWindow
   call  UpdateWindow
; ���樠�����㥬 ��ᥫ�����
   push  ID_ACCEL
   push  esi
   call  LoadAccelerators
   mov   h_accel, eax
; 横� �������� ᮮ�饭��
   pop   esi      ; ESI - �����䨪��� �᭮����� ����
   mov   edi, offset msg_   ; EDI - ������� � ᮮ�饭��� �� ����
message_loop:
   push  ebx
   push  ebx
   push  ebx
   push  edi
   call  GetMessage      ; ����稬 ᮮ�饭��
   test  eax, eax         ; �᫨ �� WM_QUIT
   jz    exit_msg_loop      ; ��� �� 横��
   push  edi
   push  h_accel
   push  esi         ; hWnd
   call  TranslateAccelerator   ; �८�ࠧ㥬 ��ᥫ����� � IDM*
   test  eax,eax
   jnz   message_loop
   push  edi
   call  TranslateMessage   ; �८�ࠧ㥬 ᮮ�饭�� �� ������
   push  edi
   call  DispatchMessage      ; � ���� ���⭮
   jmp   short message_loop
exit_msg_loop:
   push   msg_.wParam
   call   ExitProcess      ; ����� �ணࠬ��

; ��楤�� win_proc
; 
; ��楤�� �� ������ �������� ॣ����� EBP,EDI,ESI � EBX !
win_proc proc near
; ��ࠬ���� (� ���⮬ push ebp)
   wp_hWnd     equ   dword ptr [ebp+08h]
   wp_uMsg     equ   dword ptr [ebp+0Ch]
   wp_wParam   equ   dword ptr [ebp+10h]
   wp_lParam   equ   dword ptr [ebp+14h]
; ���樠�����㥬 �⥪��� ����
   push  ebp
   mov   ebp, esp      ; ᮧ����� �⥪��� ����
   pusha         ; ��࠭�� �� ॣ�����
   xor   ebx, ebx      ; 0 ��� ������ push 0
   mov   esi, wp_hWnd   ; ��� ������ push hWnd
   mov   eax, wp_uMsg
; ��ࠡ�⠥� ��襤襥 ᮮ�饭��
   cmp   eax, WM_CREATE
   je    h_wm_create
   cmp   eax, WM_SIZE
   je    h_wm_size
   cmp   eax, WM_DESTROY
   je    h_wm_destroy
   cmp   eax, WM_COMMAND
   je    h_wm_command
   cmp   eax, WM_ACTIVATE
   je    h_wm_activate
   cmp   eax, WM_CLOSE
   je    h_wm_close
def_proc:
   popa
   leave         ; �᫨ �� ���㦭�� ᮮ�饭��
   jmp   DefWindowProc   ; ��⠢��� ��� ��ࠡ��稪� �� 㬮�砭��

;
; ��ࠡ��稪 WM_CLOSE
; �᫨ �㦭�, ��訢���, ��࠭��� �� 䠩�
h_wm_close:
   call  save_contents
   jmp   short def_proc

;
; ��ࠡ��稪 WM_CREATE
;
h_wm_create:
; ����� ⠪�� ����� ᮧ���� toolbar � statusbar
; ᮧ����� ���� ।����
   push  ebx
   push  wc.hInstance   ; �����䨪��� �᭮���� �ணࠬ��
   push  EditID
   push  esi      ; hWnd
   push  ebx      ; 0
   push  ebx      ; 0
   push  ebx      ; 0
   push  ebx      ; 0
   push  WS_VISIBLE or WS_CHILD or ES_LEFT or ES_MULTILINE or ES_AUTOHSCROLL or ES_AUTOVSCROLL
   push  ebx      ; 0
   push  offset edit_class
   push  ebx      ; 0
   call  CreateWindowEx
   mov   h_editwindow, eax
; ��।���� ��� 䮪��
   push  eax
   call  SetFocus
   cmp   flag_untitled, 1
   je    continue_create
   call  skip_getopen   ; ��஥� 䠩�, 㪠����� � ��������� ��ப�
continue_create:
   call  set_title
   jmp   end_wm_check

;
; ��ࠡ��稪 WM_COMMAND
;
h_wm_command:
   mov   eax, wp_wParam
   cwde         ; low word contains IDM_*
   sub   eax,100h
   jb    def_proc
; ��ࠡ�⠥� ᮮ�饭�� �� �㭪⮢ ����
   call  dword ptr menu_handlers[eax*4]
   jmp   end_wm_check

menu_handlers
      dd offset h_idm_new, offset h_idm_open, offset h_idm_save
      dd offset h_idm_saveas, offset h_idm_exit, offset h_idm_about
      dd offset h_idm_undo, offset h_idm_cut, offset h_idm_copy
      dd offset h_idm_paste, offset h_idm_clear, offset h_idm_setsel
; ᮮ�饭�� �� �㭪⮢ ���� ������ ���� ���ᠭ� � win95pad.rc ������ � ⠪��
; ���浪� - �� IDM_NEW 100h �� IDM_CLEAR 10Ah

h_idm_setsel:
   push  -1             ; -1
   push  ebx            ; 0 
   push  EM_SETSEL      ; �뤥���� ���� ⥪��
   push  h_editwindow
   call  SendMessage
   ret

; ��ࠡ��稪� ᮮ�饭�� �� ���� EDIT:
h_idm_clear:
   mov   eax, WM_CLEAR
   jmp   short send_to_editor
h_idm_paste:
   mov   eax, WM_PASTE
   jmp   short send_to_editor
h_idm_copy:
   mov   eax, WM_COPY
   jmp   short send_to_editor
h_idm_cut:
   mov   eax,WM_CUT
   jmp   short send_to_editor
h_idm_undo:
   mov   eax, EM_UNDO
send_to_editor:
   push  ebx   ; 0
   push  ebx   ; 0
   push  eax
   push  h_editwindow
   call  SendMessage
   ret

; ��ࠡ��稪 IDM_NEW
h_idm_new:
   call  save_contents   ; ����襬 䠩� �᫨ �㦭�
   mov   byte ptr flag_untitled, 1
   call  set_title   ; �⬥⨬ �� 䠩� �� ������
   push  ebx
   push  ebx
   push  WM_SETTEXT
   push  h_editwindow
   call  SendMessage   ; ���� ���⮩ WM_SETTEXT ।�����
   ret

; ��ࠡ��稪 IDM_ABOUT
h_idm_about:
   push  ebx      ; 0
   push  offset about_proc
   push  esi      ; hWnd
   push  ID_ABOUT
   push  wc.hInstance
   call  DialogBoxParam
   ret

; ��ࠡ��稪 IDM_SAVEAS � IDM_SAVE
h_idm_save:
   cmp   flag_untitled, 1   ; �᫨ 䠩� ������
   jne   skip_getsave   ; �ய��⨬ �맮� GetSaveFileName
h_idm_saveas:
; ��ᨬ ��� 䠩��
   mov   ofn.Flags,OFN_EXPLORER or OFN_OVERWRITEPROMPT
   push  offset ofn
   call  GetSaveFileName
   test  eax, eax
   jz    file_save_failed
skip_getsave:
; ᮧ����� ���
   push  ebx
   push  FILE_ATTRIBUTE_ARCHIVE
   push  CREATE_ALWAYS
   push  ebx
   push  FILE_SHARE_READ or FILE_SHARE_WRITE
   push  GENERIC_READ or GENERIC_WRITE
   push  offset buffer
   call  CreateFile
   mov   edi,eax
; �뤥��� ������
   push  MEMSIZE
   push  GMEM_MOVEABLE or GMEM_ZEROINIT
   call  GlobalAlloc
   push  eax      ; hMemory ��� GlobalFree
   push  eax      ; hMemory ��� GlobalLock
   call  GlobalLock
   mov   esi, eax      ; ���� ���� � ESI
; ������ ⥪�� �� ।����
   push  esi
   push  MEMSIZE-1
   push  WM_GETTEXT
   push  h_editwindow
   call  SendMessage
; ����襬 � 䠩�
   push  esi      ; pMemory
   call  lstrlen   
   push  ebx
   push  offset SizeReadWrite
   push  eax      ; ࠧ��� ����
   push  esi      ; ���� ����
   push  edi      ; �����䨪��� 䠩��
   call  WriteFile
   push  esi      ; pMemory
   call  GlobalUnlock
   call  GlobalFree   ; hMemory 㦥 � �⥪�
   push  edi      ; �����䨪��� 䠩��
   call  CloseHandle
; ��ᨬ 䫠� ����䨪�樨 � ।����
   push  ebx
   push  ebx
   push  EM_SETMODIFY
   push  h_editwindow
   call  SendMessage
   mov   byte ptr flag_untitled,0
   call  set_title
file_save_failed:
   push  h_editwindow
   call  SetFocus
   ret

; ��ࠡ��稪 IDM_OPEN
h_idm_open:
   call  save_contents
; �맮��� �⠭����� ������ �롮� ����� 䠩��
   mov   ofn.Flags, OFN_FILEMUSTEXIST or OFN_PATHMUSTEXIST or OFN_EXPLORER
   push  offset ofn
   call  GetOpenFileName
   test  eax, eax
   jz    file_open_failed
skip_getopen:
; ��஥� ��࠭�� 䠩�
   push  ebx
   push  FILE_ATTRIBUTE_ARCHIVE
   push  OPEN_EXISTING
   push  ebx
   push  FILE_SHARE_READ or FILE_SHARE_WRITE
   push  GENERIC_READ or GENERIC_WRITE
   push  offset buffer
   call  CreateFile
   mov   edi, eax      ; �����䨪��� ��� ReadFile
; �뤥��� ������
   push  MEMSIZE
   push  GMEM_MOVEABLE or GMEM_ZEROINIT
   call  GlobalAlloc
   push  eax      ; hMemory ��� GlobalFree
   push  eax      ; hMemory ��� GlobalLock
   call  GlobalLock      ; ����稬 ���� �뤥������ �����
   push  eax      ; pMemory ��� GlobalUnlock
   push  eax      ; pMemory ��� SendMessage
; ���⠥� 䠩�
   push  ebx
   push  offset SizeReadWrite
   push  MEMSIZE-1
   push  eax      ; pMemory ��� ReadFile
   push  edi
   call  ReadFile
; ���� ���� ।���� ᮮ�饭�� wm_settext �⮡� �� ���ࠫ ⥪�� �� ����
   push  ebx      ; pMemory 㦥 � �⥪�
   push  WM_SETTEXT
   push  h_editwindow
   call  SendMessage
; � ⥯��� ����� ������� 䠩� � �᢮������ ������
   call  GlobalUnlock   ; pMemory 㦥 � �⥪�
   call  GlobalFree   ; hMemory 㦥 � �⥪�
   push  edi      ; hFile
   call  CloseHandle   ;
   mov   byte ptr flag_untitled,0
   call  set_title
file_open_failed:
   push  h_editwindow
   call  SetFocus
   ret

; ��ࠡ��稪 IDM_EXIT
h_idm_exit:
   call  save_contents
   push  esi      ; hWnd
   call  DestroyWindow   ; 㭨�⮦��� ��� ����
   ret

;
; ��ࠡ��稪 WM_SIZE
;
h_wm_size:
; ����� ⠪�� ���� ��᫠�� WM_SIZE ����� toolbar � statusbar
; ������� ࠧ��� ���� ।���� ⠪ �⮡� ��� ��-�०���� �뫮 �� ��� ����
   push  offset rec
   push  esi      ; hWnd
   call  GetClientRect
   push  1      ; true
   push  rec.bottom   ; height
   push  rec.right   ; width
   push  ebx      ; y
   push  ebx      ; x
   push  h_editwindow
   call  MoveWindow
   jmp   short end_wm_check

;
; ��ࠡ��稪 WM_ACTIVATE
;
h_wm_activate:
   push  h_editwindow
   call  SetFocus
   jmp   short end_wm_check
;
; ��ࠡ��稪 WM_DESTROY
;
h_wm_destroy:
   push  ebx
   call  PostQuitMessage   ; ��᫠�� WM_QUIT �᭮���� �ணࠬ��

end_wm_check:
   popa
   xor   eax, eax   ; ������ 0
   leave
   ret   16

; ��楤�� set_title
; ��⠭�������� ���� ��������� ��� �᭮����� ����
set_title:
   push  esi
   push  edi
   mov   edi, offset window_title
   cmp   byte ptr flag_untitled, 1   ; �᫨ � 䠩�� ��� �����
   je    untitled         ; �ᯮ�짮���� Untitled
   mov   esi, ofn.lpstrFile   ; [ESI] - ��� 䠩�� � ����
   movzx eax, ofn.nFileOffset   ; eax - ��砫� ����� 䠩��
   add   esi,eax
copy_filename:
   lodsb      ; ᪮��६ 䠩� �����⮢� � �������� ����
   test  al, al
   jz    add_progname   ; ���� �� �������� ����
   stosb
   jmp   short copy_filename
add_progname:
   mov   dword ptr [edi], '  - '   ; �ਯ�襬 �����
   add   edi, 3
   mov   esi, offset c_w_name
   mov   ecx, 9         ; � �������� �ணࠬ��
   rep   movsb
   pop   edi
   pop   esi
   push  offset window_title
   push  esi      ; �����䨪��� ����
   call  SetWindowText
   ret

untitled:
   mov   dword ptr [edi], 'itnU'   ; ����襬 "Unti"
   mov   dword ptr [edi+4], 'delt' ; ����襬 "tled"
   add   edi, 8
   jmp   short add_progname


; ��楤�� save_contents
; EBX=0, ESI=hWnd
save_contents:
; ��ᨬ ।����, �������� �� ⥪��
   push  ebx
   push  ebx
   push  EM_GETMODIFY
   push  h_editwindow
   call  SendMessage
   test  eax, eax
   jz    not_modified
; ��ᨬ ���짮��⥫�, ��࠭��� �� ���
   push  MB_YESNO + MB_ICONWARNING
   push  offset c_w_name
   push  offset changes_msg
   push  esi
   call  MessageBox
   cmp   eax, IDYES
   jne   not_modified
; ��࠭�� ���
   call  h_idm_save
not_modified:
   ret

win_proc endp

about_proc proc near
; ��ࠬ���� (� ���⮬ push ebp)
   ap_hDlg     equ dword ptr [ebp+08h]
   ap_uMsg     equ dword ptr [ebp+0Ch]
   ap_wParam   equ dword ptr [ebp+10h]
   ap_lParam   equ dword ptr [ebp+14h]
   push  ebp
   mov   ebp, esp      ; ᮧ����� �⥪��� ����
   cmp   ap_uMsg, WM_COMMAND
   jne   dont_proceed
   cmp   ap_wParam,IDOK
   jne   dont_proceed
   push  1
   push  ap_hDlg
   call  EndDialog
dont_proceed:
   xor   eax,eax   ; not processed
   leave
   ret 16
about_proc endp
   end   _start
