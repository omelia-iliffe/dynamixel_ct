//! Dynamixel YM070 YM080 model definitions.

use crate::model;

model![YM070, YM080 => {
    ModelNumber: 0, 2, Access::R, Area::Eeprom, None,
    ModelInformation: 2, 4, Access::R, Area::Eeprom, None,
    FirmwareVersion: 6, 1, Access::R, Area::Eeprom, None,
    Id: 7, 1, Access::Rw, Area::Eeprom, None,
    BaudRate: 12, 1, Access::Rw, Area::Eeprom, None,
    ReturnDelayTime: 13, 1, Access::Rw, Area::Eeprom, Some(UnitScale::new(Unit::Second, 0.000002f32)),
    DriveMode: 32, 1, Access::Rw, Area::Eeprom, None,
    OperatingMode: 33, 1, Access::Rw, Area::Eeprom, None,
    SecondaryId: 10, 1, Access::Rw, Area::Eeprom, None,
    HomingOffset: 52, 4, Access::Rw, Area::Eeprom, Some(UnitScale::new(Unit::Pulse, 1f32)),
    MovingThreshold: 48, 4, Access::Rw, Area::Eeprom, Some(UnitScale::new(Unit::PulsePerMillisecond, 5f32)),
    MotorTemperatureLimit: 57, 1, Access::Rw, Area::Eeprom, Some(UnitScale::new(Unit::DegreesCelsius, 1f32)),
    MaxVoltageLimit: 60, 2, Access::Rw, Area::Eeprom, Some(UnitScale::new(Unit::Volt, 0.1f32)),
    MinVoltageLimit: 62, 2, Access::Rw, Area::Eeprom, Some(UnitScale::new(Unit::Volt, 0.1f32)),
    PwmLimit: 64, 2, Access::Rw, Area::Eeprom, Some(UnitScale::new(Unit::Percent, 0.1f32)),
    CurrentLimit: 66, 2, Access::Rw, Area::Eeprom, Some(UnitScale::new(Unit::Ampere, 0.01f32)),
    AccelerationLimit: 68, 4, Access::Rw, Area::Eeprom, Some(UnitScale::new(Unit::RevPerMinuteSquared, 10f32)),
    VelocityLimit: 72, 4, Access::Rw, Area::Eeprom, Some(UnitScale::new(Unit::RevPerMinute, 0.01f32)),
    MaxPositionLimit: 76, 4, Access::Rw, Area::Eeprom, Some(UnitScale::new(Unit::Pulse, 1f32)),
    MinPositionLimit: 84, 4, Access::Rw, Area::Eeprom, Some(UnitScale::new(Unit::Pulse, 1f32)),
    StartupConfiguration: 34, 1, Access::Rw, Area::Eeprom, None,
    TorqueEnable: 512, 1, Access::Rw, Area::Ram, None,
    Led: 513, 1, Access::Rw, Area::Ram, None,
    StatusReturnLevel: 15, 1, Access::Rw, Area::Ram, None,
    RegisteredInstruction: 16, 1, Access::R, Area::Ram, None,
    VelocityIGain: 212, 4, Access::Rw, Area::Hybrid, None,
    VelocityPGain: 216, 4, Access::Rw, Area::Hybrid, None,
    PositionPGain: 232, 4, Access::Rw, Area::Hybrid, None,
    BusWatchdog: 8, 2, Access::Rw, Area::Ram, None,
    GoalPwm: 524, 2, Access::Rw, Area::Ram, Some(UnitScale::new(Unit::Percent, 0.1f32)),
    GoalCurrent: 526, 2, Access::Rw, Area::Ram, Some(UnitScale::new(Unit::Ampere, 0.01f32)),
    GoalVelocity: 528, 4, Access::Rw, Area::Ram, Some(UnitScale::new(Unit::RevPerMinute, 0.01f32)),
    ProfileAcceleration: 240, 4, Access::Rw, Area::Hybrid, Some(UnitScale::new(Unit::RevPerMinuteSquared, 10f32)),
    ProfileVelocity: 244, 4, Access::Rw, Area::Hybrid, Some(UnitScale::new(Unit::RevPerMinute, 0.01f32)),
    GoalPosition: 532, 4, Access::Rw, Area::Ram, Some(UnitScale::new(Unit::Pulse, 1f32)),
    MovingStatus: 541, 1, Access::R, Area::Ram, None,
    PresentPwm: 544, 2, Access::R, Area::Ram, Some(UnitScale::new(Unit::Percent, 0.1f32)),
    PresentCurrent: 546, 2, Access::R, Area::Ram, Some(UnitScale::new(Unit::Ampere, 0.01f32)),
    PresentVelocity: 548, 4, Access::R, Area::Ram, Some(UnitScale::new(Unit::RevPerMinute, 0.01f32)),
    PresentPosition: 552, 4, Access::R, Area::Ram, Some(UnitScale::new(Unit::Pulse, 1f32)),
    VelocityTrajectory: 564, 4, Access::R, Area::Ram, Some(UnitScale::new(Unit::PulsePerSecond, 1f32)),
    PositionTrajectory: 560, 4, Access::R, Area::Ram, Some(UnitScale::new(Unit::Pulse, 1f32)),
    PresentInputVoltage: 568, 2, Access::R, Area::Ram, Some(UnitScale::new(Unit::Volt, 0.1f32)),
    PresentMotorTemperature: 571, 1, Access::R, Area::Ram, Some(UnitScale::new(Unit::DegreesCelsius, 1f32)),
    InPositionThreshold: 40, 4, Access::Rw, Area::Eeprom, Some(UnitScale::new(Unit::Pulse, 1f32)),
    FollowingErrorThreshold: 44, 4, Access::Rw, Area::Eeprom, Some(UnitScale::new(Unit::Pulse, 1f32)),
    SafeStopTime: 104, 2, Access::Rw, Area::Eeprom, Some(UnitScale::new(Unit::Second, 0.001f32)),
    BrakeDelay: 106, 2, Access::Rw, Area::Eeprom, Some(UnitScale::new(Unit::Second, 0.001f32)),
    GoalUpdateDelay: 108, 2, Access::Rw, Area::Eeprom, Some(UnitScale::new(Unit::Second, 0.001f32)),
    OverexcitationVoltage: 110, 1, Access::Rw, Area::Eeprom, Some(UnitScale::new(Unit::Percent, 1f32)),
    NormalExcitationVoltage: 111, 1, Access::Rw, Area::Eeprom, Some(UnitScale::new(Unit::Percent, 1f32)),
    OverexcitationTime: 112, 2, Access::Rw, Area::Eeprom, Some(UnitScale::new(Unit::Second, 0.001f32)),
    PresentVelocityLpfFrequency: 132, 2, Access::Rw, Area::Eeprom, Some(UnitScale::new(Unit::Hertz, 0.1f32)),
    GoalCurrentLpfFrequency: 134, 2, Access::Rw, Area::Eeprom, Some(UnitScale::new(Unit::Hertz, 0.1f32)),
    PositionFfLpfTime: 136, 2, Access::Rw, Area::Eeprom, Some(UnitScale::new(Unit::Second, 0.0002f32)),
    VelocityFfLpfTime: 138, 2, Access::Rw, Area::Eeprom, Some(UnitScale::new(Unit::Second, 0.0002f32)),
    ErrorCode: 153, 1, Access::R, Area::Ram, None,
    VelocityFfGain: 220, 4, Access::Rw, Area::Hybrid, None,
    PositionDGain: 224, 4, Access::Rw, Area::Hybrid, None,
    PositionIGain: 228, 4, Access::Rw, Area::Hybrid, None,
    PositionFfGain: 236, 4, Access::Rw, Area::Hybrid, None,
    ProfileAccelerationTime: 248, 4, Access::Rw, Area::Hybrid, Some(UnitScale::new(Unit::Second, 0.0002f32)),
    ProfileTime: 252, 4, Access::Rw, Area::Hybrid, Some(UnitScale::new(Unit::Second, 0.0002f32)),
    PwmOffset: 516, 2, Access::Rw, Area::Ram, Some(UnitScale::new(Unit::Percent, 0.1f32)),
    CurrentOffset: 518, 2, Access::Rw, Area::Ram, Some(UnitScale::new(Unit::Ampere, 0.01f32)),
    VelocityOffset: 520, 4, Access::Rw, Area::Ram, Some(UnitScale::new(Unit::RevPerMinute, 0.01f32)),
    ProtocolType: 11, 1, Access::Rw, Area::Eeprom, None,
    RealtimeTick: 542, 2, Access::R, Area::Ram, Some(UnitScale::new(Unit::Second, 0.001f32)),
    PresentInverterTemperature: 570, 1, Access::R, Area::Ram, Some(UnitScale::new(Unit::DegreesCelsius, 1f32)),
    HybridSave: 170, 1, Access::Rw, Area::Ram, None,
    ControllerState: 152, 1, Access::R, Area::Ram, None,
    ElectronicGearRatioNumerator: 96, 4, Access::Rw, Area::Eeprom, None,
    ElectronicGearRatioDenominator: 100, 4, Access::Rw, Area::Eeprom, None,
    InverterTemperatureLimit: 56, 1, Access::Rw, Area::Eeprom, Some(UnitScale::new(Unit::DegreesCelsius, 1f32)),
    PositionLimitThreshold: 38, 2, Access::Rw, Area::Eeprom, Some(UnitScale::new(Unit::Pulse, 1f32)),
}];

