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

pub struct LedBank {
    pub led0: Pin<led_bank::Led0>,
    pub led1: Pin<led_bank::Led1>,
    pub led2: Pin<led_bank::Led2>,
    pub led3: Pin<led_bank::Led3>,
    pub led4: Pin<led_bank::Led4>,
    pub led5: Pin<led_bank::Led5>,
    pub led6: Pin<led_bank::Led6>,
    pub led7: Pin<led_bank::Led7>,
}

impl LedBank {
    /// Create a new LED bank from individual LED pins.
    pub fn new(
        led0: Pin<led_bank::Led0>,
        led1: Pin<led_bank::Led1>,
        led2: Pin<led_bank::Led2>,
        led3: Pin<led_bank::Led3>,
        led4: Pin<led_bank::Led4>,
        led5: Pin<led_bank::Led5>,
        led6: Pin<led_bank::Led6>,
        led7: Pin<led_bank::Led7>,
    ) -> Self {
        Self {
            led0,
            led1,
            led2,
            led3,
            led4,
            led5,
            led6,
            led7,
        }
    }

    /// Create a new LED bank from GPIO raw pins
    pub fn from_gpio_pins(
        gpio_led0: led_bank::Led0,
        gpio_led1: led_bank::Led1,
        gpio_led2: led_bank::Led2,
        gpio_led3: led_bank::Led3,
        gpio_led4: led_bank::Led4,
        gpio_led5: led_bank::Led5,
        gpio_led6: led_bank::Led6,
        gpio_led7: led_bank::Led7,
    ) -> Self {
        Self::new(
            Pin::new_output(gpio_led0),
            Pin::new_output(gpio_led1),
            Pin::new_output(gpio_led2),
            Pin::new_output(gpio_led3),
            Pin::new_output(gpio_led4),
            Pin::new_output(gpio_led5),
            Pin::new_output(gpio_led6),
            Pin::new_output(gpio_led7),
        )
    }

    /// Get mutable references to all LED pins as an array.
    pub fn as_pins(&mut self) -> [&mut dyn OutputPin<Error = Infallible>; 8] {
        [
            &mut self.led0,
            &mut self.led1,
            &mut self.led2,
            &mut self.led3,
            &mut self.led4,
            &mut self.led5,
            &mut self.led6,
            &mut self.led7,
        ]
    }

    #[inline(always)]
    pub fn set_all_low(&mut self) {
        self.set_all_states([false; 8]);
    }

    #[inline(always)]
    pub fn set_all_high(&mut self) {
        self.set_all_states([true; 8]);
    }

    #[inline(always)]
    pub fn set_all_states(&mut self, states: [bool; 8]) {
        // Safety: We have exclusive access to all LED pins.
        // For efficiency, we set all LED states in one operation by writing to the LED register directly.
        let gpio = unsafe { crate::pac::Gpio::steal() };
        unsafe {
            gpio.led().write_with_zero(|w| {
                let mut raw_bits: u16 = 0;
                // Select all output for writing
                raw_bits |= 0b11111111 << 8;
                // Set state for each pins
                for i in 0..8 {
                    if states[i] {
                        raw_bits |= 0b1 << i;
                    }
                }
                // Apply
                w.bits(raw_bits)
            });
        }
    }
}

pub struct BtnBank {
    pub btn1: Pin<btn_bank::Btn1>,
    pub btn2: Pin<btn_bank::Btn2>,
    pub btn3: Pin<btn_bank::Btn3>,
    pub btn4: Pin<btn_bank::Btn4>,
    pub btn5: Pin<btn_bank::Btn5>,
    pub btn6: Pin<btn_bank::Btn6>,
}

impl BtnBank {
    /// Create a new Button bank from individual Button pins.
    pub fn new(
        btn1: Pin<btn_bank::Btn1>,
        btn2: Pin<btn_bank::Btn2>,
        btn3: Pin<btn_bank::Btn3>,
        btn4: Pin<btn_bank::Btn4>,
        btn5: Pin<btn_bank::Btn5>,
        btn6: Pin<btn_bank::Btn6>,
    ) -> Self {
        Self {
            btn1,
            btn2,
            btn3,
            btn4,
            btn5,
            btn6,
        }
    }

