use crate::timer::Timer;

pub use embedded_hal::delay::DelayNs;

const NS_PER_CLOCK_CYCLES: u32 = 40; // 40 ns per clock cycle at 25 MHz
const CLOCK_CYCLES_PER_INSTRUCTION: u32 = 3; // 3 clock cycles per instruction cycle
const NS_PER_INSTRUCTION_CYCLE: u32 = NS_PER_CLOCK_CYCLES * CLOCK_CYCLES_PER_INSTRUCTION; // 120 ns per instruction cycle

pub const INTR_DELAY: IntrDelay = IntrDelay { _inner: () };
pub const TIMER0_DELAY: Timer0Delay = Timer0Delay { _inner: () };

/// Delay peripheral based on the CPU intruction cycles
///
/// 1 instruction cycle = 3 clock cycles (4 for Store/Load)
/// Clock speed = 25 MHz
/// Therefore, 1 instruction cycle = 1 / (25 MHz / 3) = 120 ns
#[derive(Clone, Copy)]
pub struct IntrDelay {
    _inner: (),
}

impl DelayNs for IntrDelay {
    #[inline(always)]
    fn delay_ns(&mut self, mut ns: u32) {
        if ns <= NS_PER_INSTRUCTION_CYCLE {
            // Less than or equal to NS_PER_INSTRUCTION_CYCLE ns, waste one cycle only
            unsafe { core::arch::asm!("nop") };
            return;
        }
        // Each loop iteration wastes approximately 3 instruction cycles (3 * 120 ns = 360 ns)
        while ns > 0 {
            // NOP to waste time
            unsafe { core::arch::asm!("nop") };
            ns = ns.saturating_sub(NS_PER_INSTRUCTION_CYCLE * 3);
        }
    }

    #[inline(always)]
    fn delay_us(&mut self, mut us: u32) {
        // As NS_PER_CLOCK_CYCLES = 120 ns, and 1000/40 = 25, each microsecond requires 25 clock cycles of delay
        // Knowing that Store/Load instructions take 4 clock cycles, and not 3 like other instructions,
        // we can design a loop that wastes exactly 25 clock cycles per iteration.

        // Each step of the following loop wastes exactly 25 clock cycles
        // 1 Branch (3 cycles) + 5 NOPs (5 * 3 = 15 cycles) + 1 Load (4 cycles) + 1 Subtraction (3 cycles) = 25 cycles
        while us > 0 {
            // 3 for the branch
            unsafe {
                core::arch::asm!(
                    "nop",          // 3 clock cycle - 1
                    "nop",          // 3 clock cycle - 2
                    "nop",          // 3 clock cycle - 3
                    "nop",          // 3 clock cycle - 4
                    "nop",          // 3 clock cycle - 5
                    "lw x0, 0(x0)", // 4 clock cycle - 1
                )
            };
            us -= 1; // 3 for the subtraction
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

/// Delay peripheral based on TIMER0 (1MHz clock)
///
/// TIMER0 is 64-bit clock which increments every 1 micros.
#[derive(Clone, Copy)]
pub struct Timer0Delay {
    _inner: (),
}

impl DelayNs for Timer0Delay {
    #[inline(always)]
    fn delay_ns(&mut self, ns: u32) {
        if ns <= NS_PER_INSTRUCTION_CYCLE {
            // Less than or equal to NS_PER_INSTRUCTION_CYCLE ns, waste one cycle only
            unsafe { core::arch::asm!("nop") };
            return;
        }

        let timer = Timer::new_timer0();
        timer.delay_us(ns / 1000);
    }

    #[inline(always)]
    fn delay_us(&mut self, us: u32) {
        let timer = Timer::new_timer0();
        timer.delay_us(us);
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
