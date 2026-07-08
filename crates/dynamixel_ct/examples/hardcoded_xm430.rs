//! Talk to a known XM430 using its control table hardcoded at compile time.
//!
//! Uses the [`dynamixel2`](https://docs.rs/dynamixel2) crate for the actual bus I/O.

use dynamixel2::Bus;
use dynamixel_ct::models::XM430;
use dynamixel_ct::Register;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut bus = Bus::open("/dev/ttyUSB0", 57_600)?;
    let id = 1;

    // The register data is known at compile time — no runtime lookup needed.
    let goal_position = XM430::GOAL_POSITION;
    let present_position = XM430::PRESENT_POSITION;

    // `goal_position.length` is 4, so use a 32-bit write.
    bus.write_u32(id, goal_position.address, 2048)?;

    let response = bus.read_u32(id, present_position.address)?;
    println!("present position: {}", response.data);

    // The consts are equivalent to the `get` lookup.
    assert_eq!(XM430::get(Register::GoalPosition), Some(goal_position));
    Ok(())
}
