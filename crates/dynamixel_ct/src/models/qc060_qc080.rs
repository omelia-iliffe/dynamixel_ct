//! Dynamixel QC060 QC080 model definitions.
//!
//! Hand-written, not generated: the Q series is not in the ROBOTIS docs yet. Taken from the
//! control table the early release of Dynamixel Wizard 2.0 (v2.13.2.1) shows for a
//! QC060-200-R020-RE (model 8030) and a QC080-240-R020-RE (model 8050), both firmware 107,
//! which agree on every register below. The `generate_control_tables` binary clears this
//! directory, so re-add this file after regenerating.
//!
//! Only the registers read off the Wizard are listed, so a lookup of anything else fails
//! rather than guessing. Every one sits at the YM070/YM080 address and size, but that is not
//! grounds to borrow the rest of the Y table. Address 546 is left out: the Wizard names it
//! Present Torque (0.01 Nm), where the Y series has Present Current.

use crate::model;

model![QC060, QC080 => {
    ModelNumber: 0, 2, Access::R, Area::Eeprom, None,
    OperatingMode: 33, 1, Access::Rw, Area::Eeprom, None,
    MovingThreshold: 48, 4, Access::Rw, Area::Eeprom, None,
    AccelerationLimit: 68, 4, Access::Rw, Area::Eeprom, Some(UnitScale::new(Unit::RevPerMinuteSquared, 10f32)),
    VelocityLimit: 72, 4, Access::Rw, Area::Eeprom, Some(UnitScale::new(Unit::RevPerMinute, 0.01f32)),
    MaxPositionLimit: 76, 4, Access::Rw, Area::Eeprom, Some(UnitScale::new(Unit::Pulse, 1f32)),
    MinPositionLimit: 84, 4, Access::Rw, Area::Eeprom, Some(UnitScale::new(Unit::Pulse, 1f32)),
    ErrorCode: 153, 1, Access::R, Area::Ram, None,
    ProfileAcceleration: 240, 4, Access::Rw, Area::Hybrid, Some(UnitScale::new(Unit::RevPerMinuteSquared, 10f32)),
    ProfileVelocity: 244, 4, Access::Rw, Area::Hybrid, Some(UnitScale::new(Unit::RevPerMinute, 0.01f32)),
    TorqueEnable: 512, 1, Access::Rw, Area::Ram, None,
    GoalPosition: 532, 4, Access::Rw, Area::Ram, Some(UnitScale::new(Unit::Pulse, 1f32)),
    PresentVelocity: 548, 4, Access::R, Area::Ram, Some(UnitScale::new(Unit::RevPerMinute, 0.01f32)),
    PresentPosition: 552, 4, Access::R, Area::Ram, Some(UnitScale::new(Unit::Pulse, 1f32)),
    PresentTemperature: 570, 1, Access::R, Area::Ram, Some(UnitScale::new(Unit::DegreesCelsius, 1f32)),
}];

// The Wizard showed no Indirect Address or Indirect Data registers, so there are no blocks.
impl QC060 {
    /// Contiguous Indirect Address runs as `(address, byte length)` — sync-usable windows.
    pub const fn indirect_address_blocks() -> &'static [crate::IndirectRange] {
        &[]
    }
    /// Contiguous Indirect Data runs as `(address, byte length)` — sync-usable windows.
    pub const fn indirect_data_blocks() -> &'static [crate::IndirectRange] {
        &[]
    }
}

impl QC080 {
    /// Contiguous Indirect Address runs as `(address, byte length)` — sync-usable windows.
    pub const fn indirect_address_blocks() -> &'static [crate::IndirectRange] {
        &[]
    }
    /// Contiguous Indirect Data runs as `(address, byte length)` — sync-usable windows.
    pub const fn indirect_data_blocks() -> &'static [crate::IndirectRange] {
        &[]
    }
}
