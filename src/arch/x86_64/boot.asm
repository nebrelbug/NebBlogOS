global start

section .text
bits 32
start:
    mov esp, stack_top
    ;call our check functions
    call check_multiboot
    call check_cpuid
    call check_long_mode
    ; print 'NeblogOS' to the screen
    mov word [0xB8000], 0x0C4e ; N
	mov word [0xb8002], 0x0E65 ; e
	mov word [0xb8004], 0x0A62 ; b
	mov word [0xb8006], 0x096c ; l
	mov word [0xb8008], 0x0C6f ; o
	mov word [0xb800a], 0x0E67 ; g
	mov word [0xb800c], 0x0A4f ; O
	mov word [0xb800e], 0x0953 ; S
    hlt
;Now a lot of checks to make sure stuff is supported
check_multiboot:
    cmp eax, 0x36d76289 ;another magic number
    jne .no_multiboot
    ret
.no_multiboot:
    mov al, "0"
    jmp error ;call error
check_cpuid: ;check if we can get info from cpuid
    ; Check if CPUID is supported by attempting to flip the ID bit (bit 21)
    ; in the FLAGS register. If we can flip it, CPUID is available.

    ; Copy FLAGS in to EAX via stack
    pushfd
    pop eax

    ; Copy to ECX as well for comparing later on
    mov ecx, eax

    ; Flip the ID bit
    xor eax, 1 << 21

    ; Copy EAX to FLAGS via the stack
    push eax
    popfd

    ; Copy FLAGS back to EAX (with the flipped bit if CPUID is supported)
    pushfd
    pop eax

    ; Restore FLAGS from the old version stored in ECX (i.e. flipping the
    ; ID bit back if it was ever flipped).
    push ecx
    popfd

    ; Compare EAX and ECX. If they are equal then that means the bit
    ; wasn't flipped, and CPUID isn't supported.
    cmp eax, ecx
    je .no_cpuid ; jump to no_cpuid
    ret
.no_cpuid:
    mov al, "1"
    jmp error;then call error
check_long_mode:;see if we can use 'long mode' (64-bit)
    ; test if extended processor info in available
    mov eax, 0x80000000    ; implicit argument for cpuid
    cpuid                  ; get highest supported argument
    cmp eax, 0x80000001    ; it needs to be at least 0x80000001
    jb .no_long_mode       ; if it's less, the CPU is too old for long mode

    ; use extended info to test if long mode is available
    mov eax, 0x80000001    ; argument for extended processor info
    cpuid                  ; returns various feature bits in ecx and edx
    test edx, 1 << 29      ; test if the LM-bit is set in the D-register
    jz .no_long_mode       ; If it's not set, there is no long mode
    ret
.no_long_mode:
    mov al, "2"
    jmp error
; Prints `ERR: ` and the given error code to screen and hangs.
; parameter: error code (in ascii) in al
error:
    mov dword [0xb8000], 0x4f524f45
    mov dword [0xb8004], 0x4f3a4f52
    mov dword [0xb8008], 0x4f204f20
    mov byte  [0xb800a], al
    hlt
section .bss
stack_bottom:
    resb 64
stack_top: