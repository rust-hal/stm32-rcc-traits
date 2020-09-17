#![no_std]

/// HSE Clock modes
///     * `Oscillator`: Use of an external crystal/ceramic resonator
///     * `Bypass`: Use of an external user clock
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSEClockMode {
    Oscillator,
    Bypass,
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

/// Bus associated to peripheral
pub trait RccBus {
    /// Bus type;
    type Bus;
}

/// Enable/disable peripheral
pub trait Enable: RccBus {
    fn enable(bus: &mut Self::Bus);
    fn disable(bus: &mut Self::Bus);
}

/// Reset peripheral
pub trait Reset: RccBus {
    fn reset(bus: &mut Self::Bus);
}

/// Fast declare instances for RccBus traits
#[macro_export]
macro_rules! rcc_bus {
    ($($PER:ident => ($busX:ty, $peren:ident, $perrst:ident),)+) => {
        $(
            impl RccBus for crate::stm32::$PER {
                type Bus = $apbX;
            }
            impl Enable for crate::stm32::$PER {
                #[inline(always)]
                fn enable(bus: &mut Self::Bus) {
                    bus.enr().modify(|_, w| w.$peren().set_bit());
                }
                #[inline(always)]
                fn disable(bus: &mut Self::Bus) {
                    bus.enr().modify(|_, w| w.$peren().clear_bit());
                }
            }
            impl Reset for crate::stm32::$PER {
                #[inline(always)]
                fn reset(bus: &mut Self::Bus) {
                    bus.rstr().modify(|_, w| w.$perrst().set_bit());
                    bus.rstr().modify(|_, w| w.$perrst().clear_bit());
                }
            }
        )+
    }
}