    /// Create a new Button bank from GPIO raw pins
    pub fn from_gpio_pins(
        gpio_btn1: btn_bank::Btn1,
        gpio_btn2: btn_bank::Btn2,
        gpio_btn3: btn_bank::Btn3,
        gpio_btn4: btn_bank::Btn4,
        gpio_btn5: btn_bank::Btn5,
        gpio_btn6: btn_bank::Btn6,
    ) -> Self {
        Self::new(
            Pin::new_input(gpio_btn1),
            Pin::new_input(gpio_btn2),
            Pin::new_input(gpio_btn3),
            Pin::new_input(gpio_btn4),
            Pin::new_input(gpio_btn5),
            Pin::new_input(gpio_btn6),
        )
    }

    /// Get references to all Button pins as an array.
    pub fn as_pins(&self) -> [&dyn InputPin<Error = Infallible>; 6] {
        [
            &self.btn1, &self.btn2, &self.btn3, &self.btn4, &self.btn5, &self.btn6,
        ]
    }

    /// Read all button states in one operation
    /// This function reads the states of all buttons in one operation by reading the BTN register once.
    #[inline(always)]
    pub fn read_all_states(&self) -> Result<[bool; 6], Infallible> {
        // Safety: Read-only, have no other effect
        let gpio = unsafe { crate::pac::Gpio::steal() };
        let reg = gpio.btn().read();
        let states = [
            reg.btn_input(1).bit_is_set(),
            reg.btn_input(2).bit_is_set(),
            reg.btn_input(3).bit_is_set(),
            reg.btn_input(4).bit_is_set(),
            reg.btn_input(5).bit_is_set(),
            reg.btn_input(6).bit_is_set(),
        ];
        Ok(states)
    }
}

/// AudioViz is a hardware vizualization tool that provides an LED energy-meter
/// display for audio signals.
///
/// It uses the LED bank to represent the amplitude of the audio signal in real-time,
/// creating a visual representation of the sound. Each LED corresponds to a specific
/// amplitude range, allowing users to easily see the intensity of the audio signal at
/// any given moment.
///
/// AudioViz takes ownership of the LED bank to ensure exclusive access to the LEDs.
pub struct AudioViz(LedBank);

impl AudioViz {
    /// Create a new AudioViz instance from a LedBank.
    pub fn new(led_bank: LedBank) -> Self {
        // Safety: We take ownership of the LED bank, ensuring exclusive access to the LEDs.
        unsafe {
            Self::enable();
        }
        Self(led_bank)
    }
    /// Consume the AudioViz instance and bring down the LED bank back to GPIO.
    pub fn bring_down(self) -> LedBank {
        // Safety: We are bringing down the LED bank, ensuring that AudioViz is not used after this.
        unsafe {
            Self::disable();
        }
        self.0
    }

    /// Enable the AudioViz functionality.
    ///
    /// This function enables the AudioViz functionality by configuring the LED bank for audio visualization.
    /// It is unsafe because the caller must ensure that the LED bank is not used for GPIO while AudioViz is enabled.
    pub unsafe fn enable() {
        // Safety: Caller must ensure that the LED bank is not used for GPIO while AudioViz is enabled.
        let gpio = unsafe { crate::pac::Gpio::steal() };
        gpio.audio_viz().write(|w| w.enable().set_bit());
    }

    /// Disable the AudioViz functionality.
    /// This function disables the AudioViz functionality, allowing the LED bank to be used for GPIO again.
    /// It is unsafe because the caller must ensure that AudioViz is not used after disabling.
    pub unsafe fn disable() {
        // Safety: Caller must ensure that AudioViz is not used after disabling.
        let gpio = unsafe { crate::pac::Gpio::steal() };
        gpio.audio_viz().write(|w| w.enable().clear_bit());
    }
}
