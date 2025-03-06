//! Control table for a specific model.
//!
//! The control table is statically allocated to reduce memory usage.
//!
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use dynamixel_registers::models::Model;
use dynamixel_registers::error::Error;
use dynamixel_registers::Register;
use dynamixel_registers::RegisterData;

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
        let table = crate::models::control_table(&model);
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

#[cfg(test)]
mod tests {
    use dynamixel_registers::models::Model;
    use crate::models::{XM430, YM070};
    use super::*;
    #[test]
    fn xm430() {
        let models = [
            Model::XM430_W210,
            Model::XM430_W350,
            Model::XM540_W150,
            Model::XM540_W270,
            Model::XC330_M181,
            Model::XC330_M288,
            Model::XC330_T181,
            Model::XC330_T288,
            Model::XH430_V210,
        ];
        for model in models {
            let table = ControlTable::new(model).unwrap();
            assert_eq!(table.table, XM430::table());
        }
    }

    #[test]
    fn ym() {
        let models = [
            Model::YM070_200_R051_RH,
            Model::YM070_200_R099_RH,
            Model::YM070_200_A099_RH,
            Model::YM080_230_R099_RH,
        ];

        for model in models {
            let table = ControlTable::new(model).unwrap();
            assert_eq!(table.table, YM070::table());
        }
    }
}