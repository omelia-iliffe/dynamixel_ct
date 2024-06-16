//! The [`ControlTable`] Trait provides a way to access the registers of a Dynamixel Servo
//! without knowing the exact model.

use crate::register::RegisterData;

macro_rules! control_table_gen {
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
                fn $reg(&self) -> Option<RegisterData> {
                    None
                }
            )+
        }
    }
}
control_table_gen!(
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
    motor_temperature_limit,
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
    status_return_level,
    registered_instruction,
    hardware_error_status,
    velocity_i_gain,
    velocity_p_gain,
    position_p_gain,
    feedforward_2nd_gain,
    feedforward_1st_gain,
    bus_watchdog,
    goal_pwm,
    goal_current,
    goal_velocity,
    profile_acceleration,
    profile_velocity,
    goal_position,
    real_time_tick,
    moving,
    moving_status,
    present_pwm,
    present_current,
    present_velocity,
    present_position,
    velocity_trajectory,
    position_trajectory,
    present_input_voltage,
    present_temperature,
    present_motor_temperature,
    backup_ready,
    startup_config,
    in_position_threshold,
    following_error_threshold,
    gear_ratio_numerator,
    gear_ratio_denominator,
    safe_stop_time,
    brake_delay,
    goal_update_delay,
    overexcitation_voltage,
    normal_excitation_voltage,
    overexcitation_time,
    notch_filter_frequency,
    notch_filter_bandwidth,
    notch_filter_depth,
    present_velocity_lpf_frequency,
    goal_current_lpf_frequency,
    position_ff_lpf_time,
    velocity_ff_lpf_time,
    control_state,
    error_code,
    gain_save,
    velocity_ff_gain,
    position_d_gain,
    position_i_gain,
    position_ff_gain,
    profile_acceleration_time,
    profile_time,
    pwm_offset,
    current_offset,
    velocity_offset,
);
