//! Conversions between raw position pulses and radians.
//!
//! DYNAMIXEL position registers are expressed in encoder *pulses*. To drive a servo with
//! an angle you need its position resolution — the number of pulses in one full
//! revolution — which differs by model (see `dynamixel_ct::position_resolution`).
//!
//! The mapping is linear with pulse `0` at `0` radians. Any centre offset (e.g. the
//! Homing Offset register) is the caller's responsibility.

use core::f32::consts::PI;

/// Convert a raw position value in `pulses` to radians, given the model's `resolution`
/// (pulses per revolution).
pub fn pulses_to_radians(pulses: i32, resolution: u32) -> f32 {
    pulses as f32 * (2.0 * PI / resolution as f32)
}

/// Convert an angle in `radians` to the nearest raw position value in pulses, given the
/// model's `resolution` (pulses per revolution).
pub fn radians_to_pulses(radians: f32, resolution: u32) -> i32 {
    let pulses = radians * (resolution as f32 / (2.0 * PI));
    // Round half away from zero without `f32::round`, which is unavailable in `no_std`.
    let rounded = if pulses >= 0.0 {
        pulses + 0.5
    } else {
        pulses - 0.5
    };
    rounded as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_and_rounding() {
        // Half a turn of a 4096-pulse encoder is 2048 pulses.
        assert_eq!(radians_to_pulses(PI, 4096), 2048);
        // A full turn back to radians.
        assert!((pulses_to_radians(4096, 4096) - 2.0 * PI).abs() < 1e-4);
        // Rounds to the nearest pulse rather than truncating (also for negatives).
        assert_eq!(radians_to_pulses(pulses_to_radians(1023, 4096), 4096), 1023);
        assert_eq!(
            radians_to_pulses(pulses_to_radians(-1023, 4096), 4096),
            -1023
        );
    }
}
