//! Dynamixel XM430 model definitions.

use crate::model;

model![XM430 {
	ModelNumber:0,2,
	ModelInformation:2,4,
	FirmwareVersion:6,1,
	Id:7,1,
	BaudRate:8,1,
	ReturnDelayTime:9,1,
	DriveMode:10,1,
	OperatingMode:11,1,
	SecondaryId:12,1,
	ProtocolVersion:13,1,
	HomingOffset:20,4,
	MovingThreshold:24,4,
	TemperatureLimit:31,1,
	MaxVoltageLimit:32,2,
	MinVoltageLimit:34,2,
	PwmLimit:36,2,
	CurrentLimit:38,2,
	AccelerationLimit:40,4,
	VelocityLimit:44,4,
	MaxPositionLimit:48,4,
	MinPositionLimit:52,4,
	StartupConfiguration:60,1,
	Shutdown:63,1,

	TorqueEnable:64,1,
	Led:65,1,
	StatusReturnLevel:68,1,
	RegisteredInstruction:69,1,
	HardwareErrorStatus:70,1,
	VelocityIGain:76,2,
	VelocityPGain:78,2,
	PositionDGain:80,2,
	PositionIGain:82,2,
	PositionPGain:84,2,
	Feedforward2ndGain:88,2,
	Feedforward1stGain:90,2,
	BusWatchdog:98,1,
	GoalPwm:100,2,
	GoalCurrent:102,2,
	GoalVelocity:104,4,
	ProfileAcceleration:108,4,
	ProfileVelocity:112,4,
	GoalPosition:116,4,

	RealTimeTick:120,4,
	Moving:122,1,
	MovingStatus:123,1,
	PresentPwm:124,2,
	PresentCurrent:126,2,
	PresentVelocity:128,4,
	VelocityTrajectory:132,4,
	PositionTrajectory:136,4,
	PresentPosition:132,4,
	PresentInputVoltage:144,1,
	PresentTemperature:146,1,
	BackupReady:168,1,
    }
];
