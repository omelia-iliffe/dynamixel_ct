//! Drive a servo with radians instead of raw pulses.
//!
//! The pulse<->radian conversion depends on the servo's encoder resolution, which differs
//! by model, so resolve the [`Model`] (here from a ping) before converting.
//!
//! Uses the [`dynamixel2`](https://docs.rs/dynamixel2) crate for the actual bus I/O.

use dynamixel2::Bus;
use dynamixel_ct::{ControlTable, Model, Register::*};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut bus = Bus::open("/dev/ttyUSB0", 57_600)?;
    let id = 1;

    // Ping reports the model number; resolve it to a `Model` and its control table.
    let model = Model::try_from(bus.ping(id)?.data.model)?;
    println!(
        "found {model} ({} pulses/rev)",
        dynamixel_ct::position_resolution(model)
    );

    // Use an exact model so the pulse<->radian conversion always resolves (`ControlTable`'s
    // conversions return `None` when only a `ModelGroup` is known and it is ambiguous).
    let table = ControlTable::new_with_model(model);

    // Drive to a quarter turn, converting radians to the model's pulse count.
    let goal = std::f32::consts::FRAC_PI_2;
    let goal_pulses = table.radians_to_pulses(goal).expect("known resolution");
    let goal_position = table.get(GoalPosition)?;
    bus.write_u32(id, goal_position.address, goal_pulses as u32)?;
    println!("goal: {goal:.3} rad = {goal_pulses} pulses");

    // Read the present position back and report it in radians.
    let present_position = table.get(PresentPosition)?;
    let pulses = bus.read_u32(id, present_position.address)?.data as i32;
    println!(
        "present position: {pulses} pulses = {:.3} rad",
        table.pulses_to_radians(pulses).expect("known resolution")
    );
    Ok(())
}
