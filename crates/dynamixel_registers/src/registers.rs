//! This module defines the RegisterData struct and each possible register in the control table.

/// The physical unit a register's value is expressed in.
///
/// Units are normalised to a single base unit per physical quantity, with any
/// metric prefix folded into [`RegisterData::unit_scale`] (e.g. a `mA` value
/// becomes [`Unit::Ampere`] with the scale multiplied by `0.001`).
#[allow(missing_docs)]
#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash, Ord, PartialOrd, derive_more::Display)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize, strum::EnumString)
)]
#[non_exhaustive]
pub enum Unit {
    Pulse,
    RevPerMinute,
    RevPerMinuteSquared,
    PulsePerSecond,
    PulsePerMillisecond,
    Percent,
    Ampere,
    Volt,
    VoltPerSecond,
    DegreesCelsius,
    Second,
    Hertz,
}

/// Whether a register can be written, or is read-only.
#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash, Ord, PartialOrd, derive_more::Display)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize, strum::EnumString)
)]
pub enum Access {
    /// Read only.
    R,
    /// Read and write.
    Rw,
}

/// The control-table memory area a register lives in.
#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash, Ord, PartialOrd, derive_more::Display)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize, strum::EnumString)
)]
pub enum Area {
    /// Non-volatile memory: persists across power cycles, and requires torque to be
    /// disabled before it can be written.
    Eeprom,
    /// Volatile memory: reset to the register's initial value on power-up.
    Ram,
    /// Writable like RAM, but its value can be persisted to non-volatile memory via the
    /// Hybrid Save instruction (Y series).
    Hybrid,
}

/// A register's physical unit together with the scale factor applied to its raw value.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UnitScale {
    /// The base physical unit of the scaled value.
    pub unit: Unit,
    /// Multiply the raw register value by this to get the value in [`unit`](Self::unit).
    pub scale: f32,
}

impl UnitScale {
    /// Create a new [`UnitScale`].
    pub const fn new(unit: Unit, scale: f32) -> Self {
        Self { unit, scale }
    }
}

/// RegisterData is used to store the address, length and unit of a register.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub struct RegisterData {
    /// The address of the register
    pub address: u16,
    /// The number of bytes in the register
    pub length: u16,
    /// Whether the register is read-only or writable.
    pub access: Access,
    /// The memory area the register lives in.
    pub area: Area,
    /// The unit and scale of the register's value, if it represents a physical quantity
    /// (and is unambiguous across every model sharing this control table).
    pub unit: Option<UnitScale>,
}

impl RegisterData {
    /// Create a new [`RegisterData`].
    pub const fn new(
        address: u16,
        length: u16,
        access: Access,
        area: Area,
        unit: Option<UnitScale>,
    ) -> Self {
        Self {
            address,
            length,
            access,
            area,
            unit,
        }
    }
}

#[allow(missing_docs)]
#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash, Ord, PartialOrd, derive_more::Display)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize, strum::EnumString)
)]
pub enum Register {
    ModelNumber,
    ModelInformation,
    FirmwareVersion,
    Id,
    BaudRate,
    ReturnDelayTime,
    DriveMode,
    OperatingMode,
    SecondaryId,
    ProtocolVersion,
    HomingOffset,
    MovingThreshold,
    TemperatureLimit,
    MotorTemperatureLimit,
    MaxVoltageLimit,
    MinVoltageLimit,
    PwmLimit,
    CurrentLimit,
    AccelerationLimit,
    VelocityLimit,
    MaxPositionLimit,
    MinPositionLimit,
    StartupConfiguration,
    Shutdown,
    TorqueEnable,
    Led,
    StatusReturnLevel,
    RegisteredInstruction,
    HardwareErrorStatus,
    VelocityIGain,
    VelocityPGain,
    PositionPGain,
    Feedforward2ndGain,
    Feedforward1stGain,
    BusWatchdog,
    GoalPwm,
    GoalCurrent,
    GoalVelocity,
    ProfileAcceleration,
    ProfileVelocity,
    GoalPosition,
    RealTimeTick,
    Moving,
    MovingStatus,
    PresentPwm,
    PresentCurrent,
    PresentVelocity,
    PresentPosition,
    VelocityTrajectory,
    PositionTrajectory,
    PresentInputVoltage,
    PresentTemperature,
    PresentMotorTemperature,
    BackupReady,
    StartupConfig,
    InPositionThreshold,
    FollowingErrorThreshold,
    GearRatioNumerator,
    GearRatioDenominator,
    SafeStopTime,
    BrakeDelay,
    GoalUpdateDelay,
    OverexcitationVoltage,
    NormalExcitationVoltage,
    OverexcitationTime,
    NotchFilterFrequency,
    NotchFilterBandwidth,
    NotchFilterDepth,
    PresentVelocityLpfFrequency,
    GoalCurrentLpfFrequency,
    PositionFfLpfTime,
    VelocityFfLpfTime,
    ControlState,
    ErrorCode,
    GainSave,
    VelocityFfGain,
    PositionDGain,
    PositionIGain,
    PositionFfGain,
    ProfileAccelerationTime,
    ProfileTime,
    PwmOffset,
    CurrentOffset,
    VelocityOffset,
    ProtocolType,
    ExternalPortMode1,
    ExternalPortMode2,
    ExternalPortMode3,
    ExternalPortMode4,
    LedRed,
    LedGreen,
    LedBlue,
    Feedforward2NdGain,
    Feedforward1StGain,
    RealtimeTick,
    ExternalPortData1,
    ExternalPortData2,
    ExternalPortData3,
    ExternalPortData4,
    PresentInverterTemperature,
    HybridSave,
    ControllerState,
    ElectronicGearRatioNumerator,
    ElectronicGearRatioDenominator,
    InverterTemperatureLimit,
    PositionLimitThreshold,
    PwmSlope,
    PresentLoad,
}
