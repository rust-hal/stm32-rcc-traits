#![no_std]

use stm32_utility_traits::time::Hertz;

/// HSE Clock modes
///     * `Oscillator`: Use of an external crystal/ceramic resonator
///     * `Bypass`: Use of an external user clock
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSEClockMode {
    Oscillator,
    Bypass,
}

/// HSE Clock
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HSEClock {
    pub freq: u32,
    pub mode: HSEClockMode,
}

impl HSEClock {
    /// Provide HSE frequency. Must be between 4 and 26 MHz
    pub fn new<F>(freq: F, mode: HSEClockMode) -> Self
    where
        F: Into<Hertz>,
    {
        let f: u32 = freq.into().0;

        assert!(4_000_000 <= f && f <= 26_000_000);
        HSEClock { freq: f, mode }
    }
}

/// Bus associated to peripheral control registers
pub trait BusRegisters {
    type Enable;
    type Reset;
}

/// Reference to registers for eanble/disable peripheral
pub trait BusOperations: BusRegisters {
    fn enr(&mut self) -> &Self::Enable;
    fn rstr(&mut self) -> &Self::Reset;
}
