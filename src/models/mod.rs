//! Dynamixel model implementations
//! Each model is defined in its own module, and implements the [`crate::ControlTable`] trait using the [`crate::model!`] macro.
mod xm430;

pub use xm430::XM430;
mod ym;
pub use ym::YM;

/// Errors that can occur when creating a model from a model number.
#[derive(Debug, PartialEq, derive_more::Display, derive_more::Error)]
pub enum Error {
    /// The model number is not known or is not yet supported.
    UnknownModel,
    /// The model known but the control table is not yet implemented.
    NotImplemented,
}

#[cfg(feature = "dynamic_models")]
pub mod dynamic_model;
#[cfg(feature = "dynamic_models")]
pub use self::dynamic_model::try_from_model;

use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::FromPrimitive;
#[cfg(feature = "serde")]
use serde::{de, Deserialize, Deserializer, Serialize};
#[cfg(feature = "serde")]
use serde_repr::Serialize_repr;
#[cfg(feature = "serde")]
use strum::EnumString;
#[cfg(feature = "serde")]
use core::str::FromStr;

/// Dynamixel model numbers
#[derive(Debug, PartialEq, Eq, Clone, Copy, FromPrimitive, ToPrimitive)]
#[repr(u16)]
#[allow(non_camel_case_types)]
#[allow(missing_docs)]
#[cfg_attr(feature = "serde", derive(Serialize_repr, EnumString))]
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

    YM070_200_A099_R = 4050, // TODO: Check this is correct
    YM070_200_R099_R = 4030,
    YM070_200_R051_R = 4020,

    YM080_230_R051_R = 4140, // TODO: Double check
    YM080_230_R099_R = 4150,

    PRO_L42_10_S300_R = 35072,
    PRO_L54_30_S400_R = 37928,
    PRO_L54_30_S500_R = 37896,
    PRO_L54_50_S290_R = 38176,
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Model {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Model, D::Error> {
        struct Visitor;
        impl<'de> de::Visitor<'de> for Visitor {
            type Value = Model;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("a valid dynamixel model or model number")
            }

            fn visit_u64<E: de::Error>(self, value: u64) -> Result<Self::Value, E> {
                Model::from_u64(value)
                    .ok_or_else(|| E::invalid_value(de::Unexpected::Unsigned(value), &self))
            }

            fn visit_str<E: de::Error>(self, value: &str) -> Result<Self::Value, E> {
                Model::from_str(value)
                    .map_err(|_| E::invalid_value(de::Unexpected::Str(value), &self))
            }
        }
        d.deserialize_any(Visitor)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, FromPrimitive, ToPrimitive, derive_more::Display)]
#[repr(u16)]
#[allow(non_camel_case_types)]
#[allow(missing_docs)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "Model"))]
pub enum SupportedModel {
    XM430_W210 = 1030,
    XM430_W350 = 1020,
    XM540_W150 = 1130,
    XM540_W270 = 1120,
    XC330_M181 = 1230,
    XC330_M288 = 1240,
    XC330_T181 = 1210,
    XC330_T288 = 1220,
    YM070_200_R051_R = 4020,
    YM070_200_R099_R = 4030,
    YM070_200_A099_R = 4050,
    YM080_230_R099_R = 4150,
}

impl TryFrom<Model> for SupportedModel {
    type Error = Error;

    fn try_from(value: Model) -> Result<Self, Self::Error> {
        match value {
            Model::XM430_W210 => Ok(SupportedModel::XM430_W210),
            Model::XM430_W350 => Ok(SupportedModel::XM430_W350),
            Model::XM540_W150 => Ok(SupportedModel::XM540_W150),
            Model::XM540_W270 => Ok(SupportedModel::XM540_W270),
            Model::XC330_M181 => Ok(SupportedModel::XC330_M181),
            Model::XC330_M288 => Ok(SupportedModel::XC330_M288),
            Model::XC330_T181 => Ok(SupportedModel::XC330_T181),
            Model::XC330_T288 => Ok(SupportedModel::XC330_T288),
            Model::YM070_200_R051_R => Ok(SupportedModel::YM070_200_R051_R),
            Model::YM070_200_R099_R => Ok(SupportedModel::YM070_200_R099_R),
            Model::YM070_200_A099_R => Ok(SupportedModel::YM070_200_A099_R),
            Model::YM080_230_R099_R => Ok(SupportedModel::YM080_230_R099_R),
            _ => Err(Error::NotImplemented),
        }
    }
}

/// If a model is not implemented, this error is returned.
/// Either the Model doesn't exist or it hasn't been implemented yet.
#[derive(Debug, PartialEq, Eq)]
pub struct UnknownModel;

impl TryFrom<u16> for Model {
    type Error = Error;

    fn try_from(model_number: u16) -> Result<Self, Self::Error> {
        Model::from_u16(model_number).ok_or(Error::UnknownModel)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_from_number() {
        let model: Result<Model, Error> = 4030.try_into();
        assert_eq!(model, Ok(Model::YM070_200_R099_R));

        let model: Result<Model, Error> = 1075.try_into();
        assert_eq!(model, Err(Error::UnknownModel));
    }

    #[test]
    #[cfg(feature = "serde")]
    fn test_serde() {
        let model = Model::XM430_W210;
        let json = serde_json::to_string(&model).unwrap();
        assert_eq!(json, "1030");

        let model: Model = serde_json::from_str("\"XM430_W350\"").unwrap();
        assert_eq!(model, Model::XM430_W210);
    }
    #[test]
    #[cfg(feature = "serde")]
    fn test_unsupported_model() {
        let model: Result<SupportedModel, _> = serde_json::from_str("\"AX12A\"");
        assert!(model.is_err());
    }
}
