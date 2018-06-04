global long_mode_start

section .text
bits 64
long_mode_start:
    ; load 0 into all data segment registers
    mov ax, 0
    mov ss, ax
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax

    ; print 'NeblogOS' to the screen
    mov word [0xB8000], 0x0C4e ; N
	mov word [0xb8002], 0x0E65 ; e
	mov word [0xb8004], 0x0A62 ; b
	mov word [0xb8006], 0x096c ; l
	mov word [0xb8008], 0x0C6f ; o
	mov word [0xb800a], 0x0E67 ; g
	mov word [0xb800c], 0x0A4f ; O
	mov word [0xb800e], 0x0953 ; S
    
    ; call the rust main function
    extern rust_main     ; new
    call rust_main       ; new
    hlt