//! Different models of Dynamixel servos use different control tables.
//! This crate aims to provide a common interface to access the control tables of different models.
//! Individual model control tables can be accessed through the `models` module.
//! Or an exact model name or number can be used to create a new `ControlTable` instance.
//!
//! ## Example
//! ```rust
//! use dynamixel_ct::{models };
//! use dynamixel_ct::models::XM430;
//! use dynamixel_registers::Register;
//!
//! println!("{:?}", XM430::get(Register::GoalPosition));
//! println!("{:?}", XM430::get(Register::GoalCurrent));
//! println!("{:?}", XM430::get(Register::GoalVelocity));
//! // or
//! println!("{:?}", XM430::GOAL_POSITION);
//! println!("{:?}", XM430::GOAL_CURRENT);
//! println!("{:?}", XM430::GOAL_VELOCITY);
//! ```
//!
//! ### Example
//! ```rust
//! use dynamixel_ct::{models, ControlTable};
//! use dynamixel_ct::models::{XM430, YM070};
//! use dynamixel_registers::Register::*;
//! use dynamixel_registers::models::Model;
//! let new = ControlTable::new_with_model(Model::XM430_W210);
//!
//! println!("{:?}", XM430::get(GoalPosition));
//! println!("{:?}", YM070::get(GoalPosition));
//! println!("{:?}", new.get(GoalPosition));
//! ```
#![warn(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
pub mod control_table;
pub mod models;
mod model_macro;

#[cfg(feature = "std")]
pub use control_table::ControlTable;
pub use dynamixel_registers::models::{Model, ModelOrModelGroup, ModelGroup};
pub use dynamixel_registers::{RegisterData, Register};

pub(crate) use model_macro::model;

