//! Different models of Dynamixel servos use different control tables.
//! This crate aims to provide a common interface to access the control tables of different models.
//! Individual model control tables can be accessed through the `models` module.
//! Or an exact model name or number can be used to create a new `ControlTable` instance.
//!
//! ## Example
//! ```rust
//! use dynamixel_ct::{models };
//! use dynamixel_registers::Register;
//! let xm430 = models::XM430::new();
//!
//! println!("{:?}", xm430.get(Register::GoalPosition));
//! println!("{:?}", xm430.get(Register::GoalCurrent));
//! println!("{:?}", xm430.get(Register::GoalVelocity));
//! ```
//!
//! ### Example
//! ```rust
//! use dynamixel_ct::{models, ControlTable};
//! use dynamixel_registers::Register::*;
//! use dynamixel_registers::models::Model;
//! let xm430 = models::XM430::new();
//! let y = models::YM070::new();
//! let new = ControlTable::new_with_model(Model::XM430_W210);
//!
//! println!("{:?}", xm430.get(GoalPosition));
//! println!("{:?}", y.get(GoalPosition));
//! println!("{:?}", new.get(GoalPosition));
//! ```
// #![warn(missing_docs)]

pub mod control_table;
pub mod models;
mod model_macro;

pub use control_table::ControlTable;
pub use dynamixel_registers::models::{Model, ModelOrModelGroup, ModelGroup};
pub use dynamixel_registers::{RegisterData, Register};

pub(crate) use model_macro::model;

