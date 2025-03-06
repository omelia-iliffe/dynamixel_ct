//! Dynamixel model implementations
//! Each model is defined in its own module.
mod generated;
pub use generated::*;

#[cfg(test)]
mod tests {
    use dynamixel_registers::error::Error;
    use dynamixel_registers::models::Model;

    #[test]
    fn test_model_from_number() {
        let model: Result<Model, Error> = 4030.try_into();
        dbg!(&model);
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
