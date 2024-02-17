//! The [`ControlTable`] Trait provides a way to access the registers of a Dynamixel Servo
//! without knowing the exact model.

use crate::register::RegisterData;

/// If a register is not implemented for a model, this error is returned.
#[derive(Debug)]
pub struct NotImplemented;

macro_rules! CONTROL_TABLE_GEN {
    ($($reg:ident,)+) => {
        /// The [`ControlTable`] Trait provides a way to access the registers of a Dynamixel Servo
        /// The trait provides getter functions for every possible register. If a model does not implement a register, the getter will return [`NotImplemented`].
        ///
        /// # Example
        ///
        /// ```
        /// use dynamixel_ct::ControlTable;
        /// use dynamixel_ct::models::XM430;
        /// use dynamixel_ct::register::RegisterData;
        ///
        /// let model = XM430;
        /// let register = model.model_number().unwrap();
        ///
        /// assert_eq!(register, RegisterData { address: 0, length: 2 });
        /// ```
        pub trait ControlTable {
            $(
                /// Returns the [`RegisterData`] for the $reg register.
                fn $reg(&self) -> Result<RegisterData, NotImplemented> {
                    Err(NotImplemented)
                }
            )+
        }
    }
}
CONTROL_TABLE_GEN!(
    model_number,
    model_information,
    firmware_version,
    id,
    baud_rate,
    return_delay_time,
    drive_mode,
    operating_mode,
    secondary_id,
    protocol_version,
    homing_offset,
    moving_threshold,
    temperature_limit,
    max_voltage_limit,
    min_voltage_limit,
    pwm_limit,
    current_limit,
    acceleration_limit,
    velocity_limit,
    max_position_limit,
    min_position_limit,
    startup_configuration,
    shutdown,
    torque_enable,
    led,
    profile_acceleration,
    profile_velocity,
    goal_position,
    real_time_tick,
    present_current,
    present_velocity,
    present_position,
    present_voltage,
    present_temperature,
    startup_config,
    in_position_threshold,
    following_error_threshold,
);
