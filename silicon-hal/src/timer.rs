use core::marker::PhantomData;

use fugit::TimerInstantU64;

use crate::pac::{self, Timer0};
use crate::typesafe::Sealed;

pub type Instant = TimerInstantU64<1_000_000>;

pub trait TimerDevice: Sealed + Clone + Copy + 'static {
    /// Get the register block for Timer0
    ///
    /// # Safety
    /// For read-only use, this is perfectly fine to use.
    fn get_perif() -> &'static pac::timer0::RegisterBlock {
        unsafe { &*Timer0::ptr() }
    }
}

#[derive(Copy, Clone)]
pub struct Timer<D: TimerDevice> {
    _device: core::marker::PhantomData<D>,
}

/// Copyable structure that represent Timer0 from the PAC.
#[derive(Clone, Copy)]
pub struct CopyableTimer0 {
    _inner: (),
}

impl Sealed for CopyableTimer0 {}
impl TimerDevice for CopyableTimer0 {}

impl Timer<CopyableTimer0> {
    pub fn new_timer0() -> Self {
        Self {
            _device: PhantomData,
        }
    }
}

impl<D> Timer<D>
where
    D: TimerDevice,
{
    /// Get the current counter value.
    pub fn get_counter(&self) -> Instant {
        // Safety: Only used for reading current timer value
        let timer = D::get_perif();
        let mut hi0 = timer.hi_value().read().bits();
        let timestamp = loop {
            let low = timer.lo_value().read().bits();
            let hi1 = timer.hi_value().read().bits();
            if hi0 == hi1 {
                break (u64::from(hi0) << 32) | u64::from(low);
            }
            hi0 = hi1;
        };
        TimerInstantU64::from_ticks(timestamp)
    }

    /// Get the value of the least significant word of the counter.
    pub fn get_counter_low(&self) -> u32 {
        // Safety: Only used for reading current timer value
        let timer = D::get_perif();
        timer.lo_value().read().bits()
    }

    /// Pauses execution for at minimum `us` microseconds.
    pub(crate) fn delay_us(&self, mut us: u32) {
        let mut start = self.get_counter_low();
        loop {
            let now = self.get_counter_low();
            let waited = now.wrapping_sub(start);
            if waited >= us {
                break;
            }
            start = now;
            us -= waited;
        }
    }
}
