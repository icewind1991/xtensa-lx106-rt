#[repr(u8)]
pub enum InterruptType {
    SLC = 1,
    SPI = 2,
    GPIO = 4,
    UART = 5,
    COMPARE = 6,
    SOFT = 7,
    WDT = 8,
    TIMER1 = 9,
}

pub fn enable_interrupt(ty: InterruptType) -> u32 {
    let type_mask = 1u32 << ty as u8;
    let mask: u32;
    unsafe {
        llvm_asm!("
            rsil a7, 2
            rsr.intenable a4
            or a4, a4, a5
            wsr.intenable a4
            wsr.ps a7
            rsync
            " :"={a4}"(mask): "{a5}"(type_mask) : "a7" : "volatile"
        );
    }
    mask
}

pub fn disable_interrupt(ty: InterruptType) -> u32 {
    let type_mask = !(1u32 << ty as u8);
    let mask: u32;
    unsafe {
        llvm_asm!("
            rsil a7, 2
            rsr.intenable a4
            and a4, a4, a5
            wsr.intenable a4
            wsr.ps a7
            rsync
            " :"={a4}"(mask): "{a5}"(type_mask) : "a7" : "volatile"
        );
    }
    mask
}