impl YM070 {
    /// Contiguous Indirect Address runs as `(address, byte length)` — sync-usable windows.
    pub const fn indirect_address_blocks() -> &'static [crate::IndirectRange] {
        const BLOCKS: &[crate::IndirectRange] = &[crate::IndirectRange::new(256, 256)];
        BLOCKS
    }
    /// Contiguous Indirect Data runs as `(address, byte length)` — sync-usable windows.
    pub const fn indirect_data_blocks() -> &'static [crate::IndirectRange] {
        const BLOCKS: &[crate::IndirectRange] = &[crate::IndirectRange::new(634, 128)];
        BLOCKS
    }
}

impl YM080 {
    /// Contiguous Indirect Address runs as `(address, byte length)` — sync-usable windows.
    pub const fn indirect_address_blocks() -> &'static [crate::IndirectRange] {
        const BLOCKS: &[crate::IndirectRange] = &[crate::IndirectRange::new(256, 256)];
        BLOCKS
    }
    /// Contiguous Indirect Data runs as `(address, byte length)` — sync-usable windows.
    pub const fn indirect_data_blocks() -> &'static [crate::IndirectRange] {
        const BLOCKS: &[crate::IndirectRange] = &[crate::IndirectRange::new(634, 128)];
        BLOCKS
    }
}
