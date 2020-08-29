use crate::ExceptionContext;

#[repr(u8)]
pub enum InterruptType {
    SLC = 1,
    SPI = 2,
    GPIO = 4,
    UART = 5,
    CCOMPARE = 6,
    SOFT = 7,
    WDT = 8,
    TIMER1 = 9,
}

extern "C" {
    fn __slc_interrupt(context: &ExceptionContext);
    fn __spi_interrupt(context: &ExceptionContext);
    fn __gpio_interrupt(context: &ExceptionContext);
    fn __uart_interrupt(context: &ExceptionContext);
    fn __ccompare_interrupt(context: &ExceptionContext);
    fn __soft_interrupt(context: &ExceptionContext);
    fn __wdt_interrupt(context: &ExceptionContext);
    fn __timer1_interrupt(context: &ExceptionContext);
}

impl InterruptType {
    const fn mask(self) -> u32 {
        1 << self as u8
    }

    fn call(self, context: &ExceptionContext) {
        unsafe {
            match self {
                Self::SLC => __slc_interrupt(context),
                Self::SPI => __spi_interrupt(context),
                Self::GPIO => __gpio_interrupt(context),
                Self::UART => __uart_interrupt(context),
                Self::CCOMPARE => __ccompare_interrupt(context),
                Self::SOFT => __soft_interrupt(context),
                Self::WDT => __wdt_interrupt(context),
                Self::TIMER1 => __timer1_interrupt(context),
            }
        }
    }
}

#[no_mangle]
#[link_section = ".iram.text"]
extern "C" fn __interrupt_trampoline(mask: u32, context: ExceptionContext) {
    if InterruptType::SLC.mask() & mask > 0 {
        InterruptType::SLC.call(&context);
    };
    if InterruptType::SPI.mask() & mask > 0 {
        InterruptType::SPI.call(&context);
    };
    if InterruptType::GPIO.mask() & mask > 0 {
        InterruptType::GPIO.call(&context);
    };
    if InterruptType::UART.mask() & mask > 0 {
        InterruptType::UART.call(&context);
    };
    if InterruptType::CCOMPARE.mask() & mask > 0 {
        InterruptType::CCOMPARE.call(&context);
    };
    if InterruptType::SOFT.mask() & mask > 0 {
        InterruptType::SOFT.call(&context);
    };
    if InterruptType::WDT.mask() & mask > 0 {
        InterruptType::WDT.call(&context);
    };
    if InterruptType::TIMER1.mask() & mask > 0 {
        InterruptType::TIMER1.call(&context);
    };
}

pub fn enable_interrupt(ty: InterruptType) -> u32 {
    let type_mask = ty.mask();
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

pub fn timer0_read() -> u32 {
    let count: u32;
    unsafe {
        llvm_asm!("esync; rsr $0,ccompare0":"=a" (count))
    }
    count
}

pub fn timer0_write(count: u32) {
    unsafe {
        llvm_asm!("wsr $0,ccompare0; esync"::"a" (count) : "memory")
    }
}

pub fn get_cycle_count() -> u32 {
    let count: u32;
    unsafe {
        llvm_asm!("rsr $0,ccount":"=a"(count))
    }
    count
}