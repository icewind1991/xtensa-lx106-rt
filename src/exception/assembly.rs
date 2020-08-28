use crate::{ExceptionCause, ExceptionContext};

global_asm!(
    "
    .set XT_STK_PC,              0
    .set XT_STK_PS,              4
    .set XT_STK_A0,              8
    .equ XT_STK_A1,             12
    .set XT_STK_A2,             16
    .set XT_STK_A3,             20
    .set XT_STK_A4,             24
    .set XT_STK_A5,             28
    .set XT_STK_A6,             32
    .set XT_STK_A7,             36
    .set XT_STK_A8,             40
    .set XT_STK_A9,             44
    .set XT_STK_A10,            48
    .set XT_STK_A11,            52
    .set XT_STK_A12,            56
    .set XT_STK_A13,            60
    .set XT_STK_A14,            64
    .set XT_STK_A15,            68
    .set XT_STK_SAR,            72
    .set XT_STK_EXCCAUSE,       76
    .set XT_STK_EXCVADDR,       80

    .set XT_STK_FRMSZ,         256  // needs to be multiple of 16 and at least 16 free
                                // (for base save region)
                                // multiple of 256 allows use of addmi instruction
    "
);

/// Save processor state to stack.
///
/// *Must only be called with call0.*
/// *For spill all window registers to work WOE must be enabled on entry
///
/// Saves all registers except PC, PS, A0, A1
///
/// Inputs:
///     A0 is the return address
///     A1 is the stack pointers
///     Exceptions are disabled (PS.EXCM = 1)
///
/// Output:
///     A0 is the return address
///     A1 is the stack pointer
///     A3, A9 are used as scratch registers
///     EPC1 is changed
#[naked]
#[no_mangle]
#[link_section = ".iram.text"]
unsafe extern "C" fn save_context() {
    llvm_asm!(
        "
        s32i    a2,  sp, +XT_STK_A2
        s32i    a3,  sp, +XT_STK_A3
        s32i    a4,  sp, +XT_STK_A4
        s32i    a5,  sp, +XT_STK_A5
        s32i    a6,  sp, +XT_STK_A6
        s32i    a7,  sp, +XT_STK_A7
        s32i    a8,  sp, +XT_STK_A8
        s32i    a9,  sp, +XT_STK_A9
        s32i    a10, sp, +XT_STK_A10
        s32i    a11, sp, +XT_STK_A11
        s32i    a12, sp, +XT_STK_A12
        s32i    a13, sp, +XT_STK_A13
        s32i    a14, sp, +XT_STK_A14
        s32i    a15, sp, +XT_STK_A15

        rsr     a3,  SAR
        s32i    a3,  sp, +XT_STK_SAR

        // Spill all windows (up to 64) to the stack
        // Uses the overflow exception: doing a noop write to the high registers
        // will trigger if needed. WOE needs to be enabled before this routine.

        ret
    "
    )
}

global_asm!(
    r#"
    .macro SAVE_CONTEXT level:req
    mov     a0, a1                     // save a1/sp
    addmi   sp, sp, -XT_STK_FRMSZ      // only allow multiple of 256

    s32i    a0, sp, +XT_STK_A1         // save interruptee's A1/SP

    .ifc \level,double
    rsr     a0, DEPC
    .else
    rsr     a0, EPC\level
    .endif
    s32i    a0, sp, +XT_STK_PC         // save interruptee's PC

    .ifc \level,double
    rsr     a0, EXCSAVE2               // ok to reuse EXCSAVE7 for double exception as long as
                                       // double exception is not in first couple of instructions
                                       // of level 7 handler
    .else
    rsr     a0, EXCSAVE\level
    .endif
    s32i    a0, sp, +XT_STK_A0         // save interruptee's A0

    .ifc \level,1
    rsr     a0, PS
    s32i    a0, sp, +XT_STK_PS         // save interruptee's PS

    rsr     a0, EXCCAUSE
    s32i    a0, sp, +XT_STK_EXCCAUSE
    rsr     a0, EXCVADDR
    s32i    a0, sp, +XT_STK_EXCVADDR
    .endif

    .ifc \level,double
    rsr     a0, EXCCAUSE
    s32i    a0, sp, +XT_STK_EXCCAUSE
    rsr     a0, EXCVADDR
    s32i    a0, sp, +XT_STK_EXCVADDR
    .endif

    rsync                              // wait for WSR.PS to complete

    call0   save_context

    .endm
    "#
);

#[naked]
#[no_mangle]
#[link_section = ".iram.text"]
unsafe extern "C" fn restore_context() {
    llvm_asm!(
        "
        l32i    a3,  sp, +XT_STK_SAR
        wsr     a3,  SAR

        // general registers
        l32i    a2,  sp, +XT_STK_A2
        l32i    a3,  sp, +XT_STK_A3
        l32i    a4,  sp, +XT_STK_A4
        l32i    a5,  sp, +XT_STK_A5
        l32i    a6,  sp, +XT_STK_A6
        l32i    a7,  sp, +XT_STK_A7
        l32i    a8,  sp, +XT_STK_A8
        l32i    a9,  sp, +XT_STK_A9
        l32i    a10, sp, +XT_STK_A10
        l32i    a11, sp, +XT_STK_A11
        l32i    a12, sp, +XT_STK_A12
        l32i    a13, sp, +XT_STK_A13
        l32i    a14, sp, +XT_STK_A14
        l32i    a15, sp, +XT_STK_A15

        ret
    "
    )
}

