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
mod xc330t;
pub use xc330t::*;
mod xc330m;
pub use xc330m::*;
mod xh430w;
pub use xh430w::*;
mod xh430v;
pub use xh430v::*;
mod qc060_qc080;
pub use qc060_qc080::*;

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
        ModelGroup::YM110 => YM080::table(),
        ModelGroup::QC060 => QC060::table(),
        ModelGroup::QC080 => QC080::table(),
        _ => unreachable!(),
    }
}

/// The [`RegisterData`](dynamixel_registers::RegisterData) for `register` on any model in
/// `model_group`, or `None` if the group has no such register. Usable without `std`.
///
/// Every [`ModelGroup`](dynamixel_registers::models::ModelGroup) has a table, so the
/// wildcard arm its `#[non_exhaustive]` forces is unreachable; the `all_models_covered`
/// test holds that.
pub const fn register_data_from_model_group(
    model_group: dynamixel_registers::models::ModelGroup,
    register: dynamixel_registers::Register,
) -> Option<dynamixel_registers::RegisterData> {
    use dynamixel_registers::models::ModelGroup;
    match model_group {
        ModelGroup::PH42 => PH42::get(register),
        ModelGroup::PH54 => PH54::get(register),
        ModelGroup::PM42 => PM42::get(register),
        ModelGroup::PM54 => PM54::get(register),
        ModelGroup::XC330 => XC330::get(register),
        ModelGroup::XL330 => XL330::get(register),
        ModelGroup::XM335 => XM335::get(register),
        ModelGroup::XC430 => XC430::get(register),
        ModelGroup::XL430 => XL430::get(register),
        ModelGroup::XD430 => XD430::get(register),
        ModelGroup::XH430 => XH430::get(register),
        ModelGroup::XM430 => XM430::get(register),
        ModelGroup::XD540 => XD540::get(register),
        ModelGroup::XH540 => XH540::get(register),
        ModelGroup::XM540 => XM540::get(register),
        ModelGroup::XW540 => XW540::get(register),
        ModelGroup::YM070 => YM070::get(register),
        ModelGroup::YM080 => YM080::get(register),
        ModelGroup::YM110 => YM080::get(register),
        ModelGroup::QC060 => QC060::get(register),
        ModelGroup::QC080 => QC080::get(register),
        _ => unreachable!(),
    }
}

/// The [`RegisterData`](dynamixel_registers::RegisterData) for `register` on a model, or `None`
/// if the model has no such register. Usable without `std`.
///
/// The lookup is by model group, so where a group's members disagree on a unit this
/// returns the group's `None` unit rather than the model's exact one.
pub const fn register_data(
    model: dynamixel_registers::models::Model,
    register: dynamixel_registers::Register,
) -> Option<dynamixel_registers::RegisterData> {
    register_data_from_model_group(model.model_group(), register)
}

#[cfg(feature = "std")]
pub(crate) fn indirect_address_blocks_from_model_group(
    model_group: &dynamixel_registers::models::ModelGroup,
) -> &'static [dynamixel_registers::IndirectRange] {
    use dynamixel_registers::models::ModelGroup;
    match model_group {
        ModelGroup::PH42 => PH42::indirect_address_blocks(),
        ModelGroup::PH54 => PH54::indirect_address_blocks(),
        ModelGroup::PM42 => PM42::indirect_address_blocks(),
        ModelGroup::PM54 => PM54::indirect_address_blocks(),
        ModelGroup::XC330 => XC330::indirect_address_blocks(),
        ModelGroup::XL330 => XL330::indirect_address_blocks(),
        ModelGroup::XM335 => XM335::indirect_address_blocks(),
        ModelGroup::XC430 => XC430::indirect_address_blocks(),
        ModelGroup::XL430 => XL430::indirect_address_blocks(),
        ModelGroup::XD430 => XD430::indirect_address_blocks(),
        ModelGroup::XH430 => XH430::indirect_address_blocks(),
        ModelGroup::XM430 => XM430::indirect_address_blocks(),
        ModelGroup::XD540 => XD540::indirect_address_blocks(),
        ModelGroup::XH540 => XH540::indirect_address_blocks(),
        ModelGroup::XM540 => XM540::indirect_address_blocks(),
        ModelGroup::XW540 => XW540::indirect_address_blocks(),
        ModelGroup::YM070 => YM070::indirect_address_blocks(),
        ModelGroup::YM080 => YM080::indirect_address_blocks(),
        ModelGroup::YM110 => YM080::indirect_address_blocks(),
        ModelGroup::QC060 => QC060::indirect_address_blocks(),
        ModelGroup::QC080 => QC080::indirect_address_blocks(),
        _ => unreachable!(),
    }
}

