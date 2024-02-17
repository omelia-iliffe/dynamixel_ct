//! Dynamixel model implementations
//! Each model is defined in its own module, and implements the [`crate::ControlTable`] trait using the [`crate::model!`] macro.

pub mod xm430;
pub use self::xm430::XM430;
pub mod ym070;
pub use self::ym070::YM070;

#[cfg(feature = "dynamic_models")]
pub mod dynamic_model;
#[cfg(feature = "dynamic_models")]
pub use self::dynamic_model::new_from_model;

use enum_primitive::*;

enum_from_primitive! {
    /// Dynamixel model numbers
    #[derive(Debug, PartialEq, Eq)]
    #[repr(u16)]
    #[allow(non_camel_case_types)]
    #[allow(missing_docs)]
    pub enum Model {
        AX12A = 12,
        AX12W = 300,
        AX18A = 18,

        RX10 = 10,
        RX24F = 24,
        RX28 = 28,
        RX64 = 64,

        DX113 = 113,
        DX116 = 116,
        DX117 = 117,

        EX106 = 107,

        MX12W = 360,
        MX28 = 29,
        MX64 = 310,
        MX106 = 320,

        MX28_2 = 30,
        MX64_2 = 311,
        MX106_2 = 321,

        XL320 = 350,

        XL330_M077 = 1190,
        XL330_M288 = 1200,

        XC330_M181 = 1230,
        XC330_M288 = 1240,

        XC330_T181 = 1210,
        XC330_T288 = 1220,

        XC430_W150 = 1070,
        XC430_W240 = 1080,
        XXC430_W250 = 1160,

        XL430_W250 = 1060,
        XXL430_W250 = 1090,

        XM430_W210 = 1030,
        XM430_W350 = 1020,

        XM540_W150 = 1130,
        XM540_W270 = 1120,

        XH430_V210 = 1050,
        XH430_V350 = 1040,
        XH430_W210 = 1010,
        XH430_W350 = 1000,

        XH540_W150 = 1110,
        XH540_W270 = 1100,
        XH540_V150 = 1150,
        XH540_V270 = 1140,

        XD430_T210 = 1011,
        XD430_T350 = 1001,

        XD540_T150 = 1111,
        XD540_T270 = 1101,

        XW430_T200 = 1280,
        XW430_T333 = 1270,
        XW540_T140 = 1180,
        XW540_T260 = 1170,

        YM070_200_R099_R = 4030,
        YM070_200_R051_R = 4020,

        PRO_L42_10_S300_R = 35072,
        PRO_L54_30_S400_R = 37928,
        PRO_L54_30_S500_R = 37896,
        PRO_L54_50_S290_R = 38176,
    }
}
/// If a model is not implemented, this error is returned.
/// Either the Model doesn't exist or it hasn't been implemented yet.
#[derive(Debug, PartialEq, Eq)]
pub struct UnknownModel;

impl TryFrom<u16> for Model {
    type Error = UnknownModel;

    fn try_from(model_number: u16) -> Result<Self, Self::Error> {
        Model::from_u16(model_number).ok_or(UnknownModel)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_from_number() {
        let model: Result<Model, UnknownModel> = 4030.try_into();
        assert_eq!(model, Ok(Model::YM070_200_R099_R));

        let model: Result<Model, UnknownModel> = 1075.try_into();
        assert_eq!(model, Err(UnknownModel));
    }
}
