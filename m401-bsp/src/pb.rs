//! Push-buttons
use stm32wlxx_hal::{
    cortex_m::interrupt::CriticalSection,
    gpio::{Exti, Input, pins, PinState, Pull},
};

const PULL: Pull = Pull::Up;

/// Push-button 3.
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Pb3 {
    gpio: Input<pins::A4>,
}

/// Push-button 2.
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Pb2 {
    gpio: Input<pins::A1>,
}

/// Push-button 1.
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Pb1 {
    gpio: Input<pins::A0>,
}

/// Simple trait for a push-button
pub trait PushButton {
    /// Input pin for the push-button
    ///
    /// This can be used to access the EXTI trait for the pin.
    type Pin: Exti;

    /// Returns `True` if the button is currently being pushed.
    fn is_pushed(&self) -> bool;
}

impl PushButton for Pb3 {
    type Pin = pins::A4;

    #[inline]
    fn is_pushed(&self) -> bool {
        self.gpio.level() == PinState::Low
    }
}

impl PushButton for Pb2 {
    type Pin = pins::A1;

    #[inline]
    fn is_pushed(&self) -> bool {
        self.gpio.level() == PinState::Low
    }
}

impl PushButton for Pb1 {
    type Pin = pins::A0;

    #[inline]
    fn is_pushed(&self) -> bool {
        self.gpio.level() == PinState::Low
    }
}

impl Pb3 {
    /// Create a new push-button 3.
    pub fn new(a4: pins::A4, cs: &CriticalSection) -> Self {
        Self {
            gpio: Input::new(a4, PULL, cs),
        }
    }

    /// Free the GPIO pin from the push-button struct.
    pub fn free(self) -> pins::A4 {
        self.gpio.free()
    }

    /// Steal the push-button from whatever is currently using it.
    ///
    /// # Safety
    ///
    /// 1. Ensure that the code stealing the push-button has exclusive access
    ///    to the underlying GPIO.
    ///    Singleton checks are bypassed with this method.
    /// 2. You are responsible for setting up the underlying GPIO correctly.
    ///    No setup will occur when using this method.
    pub unsafe fn steal() -> Self {
        Self {
            gpio: Input::steal(),
        }
    }
}

impl Pb2 {
    /// Create a new push-button 2.
    pub fn new(a1: pins::A1, cs: &CriticalSection) -> Self {
        Self {
            gpio: Input::new(a1, PULL, cs),
        }
    }

    /// Free the GPIO pin from the push-button struct.
    pub fn free(self) -> pins::A1 {
        self.gpio.free()
    }

    /// Steal the push-button from whatever is currently using it.
    ///
    /// This will **not** initialize the GPIO peripheral.
    ///
    /// # Safety
    ///
    /// 1. Ensure that the code stealing the push-button has exclusive access
    ///    to the underlying GPIO.
    ///    Singleton checks are bypassed with this method.
    /// 2. You are responsible for setting up the underlying GPIO correctly.
    ///    No setup will occur when using this method.
    pub unsafe fn steal() -> Self {
        Self {
            gpio: Input::steal(),
        }
    }
}

impl Pb1 {
    /// Create a new push-button 2.
    pub fn new(a0: pins::A0, cs: &CriticalSection) -> Self {
        Self {
            gpio: Input::new(a0, PULL, cs),
        }
    }

    /// Free the GPIO pin from the push-button struct.
    pub fn free(self) -> pins::A0 {
        self.gpio.free()
    }

    /// Steal the push-button from whatever is currently using it.
    ///
    /// This will **not** initialize the GPIO peripheral.
    ///
    /// # Safety
    ///
    /// 1. Ensure that the code stealing the push-button has exclusive access
    ///    to the underlying GPIO.
    ///    Singleton checks are bypassed with this method.
    /// 2. You are responsible for setting up the underlying GPIO correctly.
    ///    No setup will occur when using this method.
    pub unsafe fn steal() -> Self {
        Self {
            gpio: Input::steal(),
        }
    }
}
