//! Control table for a specific model.
//!
//! The control table is statically allocated to reduce memory usage.
//!
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::models::{Error, Model, XM430, YM};
use crate::register::Register;
use crate::RegisterData;

/// A control table for a specific model.
/// The table is statically allocated to reduce memory usage.
#[derive(Debug, PartialEq, Eq, Clone, derive_more::Display)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "Model"))]
#[display("ControlTable({model})")]
pub struct ControlTable {
    model: Model,
    table: &'static HashMap<Register, RegisterData>,
}

impl ControlTable {
    /// Create a new control table for a specific model. If the model is not yet implemented, the error [`Error::NotImplemented`] is returned.
    pub fn new(model: Model) -> Result<Self, Error> {
        let table = match model {
            Model::XM430_W210 | Model::XM430_W350 => XM430::table(),
            Model::XM540_W150 | Model::XM540_W270 => XM430::table(),
            Model::XC330_M181
            | Model::XC330_M288
            | Model::XC330_T181
            | Model::XC330_T288 => XM430::table(),
            Model::YM070_200_R051_R
            | Model::YM070_200_R099_R
            | Model::YM070_200_A099_R
            | Model::YM080_230_R099_R => YM::table(),
            _ => return Err(Error::NotImplemented),
        };
        Ok(ControlTable {
            model,
            table,
        })
    }

    /// Get the model for this control table.
    pub fn model(&self) -> Model {
        self.model
    }

    /// Get the register data for a specific register.
    pub fn get(&self, register: Register) -> Option<&RegisterData> {
        self.table.get(&register)
    }
}

impl TryFrom<Model> for ControlTable {
    type Error = Error;
    fn try_from(model: Model) -> Result<Self, Self::Error> {
        ControlTable::new(model)
    }
}