/// Rom functions

use core::ffi::c_void;
use crate::ExceptionContext;

#[repr(C)]
pub struct MD5Context {
    buff: [u32; 4],
    bits: [u32; 4],
    input: [u8; 64],
}

impl Default for MD5Context {
    fn default() -> Self {
        MD5Context {
            buff: [0; 4],
            bits: [0; 4],
            input: [0; 64],
        }
    }
}

#[repr(C)]
pub struct SHA1Context {
    state: [u32; 5],
    count: [u32; 2],
    buff: [u8; 64],
}

impl Default for SHA1Context {
    fn default() -> Self {
        SHA1Context {
            state: [0; 5],
            count: [0; 2],
            buff: [0; 64],
        }
    }
}

extern "C" {
    pub fn rom_i2c_writeReg(block: u8, host_id: u8, reg_add: u8, data: u8);
    pub fn software_reset();
    pub fn rom_software_reboot();
    /// Note: this rng seems to be predictable post reset, esp8266-hal has a better rng
    pub fn rand() -> u32;

    pub fn SPIWrite(addr: u32, src: *const u8, size: u32) -> u32;
    pub fn SPIRead(addr: u32, dst: *mut c_void, size: u32) -> u32;
    pub fn SPIEraseSector(sector_num: u32) -> u32;
    pub fn SPIEraseBlock(block_num: u32) -> u32;
    pub fn SPIEraseChip() -> u32;
    pub fn SPILock() -> u32;
    pub fn SPIUnlock() -> u32;

    pub fn Cache_Read_Disable();
    pub fn Cache_Read_Enable(map: u8, p: u8, v: u8);

    pub fn MD5Init(context: *mut MD5Context);
    pub fn MD5Update(context: *mut MD5Context, buf: *const u8, len: u32);
    pub fn MD5Final(digest: *mut [u8; 16], context: *mut MD5Context);

    pub fn SHA1Init(context: *mut SHA1Context);
    pub fn SHA1Update(context: *mut SHA1Context, buf: *const u8, len: u32);
    pub fn SHA1Final(digest: *mut [u8; 20], context: *mut SHA1Context);
    pub fn SHA1Transform(state: *mut [u32; 5], digest: *const [u8; 64]);

    pub fn _xtos_set_exception_handler(cause: u32, handler: unsafe extern "C" fn(&ExceptionContext));
}