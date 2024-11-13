//! This module contains functions to create a model from a model number.

extern crate alloc;
use alloc::boxed::Box;

use crate::control_table::ControlTable;
use crate::models::*;

/// Returns a new model from the model number.
///
/// ```
/// use dynamixel_ct::models::try_from_model;
/// let model = try_from_model(1030);
/// assert!(model.is_ok());
/// ```
pub fn try_from_model<T>(model: T) -> Result<Box<dyn ControlTable + Send>, Error>
where
    T: TryInto<Model>,
{
    let model = model.try_into().map_err(|_| Error::UnknownModel)?;
    let model = SupportedModel::try_from(model)?;
    Ok(from_model(model))
}

/// Returns a boxed [`ControlTable`] from a [`SupportedModel`].
/// The method is infallible due to the model being a known supported model.
pub fn from_model(model: SupportedModel) -> Box<dyn ControlTable + Send> {
    match model {
        SupportedModel::XM430_W210 | SupportedModel::XM430_W350 => Box::new(XM430),
        SupportedModel::XM540_W150 | SupportedModel::XM540_W270 => Box::new(XM430),
        SupportedModel::XC330_M181
        | SupportedModel::XC330_M288
        | SupportedModel::XC330_T181
        | SupportedModel::XC330_T288 => Box::new(XM430),
        SupportedModel::YM070_200_R051_R
        | SupportedModel::YM070_200_R099_R
        | SupportedModel::YM070_200_A099_R
        | SupportedModel::YM080_230_R099_R => Box::new(YM),
    }
}
