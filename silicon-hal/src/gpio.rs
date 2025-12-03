use crate::config::{GPIO_REG_BTNS, GPIO_REG_LEDS};
use core::ptr::{read_volatile, write_volatile};
use embedded_hal::digital::{ErrorType, InputPin, OutputPin, StatefulOutputPin};

/// Internal GPIO peripheral
/// It handles the operations through the SoC memory mapped GPIO peripheral
pub(crate) struct GpioIPeripheral;
pub(crate) const INTERNAL_GPIO: GpioIPeripheral = GpioIPeripheral;

impl GpioIPeripheral {
    /// Set individual LED state
    fn set_led(&self, n: u8, state: bool) {
        let mut leds = self.get_leds();
        if state {
            leds |= 1 << n;
        } else {
            leds &= !(1 << n);
        }
        self.set_leds(leds);
    }

    /// Set the state of the LEDs
    /// 
    /// # Arguments
    /// * `value` - A 8-bit value representing the state of the LEDs (0-7)
    fn set_leds(&self, value: u8) {
        let value = value as u32;
        unsafe {
            write_volatile(GPIO_REG_LEDS as *mut u32, value);
        }
    }

    /// Get the individual state of an LED
    /// 
    /// # Arguments
    /// * `n` - The LED number (0-7)
    /// # Returns   
    /// A boolean representing the state of the LED
    fn get_led(&self, n: u8) -> bool {
        let leds = self.get_leds();
        return (leds & (1 << n)) != 0;
    }

    /// Get the current state of the LEDs
    /// 
    /// # Returns
    /// A 8-bit value representing the state of the LEDs (0-7)
    fn get_leds(&self) -> u8 {
        let value = unsafe { read_volatile(GPIO_REG_LEDS as *const u32) };
        return (value & 0xFF) as u8;
    }

    /// Get the individual state of a Button
    /// 
    /// # Arguments
    /// * `n` - The Button number (0-7)
    /// # Returns
    /// A boolean representing the state of the Button
    fn get_button(&self, n: u8) -> bool {
        let buttons = self.get_buttons();
        return (buttons & (1 << n)) != 0;
    }

    /// Get the current state of the Buttons
    /// 
    /// # Returns
    /// A 8-bit value representing the state of the Buttons (0-7)
    fn get_buttons(&self) -> u8 {
        let value = unsafe { read_volatile(GPIO_REG_BTNS as *const u32) };
        return (value & 0xFF) as u8;
    }
}

/// GPIO Pin Trait
/// This trait is implemented by all GPIO pin types to provide
/// the necessary information for accessing the GPIO peripheral.
trait GpioPin : Clone + Copy + PartialEq + Eq {
    /// Returns the memory-mapped address of the GPIO pin"bus"
    fn addr() -> usize;
    /// Returns the pin number withint this GPIO pin"bus"
    fn nth() -> u8;
}

macro_rules! impl_led_pin {
    ($pin:ident, $pin_n:literal) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $pin;

        impl GpioPin for $pin {
            #[inline(always)]
            fn addr() -> usize {
                GPIO_REG_LEDS
            }
            #[inline(always)]
            fn nth() -> u8 {
                $pin_n
            }
        }

        impl ErrorType for $pin {
            type Error = core::convert::Infallible;
        }
        
        impl OutputPin for $pin {
            fn set_low(&mut self) -> Result<(), Self::Error> {
                INTERNAL_GPIO.set_led($pin::nth(), false);
                Ok(())
            }
        
            fn set_high(&mut self) -> Result<(), Self::Error> {
                INTERNAL_GPIO.set_led($pin::nth(), true);
                Ok(())
            }
        }
        
        impl StatefulOutputPin for $pin {
            fn is_set_high(&mut self) -> Result<bool, Self::Error> {
                Ok(INTERNAL_GPIO.get_led($pin::nth()))
            }
        
            fn is_set_low(&mut self) -> Result<bool, Self::Error> {
                Ok(!INTERNAL_GPIO.get_led($pin::nth()))
            }
        }
    };
} 

macro_rules! impl_btn_pin {
    ($pin:ident, $pin_n:literal) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $pin;

        impl GpioPin for $pin {
            #[inline(always)]
            fn addr() -> usize {
                GPIO_REG_LEDS
            }
            #[inline(always)]
            fn nth() -> u8 {
                $pin_n
            }
        }

        impl ErrorType for $pin {
            type Error = core::convert::Infallible;
        }

        impl InputPin for $pin {
            fn is_high(&mut self) -> Result<bool, Self::Error> {
                Ok(INTERNAL_GPIO.get_button($pin::nth()))
            }
        
            fn is_low(&mut self) -> Result<bool, Self::Error> {
                Ok(!INTERNAL_GPIO.get_button($pin::nth()))
            }
        }
    };
} 

impl_led_pin!(GpioLed0, 0);
impl_led_pin!(GpioLed1, 1);
impl_led_pin!(GpioLed2, 2);
impl_led_pin!(GpioLed3, 3);
impl_led_pin!(GpioLed4, 4);
impl_led_pin!(GpioLed5, 5);
impl_led_pin!(GpioLed6, 6);
impl_led_pin!(GpioLed7, 7);

impl_btn_pin!(GpioBtn0, 0);
impl_btn_pin!(GpioBtn1, 1);
impl_btn_pin!(GpioBtn2, 2);
impl_btn_pin!(GpioBtn3, 3);
impl_btn_pin!(GpioBtn4, 4);
impl_btn_pin!(GpioBtn5, 5);
impl_btn_pin!(GpioBtn6, 6);
impl_btn_pin!(GpioBtn7, 7);

/// NoPin for unused pins
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GpioNoPin;

impl ErrorType for GpioNoPin {
    type Error = core::convert::Infallible;
}

impl InputPin for GpioNoPin {
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        Ok(false)
    }

    fn is_low(&mut self) -> Result<bool, Self::Error> {
        Ok(true)
    }
}

impl OutputPin for GpioNoPin {
    fn set_low(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[derive(Debug)]
pub struct GpioPeripheral {
    pub led0: GpioLed0,
    pub led1: GpioLed1,
    pub led2: GpioLed2,
    pub led3: GpioLed3,
    pub led4: GpioLed4,
    pub led5: GpioLed5,
    pub led6: GpioLed6,
    pub led7: GpioLed7,
    pub btn0: GpioBtn0,
    pub btn1: GpioBtn1,
    pub btn2: GpioBtn2,
    pub btn3: GpioBtn3,
    pub btn4: GpioBtn4,
    pub btn5: GpioBtn5,
    pub btn6: GpioBtn6,
    pub btn7: GpioBtn7,
}

impl GpioPeripheral {
    pub(crate) fn new() -> Self {
        GpioPeripheral {
            led0: GpioLed0,
            led1: GpioLed1,
            led2: GpioLed2,
            led3: GpioLed3,
            led4: GpioLed4,
            led5: GpioLed5,
            led6: GpioLed6,
            led7: GpioLed7,
            btn0: GpioBtn0,
            btn1: GpioBtn1,
            btn2: GpioBtn2,
            btn3: GpioBtn3,
            btn4: GpioBtn4,
            btn5: GpioBtn5,
            btn6: GpioBtn6,
            btn7: GpioBtn7,
        }
    }

    pub fn set_leds(&self, value: u8) {
        INTERNAL_GPIO.set_leds(value);
    }

    pub fn get_leds(&self) -> u8 {
        INTERNAL_GPIO.get_leds()
    }

    pub fn get_buttons(&self) -> u8 {
        INTERNAL_GPIO.get_buttons()
    }
}