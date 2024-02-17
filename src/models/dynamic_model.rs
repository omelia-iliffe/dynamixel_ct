//! This module contains functions to create a model from a model number.

extern crate alloc;
use alloc::boxed::Box;

use crate::control_table::ControlTable;
use crate::models::*;

#[derive(Debug, PartialEq)]

/// Errors that can occur when creating a model from a model number.
pub enum Error {
    /// The model number is not known or is not yet supported.
    UnknownModel,
    /// The model known but the control table is not yet implemented.
    NotImplemented,
}

/// Returns a new model from the model number.
///
/// ```
/// use dynamixel_ct::models::new_from_model;
/// let model = new_from_model(1030);
/// assert!(model.is_ok());
/// ```
pub fn new_from_model<T>(model_number: T) -> Result<Box<dyn ControlTable>, Error>
where
    T: TryInto<Model>,
{
    let model_number = model_number.try_into().map_err(|_| Error::UnknownModel)?;
    match model_number {
        Model::XM430_W210 | Model::XM430_W350 => Ok(Box::new(XM430)),
        Model::XM540_W150 | Model::XM540_W270 => Ok(Box::new(XM430)),
        Model::XC330_M181 | Model::XC330_M288 | Model::XC330_T181 | Model::XC330_T288 => {
            Ok(Box::new(XM430))
        }
        Model::YM070_200_R051_R | Model::YM070_200_R099_R => Ok(Box::new(YM070)),
        _ => Err(Error::NotImplemented),
    }
}