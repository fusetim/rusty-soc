use crate::config;

pub struct LedPeripherals {
    pub led0: LedDriver<'static, Led0>,
    pub led1: LedDriver<'static, Led1>,
    pub led2: LedDriver<'static, Led2>,
    pub led3: LedDriver<'static, Led3>,
    pub led4: LedDriver<'static, Led4>,
    pub led5: LedDriver<'static, Led5>,
    pub led6: LedDriver<'static, Led6>,
    pub led7: LedDriver<'static, Led7>,
}

impl LedPeripherals {
    pub(crate) fn new() -> Self {
        LedPeripherals {
            led0: LedDriver::new(Led0 {}, &SOC_LED),
            led1: LedDriver::new(Led1 {}, &SOC_LED),
            led2: LedDriver::new(Led2 {}, &SOC_LED),
            led3: LedDriver::new(Led3 {}, &SOC_LED),
            led4: LedDriver::new(Led4 {}, &SOC_LED),
            led5: LedDriver::new(Led5 {}, &SOC_LED),
            led6: LedDriver::new(Led6 {}, &SOC_LED),
            led7: LedDriver::new(Led7 {}, &SOC_LED),
        }
    }
}

pub const SOC_LED: SocLed = SocLed {};
pub struct SocLed;
impl SocLed {
    /// Create a new LED peripheral instance.
    pub(crate) fn new() -> Self {
        SocLed {}
    }

    /// Set the LEDs to the given value.
    pub fn set(&self, value: u32) {
        let led_peripheral = config::LEDS_ADDR as *mut u32;
        unsafe {
            core::ptr::write_volatile(led_peripheral, value);
        }
    }

    /// Get the current value of the LEDs.
    pub fn get(&self) -> u32 {
        let led_peripheral = config::LEDS_ADDR as *const u32;
        unsafe { core::ptr::read_volatile(led_peripheral) }
    }
}

trait Led {
    fn number(&self) -> u32;
    fn mask(&self) -> u32;
}

macro_rules! define_led {
    ($name:ident, $n:literal, $mask:literal) => {
        #[derive(Copy, Clone)]
        pub struct $name;
        impl Led for $name {
            fn number(&self) -> u32 {
                $n as u32
            }
            fn mask(&self) -> u32 {
                $mask as u32
            }
        }
    };
}

define_led!(Led0, 0, 0b00000001);
define_led!(Led1, 1, 0b00000010);
define_led!(Led2, 2, 0b00000100);
define_led!(Led3, 3, 0b00001000);
define_led!(Led4, 4, 0b00010000);
define_led!(Led5, 5, 0b00100000);
define_led!(Led6, 6, 0b01000000);
define_led!(Led7, 7, 0b10000000);

pub struct LedDriver<'a, L: Led> {
    led: L,
    soc: &'a SocLed,
}

impl<'a, L> LedDriver<'a, L>
where L : Led {
    pub(crate) fn new(led: L, soc: &'a SocLed) -> Self {
        LedDriver { led, soc }
    }

    pub fn on(&self) {
        let current = self.soc.get();
        self.soc.set(current | self.led.mask());
    }

    pub fn off(&self) {
        let current = self.soc.get();
        self.soc.set(current & !self.led.mask());
    }

    pub fn toggle(&self) {
        let current = self.soc.get();
        self.soc.set(current ^ self.led.mask());
    }

    pub fn is_on(&self) -> bool {
        let current = self.soc.get();
        (current & self.led.mask()) != 0
    }
}