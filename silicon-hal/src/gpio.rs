use core::convert::Infallible;

use paste::paste;

use crate::typesafe::Sealed;
use embedded_hal::digital::{ErrorType, InputPin, OutputPin, StatefulOutputPin};

pub trait BankPinIds: Sealed {
    const BANK_ID: u8;
}

pub trait OutputCapablePin: BankPinIds + ErrorType {
    fn set_high(&mut self) -> Result<(), Self::Error>;
    fn set_low(&mut self) -> Result<(), Self::Error>;
}
pub trait InputCapablePin: BankPinIds + ErrorType {
    fn is_high(&self) -> Result<bool, Self::Error>;
    fn is_low(&self) -> Result<bool, Self::Error>;
}
pub trait StatefulOutputCapablePin: OutputCapablePin {
    fn is_set_high(&self) -> Result<bool, Self::Error>;
    fn is_set_low(&self) -> Result<bool, Self::Error>;
}

pub trait IntoPin<I>
where
    I: BankPinIds,
{
    fn into_pin(self) -> Pin<I>;
}

pub mod never_bank {
    use core::convert::Infallible;

    use embedded_hal::digital::PinState;

    use crate::typesafe::Sealed;

    #[derive(Clone, Copy)]
    pub struct NeverPin(pub PinState);

    impl Sealed for NeverPin {}

    impl super::BankPinIds for NeverPin {
        const BANK_ID: u8 = 0;
    }

    impl super::ErrorType for NeverPin {
        type Error = Infallible;
    }

    impl super::OutputCapablePin for NeverPin {
        #[inline(always)]
        fn set_high(&mut self) -> Result<(), Self::Error> {
            Ok(())
        }
        #[inline(always)]
        fn set_low(&mut self) -> Result<(), Self::Error> {
            Ok(())
        }
    }

    impl super::InputCapablePin for NeverPin {
        #[inline(always)]
        fn is_high(&self) -> Result<bool, Self::Error> {
            Ok(self.0 == PinState::High)
        }
        #[inline(always)]
        fn is_low(&self) -> Result<bool, Self::Error> {
            Ok(self.0 == PinState::Low)
        }
    }
}

pub mod led_bank {
    use crate::{
        gpio::{BankPinIds, IntoPin, OutputCapablePin, Pin, StatefulOutputCapablePin},
        pac,
        typesafe::Sealed,
    };
    use core::convert::Infallible;
    use embedded_hal::digital::ErrorType;

    pub trait LedBankPin: Sealed + OutputCapablePin + StatefulOutputCapablePin {}

