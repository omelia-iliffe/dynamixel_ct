//! This module defines the RegisterData struct and the [`crate::model!`] macro.

/// RegisterData is used to store the address, length and type of a register.
///  
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RegisterData {
    /// The address of the register
    pub address: u16,
    /// The number of bytes in the register
    pub length: u16,
}

/// This macro is used to define a model's control table.
/// The registers must be defined in [`crate::ControlTable`] trait.
///
/// See the [`crate::models::XM430`] model for an example.
///
#[macro_export]
macro_rules! model {
    ($model:ident { $($reg:ident : $addr:expr, $len:expr,)+ } ) => {
        #[derive(Clone, Debug)]
        /// This is the control table for the $model Dynamixel
        pub struct $model;
        impl $crate::control_table::ControlTable for $model {
            $(
                fn $reg(&self) -> Option<$crate::register::RegisterData>  {
                    Some(RegisterData {
                        address: $addr,
                        length: $len,
                    })
                }
            )+
        }
    }
}
