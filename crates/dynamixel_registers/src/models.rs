use crate::error::Error;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
#[cfg(feature = "serde")]
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, Copy, derive_more::Display, Ord, PartialOrd)]
#[repr(u16)]
#[allow(non_camel_case_types)]
#[allow(missing_docs)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize)
)]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ModelOrModelGroup {
    Model(Model),
    ModelGroup(ModelGroup),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, derive_more::Display, Ord, PartialOrd)]
#[repr(u16)]
#[allow(non_camel_case_types)]
#[allow(missing_docs)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize, strum::EnumString)
)]
pub enum ModelGroup {
    PH42,
    PH54,
    PM42,
    PM54,
    XC330,
    XC430,
    XD430,
    XD540,
    XH430,
    XH540,
    XL330,
    XL430,
    XM430,
    XM540,
    XW540,
    YM070,
    YM080,
}

impl From<Model> for ModelGroup {
    fn from(model: Model) -> Self {
        model.model_group()
    }
}

/// Dynamixel model names and numbers
#[derive(
    PartialEq,
    Eq,
    Clone,
    Copy,
    FromPrimitive,
    ToPrimitive,
    derive_more::Display,
    Ord,
    PartialOrd,
)]
#[repr(u16)]
#[allow(non_camel_case_types)]
#[allow(missing_docs)]
#[cfg_attr(
    feature = "serde",
    derive(serde_repr::Serialize_repr, strum::EnumString)
)]
pub enum Model {
    XL330_M077 = 1190,
    XL330_M288 = 1200,

    XC330_M181 = 1230,
    XC330_M288 = 1240,

    XC330_T181 = 1210,
    XC330_T288 = 1220,

    XC430_W150 = 1070,
    XC430_W240 = 1080,

    XL430_W250 = 1060,

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

    XW540_T140 = 1180,
    XW540_T260 = 1170,

    XW540_H260 = 1310,

    PH42_020_S300_R = 2000,
    PH54_100_S500_R = 2010,
    PH54_200_S500_R = 2020,
    PM42_010_S260_R = 2100,
    PM54_040_S250_R = 2110,
    PM54_060_S250_R = 2120,

    YM070_210_M001_RH = 4000,
    YM070_210_B001_RH = 4010,
    YM070_200_R051_RH = 4020,
    YM070_200_R099_RH = 4030,
    YM070_210_A051_RH = 4040,
    YM070_200_A099_RH = 4050,

    YM080_230_M001_RH = 4120,
    YM080_230_B001_RH = 4130,
    YM080_230_R051_RH = 4140,
    YM080_230_R099_RH = 4150,
    YM080_230_A051_RH = 4160,
    YM080_230_A099_RH = 4170,
}

impl Model {
    pub fn model_group(&self) -> ModelGroup {
        use Model::*;
        match self {
            XD540_T270 | XD540_T150 => ModelGroup::XD540,
            XH540_W270 | XH540_W150 | XH540_V270 | XH540_V150 => ModelGroup::XH540,
            XM540_W270 | XM540_W150 => ModelGroup::XM540,
            XC430_W150 | XC430_W240 => ModelGroup::XC430,
            XL430_W250 => ModelGroup::XL430,
            XW540_T260 | XW540_T140 | XW540_H260 => ModelGroup::XW540,
            XC330_T181 | XC330_T288 | XC330_M181 | XC330_M288 => ModelGroup::XC330,
            XL330_M077 | XL330_M288 => ModelGroup::XL330,
            XD430_T350 | XD430_T210 => ModelGroup::XD430,
            XH430_W350 | XH430_W210 | XH430_V350 | XH430_V210 => ModelGroup::XH430,
            XM430_W350 | XM430_W210 => ModelGroup::XM430,
            YM070_210_M001_RH | YM070_210_B001_RH | YM070_200_R051_RH | YM070_200_R099_RH | YM070_210_A051_RH | YM070_200_A099_RH => ModelGroup::YM070,
            YM080_230_M001_RH | YM080_230_B001_RH | YM080_230_R051_RH | YM080_230_R099_RH | YM080_230_A051_RH | YM080_230_A099_RH => ModelGroup::YM080,
            PH42_020_S300_R =>ModelGroup:: PH42,
            PH54_100_S500_R | PH54_200_S500_R =>ModelGroup:: PH54,
            PM42_010_S260_R =>ModelGroup:: PM42,
            PM54_040_S250_R | PM54_060_S250_R =>ModelGroup:: PM54,
        }

    }
}

#[cfg(feature = "serde")]
impl<'de> serde::de::Deserialize<'de> for Model {
    fn deserialize<D: serde::de::Deserializer<'de>>(d: D) -> Result<Model, D::Error> {
        struct Visitor;
        impl serde::de::Visitor<'_> for Visitor {
            type Value = Model;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("a valid dynamixel model or model number")
            }

            fn visit_i64<E: serde::de::Error>(self, value: i64) -> Result<Self::Value, E> {
                Model::from_i64(value)
                    .ok_or_else(|| E::invalid_value(serde::de::Unexpected::Signed(value), &self))
            }
            fn visit_u64<E: serde::de::Error>(self, value: u64) -> Result<Self::Value, E> {
                Model::from_u64(value)
                    .ok_or_else(|| E::invalid_value(serde::de::Unexpected::Unsigned(value), &self))
            }

            fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<Self::Value, E> {
                Model::from_str(value)
                    .map_err(|_| E::invalid_value(serde::de::Unexpected::Str(value), &self))
            }
        }
        d.deserialize_any(Visitor)
    }
}

impl core::fmt::Debug for Model {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}({})", self, *self as u16)
    }
}

impl TryFrom<u16> for Model {
    type Error = Error;

    fn try_from(model_number: u16) -> Result<Self, Self::Error> {
        Model::from_u16(model_number).ok_or(Error::UnknownModel)
    }
}

impl PartialEq<u16> for Model {
    fn eq(&self, other: &u16) -> bool {
        self.to_u16() == Some(*other)
    }
}

#[cfg(test)]
mod tests {
    use super::Model;
    use crate::error::Error;

    #[test]
    fn test_model_from_number() {
        let model: Result<Model, Error> = 4030.try_into();
        assert_eq!(model, Ok(Model::YM070_200_R099_RH));

        let model: Result<Model, Error> = 1075.try_into();
        assert_eq!(model, Err(Error::UnknownModel));
    }

    #[test]
    #[cfg(feature = "serde")]
    fn test_serde_json() {
        let model = Model::XM430_W210;
        let json = serde_json::to_string(&model).unwrap();
        assert_eq!(json, "1030");

        let model: Model = serde_json::from_str("\"XM430_W350\"").unwrap();
        assert_eq!(model, Model::XM430_W350);

        let model: Model = serde_json::from_str("1020").unwrap();
        assert_eq!(model, Model::XM430_W350);
    }
    #[test]
    #[cfg(feature = "serde")]
    fn test_serde_toml() {
        #[derive(serde::Deserialize, serde::Serialize)]
        struct Test {
            model: Model,
        }
        let model = Test {
            model: Model::XM430_W210,
        };
        let toml = toml::to_string(&model).unwrap();
        assert_eq!(toml, "model = 1030\n");

        let model: Test = toml::from_str("model = \"XM430_W350\"").unwrap();
        assert_eq!(model.model, Model::XM430_W350);

        let model: Test = toml::from_str("model = 1020").unwrap();
        assert_eq!(model.model, Model::XM430_W350);
    }
}
