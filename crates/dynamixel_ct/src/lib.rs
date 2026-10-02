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
//!
//! ### Example
//! Without `std` there is no [`ControlTable`], but a runtime [`Model`] or [`ModelGroup`]
//! can still be looked up with [`register_data`] / [`register_data_from_model_group`]:
//! ```rust
//! use dynamixel_ct::{register_data, register_data_from_model_group, Model, ModelGroup};
//! use dynamixel_registers::Register::GoalPosition;
//!
//! println!("{:?}", register_data(Model::XM430_W210, GoalPosition));
//! println!("{:?}", register_data_from_model_group(ModelGroup::XM430, GoalPosition));
//! ```
#![warn(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
pub mod control_table;
mod model_macro;
pub mod models;

#[cfg(feature = "std")]
pub use control_table::{common_indirect_address, common_indirect_data, ControlTable};
pub use dynamixel_registers::models::{Model, ModelGroup, ModelOrModelGroup};
pub use dynamixel_registers::{
    pulses_to_radians, radians_to_pulses, Access, Addr, Area, IndirectRange, Register,
    RegisterData, Unit, UnitScale,
};
pub use models::{
    position_resolution, position_resolution_from_model_group, register_data,
    register_data_from_model_group,
};

pub(crate) use model_macro::model;
