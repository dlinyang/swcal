; x86_64 assembly test source code

section .data
    ; 测试数据定义
    msg db 'Hello, Assembler!', 0xA, 0

    ; 测试各种数据大小
    byte_val  db 0x12
    word_val  dw 0x1234
    dword_val dd 0x12345678
    qword_val dq 0x1234567890ABCDEF

section .bss
    ; 测试未初始化数据
    buffer resb 256
    array resd 64
    temp resq 8

section .text
    global _start

_start:
    ; 1. 测试基本数据传输指令
    mov rax, 0          ; 立即数 -> 寄存器
    mov rbx, 0x12345678 ; 32位立即数
    mov rcx, 0x123456789ABCDEF0 ; 64位立即数

    mov rdx, qword_val  ; 内存 -> 寄存器
    mov qword [temp], rax ; 寄存器 -> 内存

    ; 测试不同大小的mov
    mov al, [byte_val]
    mov ax, [word_val]
    mov eax, [dword_val]
    mov rax, [qword_val]

    ; 测试movsx/movzx (符号/零扩展)
    movsx rax, byte [byte_val]
    movzx rax, word [word_val]

    ; 测试xchg
    mov rax, 10
    mov rbx, 20
    xchg rax, rbx      ; rax=20, rbx=10

    ; 2. 测试算术运算指令
    mov rax, 100
    mov rbx, 30

    add rax, rbx        ; rax = 130
    sub rax, rbx        ; rax = 100
    inc rax             ; rax = 101
    dec rax             ; rax = 100

    ; 乘法
    mov rax, 5
    mov rbx, 3
    mul rbx             ; rax = 15, rdx = 0
    imul rcx, rax, 4    ; rcx = 60

    ; 除法
    xor rdx, rdx
    mov rax, 100
    mov rbx, 7
    div rbx             ; rax = 14 (商), rdx = 2 (余数)

    ; 逻辑运算
    mov rax, 0xFF
    and rax, 0x0F       ; rax = 0x0F
    or  rax, 0xF0       ; rax = 0xFF
    xor rax, rax        ; rax = 0
    not rax             ; rax = -1 (0xFFFFFFFFFFFFFFFF)

    ; 移位
    mov rax, 0x01
    shl rax, 4          ; rax = 0x10
    shr rax, 2          ; rax = 0x04

    ; 带符号移位
    mov rax, -8
    sar rax, 1          ; rax = -4

    ; 3. 测试条件码和分支
    mov rax, 50
    mov rbx, 50

    cmp rax, rbx
    je  equal_label
    jmp not_equal

equal_label:
    ; 相等分支
    mov rcx, 1
    jmp continue_branch

not_equal:
    mov rcx, 0

continue_branch:
    ; 测试其他条件跳转
    mov rax, 10
    mov rbx, 20

    cmp rax, rbx
    jl  less_than
    jge greater_or_equal

less_than:
    mov rdx, -1
    jmp end_branch

greater_or_equal:
    mov rdx, 1

end_branch:
    nop

    ;测试循环
    mov rcx, 5
loop_start:
    dec rcx
    jnz loop_start

    ; 使用loop指令
    mov rcx, 3
loop_instruction:
    nop
    loop loop_instruction

    ; 5. 测试栈操作
    push rax
    push rbx
    push rcx

    pop rcx
    pop rbx
    pop rax

    ; 测试pushf/popf
    pushfq
    popfq

    ; 测试调用和返回
    call test_function
    jmp after_test_function

test_function:
    push rbp
    mov rbp, rsp

    ; 函数体
    mov rax, 42

    mov rsp, rbp
    pop rbp
    ret

after_test_function:
    ; rax = 42 来自函数返回值

    ; 测试系统调用 (write + exit)
    ; write(1, msg, msg_len)
    mov rax, 1          ; sys_write
    mov rdi, 1          ; fd = stdout
    lea rsi, [msg]      ; buf = msg
    mov rdx, 15    ; count
    syscall

    ; exit(0)
    mov rax, 60         ; sys_exit
    xor rdi, rdi        ; status = 0
    syscall

; Align
align 16
aligned_data:
    nop
    nop

; 10. 测试各种寻址方式
    ; 寄存器间接寻址
    lea rsi, [buffer]
    mov byte [rsi], 0x41

    ; 基址+偏移
    mov byte [rsi + 5], 0x42

    ; 基址+变址
    mov rdi, 10
    mov byte [rsi + rdi], 0x43

    ; 基址+变址*比例
    mov rdi, 0
    lea rax, [array]
    mov dword [rax + rdi*4], 0x12345678

    ; RIP相对寻址
    lea rax, [msg]
