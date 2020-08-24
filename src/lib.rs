#![no_std]
#![feature(llvm_asm)]
#![feature(global_asm)]
#![feature(naked_functions)]


use r0;

pub use xtensa_lx106_rt_proc_macros::{entry, pre_init, exception};
pub use crate::exception::{ExceptionCause, ExceptionContext};

pub mod interrupt;
pub mod exception;
pub mod rom;

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn DefaultPreInit() {}

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
    }

    extern "Rust" {
        // This symbol will be provided by the user via `#[entry]`
        fn main() -> !;

        // This symbol will be provided by the user via `#[pre_init]`
        fn __pre_init();
    }

    // Initialize PLL.
    // I'm not quite sure what this magic incantation means, but it does set the
    // esp8266 to the right clock speed. Without this, it is running too slow.
    rom::rom_i2c_writeReg(103, 4, 1, 136);
    rom::rom_i2c_writeReg(103, 4, 2, 145);

    __pre_init();

    for cause in ExceptionCause::Illegal as u32..ExceptionCause::Cp7Disabled as u32 {
        rom::_xtos_set_exception_handler(cause, exception::__exception)
    }

    // Initialize RAM
    r0::zero_bss(&mut _bss_start, &mut _bss_end);
    r0::init_data(&mut _data_start, &mut _data_end, &_sidata);

    main()
}
