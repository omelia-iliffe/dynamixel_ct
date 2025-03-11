use dynamixel_ct::{models, ControlTable, Model, ModelGroup, Register::*};

fn main() {
    let xm430 = models::XM430::new();
    let y = models::YM070::new();
    let model = Model::try_from(1030).unwrap();
    let ct_model = ControlTable::from(model);
    let model_group = ModelGroup::PH42;
    let ct_model_group = ControlTable::from(model_group);

    println!("{:?}", xm430.get(GoalPosition));
    println!("{:?}", y.get(GoalPosition));
    println!("{:?}", ct_model.get(GoalPosition));
    println!("{:?}", ct_model_group.get(GoalPosition));
}
