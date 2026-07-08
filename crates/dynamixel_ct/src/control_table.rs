//! Control table for a specific model.
//!
//! The control table is statically allocated to reduce memory usage.
//!
use derive_more::{Display, Error};
use dynamixel_registers::models::{Model, ModelGroup, ModelOrModelGroup};
use dynamixel_registers::IndirectRange;
use dynamixel_registers::Register;
use dynamixel_registers::RegisterData;

#[derive(Clone, Debug, Display, Error, PartialEq, PartialOrd)]
#[display("{} does not have {} register", model_or_group, register)]
/// The `ControlTable` doesn't contain the `Register` requested
pub struct RegisterError {
    model_or_group: ModelOrModelGroup,
    register: Register,
}

impl RegisterError {
    fn new(model: Option<Model>, model_group: ModelGroup, register: Register) -> Self {
        let model_or_group = model.map_or(model_group.into(), Into::into);
        Self {
            model_or_group,
            register,
        }
    }
}

/// A control table for a specific model.
/// The table is statically allocated to reduce memory usage.
#[derive(PartialEq, Clone, derive_more::Debug)]
pub struct ControlTable {
    model: Option<Model>,
    model_group: ModelGroup,
    #[cfg_attr(not(feature = "debug_full_ct"), debug(ignore))]
    table: &'static std::collections::HashMap<Register, RegisterData>,
}

impl ControlTable {
    /// Create a new control table for a specific model.
    pub fn new(model_group: ModelGroup) -> Self {
        let table = crate::models::control_table_from_model_group(&model_group);
        ControlTable {
            model: None,
            model_group,
            table,
        }
    }

    /// Create a ControlTable with an exact [`Model`]
    pub fn new_with_model(model: Model) -> Self {
        let model_group = model.into();
        let table = crate::models::control_table_from_model_group(&model_group);
        ControlTable {
            model: Some(model),
            model_group,
            table,
        }
    }

    /// Get the [`Model`] for this [`ControlTable`]. Returns an `Option` as there may not be an exact [`Model`] and only a [`ModelGroup`]
    pub fn model(&self) -> Option<Model> {
        self.model
    }

    /// Get the [`ModelGroup`] for this [`ControlTable`]
    pub fn model_group(&self) -> ModelGroup {
        self.model_group
    }

    /// Get the register data for a specific register.
    pub fn get(&self, register: Register) -> Result<&RegisterData, RegisterError> {
        self.table
            .get(&register)
            .ok_or_else(|| RegisterError::new(self.model, self.model_group, register))
    }

    /// The contiguous Indirect Address runs for this model (each a sync-usable window).
    pub fn indirect_address_blocks(&self) -> &'static [IndirectRange] {
        crate::models::indirect_address_blocks_from_model_group(&self.model_group)
    }

    /// The contiguous Indirect Data runs for this model (each a sync-usable window).
    pub fn indirect_data_blocks(&self) -> &'static [IndirectRange] {
        crate::models::indirect_data_blocks_from_model_group(&self.model_group)
    }
}

/// The Indirect Data windows every given model shares — the `(address, length)` regions a
/// sync read/write can cover across all of them. Empty if they share no indirect data.
pub fn common_indirect_data(tables: &[&ControlTable]) -> Vec<IndirectRange> {
    common_indirect(tables, ControlTable::indirect_data_blocks)
}

/// As [`common_indirect_data`], for the Indirect Address (configuration) registers.
pub fn common_indirect_address(tables: &[&ControlTable]) -> Vec<IndirectRange> {
    common_indirect(tables, ControlTable::indirect_address_blocks)
}

/// A half-open byte interval `[start, end)`, used to intersect indirect ranges.
#[derive(Clone, Copy)]
struct Interval {
    start: u16,
    end: u16,
}

impl Interval {
    /// The overlap of two intervals, or `None` if they are disjoint.
    fn overlap(self, other: Interval) -> Option<Interval> {
        let start = self.start.max(other.start);
        let end = self.end.min(other.end);
        (start < end).then_some(Interval { start, end })
    }
}

/// Intersect one indirect family's byte ranges across every table, returning the windows
/// they all share. Starts from the first table's ranges and narrows against each of the rest.
fn common_indirect(
    tables: &[&ControlTable],
    blocks: impl Fn(&ControlTable) -> &'static [IndirectRange],
) -> Vec<IndirectRange> {
    let intervals = |table: &ControlTable| -> Vec<Interval> {
        blocks(table)
            .iter()
            .map(|r| Interval {
                start: r.address,
                end: r.address + r.length,
            })
            .collect()
    };

    let Some((first, rest)) = tables.split_first() else {
        return Vec::new();
    };

    let mut shared = intervals(first);
    for table in rest {
        let next = intervals(table);
        shared = shared
            .iter()
            .flat_map(|a| next.iter().filter_map(|b| a.overlap(*b)))
            .collect();
        if shared.is_empty() {
            break;
        }
    }

    shared.sort_unstable_by_key(|i| i.start);
    shared
        .into_iter()
        .map(|i| IndirectRange::new(i.start, i.end - i.start))
        .collect()
}

