use core::{convert::Infallible, marker::PhantomData};
use paste::paste;

use crate::{
    gpio::{btn_bank::BtnBank, led_bank::LedBank},
    pac,
    typesafe::Sealed,
};
use embedded_hal::digital::{ErrorType, InputPin, OutputPin, StatefulOutputPin};

pub trait BankId: Sealed {}

pub trait BankPinIds: Sealed {
    type Bank: BankId;
    const BANK_ID: u8;
}

pub trait IntoPin<I>
where
    I: BankPinIds,
{
    fn into_pin(self) -> Pin<I>;
}

pub mod led_bank {
    use crate::{
        gpio::{BankId, BankPinIds, IntoPin, Pin},
        typesafe::Sealed,
    };

    pub struct LedBank {}
    impl Sealed for LedBank {}
    impl BankId for LedBank {}

    macro_rules! define_led_pins {
        ($($pin_name:ident => $pin_id:expr),* $(,)?) => {
            $(
                pub struct $pin_name {
                    pub(crate) _inner: (),
                }

                impl Sealed for $pin_name {}
                impl BankPinIds for $pin_name {
                    type Bank = LedBank;
                    const BANK_ID: u8 = $pin_id;
                }

                impl IntoPin<$pin_name> for $pin_name {
                    #[inline(always)]
                    fn into_pin(self) -> Pin<$pin_name> {
                        Pin::new_output(self)
                    }
                }
            )*
        };
    }

    define_led_pins! {
        Led0 => 0,
        Led1 => 1,
        Led2 => 2,
        Led3 => 3,
        Led4 => 4,
        Led5 => 5,
        Led6 => 6,
        Led7 => 7,
    }
}

pub mod btn_bank {
    use crate::{
        gpio::{BankId, BankPinIds, IntoPin, Pin},
        typesafe::Sealed,
    };

    pub struct BtnBank {}

    impl Sealed for BtnBank {}
    impl BankId for BtnBank {}

    macro_rules! define_btn_pins {
        ($($pin_name:ident => $pin_id:expr),* $(,)?) => {
            $(
                pub struct $pin_name {
                    pub(crate) _inner: (),
                }

                impl Sealed for $pin_name {}
                impl BankPinIds for $pin_name {
                    type Bank = BtnBank;
                    const BANK_ID: u8 = $pin_id;
                }

                impl IntoPin<$pin_name> for $pin_name {
                    #[inline(always)]
                    fn into_pin(self) -> Pin<$pin_name> {
                        Pin::new_input(self)
                    }
                }
            )*
        };
    }

    define_btn_pins! {
        Btn1 => 1,
        Btn2 => 2,
        Btn3 => 3,
        Btn4 => 4,
        Btn5 => 5,
        Btn6 => 6,
    }
}

pub struct Pin<I>
where
    I: BankPinIds,
{
    _pin: PhantomData<I>,
}

impl<I> Pin<I>
where
    I: BankPinIds<Bank = LedBank>,
{
    pub fn new_output(_pin: I) -> Self {
        Self { _pin: PhantomData }
    }
}

impl<I> Pin<I>
where
    I: BankPinIds<Bank = BtnBank>,
{
    pub fn new_input(_pin: I) -> Self {
        Self { _pin: PhantomData }
    }
}

impl<I> ErrorType for Pin<I>
where
    I: BankPinIds,
{
    type Error = Infallible;
}

impl<I> InputPin for Pin<I>
where
    I: BankPinIds<Bank = BtnBank>,
{
    #[inline(always)]
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        // Safety: Read-only, have no other effect
        let gpio = unsafe { pac::Gpio::steal() };
        Ok(gpio.btn().read().btn_input(I::BANK_ID).bit_is_set())
    }

    #[inline(always)]
    fn is_low(&mut self) -> Result<bool, Self::Error> {
        // Safety: Read-only, have no other effect
        let gpio = unsafe { pac::Gpio::steal() };
        Ok(gpio.btn().read().btn_input(I::BANK_ID).bit_is_clear())
    }
}

impl<I> OutputPin for Pin<I>
where
    I: BankPinIds<Bank = LedBank>,
{
    #[inline(always)]
    fn set_low(&mut self) -> Result<(), Self::Error> {
        // Safety: Volatile write, outside the PIN output, it has no other effect
        let gpio = unsafe { pac::Gpio::steal() };
        unsafe {
            gpio.led().write_with_zero(|w| {
                w.led_mask(I::BANK_ID)
                    .set_bit()
                    .led_output(I::BANK_ID)
                    .clear_bit()
            });
        }
        Ok(())
    }

    #[inline(always)]
    fn set_high(&mut self) -> Result<(), Self::Error> {
        // Safety: Volatile write, outside the PIN output, it has no other effect
        let gpio = unsafe { pac::Gpio::steal() };
        unsafe {
            gpio.led().write_with_zero(|w| {
                w.led_mask(I::BANK_ID)
                    .set_bit()
                    .led_output(I::BANK_ID)
                    .set_bit()
            });
        }
        Ok(())
    }
}

