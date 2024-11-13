use dynamixel_ct::{models, ControlTable};
fn main() {
    let xm430 = models::XM430;
    let y = models::YM;
    let new = dynamixel_ct::models::try_from_model(1030).unwrap();

    println!("{:?}", xm430.goal_position());
    println!("{:?}", y.goal_position());
    println!("{:?}", new.goal_position());
}
