//! This crate provides the address and length of registers for Dynamixel servos.
//!
//! ## Optional Features
//! You can enable the `dynamic_model` feature to use the [`crate::models::new_from_model`] function to create a model from a model number.
#![no_std]
#![warn(missing_docs)]

pub mod control_table;
pub mod models;
pub mod register;

pub use control_table::{ControlTable, NotImplemented};
pub use register::RegisterData;
