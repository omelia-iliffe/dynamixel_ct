//! Control table for a specific model.
//!
//! The control table is statically allocated to reduce memory usage.
//!
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use dynamixel_registers::models::{Model, ModelGroup, ModelOrModelGroup};
use dynamixel_registers::Register;
use dynamixel_registers::RegisterData;

/// A control table for a specific model.
/// The table is statically allocated to reduce memory usage.
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[cfg_attr(feature = "serde", serde(from = "ModelOrModelGroup"))]
pub struct ControlTable {
    model: Option<Model>,
    model_group: ModelGroup,
    table: &'static HashMap<Register, RegisterData>,
}

impl ControlTable {
    /// Create a new control table for a specific model. If the model is not yet implemented, the error [`Error::NotImplemented`] is returned.
    pub fn new(model_group: ModelGroup) -> Self {
        let table = crate::models::control_table_from_model_group(&model_group);
        ControlTable {
            model: None,
            model_group,
            table,
        }
    }

    pub fn new_with_model(model: Model, model_group: ModelGroup) -> Self {
        let table = crate::models::control_table_from_model_group(&model_group);
        ControlTable {
            model: Some(model),
            model_group,
            table
        }
    }

    /// Get the model for this control table.
    pub fn model(&self) -> Option<Model> {
        self.model
    }

    pub fn model_group(&self) -> ModelGroup {
        self.model_group
    }

    /// Get the register data for a specific register.
    pub fn get(&self, register: Register) -> Option<&RegisterData> {
        self.table.get(&register)
    }
}

impl From<ModelOrModelGroup> for ControlTable {
    fn from(model: ModelOrModelGroup) -> Self {
        match model {
            ModelOrModelGroup::ModelGroup(model_group) => ControlTable::new(model_group),
            ModelOrModelGroup::Model(model) => ControlTable::new_with_model(model, model.into()),
        }
    }
}

impl core::fmt::Display for ControlTable {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if let Some(model) = self.model {
            write!(f, "ControlTable({})", model)
        } else {
            write!(f, "ControlTable({})", self.model_group)
        }
    }
}