    macro_rules! define_led_pins {
        ($($pin_name:ident => $pin_id:expr),* $(,)?) => {
            $(
                pub struct $pin_name {
                    pub(crate) _inner: (),
                }

                impl Sealed for $pin_name {}
                impl BankPinIds for $pin_name {
                    const BANK_ID: u8 = $pin_id;
                }
                impl ErrorType for $pin_name {
                    type Error = Infallible;
                }
                impl OutputCapablePin for $pin_name {
                    #[inline(always)]
                    fn set_high(&mut self) -> Result<(), Self::Error> {
                        // Safety: Volatile write, outside the PIN output, it has no other effect
                        let gpio = unsafe { pac::Gpio::steal() };
                        unsafe {
                            gpio.led().write_with_zero(|w| {
                                w.led_mask(Self::BANK_ID)
                                    .set_bit()
                                    .led_output(Self::BANK_ID)
                                    .set_bit()
                            });
                        }
                        Ok(())
                    }
                    #[inline(always)]
                    fn set_low(&mut self) -> Result<(), Self::Error> {
                        // Safety: Volatile write, outside the PIN output, it has no other effect
                        let gpio = unsafe { pac::Gpio::steal() };
                        unsafe {
                            gpio.led().write_with_zero(|w| {
                                w.led_mask(Self::BANK_ID)
                                    .set_bit()
                                    .led_output(Self::BANK_ID)
                                    .clear_bit()
                            });
                        }
                        Ok(())
                    }
                }
                impl StatefulOutputCapablePin for $pin_name {
                    #[inline(always)]
                    fn is_set_high(&self) -> Result<bool, Self::Error> {
                        let gpio = unsafe { pac::Gpio::steal() };
                        Ok(gpio.led().read().led_output(Self::BANK_ID).bit_is_set())
                    }
                    #[inline(always)]
                    fn is_set_low(&self) -> Result<bool, Self::Error> {
                        let gpio = unsafe { pac::Gpio::steal() };
                        Ok(gpio.led().read().led_output(Self::BANK_ID).bit_is_clear())
                    }
                }
                impl LedBankPin for $pin_name {}

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
        gpio::{BankPinIds, InputCapablePin, IntoPin, Pin},
        pac,
        typesafe::Sealed,
    };
    use core::convert::Infallible;
    use embedded_hal::digital::ErrorType;

    pub trait BtnBankPin: Sealed + InputCapablePin {}

    macro_rules! define_btn_pins {
        ($($pin_name:ident => $pin_id:expr),* $(,)?) => {
            $(
                pub struct $pin_name {
                    pub(crate) _inner: (),
                }

                impl Sealed for $pin_name {}
                impl BankPinIds for $pin_name {
                    const BANK_ID: u8 = $pin_id;
                }
                impl InputCapablePin for $pin_name {
                    #[inline(always)]
                    fn is_high(&self) -> Result<bool, Self::Error> {
                        // Safety: Read-only, have no other effect
                        let gpio = unsafe { pac::Gpio::steal() };
                        Ok(gpio.btn().read().btn_input(Self::BANK_ID).bit_is_set())
                    }

                    #[inline(always)]
                    fn is_low(&self) -> Result<bool, Self::Error> {
                        // Safety: Read-only, have no other effect
                        let gpio = unsafe { pac::Gpio::steal() };
                        Ok(gpio.btn().read().btn_input(Self::BANK_ID).bit_is_clear())
                    }
                }
                impl ErrorType for $pin_name {
                    type Error = Infallible;
                }
                impl BtnBankPin for $pin_name {}

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

pub mod spi_sdcard_bank {
    use crate::{
        gpio::{
            BankPinIds, InputCapablePin, IntoPin, OutputCapablePin, Pin, StatefulOutputCapablePin,
        },
        pac,
        typesafe::Sealed,
    };
    use core::convert::Infallible;
    use embedded_hal::digital::ErrorType;

    pub trait SpiSdcardOutputBankPin: Sealed + OutputCapablePin + StatefulOutputCapablePin {}
    pub trait SpiSdcardInputBankPin: Sealed + InputCapablePin {}

    macro_rules! define_spi_sdcard_pins {
        ($($pin_name:ident => $pin_id:expr),* $(,)?) => {
            $(
                pub struct $pin_name {
                    pub(crate) _inner: (),
                }

                impl Sealed for $pin_name {}
                impl BankPinIds for $pin_name {
                    const BANK_ID: u8 = $pin_id;
                }
                impl ErrorType for $pin_name {
                    type Error = Infallible;
                }
            )*
        };
    }

    macro_rules! impl_spi_sdcard_output {
        ($(($pin_name:ident, $pin_id:expr, $pin_mask:ident, $pin_output:ident)),* $(,)?) => {
            $(
                impl OutputCapablePin for $pin_name {
                    #[inline(always)]
                    fn set_high(&mut self) -> Result<(), Self::Error> {
                        let gpio = unsafe { pac::Gpio::steal() };
                        unsafe {
                            gpio.spi_sdcard().write_with_zero(|w| {
                                w.$pin_mask()
                                    .set_bit()
                                    .$pin_output()
                                    .set_bit()
                            });
                        }
                        Ok(())
                    }
                    #[inline(always)]
                    fn set_low(&mut self) -> Result<(), Self::Error> {
                        let gpio = unsafe { pac::Gpio::steal() };
                        unsafe {
                            gpio.spi_sdcard().write_with_zero(|w| {
                                w.$pin_mask()
                                    .set_bit()
                                    .$pin_output()
                                    .clear_bit()
                            });
                        }
                        Ok(())
                    }
                }

                impl StatefulOutputCapablePin for $pin_name {
                    #[inline(always)]
                    fn is_set_high(&self) -> Result<bool, Self::Error> {
                        let gpio = unsafe { pac::Gpio::steal() };
                        Ok(gpio.spi_sdcard().read().$pin_output().bit_is_set())
                    }
                    #[inline(always)]
                    fn is_set_low(&self) -> Result<bool, Self::Error> {
                        let gpio = unsafe { pac::Gpio::steal() };
                        Ok(gpio.spi_sdcard().read().$pin_output().bit_is_clear())
                    }
                }

                impl SpiSdcardOutputBankPin for $pin_name {}

                impl IntoPin<$pin_name> for $pin_name {
                    #[inline(always)]
                    fn into_pin(self) -> Pin<$pin_name> {
                        Pin::new_output(self)
                    }
                }
            )*
        };
    }

    macro_rules! impl_spi_sdcard_input {
        ($(($pin_name:ident, $pin_id:expr, $pin_mask:ident, $pin_output:ident)),* $(,)?) => {
            $(
                impl InputCapablePin for $pin_name {
                    #[inline(always)]
                    fn is_high(&self) -> Result<bool, Self::Error> {
                        let gpio = unsafe { pac::Gpio::steal() };
                        Ok(gpio.spi_sdcard().read().$pin_output().bit_is_set())
                    }

                    #[inline(always)]
                    fn is_low(&self) -> Result<bool, Self::Error> {
                        let gpio = unsafe { pac::Gpio::steal() };
                        Ok(gpio.spi_sdcard().read().$pin_output().bit_is_clear())
                    }
                }

                impl SpiSdcardInputBankPin for $pin_name {}

                impl IntoPin<$pin_name> for $pin_name {
                    #[inline(always)]
                    fn into_pin(self) -> Pin<$pin_name> {
                        Pin::new_input(self)
                    }
                }
            )*
        };
    }

    define_spi_sdcard_pins! {
        SpiSdCs => 0,
        SpiSdMosi => 1,
        SpiSdClk => 2,
        SpiSdMiso => 3,
    }

    impl_spi_sdcard_output! {
        (SpiSdCs, 0, cs_mask, cs_output),
        (SpiSdMosi, 1, mosi_mask, mosi_output),
        (SpiSdClk, 2, clk_mask, clk_output),
    }
    impl_spi_sdcard_input! {
        (SpiSdMiso, 3, miso_mask, miso_input),
    }
}

pub mod spi_oled_bank {
    use crate::{
        gpio::{BankPinIds, IntoPin, OutputCapablePin, Pin, StatefulOutputCapablePin},
        pac,
        typesafe::Sealed,
    };
    use core::convert::Infallible;
    use embedded_hal::digital::ErrorType;

    pub trait SpiOledBankPin: Sealed + OutputCapablePin + StatefulOutputCapablePin {}

    macro_rules! define_spi_oled_pins {
        ($($pin_name:ident => $pin_id:expr),* $(,)?) => {
            $(
                pub struct $pin_name {
                    pub(crate) _inner: (),
                }

                impl Sealed for $pin_name {}
                impl BankPinIds for $pin_name {
                    const BANK_ID: u8 = $pin_id;
                }
                impl ErrorType for $pin_name {
                    type Error = Infallible;
                }
            )*
        };
    }

    macro_rules! impl_spi_oled_output {
        ($(($pin_name:ident, $pin_id:expr, $pin_mask:ident, $pin_output:ident)),* $(,)?) => {
            $(
                impl OutputCapablePin for $pin_name {
                    #[inline(always)]
                    fn set_high(&mut self) -> Result<(), Self::Error> {
                        let gpio = unsafe { pac::Gpio::steal() };
                        unsafe {
                            gpio.spi_oled().write_with_zero(|w| {
                                w.$pin_mask()
                                    .set_bit()
                                    .$pin_output()
                                    .set_bit()
                            });
                        }
                        Ok(())
                    }
                    #[inline(always)]
                    fn set_low(&mut self) -> Result<(), Self::Error> {
                        let gpio = unsafe { pac::Gpio::steal() };
                        unsafe {
                            gpio.spi_oled().write_with_zero(|w| {
                                w.$pin_mask()
                                    .set_bit()
                                    .$pin_output()
                                    .clear_bit()
                            });
                        }
                        Ok(())
                    }
                }

                impl StatefulOutputCapablePin for $pin_name {
                    #[inline(always)]
                    fn is_set_high(&self) -> Result<bool, Self::Error> {
                        let gpio = unsafe { pac::Gpio::steal() };
                        Ok(gpio.spi_oled().read().$pin_output().bit_is_set())
                    }
                    #[inline(always)]
                    fn is_set_low(&self) -> Result<bool, Self::Error> {
                        let gpio = unsafe { pac::Gpio::steal() };
                        Ok(gpio.spi_oled().read().$pin_output().bit_is_clear())
                    }
                }

                impl SpiOledBankPin for $pin_name {}

                impl IntoPin<$pin_name> for $pin_name {
                    #[inline(always)]
                    fn into_pin(self) -> Pin<$pin_name> {
                        Pin::new_output(self)
                    }
                }
            )*
        };
    }

    define_spi_oled_pins! {
        SpiOledCs => 0,
        SpiOledMosi => 1,
        SpiOledClk => 2,
        SpiOledDc => 3,
        SpiOledRes => 4,
    }

    impl_spi_oled_output! {
        (SpiOledCs, 0, cs_mask, cs_output),
        (SpiOledMosi, 1, mosi_mask, mosi_output),
        (SpiOledClk, 2, clk_mask, clk_output),
        (SpiOledDc, 3, dc_mask, dc_output),
        (SpiOledRes, 4, reset_mask, reset_output),
    }
}

pub struct Pin<I>
where
    I: BankPinIds,
{
    pin: I,
}

impl<I> Pin<I>
where
    I: BankPinIds,
{
    pub fn bring_down(self) -> I {
        self.pin
    }
}

impl<I> Pin<I>
where
    I: OutputCapablePin,
{
    pub fn new_output(pin: I) -> Self {
        Self { pin }
    }
}

impl<I> Pin<I>
where
    I: InputCapablePin,
{
    pub fn new_input(pin: I) -> Self {
        Self { pin }
    }
}

impl<I> ErrorType for Pin<I>
where
    I: BankPinIds + ErrorType,
{
    type Error = I::Error;
}

impl<I> InputPin for Pin<I>
where
    I: InputCapablePin,
{
    #[inline(always)]
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        return self.pin.is_high();
    }

    #[inline(always)]
    fn is_low(&mut self) -> Result<bool, Self::Error> {
        return self.pin.is_low();
    }
}

impl<I> OutputPin for Pin<I>
where
    I: OutputCapablePin,
{
    #[inline(always)]
    fn set_low(&mut self) -> Result<(), Self::Error> {
        return self.pin.set_low();
    }

    #[inline(always)]
    fn set_high(&mut self) -> Result<(), Self::Error> {
        return self.pin.set_high();
    }
}

impl<I> StatefulOutputPin for Pin<I>
where
    I: StatefulOutputCapablePin,
{
    #[inline(always)]
    fn is_set_high(&mut self) -> Result<bool, Self::Error> {
        return self.pin.is_set_high();
    }

    #[inline(always)]
    fn is_set_low(&mut self) -> Result<bool, Self::Error> {
        return self.pin.is_set_low();
    }
}

pub struct SpiPins<MOSI, CLK, MISO>
where
    MOSI: OutputCapablePin,
    CLK: OutputCapablePin,
    MISO: InputCapablePin,
{
    pub mosi: MOSI,
    pub clk: CLK,
    pub miso: MISO,
}

pub type SpiSdPins =
    SpiPins<spi_sdcard_bank::SpiSdMosi, spi_sdcard_bank::SpiSdClk, spi_sdcard_bank::SpiSdMiso>;

pub type SpiOledPins =
    SpiPins<spi_oled_bank::SpiOledMosi, spi_oled_bank::SpiOledClk, never_bank::NeverPin>;

pub type OledPins = (
    spi_oled_bank::SpiOledCs,
    spi_oled_bank::SpiOledMosi,
    spi_oled_bank::SpiOledClk,
    spi_oled_bank::SpiOledDc,
    spi_oled_bank::SpiOledRes,
);
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
    spi_sd: Option<SpiSdPins>,
    spi_sd_cs: Option<spi_sdcard_bank::SpiSdCs>,
    oled: Option<OledPins>,
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
            spi_sd: Some(SpiPins {
                mosi: spi_sdcard_bank::SpiSdMosi { _inner: () },
                clk: spi_sdcard_bank::SpiSdClk { _inner: () },
                miso: spi_sdcard_bank::SpiSdMiso { _inner: () },
            }),
            spi_sd_cs: Some(spi_sdcard_bank::SpiSdCs { _inner: () }),
            oled: Some((
                spi_oled_bank::SpiOledCs { _inner: () },
                spi_oled_bank::SpiOledMosi { _inner: () },
                spi_oled_bank::SpiOledClk { _inner: () },
                spi_oled_bank::SpiOledDc { _inner: () },
                spi_oled_bank::SpiOledRes { _inner: () },
            )),
        }
    }

    pub unsafe fn steal() -> Self {
        Self::new()
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
    pub fn take_oled(&mut self) -> Option<OledPins> {
        self.oled.take()
    }

    #[inline(always)]
    pub fn take_spi_sd(&mut self) -> Option<SpiSdPins> {
        self.spi_sd.take()
    }

    #[inline(always)]
    pub fn take_spi_sd_cs(&mut self) -> Option<spi_sdcard_bank::SpiSdCs> {
        self.spi_sd_cs.take()
    }

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

impl SpiSdPins {
    /// Decompose into individual pins
    #[inline(always)]
    pub fn into_pins(
        self,
    ) -> (
        spi_sdcard_bank::SpiSdMosi,
        spi_sdcard_bank::SpiSdClk,
        spi_sdcard_bank::SpiSdMiso,
    ) {
        (self.mosi, self.clk, self.miso)
    }

    /// Compose from individual pins
    #[inline(always)]
    pub fn from_pins(
        mosi: spi_sdcard_bank::SpiSdMosi,
        clk: spi_sdcard_bank::SpiSdClk,
        miso: spi_sdcard_bank::SpiSdMiso,
    ) -> Self {
        Self { mosi, clk, miso }
    }

    /// Set output state in one operation
    /// This function sets the output state of the SPI SD card pins.
    ///
    /// It is more efficient to set both outputs in one operation rather than
    /// setting them individually. Internally, this function writes to the SPI_SDCARD
    /// register once to set both MOSI and CLK outputs.
    #[inline(always)]
    pub fn set_outputs(&mut self, mosi_high: bool, clk_high: bool) -> Result<(), Infallible> {
        // Safety: Volatile write, outside the PIN output, it has no other effect
        let gpio = unsafe { crate::pac::Gpio::steal() };
        unsafe {
            gpio.spi_sdcard().write_with_zero(|w| {
                w.mosi_mask()
                    .set_bit()
                    .mosi_output()
                    .bit(mosi_high)
                    .clk_mask()
                    .set_bit()
                    .clk_output()
                    .bit(clk_high)
            });
        }
        Ok(())
    }

    /// Read input / output current states in one operation
    /// This function reads the input state of MISO and the output states of MOSI and CLK.
    ///
    /// It is more efficient to read all three states in one operation rather than
    /// reading them individually. Internally, this function reads the SPI_SDCARD
    /// register once to get the states.
    #[inline(always)]
    pub fn read_states(&self) -> Result<(bool, bool, bool), Infallible> {
        // Safety: Read-only, have no other effect
        let gpio = unsafe { crate::pac::Gpio::steal() };
        let reg = gpio.spi_sdcard().read();
        let miso_state = reg.miso_input().bit_is_set();
        let mosi_state = reg.mosi_output().bit_is_set();
        let clk_state = reg.clk_output().bit_is_set();
        Ok((miso_state, mosi_state, clk_state))
    }
}
