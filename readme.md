# Dynamixel Control Table

The goal of this library is to provide the information for each register in the control table of a Dynamixel servo.  
This library is intended to be used with an additional library that provides the communication protocol to the Dynamixels.

The data for register is currently limited to address and size (length), however expanding this to include the data type and access level is planned.

## Dynamic Control Table
**Only available with `std` feature**  
The library can be used to return a control table when the model number is not known at compile time,
allowing for a more dynamic approach to working with servos.

If the Dynamixel model doesn't implement a register, the control table will return `None`.

## Supported Servos
This crate focuses on Dynamixels that use Protocol2.0.  
Currently supported servos include:
 - X series
 - Y series
 - P series

MX will not be supported.

## Usage

#### Using a known model
```rust
use dynamixel_ct::{models::XM430, Register::*};
fn main() {
    println!("{:?}", XM430::get(goal_position););
    println!("{:?}", XM430::goal_position(););
    /// Output: Some(RegisterData { address: 116, length: 4 })
}
```

#### Using a dynamic model
```rust
use dynamixel_ct::{models, ControlTable, Register::*};
fn main() {
    // ping the motor to get the correct model number
    let model_num: u16 = 1030;
    let model = Model::try_from(model_num).unwrap();
    let control_table = ControlTable::from(model);

    println!("{:?}", control_table.get(goal_position));
    /// Output: Some(RegisterData { address: 116, length: 4 })
}
```


## Other Dynamixel Rust Libraries
A few different libraries exist for communicating with Dynamixel servos in Rust:
- [dynamixel2-rs](https://github.com/robohouse-delft/dynamixel2-rs) by RoboHouse Delft
  - This implements protocol2.0.
- [rustypot](https://github.com/pollen-robotics/rustypot) by Pollen Robotics
  - This library has its own method of implementing registers.
- [dynamixel.rs](https://github.com/kjetilkjeka/dynamixel.rs) by kjetilkjeka (seems to be unmaintained)