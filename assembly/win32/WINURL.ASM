; winurl.asm
; �ਬ�� �ணࠬ� ��� win32.
; ����᪠�� ��⠭������� �� 㬮�砭�� ��㧥� �� ����, 㪠����� � ��ப� URL
; �������筮 ����� ����᪠�� ���� �ணࠬ��, ���㬥��, � �� ��㣮� 䠩�,
; ��� ���ண� ��।����� ������ open
;
; ��������� MASM
;  ml /c /coff /Cp winurl.asm
;  link winurl.obj /subsystem:windows
;
; ��������� TASM
;  tasm /m /ml /D_TASM_ winurl.asm
;  tlink32 /Tpe /aa /c /x winurl.obj
;
; ��������� WASM
;  wasm winurl.asm
;  wlink file winurl.obj form windows nt
;

include   shell32.inc
include   kernel32.inc

   .386
   .model   flat
   .const
URL db      'http://www.lionking.org/~cubbi/',0
   .code
_start:         ; ��⪠ �窨 �室� ������ ��稭����� � �����ન�����
   xor   ebx, ebx
   push  ebx      ; ��� �ᯮ������ 䠩��� - ᯮᮡ ������
   push  ebx      ; ࠡ��� ��४���
   push  ebx      ; ��������� ��ப�
   push  offset URL   ; ��� 䠩�� � ����
   push  ebx      ; ������ open ��� print (�᫨ NULL - open)
   push  ebx      ; �����䨪��� ����, ���஥ ������ ᮮ�饭��
   call  ShellExecute   ; ShellExecute(NULL,NULL,url,NULL,NULL,NULL)
   push  ebx      ; ��� ��室� 
   call  ExitProcess   ; ExitProcess(0)
end   _start
