/// EXCCAUSE register values
///
/// General Exception Causes. (Values of EXCCAUSE special register set by general exceptions,
/// which vector to the user, kernel, or double-exception vectors).
///
#[allow(unused)]
#[derive(Debug, Copy, Clone)]
#[repr(u32)]
pub enum ExceptionCause {
    /// Illegal Instruction
    Illegal = 0,
    /// System Call (Syscall Instruction)
    Syscall = 1,
    /// Instruction Fetch Error
    InstrError = 2,
    /// Load Store Error
    LoadStoreError = 3,
    /// Level 1 Interrupt
    LevelOneInterrupt = 4,
    /// Stack Extension Assist (movsp Instruction) For Alloca
    Alloca = 5,
    /// Integer Divide By Zero
    DivideByZero = 6,
    /// Use Of Failed Speculative Access (Not Implemented)
    Speculation = 7,
    /// Privileged Instruction
    Privileged = 8,
    /// Unaligned Load Or Store
    Unaligned = 9,
    /// Reserved
    Reserved10 = 10,
    /// Reserved
    Reserved11 = 11,
    /// Pif Data Error On Instruction Fetch (Rb-200x And Later)
    InstrDataError = 12,
    /// Pif Data Error On Load Or Store (Rb-200x And Later)
    LoadStoreDataError = 13,
    /// Pif Address Error On Instruction Fetch (Rb-200x And Later)
    InstrAddrError = 14,
    /// Pif Address Error On Load Or Store (Rb-200x And Later)
    LoadStoreAddrError = 15,
    /// Itlb Miss (No Itlb Entry Matches, Hw Refill Also Missed)
    ItlbMiss = 16,
    /// Itlb Multihit (Multiple Itlb Entries Match)
    ItlbMultiHit = 17,
    /// Ring Privilege Violation On Instruction Fetch
    InstrRing = 18,
    /// Size Restriction On Ifetch (Not Implemented)
    Reserved19 = 19,
    /// Cache Attribute Does Not Allow Instruction Fetch
    InstrProhibited = 20,
    /// Reserved
    Reserved21 = 21,
    /// Reserved
    Reserved22 = 22,
    /// Reserved
    Reserved23 = 23,
    /// Dtlb Miss (No Dtlb Entry Matches, Hw Refill Also Missed)
    DtlbMiss = 24,
    /// Dtlb Multihit (Multiple Dtlb Entries Match)
    DtlbMultiHit = 25,
    /// Ring Privilege Violation On Load Or Store
    LoadStoreRing = 26,
    /// Size Restriction On Load/Store (Not Implemented)
    Reserved27 = 27,
    /// Cache Attribute Does Not Allow Load
    LoadProhibited = 28,
    /// Cache Attribute Does Not Allow Store
    StoreProhibited = 29,
    /// Reserved
    Reserved30 = 30,
    /// Reserved
    Reserved31 = 31,
    /// Access To Coprocessor 0 When Disabled
    Cp0Disabled = 32,
    /// Access To Coprocessor 1 When Disabled
    Cp1Disabled = 33,
    /// Access To Coprocessor 2 When Disabled
    Cp2Disabled = 34,
    /// Access To Coprocessor 3 When Disabled
    Cp3Disabled = 35,
    /// Access To Coprocessor 4 When Disabled
    Cp4Disabled = 36,
    /// Access To Coprocessor 5 When Disabled
    Cp5Disabled = 37,
    /// Access To Coprocessor 6 When Disabled
    Cp6Disabled = 38,
    /// Access To Coprocessor 7 When Disabled
    Cp7Disabled = 39,

    None = 255,
}

/// State of the CPU saved when entering exception or interrupt
///
/// Must be aligned with assembly frame format in assembly.rs
#[repr(C)]
#[allow(non_snake_case)]
#[derive(Debug)]
pub struct ExceptionContext {
    PC: u32,
    PS: u32,
    SAR: u32,
    VPRI: u32,
    A0: u32,
    // no A1
    A2: u32,
    A3: u32,
    A4: u32,
    A5: u32,
    A6: u32,
    A7: u32,
    A8: u32,
    A9: u32,
    A10: u32,
    A11: u32,
    A12: u32,
    A13: u32,
    A14: u32,
    A15: u32,
    reason: ExceptionCause,
}

impl ExceptionContext {
    pub fn reason(&self) -> ExceptionCause {
        self.reason
    }
}

extern "C" {
    /// This symbol will be provided by the user via `#[exception]`
    pub fn __exception(context: &ExceptionContext);
}

#[no_mangle]
#[link_section = ".rwtext"]
extern "C" fn __default_exception(context: &ExceptionContext) {
    panic!("Exception: {:?}", context)
}


extern "C" {
    pub fn _xtos_set_exception_handler(cause: u32, handler: unsafe extern "C" fn(&ExceptionContext));
}

