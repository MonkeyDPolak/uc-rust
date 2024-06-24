// Detection du uC

#[cfg(any(avr_mcu_atmega328, feature = "all-mcus", not(target_arch = "avr")))] pub mod atmega328;
#[cfg(any(avr_mcu_atmega328, not(target_arch = "avr")))] pub use self::atmega328 as current;

#[cfg(any(avr_mcu_atmega328p, feature = "all-mcus"))] pub mod atmega328p;
#[cfg(avr_mcu_atmega328p)] pub use self::atmega328p as current;