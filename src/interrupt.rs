use core::ffi::c_void;
use core::mem::transmute;
use core::ptr::null;
use esp8266::Peripherals;

type RawInterruptHandler = unsafe extern "C" fn(arg: *mut c_void, exception_frame: *mut c_void);

extern "C" {
    // These symbols come from `link.x`
    fn ets_isr_mask(intr: u32);
    fn ets_isr_unmask(intr: u32);
    fn ets_isr_attach(intr: u32, handler: RawInterruptHandler, arg: *mut c_void);
}

unsafe extern "C" fn trampoline<F, Type: InterruptType>(user_data: *mut c_void, _frame: *mut c_void)
    where
        F: FnMut(Type::Status),
{
    let user_data = &mut *(user_data as *mut F);
    user_data(Type::status());
}

unsafe extern "C" fn noop(_user_data: *mut c_void, _frame: *mut c_void) {}

pub trait InterruptType: private::Sealed + Sized {
    const TYPE: u32;
    type Status;

    fn attach<F: FnMut(Self::Status) + 'static>(self, mut closure: F) -> InterruptHandle<Self> {
        unsafe {
            ets_isr_attach(Self::TYPE, trampoline::<F, Self>, &mut closure as *mut _ as *mut c_void);
        }
        enable_interrupt::<Self>();
        InterruptHandle {
            ty: self,
        }
    }

    /// Read and clear the interrupt status
    fn status() -> Self::Status;
}

macro_rules! interrupt_types {
    ($($name:ident: $ty:ident,)+) => {
        $(pub struct $ty;)+
        mod private {
            pub trait Sealed {}

            $(impl Sealed for super::$ty {})+
        }

        impl InterruptTypes {
            pub unsafe fn steal() -> InterruptTypes {
                InterruptTypes {
                    $($name: $ty,)+
                }
            }
        }
    }
}

#[allow(dead_code)]
pub struct InterruptTypes {
    // a fields being private means that the interrupt type ins't fully implemented yet
    slc: SLC,
    spi: SPI,
    pub gpio: GPIO,
    uart: UART,
    compare: COMPARE,
    soft: SOFT,
    wdt: WDT,
    timer1: TIMER1,
}

interrupt_types!(
    slc: SLC,
    spi: SPI,
    gpio: GPIO,
    uart: UART,
    compare: COMPARE,
    soft: SOFT,
    wdt: WDT,
    timer1: TIMER1,
);

pub struct GpioInterruptStatus {
    bits: u16,
}

impl GpioInterruptStatus {
    /// Get the bitmask of triggered interrupts
    pub fn bits(&self) -> u16 {
        self.bits
    }

    /// Check if an interrupt was triggered for a specific pin
    pub fn pin(&self, pin: u8) -> bool {
        (self.bits & 1 << pin) > 0
    }

    /// Get all pins for which the interrupt was triggered
    pub fn pins(&self) -> impl Iterator<Item=u8> {
        let bits = self.bits();
        (0..16).filter(move |pin| (bits & 1 << pin) > 0)
    }
}

impl core::fmt::Debug for GpioInterruptStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "[")?;
        let mut first = true;
        for pin in self.pins() {
            if !first {
                write!(f, ", ")?;
            }
            first = false;
            write!(f, "{}", pin)?;
        }
        write!(f, "]")
    }
}

impl InterruptType for SLC {
    const TYPE: u32 = 1;
    type Status = ();
    fn status() -> Self::Status {}
}

impl InterruptType for SPI {
    const TYPE: u32 = 2;
    type Status = ();
    fn status() -> Self::Status {}
}

impl InterruptType for GPIO {
    const TYPE: u32 = 4;
    type Status = GpioInterruptStatus;
    fn status() -> Self::Status {
        let dp = unsafe { Peripherals::steal() };
        let bits = dp.GPIO.gpio_status.read().bits();
        dp.GPIO.gpio_status_w1tc.write(|w| unsafe { w.bits(bits) });
        GpioInterruptStatus {
            bits: bits as u16
        }
    }
}

impl InterruptType for UART {
    const TYPE: u32 = 5;
    type Status = ();
    fn status() -> Self::Status {}
}

impl InterruptType for COMPARE {
    const TYPE: u32 = 6;
    type Status = ();
    fn status() -> Self::Status {}
}

impl InterruptType for SOFT {
    const TYPE: u32 = 7;
    type Status = ();
    fn status() -> Self::Status {}
}

impl InterruptType for WDT {
    const TYPE: u32 = 8;
    type Status = ();
    fn status() -> Self::Status {}
}

impl InterruptType for TIMER1 {
    const TYPE: u32 = 9;
    type Status = ();
    fn status() -> Self::Status {}
}


pub struct InterruptHandle<Type: InterruptType> {
    ty: Type,
}

impl<Type: InterruptType> InterruptHandle<Type> {
    pub fn disable(&mut self) {
        disable_interrupt::<Type>()
    }

    pub fn enable(&mut self) {
        enable_interrupt::<Type>()
    }

    pub fn detach(mut self) -> Type {
        self.disable();
        unsafe {
            ets_isr_attach(Type::TYPE, transmute(noop as *const c_void), null::<c_void>() as *mut c_void);
        }
        self.ty
    }
}

pub fn enable_interrupt<Type: InterruptType>() {
    unsafe {
        ets_isr_unmask(1 << Type::TYPE);
    }
}

pub fn disable_interrupt<Type: InterruptType>() {
    unsafe {
        ets_isr_mask(1 << Type::TYPE);
    }
}
