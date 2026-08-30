#[macro_export]
macro_rules! arch_entry {
    ($path:path) => {
        use core::arch::global_asm;

        global_asm!(
            r#"
        .section .text, "ax"
        .global _start
        _start:
            adrp x1, __init_stack_end
            add  x1, x1, :lo12:__init_stack_end
            mov  sp, x1
            adrp x0, __init_info_start
            add  x0, x0, :lo12:__init_info_start
            bl   {entry}
        1:
            wfe
            b 1b
    "#,
        entry =  sym $path
        );
    };
}
