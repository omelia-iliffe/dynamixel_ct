use dynamixel_ct::{models, ControlTable};
use dynamixel_ct::models::Model;
use dynamixel_ct::register::Register::*;
fn main() {
    let xm430 = models::XM430::new();
    let y = models::YM::new();
    let model = Model::try_from(1030).unwrap();
    let new = ControlTable::new(model).unwrap();

    println!("{:?}", xm430.get(goal_position));
    println!("{:?}", y.get(goal_position));
    println!("{:?}", new.get(goal_position));
}
