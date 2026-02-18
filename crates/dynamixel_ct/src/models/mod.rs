//! The Control Tables for all supported models.
//! These structs can be used with `no_std`

mod xc330_xl330_xm335;
pub use xc330_xl330_xm335::*;
mod xd540_xh540_xm540;
pub use xd540_xh540_xm540::*;
mod xd430_xh430_xm430;
pub use xd430_xh430_xm430::*;
mod xw540;
pub use xw540::*;
mod xc430_xl430;
pub use xc430_xl430::*;
mod ym070_ym080;
pub use ym070_ym080::*;
mod ph42_ph54_pm42_pm54;
pub use ph42_ph54_pm42_pm54::*;

#[expect(unused)]
#[cfg(feature = "std")]
pub(crate) fn control_table_from_model(
    model: &dynamixel_registers::models::Model,
) -> &'static std::collections::HashMap<
    dynamixel_registers::Register,
    dynamixel_registers::RegisterData,
> {
    use dynamixel_registers::models::Model::*;
    match model {
        XC330_T181 | XC330_T288 | XC330_M181 | XC330_M288 => XC330::table(),
        XL330_M077 | XL330_M288 => XL330::table(),
        XM335_T323 => XM335::table(),
        XD540_T270 | XD540_T150 => XD540::table(),
        XH540_W270 | XH540_W150 | XH540_V270 | XH540_V150 => XH540::table(),
        XM540_W270 | XM540_W150 => XM540::table(),
        XD430_T350 | XD430_T210 => XD430::table(),
        XH430_W350 | XH430_W210 | XH430_V350 | XH430_V210 => XH430::table(),
        XM430_W350 | XM430_W210 => XM430::table(),
        XW540_T260 | XW540_T140 | XW540_H260 => XW540::table(),
        XC430_W150 | XC430_W240 => XC430::table(),
        XL430_W250 => XL430::table(),
        YM070_210_M001_RH | YM070_210_B001_RH | YM070_200_R051_RH | YM070_200_R099_RH
        | YM070_210_A051_RH | YM070_200_A099_RH => YM070::table(),
        YM080_230_M001_RH | YM080_230_B001_RH | YM080_230_R051_RH | YM080_230_R099_RH
        | YM080_230_A051_RH | YM080_230_A099_RH => YM080::table(),
        PH42_020_S300_R => PH42::table(),
        PH54_100_S500_R | PH54_200_S500_R => PH54::table(),
        PM42_010_S260_R => PM42::table(),
        PM54_040_S250_R | PM54_060_S250_R => PM54::table(),
        _ => panic!("unknown model"),
    }
}

#[cfg(feature = "std")]
pub(crate) fn control_table_from_model_group(
    model_group: &dynamixel_registers::models::ModelGroup,
) -> &'static std::collections::HashMap<
    dynamixel_registers::Register,
    dynamixel_registers::RegisterData,
> {
    use dynamixel_registers::models::ModelGroup;
    match model_group {
        ModelGroup::XC330 => XC330::table(),
        ModelGroup::XL330 => XL330::table(),
        ModelGroup::XM335 => XM335::table(),
        ModelGroup::XD540 => XD540::table(),
        ModelGroup::XH540 => XH540::table(),
        ModelGroup::XM540 => XM540::table(),
        ModelGroup::XD430 => XD430::table(),
        ModelGroup::XH430 => XH430::table(),
        ModelGroup::XM430 => XM430::table(),
        ModelGroup::XW540 => XW540::table(),
        ModelGroup::XC430 => XC430::table(),
        ModelGroup::XL430 => XL430::table(),
        ModelGroup::YM070 => YM070::table(),
        ModelGroup::YM080 => YM080::table(),
        ModelGroup::PH42 => PH42::table(),
        ModelGroup::PH54 => PH54::table(),
        ModelGroup::PM42 => PM42::table(),
        ModelGroup::PM54 => PM54::table(),
        _ => panic!("unknown model group"),
    }
}
