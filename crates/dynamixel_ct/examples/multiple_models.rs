use dynamixel_ct::models::{XM430, YM070};
use dynamixel_ct::{ControlTable, Model, ModelGroup, Register::*};

fn main() {
    let model = Model::try_from(1030).unwrap();
    let ct_model = ControlTable::from(model);
    let model_group = ModelGroup::PH42;
    let ct_model_group = ControlTable::from(model_group);

    println!("{:?}", XM430::get(GoalPosition));
    println!("{:?}", YM070::get(GoalPosition));
    println!("{:?}", ct_model.get(GoalPosition));
    println!("{:?}", ct_model_group.get(GoalPosition));
}
