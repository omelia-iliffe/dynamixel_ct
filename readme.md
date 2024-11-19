```
      dP                                       oo                   dP                     dP    
      88                                                            88                     88    
.d888b88 dP    dP 88d888b. .d8888b. 88d8b.d8b. dP dP.  .dP .d8888b. 88          .d8888b. d8888P  
88'  `88 88    88 88'  `88 88'  `88 88'`88'`88 88  `8bd8'  88ooood8 88 88888888 88'  `""   88    
88.  .88 88.  .88 88    88 88.  .88 88  88  88 88  .d88b.  88.  ... 88          88.  ...   88    
`88888P8 `8888P88 dP    dP `88888P8 dP  dP  dP dP dP'  `dP `88888P' dP          `88888P'   dP    
              .88                                                                                
          d8888P
```

# Dynamixel Control Table Library

The goal of this library is to provide the information for each register in the control table of a Dynamixel servo.  
This library is intended to be used with an additional library that provides the communication protocol to the Dynamixel servo.

The data for register is currently limited to address and size (length), however expanding this to include the data type and access level is planned.

The current implementation uses static hashmaps of registers internally so is not currently `no_std` compatible.

## Dynamic Control Table
The library can be used to return a control table when the model number is not known at compile time,
allowing for a more dynamic approach to working with servos.

If the servo model doesn't implement a register, the control table will return `` for the register.

## Supported Servos

Currently supported servos include:
 - XM430
 - XM540
 - XC330
 - YM070

with more to be added in the future.

## Usage

#### Using a known model
```rust
use dynamixel_ct::{models, Register::*};
fn main() {
    let xm430 = models::XM430::new();
    let goal_pos_register = xm430.get(goal_position);
    println!("Goal Position Register: {:?}", goal_pos_register);
    /// Output: Ok(RegisterData { address: 116, length: 4 })
}
```

#### Using a dynamic model
```rust
use dynamixel_ct::{models, ControlTable, Register::*};
fn main() {
    // ping the motor to get the correct model number
    let model_num: u16 = 1030;
    let model = Model::try_from(model_num).unwrap();
    let motor = ControlTable::new(model).unwrap();

    println!("{:?}", motor.get(goal_position));
    /// Output: Ok(RegisterData { address: 116, length: 4 })
}
```




## Other Dynamixel Rust Libraries
A few different libraries exist for communicating with Dynamixel servos in Rust:
- [dynamixel2-rs](https://github.com/robohouse-delft/dynamixel2-rs) by RoboHouse Delft
  - This implements protocol2.0.
- [rustypot](https://github.com/pollen-robotics/rustypot) by Pollen Robotics
  - This library has its own method of implementing registers.
- [dynamixel.rs](https://github.com/kjetilkjeka/dynamixel.rs) by kjetilkjeka (seems to be unmaintained)