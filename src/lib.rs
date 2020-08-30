#![no_std]
#![feature(llvm_asm)]
#![feature(global_asm)]
#![feature(naked_functions)]

use r0;

pub use xtensa_lx106_rt_proc_macros::{entry, pre_init, exception, interrupt};
pub use crate::exception::{ExceptionCause, ExceptionContext};

pub mod exception;
pub mod interrupt;

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn DefaultPreInit() {}

const CRYSTAL_26MHZ: (u8, u8) = (136, 145);
const CRYSTAL_40MHZ: (u8, u8) = (8, 129);

pub enum CrystalFrequency {
    Crystal26MHz,
    Crystal40MHz,
}

/// Configure the internal PLL for a given crystal frequency
///
/// Most boards use a 26MHz crystal, and the PLL will be configured for this by default.
/// If your board uses a 40MHz crystal, you'll need to use this method to get your clock
/// running at the expected 80MHz
pub fn set_crystal_frequency(crystal: CrystalFrequency) {
    match crystal {
        CrystalFrequency::Crystal26MHz => unsafe { configure_pll(CRYSTAL_26MHZ) },
        CrystalFrequency::Crystal40MHz => unsafe { configure_pll(CRYSTAL_40MHZ) }
    }
}

extern "C" {
    fn rom_i2c_writeReg(block: u8, host_id: u8, reg_add: u8, data: u8);
}

unsafe fn configure_pll((reg1, reg2): (u8, u8)) {
    rom_i2c_writeReg(103, 4, 1, reg1);
    rom_i2c_writeReg(103, 4, 2, reg2);
}

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn Reset() -> ! {
    extern "C" {
        // These symbols come from `link.x`
        static mut _bss_start: u32;
        static mut _bss_end: u32;

        static mut _data_start: u32;
        static mut _data_end: u32;
        static _sidata: u32;

        static mut _init_start: u32;
    }

    extern "Rust" {
        // This symbol will be provided by the user via `#[entry]`
        fn main() -> !;

        // This symbol will be provided by the user via `#[pre_init]`
        fn __pre_init();
    }

    set_crystal_frequency(CrystalFrequency::Crystal26MHz);

    __pre_init();

    // Initialize RAM
    r0::zero_bss(&mut _bss_start, &mut _bss_end);
    r0::init_data(&mut _data_start, &mut _data_end, &_sidata);

    // move vec table
    set_vecbase(&_init_start as *const u32);

    main()
}

#[doc(hidden)]
#[inline]
unsafe fn set_vecbase(base: *const u32) {
    llvm_asm!("wsr.vecbase $0" ::"r"(base) :: "volatile");
}