global_asm!(
    r#"
    .macro RESTORE_CONTEXT

    // Restore context and return
    call0   restore_context

    .ifc \level,1
    l32i    a0, sp, +XT_STK_PS        // retrieve interruptee's PS
    wsr     a0, PS
    l32i    a0, sp, +XT_STK_PC        // retrieve interruptee's PC
    .endif

    l32i    a0, sp, +XT_STK_A0        // retrieve interruptee's A0
    l32i    sp, sp, +XT_STK_A1        // remove exception frame
    rsync                             // ensure PS and EPC written

    .endm
    "#
);

/// Handle Other Exceptions or Level 1 interrupt by storing full context and then
/// calling regular function
///
/// # Input:
///    * A0 stored in EXCSAVE1
#[naked]
#[no_mangle]
#[link_section = ".text"]
unsafe extern "C" fn __default_naked_user_exception() {
    llvm_asm!(
        "
        SAVE_CONTEXT 1

        l32i    a2, sp, +XT_STK_EXCCAUSE  // put cause in a2
        beqi    a2, 4, .Level1Interrupt

        .Level1Interrupt:
        movi    a2, 1                     // put interrupt level in a2
        mov     a3, sp                    // put address of save frame in a3
        call0   __level_1_interrupt       // call handler <= actual call!

        mov     a3, sp                    // put address of save frame in a3
        call0   __user_exception               // call handler <= actual call!

        RESTORE_CONTEXT

        .byte 0x00, 0x30, 0x00            // rfe   // PS.EXCM is cleared
                                          // TODO: 20200509, not supported in llvm yet
        "
    )
}

/// Handle Double Exceptions by storing full context and then calling regular function
///
/// # Input:
///    * A0 stored in ???
#[naked]
#[no_mangle]
#[link_section = ".text"]
unsafe extern "C" fn __default_naked_double_exception() {
    llvm_asm!(
        "
        SAVE_CONTEXT double

        l32i    a2, sp, +XT_STK_EXCCAUSE  // put cause in a2
        mov     a3, sp                    // put address of save frame in a3
        call0   __double_exception        // call handler <= actual call!

        RESTORE_CONTEXT

        .byte 0x00, 0x32, 0x00            // rfde
                                          // TODO: 20200509, not supported in llvm yet
        "
    )
}

/// Handle Kernel Exceptions by storing full context and then calling regular function
///
/// # Input:
///    * A0 stored in EXCSAVE1
#[naked]
#[no_mangle]
#[link_section = ".text"]
unsafe extern "C" fn __default_naked_kernel_exception() {
    llvm_asm!(
        "
        SAVE_CONTEXT 1

        l32i    a2, sp, +XT_STK_EXCCAUSE  // put cause in a2

        mov     a3, sp                    // put address of save frame in a3
        call0   __kernel_exception               // call handler <= actual call!

        RESTORE_CONTEXT

        .byte 0x00, 0x30, 0x00            // rfe   // PS.EXCM is cleared
                                          // TODO: 20200509, not supported in llvm yet
        "
    )
}

/// Handle NMI Exceptions by storing full context and then calling regular function
///
/// # Input:
///    * A0 stored in EXCSAVE1
#[naked]
#[no_mangle]
#[link_section = ".text"]
unsafe extern "C" fn __default_naked_nmi_exception() {
    llvm_asm!(
        "
        SAVE_CONTEXT 1

        l32i    a2, sp, +XT_STK_EXCCAUSE  // put cause in a2

        mov     a3, sp                    // put address of save frame in a3
        call0   __nmi_exception               // call handler <= actual call!

        RESTORE_CONTEXT

        .byte 0x00, 0x30, 0x00            // rfe   // PS.EXCM is cleared
                                          // TODO: 20200509, not supported in llvm yet
        "
    )
}

/// Handle Debug Exceptions by storing full context and then calling regular function
///
/// # Input:
///    * A0 stored in EXCSAVE1
#[naked]
#[no_mangle]
#[link_section = ".text"]
unsafe extern "C" fn __default_naked_debug_exception() {
    llvm_asm!(
        "
        SAVE_CONTEXT 1

        l32i    a2, sp, +XT_STK_EXCCAUSE  // put cause in a2

        mov     a3, sp                    // put address of save frame in a3
        call0   __debug_exception               // call handler <= actual call!

        RESTORE_CONTEXT

        .byte 0x00, 0x30, 0x00            // rfe   // PS.EXCM is cleared
                                          // TODO: 20200509, not supported in llvm yet
        "
    )
}

/// Handle Alloc Exceptions by storing full context and then calling regular function
///
/// # Input:
///    * A0 stored in EXCSAVE1
#[naked]
#[no_mangle]
#[link_section = ".text"]
unsafe extern "C" fn __default_naked_alloc_exception() {
    llvm_asm!(
        "
        SAVE_CONTEXT 1

        l32i    a2, sp, +XT_STK_EXCCAUSE  // put cause in a2

        mov     a3, sp                    // put address of save frame in a3
        call0   __alloc_exception               // call handler <= actual call!

        RESTORE_CONTEXT

        .byte 0x00, 0x30, 0x00            // rfe   // PS.EXCM is cleared
                                          // TODO: 20200509, not supported in llvm yet
        "
    )
}

#[no_mangle]
#[link_section = ".text"]
extern "C" fn __default_exception(cause: ExceptionCause, save_frame: &ExceptionContext) {
    panic!("Exception: {:?}, {:08x?}", cause, save_frame)
}

#[no_mangle]
#[link_section = ".text"]
extern "C" fn __default_double_exception(cause: ExceptionCause, save_frame: &ExceptionContext) {
    panic!("Double Exception: {:?}, {:08x?}", cause, save_frame)
}
#[no_mangle]
#[link_section = ".text"]
extern "C" fn __default_interrupt(level: u32, save_frame: &ExceptionContext) {
    panic!("Interrupt: {:?}, {:08x?}", level, save_frame)
}