impl<I> StatefulOutputPin for Pin<I>
where
    I: BankPinIds<Bank = LedBank>,
{
    #[inline(always)]
    fn is_set_high(&mut self) -> Result<bool, Self::Error> {
        // Safety: Read-only, have no other effect
        let gpio = unsafe { pac::Gpio::steal() };
        Ok(gpio.led().read().led_output(I::BANK_ID).bit_is_set())
    }

    #[inline(always)]
    fn is_set_low(&mut self) -> Result<bool, Self::Error> {
        // Safety: Read-only, have no other effect
        let gpio = unsafe { pac::Gpio::steal() };
        Ok(gpio.led().read().led_output(I::BANK_ID).bit_is_clear())
    }
}

pub struct Gpio {
    led0: Option<led_bank::Led0>,
    led1: Option<led_bank::Led1>,
    led2: Option<led_bank::Led2>,
    led3: Option<led_bank::Led3>,
    led4: Option<led_bank::Led4>,
    led5: Option<led_bank::Led5>,
    led6: Option<led_bank::Led6>,
    led7: Option<led_bank::Led7>,
    btn1: Option<btn_bank::Btn1>,
    btn2: Option<btn_bank::Btn2>,
    btn3: Option<btn_bank::Btn3>,
    btn4: Option<btn_bank::Btn4>,
    btn5: Option<btn_bank::Btn5>,
    btn6: Option<btn_bank::Btn6>,
}

impl Gpio {
    pub(crate) fn new() -> Self {
        Self {
            led0: Some(led_bank::Led0 { _inner: () }),
            led1: Some(led_bank::Led1 { _inner: () }),
            led2: Some(led_bank::Led2 { _inner: () }),
            led3: Some(led_bank::Led3 { _inner: () }),
            led4: Some(led_bank::Led4 { _inner: () }),
            led5: Some(led_bank::Led5 { _inner: () }),
            led6: Some(led_bank::Led6 { _inner: () }),
            led7: Some(led_bank::Led7 { _inner: () }),
            btn1: Some(btn_bank::Btn1 { _inner: () }),
            btn2: Some(btn_bank::Btn2 { _inner: () }),
            btn3: Some(btn_bank::Btn3 { _inner: () }),
            btn4: Some(btn_bank::Btn4 { _inner: () }),
            btn5: Some(btn_bank::Btn5 { _inner: () }),
            btn6: Some(btn_bank::Btn6 { _inner: () }),
        }
    }
}

macro_rules! gpio_take_led {
    ($($id:expr),*) => {
        $(
            paste! {
                impl Gpio {
                    #[inline(always)]
                    pub fn [<take_led $id>](&mut self) -> Option<led_bank::[<Led $id>]> {
                        self.[<led $id>].take()
                    }
                }
            }
        )*
    };
}

// Define the gpio_take_btn macro
macro_rules! gpio_take_btn {
    ($($id:expr),*) => {
        $(
            paste! {
                impl Gpio {
                    #[inline(always)]
                    pub fn [<take_btn $id>](&mut self) -> Option<btn_bank::[<Btn $id>]> {
                        self.[<btn $id>].take()
                    }
                }
            }
        )*
    };
}

gpio_take_led! {0,1,2,3,4,5,6,7}
gpio_take_btn! {1,2,3,4,5,6}

impl Gpio {
    #[inline(always)]
    pub fn take_all_btns(
        &mut self,
    ) -> Option<(
        btn_bank::Btn1,
        btn_bank::Btn2,
        btn_bank::Btn3,
        btn_bank::Btn4,
        btn_bank::Btn5,
        btn_bank::Btn6,
    )> {
        if self.btn1.is_some()
            && self.btn2.is_some()
            && self.btn3.is_some()
            && self.btn4.is_some()
            && self.btn5.is_some()
            && self.btn6.is_some()
        {
            Some((
                self.btn1.take().unwrap(),
                self.btn2.take().unwrap(),
                self.btn3.take().unwrap(),
                self.btn4.take().unwrap(),
                self.btn5.take().unwrap(),
                self.btn6.take().unwrap(),
            ))
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn take_all_leds(
        &mut self,
    ) -> Option<(
        led_bank::Led0,
        led_bank::Led1,
        led_bank::Led2,
        led_bank::Led3,
        led_bank::Led4,
        led_bank::Led5,
        led_bank::Led6,
        led_bank::Led7,
    )> {
        if self.led0.is_some()
            && self.led1.is_some()
            && self.led2.is_some()
            && self.led3.is_some()
            && self.led4.is_some()
            && self.led5.is_some()
            && self.led6.is_some()
            && self.led7.is_some()
        {
            Some((
                self.led0.take().unwrap(),
                self.led1.take().unwrap(),
                self.led2.take().unwrap(),
                self.led3.take().unwrap(),
                self.led4.take().unwrap(),
                self.led5.take().unwrap(),
                self.led6.take().unwrap(),
                self.led7.take().unwrap(),
            ))
        } else {
            None
        }
    }
}
