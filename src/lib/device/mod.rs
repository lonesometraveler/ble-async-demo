//! Device definitions
#[cfg_attr(feature = "nrf52dk", path = "nrf52dk.rs")]
#[cfg_attr(feature = "nrf52840dk", path = "nrf52840dk.rs")]
#[cfg_attr(feature = "microbit-v2", path = "microbit.rs")]
mod board;

pub use board::Board;
