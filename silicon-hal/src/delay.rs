pub use embedded_hal::delay::DelayNs;

/// Delay peripheral
#[derive(Debug, Clone, Copy, Default)]
pub struct SocDelay;

impl DelayNs for SocDelay {

    #[inline(always)]
    fn delay_ns(&mut self, ns: u32) {
        let mut cycles = ns / 2048; // Round up to nearest cycle
        while cycles > 0 {
            // NOP to waste time
            unsafe { core::arch::asm!("nop") };
            cycles -= 1;
        }
    }

    #[inline(always)]
    fn delay_us(&mut self, us: u32) {
        let mut cycles = us / 2; // 1 microsecond per cycle
        while cycles > 0 {
            // NOP to waste time
            unsafe { core::arch::asm!("nop") };
            cycles -= 1;
        }
    }

    #[inline(always)]
    fn delay_ms(&mut self, mut ms: u32) {
        while ms > u32::MAX / 1024 {
            self.delay_us(u32::MAX / 1024 * 1000);
            ms -= u32::MAX / 1024;
        }
        self.delay_us(ms * 1000);
    }
}