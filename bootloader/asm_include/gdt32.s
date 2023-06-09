; GLOBAL DESCRIPTION TABLE (32 bit bit)

GDT32_Start:
    ; Null segment
    gdt32_null_entry:
        dq 0x00

    ; Kernel mode code segment
    gdt32_kernel_code_entry:
        ; Segment limit
        dw 0xffff
        ; Segment base 0-15 bits
        dw 0x0
        ; Segment base 16-23 bits
        db 0x0
        ; Access Byte
        db 0x9A
        ; Flags 4 bits + segment limit bits 16-19
        db 11001111b
        ; Segment base bits 24-31
        db 0x00

    ; Kernel mode data segment
    gdt32_kernel_data_entry:
        ; Follow https://wiki.osdev.org/GDT_Tutorial
        ; and https://wiki.osdev.org/Global_Descriptor_Table#Segment_Descriptor
        dw 0xffff
        dw 0x0
        db 0x0
        db 0x92
        db 11001111b
        db 0x0

; Label to calculate GDT size
GDT32_End:

; Metadata about GDT
GDT32_Descriptor:
    ; As in documentation, the size should be subtracted by 1
    dw GDT32_End - GDT32_Start - 1
    ; The adress to the beginning of the GDT followed empty dword
    ; that is used when changing to long mode and where a adress is
    ; a qword
    dd GDT32_Start;

KERNEL32_CODE_SEG equ gdt32_kernel_code_entry - GDT32_Start
KERNEL32_DATA_SEG equ gdt32_kernel_data_entry - GDT32_Start