impl From<ModelOrModelGroup> for ControlTable {
    fn from(model: ModelOrModelGroup) -> Self {
        match model {
            ModelOrModelGroup::ModelGroup(model_group) => ControlTable::new(model_group),
            ModelOrModelGroup::Model(model) => ControlTable::new_with_model(model),
        }
    }
}

impl From<ControlTable> for ModelOrModelGroup {
    fn from(control_table: ControlTable) -> Self {
        if let Some(model) = control_table.model {
            ModelOrModelGroup::Model(model)
        } else {
            ModelOrModelGroup::ModelGroup(control_table.model_group)
        }
    }
}

impl From<Model> for ControlTable {
    fn from(model: Model) -> Self {
        ControlTable::new_with_model(model)
    }
}

impl From<ModelGroup> for ControlTable {
    fn from(model_group: ModelGroup) -> Self {
        ControlTable::new(model_group)
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

#[cfg(test)]
#[cfg(feature = "serde")]
mod serde_tests {
    use super::*;
    use dynamixel_registers::models::Model::{XM430_W210, XM430_W350};
    use dynamixel_registers::models::ModelGroup::XM430;

    #[test]
    fn test_serde_json() {
        let model: Model = serde_json::from_str("\"XM430_W350\"").unwrap();
        let control_table: ControlTable = model.into();
        assert_eq!(control_table, ControlTable::new_with_model(XM430_W350));

        let model: Model = serde_json::from_str("1020").unwrap();
        let control_table: ControlTable = model.into();
        assert_eq!(control_table, ControlTable::new_with_model(XM430_W350));
    }
    #[test]
    fn test_serde_toml() {
        #[derive(serde::Deserialize, serde::Serialize)]
        struct Test {
            pub model: ModelOrModelGroup,
        }
        let model = Test {
            model: ModelOrModelGroup::Model(XM430_W210),
        };

        let toml = toml::to_string(&model).unwrap();
        assert_eq!(toml, "model = \"XM430_W210\"\n");

        let model: Test = toml::from_str("model = \"XM430_W350\"").unwrap();
        assert_eq!(model.model, ModelOrModelGroup::Model(XM430_W350));

        let model: Test = toml::from_str("model = 1020").unwrap();
        assert_eq!(model.model, ModelOrModelGroup::Model(XM430_W350));

        let model: Test = toml::from_str("model = \"XM430\"").unwrap();
        assert_eq!(model.model, ModelOrModelGroup::ModelGroup(XM430));
    }
}

#[cfg(test)]
mod test {
    use dynamixel_registers::{models::Model, Register};

    use crate::control_table::RegisterError;
    use crate::ControlTable;

    #[test]
    fn test_register_error() {
        let model = Model::YM070_210_M001_RH;
        let register = Register::PresentTemperature;
        let control_table = ControlTable::new_with_model(model);

        assert_eq!(
            control_table
                .get(register)
                .inspect_err(|e| println!("{e}"))
                .unwrap_err(),
            RegisterError::new(Some(model), model.into(), register)
        );
    }

    #[test]
    fn test_indirect_blocks() {
        use crate::models::XM430;
        use dynamixel_registers::IndirectRange;

        // Two blocks: the X series splits at index 29 (224-251, then 634-661).
        assert_eq!(
            XM430::indirect_data_blocks(),
            &[IndirectRange::new(224, 28), IndirectRange::new(634, 28)]
        );
        assert_eq!(
            ControlTable::new_with_model(Model::XM430_W210).indirect_data_blocks(),
            XM430::indirect_data_blocks()
        );

        let xm = ControlTable::new_with_model(Model::XM430_W210);
        let ym = ControlTable::new_with_model(Model::YM070_210_M001_RH);
        let xc = ControlTable::new_with_model(Model::XC330_M181);

        // XM (28@224, 28@634) ∩ YM (128@634) => 28 bytes at 634.
        assert_eq!(
            super::common_indirect_data(&[&xm, &ym]),
            vec![IndirectRange::new(634, 28)]
        );
        // XC330 and XM430 share the first block.
        assert_eq!(
            super::common_indirect_data(&[&xc, &xm]),
            vec![IndirectRange::new(224, 28)]
        );
        // Small-X data (224) and Y data (634) never overlap.
        assert!(super::common_indirect_data(&[&xc, &ym]).is_empty());
    }
}
