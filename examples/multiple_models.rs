use dynamixel_ct::{models, ControlTable};
use dynamixel_ct::models::Model;
use dynamixel_ct::register::Register::*;
fn main() {
    let xm430 = models::XM430::new();
    let y = models::YM070::new();
    let model = Model::try_from(1030).unwrap();
    let new = ControlTable::new(model).unwrap();

    println!("{:?}", xm430.get(GoalPosition));
    println!("{:?}", y.get(GoalPosition));
    println!("{:?}", new.get(GoalPosition));
}
