#![cfg_attr(not(feature = "std"), no_std)]

pub mod models;
mod registers;
pub use registers::{Access, Area, IndirectRange, Register, RegisterData, Unit, UnitScale};
