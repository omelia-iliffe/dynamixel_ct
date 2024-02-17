//! Dynamixel XM430 model definitions.

use crate::{model, RegisterData};

model![YM070 {
        model_number: 0, 2,
        model_information: 2, 4,
        firmware_version: 6, 1,
        id: 7, 1,
        baud_rate: 8, 1,
        return_delay_time: 9, 1,
        drive_mode: 10, 1,
        operating_mode: 11, 1,
        secondary_id: 12, 1,
        protocol_version: 13, 1,
        homing_offset: 20, 4,
        moving_threshold: 24, 4,
        temperature_limit: 31, 1,
        max_voltage_limit: 32, 2,
        min_voltage_limit: 34, 2,
        pwm_limit: 36, 2,
        current_limit: 38, 2,
        acceleration_limit: 40, 4,
        velocity_limit: 44, 4,
        max_position_limit: 48, 4,
        min_position_limit: 52, 4,
        startup_configuration: 60, 1,
        shutdown: 63, 1,

        torque_enable: 64, 1,
        led: 65, 1,

        profile_acceleration: 108, 4,
        profile_velocity: 112, 4,
        goal_position: 116, 4,
        real_time_tick: 120, 4,

        present_current: 126, 2,
        present_velocity: 128, 4,
        present_position: 132, 4,

        present_voltage: 144, 1,
        present_temperature: 146, 1,
    }
];
