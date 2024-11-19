//! Different models of Dynamixel servos use different control tables.
//! This crate aims to provide a common interface to access the control tables of different models.
//! Individual model control tables can be accessed through the `models` module.
//! Or an exact model name or number can be used to create a new `ControlTable` instance.
//!
//! ## Example
//! ```rust
//! use dynamixel_ct::{models, Register::*};
//! let xm430 = models::XM430::new();
//!
//! println!("{:?}", xm430.get(goal_position));
//! println!("{:?}", xm430.get(goal_current));
//! println!("{:?}", xm430.get(goal_velocity));
//! ```
//!
//! ### Example
//! ```rust
//! use dynamixel_ct::{models, ControlTable, Register::*};
//! let xm430 = models::XM430::new();
//! let y = models::YM::new();
//! let new = ControlTable::new(models::Model::XM430_W210).unwrap();
//!
//! println!("{:?}", xm430.get(goal_position));
//! println!("{:?}", y.get(goal_position));
//! println!("{:?}", new.get(goal_position));
//! ```
#![warn(missing_docs)]

pub mod control_table;
pub mod models;
pub mod register;
mod model_macro;

pub use control_table::ControlTable;
pub use register::{RegisterData, Register};

pub(crate) use model_macro::model;

