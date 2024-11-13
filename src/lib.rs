//! This crate provides the address and length of registers for Dynamixel servos.
//! ## Example
//! ```rust
//! use dynamixel_ct::{models, ControlTable};
//!     let xm430 = models::XM430;
//!
//!     println!("{:?}", xm430.goal_position());
//!     println!("{:?}", xm430.present_position());
//!     println!("{:?}", xm430.goal_velocity());
//! ```
//! ## Optional Features
//! You can enable the `dynamic_model` feature to use the [`crate::models::try_from_model`] function to create a model from a model number.
//!
//! ### Example
//! ```rust
//! use dynamixel_ct::{models, ControlTable};
//!     let xm430 = models::XM430;
//!     let y = models::YM;
//!     let new = models::try_from_model(1030).unwrap();
//!
//!     println!("{:?}", xm430.goal_position());
//!     println!("{:?}", y.goal_position());
//!     println!("{:?}", new.goal_position());
//! ```
#![no_std]
#![warn(missing_docs)]

pub mod control_table;
pub mod models;
pub mod register;

pub use control_table::ControlTable;
pub use register::RegisterData;

#[cfg(feature = "dynamic_models")]
pub use models::dynamic_model::*;