#[cfg(feature = "std")]
pub(crate) fn indirect_data_blocks_from_model_group(
    model_group: &dynamixel_registers::models::ModelGroup,
) -> &'static [dynamixel_registers::IndirectRange] {
    use dynamixel_registers::models::ModelGroup;
    match model_group {
        ModelGroup::PH42 => PH42::indirect_data_blocks(),
        ModelGroup::PH54 => PH54::indirect_data_blocks(),
        ModelGroup::PM42 => PM42::indirect_data_blocks(),
        ModelGroup::PM54 => PM54::indirect_data_blocks(),
        ModelGroup::XC330 => XC330::indirect_data_blocks(),
        ModelGroup::XL330 => XL330::indirect_data_blocks(),
        ModelGroup::XM335 => XM335::indirect_data_blocks(),
        ModelGroup::XC430 => XC430::indirect_data_blocks(),
        ModelGroup::XL430 => XL430::indirect_data_blocks(),
        ModelGroup::XD430 => XD430::indirect_data_blocks(),
        ModelGroup::XH430 => XH430::indirect_data_blocks(),
        ModelGroup::XM430 => XM430::indirect_data_blocks(),
        ModelGroup::XD540 => XD540::indirect_data_blocks(),
        ModelGroup::XH540 => XH540::indirect_data_blocks(),
        ModelGroup::XM540 => XM540::indirect_data_blocks(),
        ModelGroup::XW540 => XW540::indirect_data_blocks(),
        ModelGroup::YM070 => YM070::indirect_data_blocks(),
        ModelGroup::YM080 => YM080::indirect_data_blocks(),
        ModelGroup::YM110 => YM080::indirect_data_blocks(),
        ModelGroup::QC060 => QC060::indirect_data_blocks(),
        ModelGroup::QC080 => QC080::indirect_data_blocks(),
        _ => unreachable!(),
    }
}

/// The position encoder resolution in pulses per revolution for a model, from the
/// ROBOTIS docs.
///
/// Every [`Model`](dynamixel_registers::models::Model) has one, so the wildcard arm
/// `Model` being `#[non_exhaustive]` forces is unreachable; the `all_models_covered`
/// test holds that.
pub const fn position_resolution(model: dynamixel_registers::models::Model) -> u32 {
    use dynamixel_registers::models::Model;
    match model {
        Model::XH430_W350 => 4096,
        Model::XD430_T350 => 4096,
        Model::XH430_W210 => 4096,
        Model::XD430_T210 => 4096,
        Model::XM430_W350 => 4096,
        Model::XM430_W210 => 4096,
        Model::XH430_V350 => 4096,
        Model::XH430_V210 => 4096,
        Model::XL430_W250 => 4096,
        Model::XC430_W150 => 4096,
        Model::XC430_W240 => 4096,
        Model::XH540_W270 => 4096,
        Model::XD540_T270 => 4096,
        Model::XH540_W150 => 4096,
        Model::XD540_T150 => 4096,
        Model::XM540_W270 => 4096,
        Model::XM540_W150 => 4096,
        Model::XH540_V270 => 4096,
        Model::XH540_V150 => 4096,
        Model::XW540_T260 => 4096,
        Model::XW540_T140 => 4096,
        Model::XL330_M077 => 4096,
        Model::XL330_M288 => 4096,
        Model::XC330_T181 => 4096,
        Model::XC330_T288 => 4096,
        Model::XC330_M181 => 4096,
        Model::XC330_M288 => 4096,
        Model::XW540_H260 => 4096,
        Model::XM335_T323 => 4096,
        Model::PH42_020_S300_R => 607500,
        Model::PH54_100_S500_R => 1003846,
        Model::PH54_200_S500_R => 1003846,
        Model::PM42_010_S260_R => 526374,
        Model::PM54_040_S250_R => 502834,
        Model::PM54_060_S250_R => 502834,
        Model::YM070_210_M001_RH => 524288,
        Model::YM070_210_B001_RH => 524288,
        Model::YM070_200_R051_RH => 26738688,
        Model::YM070_200_R099_RH => 51904512,
        Model::YM070_210_A051_RH => 26738688,
        Model::YM070_200_A099_RH => 51904512,
        Model::YM080_230_M001_RH => 524288,
        Model::YM080_230_B001_RH => 524288,
        Model::YM080_230_R051_RH => 26738688,
        Model::YM080_230_R099_RH => 51904512,
        Model::YM080_230_A051_RH => 26738688,
        Model::YM080_230_A099_RH => 51904512,
        Model::YM110_400_R099_RH => 51904512,
        // From the Wizard: Max Position Limit 32767 reads as 179.99°.
        Model::QC060_200_R020_RE => 65536,
        Model::QC080_240_R020_RE => 65536,
        _ => unreachable!(),
    }
}

/// The shared position resolution of a model group, or `None` if its models disagree (the Y series).
pub const fn position_resolution_from_model_group(
    model_group: dynamixel_registers::models::ModelGroup,
) -> Option<u32> {
    use dynamixel_registers::models::ModelGroup;
    match model_group {
        ModelGroup::PH42 => Some(607500),
        ModelGroup::PH54 => Some(1003846),
        ModelGroup::PM42 => Some(526374),
        ModelGroup::PM54 => Some(502834),
        ModelGroup::XC330 => Some(4096),
        ModelGroup::XC430 => Some(4096),
        ModelGroup::XD430 => Some(4096),
        ModelGroup::XD540 => Some(4096),
        ModelGroup::XH430 => Some(4096),
        ModelGroup::XH540 => Some(4096),
        ModelGroup::XL330 => Some(4096),
        ModelGroup::XL430 => Some(4096),
        ModelGroup::XM335 => Some(4096),
        ModelGroup::XM430 => Some(4096),
        ModelGroup::XM540 => Some(4096),
        ModelGroup::XW540 => Some(4096),
        ModelGroup::YM070 => None,
        ModelGroup::YM080 => None,
        ModelGroup::YM110 => None,
        ModelGroup::QC060 => Some(65536),
        ModelGroup::QC080 => Some(65536),
        _ => unreachable!(),
    }
}
