//! LEDs

use hal::{
    cortex_m::interrupt::CriticalSection,
    embedded_hal::digital::v2::{OutputPin, ToggleableOutputPin},
    gpio::{self, Output, OutputArgs, pins},
};
use stm32wlxx_hal as hal;

const LED_ARGS: OutputArgs = OutputArgs {
    speed: gpio::Speed::Fast,
    level: gpio::PinState::Low,
    ot: gpio::OutputType::PushPull,
    pull: gpio::Pull::None,
};

/// Simple trait for an LED
pub trait Led<OutPin>
    where
        OutPin: ToggleableOutputPin<Error=core::convert::Infallible>
        + OutputPin<Error=core::convert::Infallible>,
{
    /// Output pin driving the LED.
    fn output(&mut self) -> &mut OutPin;

    /// Set the LED on.
    fn set_on(&mut self) {
        self.output().set_high().unwrap()
    }

    /// Set the LED off.
    fn set_off(&mut self) {
        self.output().set_low().unwrap()
    }

    /// Toggle the LED state.
    fn toggle(&mut self) {
        self.output().toggle().unwrap()
    }
}

/// Red LED
///
/// Marked as LED3 on the PCB
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Red {
    gpio: Output<pins::B3>,
}

impl Red {
    /// Create a new red LED.
    pub fn new(b3: pins::B3, cs: &CriticalSection) -> Self {
        Self {
            gpio: Output::new(b3, &LED_ARGS, cs),
        }
    }

    /// Free the GPIO pin from the LED struct.
    pub fn free(self) -> pins::B3 {
        self.gpio.free()
    }

    /// Steal the LED from whatever is currently using it.
    ///
    /// This will **not** initialize the GPIO peripheral.
    ///
    /// # Safety
    ///
    /// 1. Ensure that the code stealing the LED has exclusive access
    ///    to the underlying GPIO.
    ///    Singleton checks are bypassed with this method.
    /// 2. You are responsible for setting up the underlying GPIO correctly.
    ///    No setup will occur when using this method.
    pub unsafe fn steal() -> Self {
        Self {
            gpio: Output::steal(),
        }
    }
}

impl Led<Output<pins::B3>> for Red {
    fn output(&mut self) -> &mut Output<pins::B3> {
        &mut self.gpio
    }
}

/// Green LED
///
/// Marked as LED2 on the PCB
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Green {
    gpio: Output<pins::B4>,
}

impl Green {
    /// Create a new green LED.
    pub fn new(b4: pins::B4, cs: &CriticalSection) -> Self {
        Self {
            gpio: Output::new(b4, &LED_ARGS, cs),
        }
    }

    /// Free the GPIO pin from the LED struct.
    pub fn free(self) -> pins::B4 {
        self.gpio.free()
    }

    /// Steal the LED from whatever is currently using it.
    ///
    /// This will **not** initialize the GPIO peripheral.
    ///
    /// # Safety
    ///
    /// 1. Ensure that the code stealing the LED has exclusive access
    ///    to the underlying GPIO.
    ///    Singleton checks are bypassed with this method.
    /// 2. You are responsible for setting up the underlying GPIO correctly.
    ///    No setup will occur when using this method.
    pub unsafe fn steal() -> Self {
        Self {
            gpio: Output::steal(),
        }
    }
}

impl Led<Output<pins::B4>> for Green {
    fn output(&mut self) -> &mut Output<pins::B4> {
        &mut self.gpio
    }
}

/// Blue LED
///
/// Marked as LED1 on the PCB
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Blue {
    gpio: Output<pins::B5>,
}

impl Blue {
    /// Create a new blue LED.
    pub fn new(b5: pins::B5, cs: &CriticalSection) -> Self {
        Self {
            gpio: Output::new(b5, &LED_ARGS, cs),
        }
    }

    /// Free the GPIO pin from the LED struct.
    pub fn free(self) -> pins::B5 {
        self.gpio.free()
    }

    /// Steal the LED from whatever is currently using it.
    ///
    /// This will **not** initialize the GPIO peripheral.
    ///
    /// # Safety
    ///
    /// 1. Ensure that the code stealing the LED has exclusive access
    ///    to the underlying GPIO.
    ///    Singleton checks are bypassed with this method.
    /// 2. You are responsible for setting up the underlying GPIO correctly.
    ///    No setup will occur when using this method.
    pub unsafe fn steal() -> Self {
        Self {
            gpio: Output::steal(),
        }
    }
}

impl Led<Output<pins::B5>> for Blue {
    fn output(&mut self) -> &mut Output<pins::B5> {
        &mut self.gpio
    }
}
