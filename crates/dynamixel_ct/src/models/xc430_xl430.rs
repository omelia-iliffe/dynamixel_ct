use crate::model;


model![XC430 XL430 => {
ModelNumber: 0, 2,
ModelInformation: 2, 4,
FirmwareVersion: 6, 1,
Id: 7, 1,
BaudRate: 8, 1,
ReturnDelayTime: 9, 1,
DriveMode: 10, 1,
OperatingMode: 11, 1,
SecondaryId: 12, 1,
HomingOffset: 20, 4,
MovingThreshold: 24, 4,
TemperatureLimit: 31, 1,
MaxVoltageLimit: 32, 2,
MinVoltageLimit: 34, 2,
PwmLimit: 36, 2,
VelocityLimit: 44, 4,
MaxPositionLimit: 48, 4,
MinPositionLimit: 52, 4,
StartupConfiguration: 60, 1,
Shutdown: 63, 1,
TorqueEnable: 64, 1,
Led: 65, 1,
StatusReturnLevel: 68, 1,
RegisteredInstruction: 69, 1,
HardwareErrorStatus: 70, 1,
VelocityIGain: 76, 2,
VelocityPGain: 78, 2,
PositionPGain: 84, 2,
BusWatchdog: 98, 1,
GoalPwm: 100, 2,
GoalVelocity: 104, 4,
ProfileAcceleration: 108, 4,
ProfileVelocity: 112, 4,
GoalPosition: 116, 4,
Moving: 122, 1,
MovingStatus: 123, 1,
PresentPwm: 124, 2,
PresentVelocity: 128, 4,
PresentPosition: 132, 4,
VelocityTrajectory: 136, 4,
PositionTrajectory: 140, 4,
PresentInputVoltage: 144, 2,
PresentTemperature: 146, 1,
BackupReady: 147, 1,
PositionDGain: 80, 2,
PositionIGain: 82, 2,
ProtocolType: 13, 1,
Feedforward2NdGain: 88, 2,
Feedforward1StGain: 90, 2,
RealtimeTick: 120, 2,
PresentLoad: 126, 2,
}];
