//! Mirror scattered registers into an indirect block so they can be read in one transfer.
//!
//! Each Indirect Address register (2 bytes) points at one byte of a real register; the
//! matching Indirect Data register then mirrors that byte. Pointing consecutive indirect
//! addresses at the bytes of Present Current and Present Position makes them contiguous, so
//! a single read returns both.
//!
//! For reading the same window across a mix of models, see
//! [`dynamixel_ct::common_indirect_data`] and `dynamixel2`'s `sync_read`.
//!
//! Uses the [`dynamixel2`](https://docs.rs/dynamixel2) crate for the actual bus I/O.

use dynamixel2::Bus;
use dynamixel_ct::models::XM430;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut bus = Bus::open("/dev/ttyUSB0", 57_600)?;
    let id = 1;

    // First contiguous run of the config (address) and mirror (data) registers.
    let address_block = XM430::indirect_address_blocks()[0]; // e.g. { address: 168, length: 56 }
    let data_block = XM430::indirect_data_blocks()[0]; //       e.g. { address: 224, length: 28 }

    // Byte addresses of the registers we want to mirror:
    // Present Current (126, 2 bytes) then Present Position (132, 4 bytes).
    let targets: [u16; 6] = [126, 127, 132, 133, 134, 135];
    assert!(targets.len() as u16 <= data_block.length);

    // Write those addresses into consecutive Indirect Address registers.
    let mut config = Vec::new();
    for target in targets {
        config.extend_from_slice(&target.to_le_bytes());
    }
    bus.write(id, address_block.address, &config)?;

    // The Indirect Data registers now mirror those 6 bytes contiguously.
    let response = bus.read(id, data_block.address, targets.len() as u16)?;
    let data = response.data;
    let current = u16::from_le_bytes([data[0], data[1]]);
    let position = u32::from_le_bytes([data[2], data[3], data[4], data[5]]);
    println!("present current: {current}, present position: {position}");
    Ok(())
}
