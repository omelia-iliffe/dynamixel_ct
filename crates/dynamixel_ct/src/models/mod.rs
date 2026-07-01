//! The Control Tables for all supported models.
//! These structs can be used with `no_std`

mod ph42_ph54_pm42_pm54;
pub use ph42_ph54_pm42_pm54::*;
mod xc330_xl330_xm335;
pub use xc330_xl330_xm335::*;
mod xc430_xl430;
pub use xc430_xl430::*;
mod xd430_xh430_xm430;
pub use xd430_xh430_xm430::*;
mod xd540_xh540_xm540;
pub use xd540_xh540_xm540::*;
mod xw540;
pub use xw540::*;
mod ym070_ym080;
pub use ym070_ym080::*;

#[cfg(feature = "std")]
pub(crate) fn control_table_from_model_group(
    model_group: &dynamixel_registers::models::ModelGroup,
) -> &'static std::collections::HashMap<
    dynamixel_registers::Register,
    dynamixel_registers::RegisterData,
> {
    use dynamixel_registers::models::ModelGroup;
    match model_group {
        ModelGroup::PH42 => PH42::table(),
        ModelGroup::PH54 => PH54::table(),
        ModelGroup::PM42 => PM42::table(),
        ModelGroup::PM54 => PM54::table(),
        ModelGroup::XC330 => XC330::table(),
        ModelGroup::XL330 => XL330::table(),
        ModelGroup::XM335 => XM335::table(),
        ModelGroup::XC430 => XC430::table(),
        ModelGroup::XL430 => XL430::table(),
        ModelGroup::XD430 => XD430::table(),
        ModelGroup::XH430 => XH430::table(),
        ModelGroup::XM430 => XM430::table(),
        ModelGroup::XD540 => XD540::table(),
        ModelGroup::XH540 => XH540::table(),
        ModelGroup::XM540 => XM540::table(),
        ModelGroup::XW540 => XW540::table(),
        ModelGroup::YM070 => YM070::table(),
        ModelGroup::YM080 => YM080::table(),
        _ => panic!("unknown model group"),
    }
}
