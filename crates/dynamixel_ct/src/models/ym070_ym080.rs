//! Dynamixel XM430 model definitions.

use crate::model;


model![YM070 YM080 {
ModelNumber: 0, 2,
ModelInformation: 2, 4,
FirmwareVersion: 6, 1,
Id: 7, 1,
BaudRate: 12, 1,
ReturnDelayTime: 13, 1,
DriveMode: 32, 1,
OperatingMode: 33, 1,
SecondaryId: 10, 1,
HomingOffset: 52, 4,
MovingThreshold: 48, 4,
MotorTemperatureLimit: 57, 1,
MaxVoltageLimit: 60, 2,
MinVoltageLimit: 62, 2,
PwmLimit: 64, 2,
CurrentLimit: 66, 2,
AccelerationLimit: 68, 4,
VelocityLimit: 72, 4,
MaxPositionLimit: 76, 4,
MinPositionLimit: 84, 4,
StartupConfiguration: 34, 1,
TorqueEnable: 512, 1,
Led: 513, 1,
StatusReturnLevel: 15, 1,
RegisteredInstruction: 16, 1,
VelocityIGain: 212, 4,
VelocityPGain: 216, 4,
PositionPGain: 232, 4,
BusWatchdog: 8, 2,
GoalPwm: 524, 2,
GoalCurrent: 526, 2,
GoalVelocity: 528, 4,
ProfileAcceleration: 240, 4,
ProfileVelocity: 244, 4,
GoalPosition: 532, 4,
MovingStatus: 541, 1,
PresentPwm: 544, 2,
PresentCurrent: 546, 2,
PresentVelocity: 548, 4,
PresentPosition: 552, 4,
VelocityTrajectory: 564, 4,
PositionTrajectory: 560, 4,
PresentInputVoltage: 568, 2,
PresentMotorTemperature: 571, 1,
InPositionThreshold: 40, 4,
FollowingErrorThreshold: 44, 4,
SafeStopTime: 104, 2,
BrakeDelay: 106, 2,
GoalUpdateDelay: 108, 2,
OverexcitationVoltage: 110, 1,
NormalExcitationVoltage: 111, 1,
OverexcitationTime: 112, 2,
PresentVelocityLpfFrequency: 132, 2,
GoalCurrentLpfFrequency: 134, 2,
PositionFfLpfTime: 136, 2,
VelocityFfLpfTime: 138, 2,
ErrorCode: 153, 1,
VelocityFfGain: 220, 4,
PositionDGain: 224, 4,
PositionIGain: 228, 4,
PositionFfGain: 236, 4,
ProfileAccelerationTime: 248, 4,
ProfileTime: 252, 4,
PwmOffset: 516, 2,
CurrentOffset: 518, 2,
VelocityOffset: 520, 4,
ProtocolType: 11, 1,
RealtimeTick: 542, 2,
PresentInverterTemperature: 570, 1,
HybridSave: 170, 1,
ControllerState: 152, 1,
ElectronicGearRatioNumerator: 96, 4,
ElectronicGearRatioDenominator: 100, 4,
InverterTemperatureLimit: 56, 1,
PositionLimitThreshold: 38, 2,
}];
