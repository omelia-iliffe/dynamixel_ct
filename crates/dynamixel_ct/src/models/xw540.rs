//! Dynamixel XW540 model definitions.

use crate::model;

model![XW540 => {
    ModelNumber: 0, 2, Access::R, Area::Eeprom, None,
    ModelInformation: 2, 4, Access::R, Area::Eeprom, None,
    FirmwareVersion: 6, 1, Access::R, Area::Eeprom, None,
    Id: 7, 1, Access::Rw, Area::Eeprom, None,
    BaudRate: 8, 1, Access::Rw, Area::Eeprom, None,
    ReturnDelayTime: 9, 1, Access::Rw, Area::Eeprom, Some(UnitScale::new(Unit::Second, 0.000002f32)),
    DriveMode: 10, 1, Access::Rw, Area::Eeprom, None,
    OperatingMode: 11, 1, Access::Rw, Area::Eeprom, None,
    SecondaryId: 12, 1, Access::Rw, Area::Eeprom, None,
    HomingOffset: 20, 4, Access::Rw, Area::Eeprom, Some(UnitScale::new(Unit::Pulse, 1f32)),
    MovingThreshold: 24, 4, Access::Rw, Area::Eeprom, Some(UnitScale::new(Unit::RevPerMinute, 0.229f32)),
    TemperatureLimit: 31, 1, Access::Rw, Area::Eeprom, Some(UnitScale::new(Unit::DegreesCelsius, 1f32)),
    MaxVoltageLimit: 32, 2, Access::Rw, Area::Eeprom, Some(UnitScale::new(Unit::Volt, 0.1f32)),
    MinVoltageLimit: 34, 2, Access::Rw, Area::Eeprom, Some(UnitScale::new(Unit::Volt, 0.1f32)),
    PwmLimit: 36, 2, Access::Rw, Area::Eeprom, Some(UnitScale::new(Unit::Percent, 0.113f32)),
    CurrentLimit: 38, 2, Access::Rw, Area::Eeprom, Some(UnitScale::new(Unit::Ampere, 0.00269f32)),
    VelocityLimit: 44, 4, Access::Rw, Area::Eeprom, Some(UnitScale::new(Unit::RevPerMinute, 0.229f32)),
    MaxPositionLimit: 48, 4, Access::Rw, Area::Eeprom, Some(UnitScale::new(Unit::Pulse, 1f32)),
    MinPositionLimit: 52, 4, Access::Rw, Area::Eeprom, Some(UnitScale::new(Unit::Pulse, 1f32)),
    StartupConfiguration: 60, 1, Access::Rw, Area::Eeprom, None,
    Shutdown: 63, 1, Access::Rw, Area::Eeprom, None,
    TorqueEnable: 64, 1, Access::Rw, Area::Ram, None,
    StatusReturnLevel: 68, 1, Access::Rw, Area::Ram, None,
    RegisteredInstruction: 69, 1, Access::R, Area::Ram, None,
    HardwareErrorStatus: 70, 1, Access::R, Area::Ram, None,
    VelocityIGain: 76, 2, Access::Rw, Area::Ram, None,
    VelocityPGain: 78, 2, Access::Rw, Area::Ram, None,
    PositionPGain: 84, 2, Access::Rw, Area::Ram, None,
    BusWatchdog: 98, 1, Access::Rw, Area::Ram, Some(UnitScale::new(Unit::Second, 0.02f32)),
    GoalPwm: 100, 2, Access::Rw, Area::Ram, Some(UnitScale::new(Unit::Percent, 0.113f32)),
    GoalCurrent: 102, 2, Access::Rw, Area::Ram, Some(UnitScale::new(Unit::Ampere, 0.00269f32)),
    GoalVelocity: 104, 4, Access::Rw, Area::Ram, Some(UnitScale::new(Unit::RevPerMinute, 0.229f32)),
    ProfileAcceleration: 108, 4, Access::Rw, Area::Ram, Some(UnitScale::new(Unit::RevPerMinuteSquared, 214.577f32)),
    ProfileVelocity: 112, 4, Access::Rw, Area::Ram, Some(UnitScale::new(Unit::RevPerMinute, 0.229f32)),
    GoalPosition: 116, 4, Access::Rw, Area::Ram, Some(UnitScale::new(Unit::Pulse, 1f32)),
    Moving: 122, 1, Access::R, Area::Ram, None,
    MovingStatus: 123, 1, Access::R, Area::Ram, None,
    PresentPwm: 124, 2, Access::R, Area::Ram, None,
    PresentCurrent: 126, 2, Access::R, Area::Ram, Some(UnitScale::new(Unit::Ampere, 0.00269f32)),
    PresentVelocity: 128, 4, Access::R, Area::Ram, Some(UnitScale::new(Unit::RevPerMinute, 0.229f32)),
    PresentPosition: 132, 4, Access::R, Area::Ram, Some(UnitScale::new(Unit::Pulse, 1f32)),
    VelocityTrajectory: 136, 4, Access::R, Area::Ram, Some(UnitScale::new(Unit::RevPerMinute, 0.229f32)),
    PositionTrajectory: 140, 4, Access::R, Area::Ram, Some(UnitScale::new(Unit::Pulse, 1f32)),
    PresentInputVoltage: 144, 2, Access::R, Area::Ram, Some(UnitScale::new(Unit::Volt, 0.1f32)),
    PresentTemperature: 146, 1, Access::R, Area::Ram, Some(UnitScale::new(Unit::DegreesCelsius, 1f32)),
    BackupReady: 147, 1, Access::R, Area::Ram, None,
    PositionDGain: 80, 2, Access::Rw, Area::Ram, None,
    PositionIGain: 82, 2, Access::Rw, Area::Ram, None,
    ProtocolType: 13, 1, Access::Rw, Area::Eeprom, None,
    Feedforward2NdGain: 88, 2, Access::Rw, Area::Ram, None,
    Feedforward1StGain: 90, 2, Access::Rw, Area::Ram, None,
    RealtimeTick: 120, 2, Access::R, Area::Ram, Some(UnitScale::new(Unit::Second, 0.001f32)),
}];

impl XW540 {
    /// Contiguous Indirect Address runs as `(address, byte length)` — sync-usable windows.
    pub const fn indirect_address_blocks() -> &'static [crate::IndirectRange] {
        const BLOCKS: &[crate::IndirectRange] = &[
            crate::IndirectRange::new(168, 56),
            crate::IndirectRange::new(578, 56),
        ];
        BLOCKS
    }
    /// Contiguous Indirect Data runs as `(address, byte length)` — sync-usable windows.
    pub const fn indirect_data_blocks() -> &'static [crate::IndirectRange] {
        const BLOCKS: &[crate::IndirectRange] = &[
            crate::IndirectRange::new(224, 28),
            crate::IndirectRange::new(634, 28),
        ];
        BLOCKS
    }
}
