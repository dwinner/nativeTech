; winhello.asm
; ����᪮� win32-�ਫ�����e
; �뢮��� ���� ⨯� mesagebox � ⥪�⮬ "Hello world!"
;
; ��������� MASM
;  ml /c /coff /Cp winhello.asm
;  link winhello.obj /subsystem:windows
;
; ��������� TASM
;  tasm /m  /ml /D_TASM_ winhello.asm
;  tlink32 /Tpe /aa /c /x winhello.obj
;
; ��������� WASM
;  wasm winhello.asm
;  wlink file winhello.obj form windows nt
;
include def32.inc
include kernel32.inc
include user32.inc

   .386
   .model flat
   .const
; ��������� ����
hello_title    db 'First win32 GUI program',0
; ᮮ�饭��
hello_message  db 'Hello world!',0
   .code
_start:
   push  MB_ICONINFORMATION   ; �⨫� ����
   push  offset hello_title   ; ���� ��ப� � ����������
   push  offset hello_message   ; ���� ��ப� � ᮮ�饭���
   push  0         ; �����䨪��� �।��
   call  MessageBox

   push  0      ; ��� ��室� 
   call  ExitProcess   ; �����襭�� �ணࠬ��
end   _start
