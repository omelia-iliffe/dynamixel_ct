//! Ping a servo of unknown model, then use its control table dynamically.
//!
//! Uses the [`dynamixel2`](https://docs.rs/dynamixel2) crate for the actual bus I/O.

use dynamixel2::Bus;
use dynamixel_ct::{ControlTable, Model, Register::*};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut bus = Bus::open("/dev/ttyUSB0", 57_600)?;
    let id = 1;

    // Ping reports the model number; resolve it to a `Model` and its control table.
    let model = Model::try_from(bus.ping(id)?.data.model)?;
    println!("found {model}");

    let table = ControlTable::from(model);
    let present_position = table.get(PresentPosition)?;

    let response = bus.read_u32(id, present_position.address)?;
    println!("present position: {}", response.data);
    Ok(())
}
