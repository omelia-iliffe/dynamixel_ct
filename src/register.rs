//! This module defines the RegisterData struct and each possible register in the control table.

/// RegisterData is used to store the address, length and type of register.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RegisterData {
    /// The address of the register
    pub address: u16,
    /// The number of bytes in the register
    pub length: u16,
}

#[allow(non_camel_case_types, missing_docs)]
#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Register {
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
}