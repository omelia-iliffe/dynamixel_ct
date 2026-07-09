#![cfg_attr(not(feature = "std"), no_std)]

pub mod convert;
pub mod models;
mod registers;
pub use convert::{pulses_to_radians, radians_to_pulses};
pub use registers::{Access, Addr, Area, IndirectRange, Register, RegisterData, Unit, UnitScale};
