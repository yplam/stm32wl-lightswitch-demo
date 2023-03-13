#![cfg_attr(not(test), no_std)]

use hal::{
    cortex_m::interrupt::CriticalSection,
    gpio::{self, Output, OutputArgs, pins, PinState},
};
pub use stm32wlxx_hal as hal;

pub mod led;
pub mod pb;

/// RF switch.
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct RfSwitch {
    fe_ctrl1: Output<pins::B0>,
    fe_ctrl3: Output<pins::A8>,
}

impl RfSwitch {
    /// Create a new `RfSwitch` struct from GPIOs.
    pub fn new(b0: pins::B0, a8: pins::A8, cs: &CriticalSection) -> RfSwitch {
        const ARGS: OutputArgs = OutputArgs {
            speed: gpio::Speed::Fast,
            level: gpio::PinState::High,
            ot: gpio::OutputType::PushPull,
            pull: gpio::Pull::None,
        };
        RfSwitch {
            fe_ctrl1: Output::new(b0, &ARGS, cs),
            fe_ctrl3: Output::new(a8, &ARGS, cs),
        }
    }

    /// Steal the RF switch from whatever is currently using it.
    ///
    /// # Safety
    ///
    /// 1. Ensure that the code stealing the RF switch has exclusive access.
    ///    Singleton checks are bypassed with this method.
    /// 2. You must set up the RF switch pins.
    ///    No setup will occur when using this method.
    ///
    #[inline]
    pub unsafe fn steal() -> Self {
        RfSwitch {
            fe_ctrl1: Output::steal(),
            fe_ctrl3: Output::steal(),
        }
    }

    /// Set the RF switch to receive.
    #[inline]
    pub fn set_rx(&mut self) {
        self.fe_ctrl1.set_level(PinState::Low);
        self.fe_ctrl3.set_level(PinState::High);
    }

    /// Set the RF switch to low power transmit.
    #[inline]
    pub fn set_tx_lp(&mut self) {
        self.fe_ctrl1.set_level(PinState::Low);
        self.fe_ctrl3.set_level(PinState::Low);
    }

    /// Set the RF switch to high power transmit.
    #[inline]
    pub fn set_tx_hp(&mut self) {
        self.fe_ctrl1.set_level(PinState::High);
        self.fe_ctrl3.set_level(PinState::Low);
    }
}
