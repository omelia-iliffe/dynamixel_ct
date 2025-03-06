//! The [`crate::model!`] macro definition.
//!

/// The [`model!`] macro is used to define the control table for a specific model.
/// It creates a static HashMap of RegisterData for each register in the model.
macro_rules! model {
    ($($model:ident)+ { $($reg:ident : $addr:expr, $len:expr,)+ } ) => {
        pub(crate) static TABLE: std::sync::LazyLock<std::collections::HashMap<$crate::Register, $crate::RegisterData>> = std::sync::LazyLock::new(|| {
            [
                $(
                    ($crate::Register::$reg, $crate::RegisterData {
                        address: $addr,
                        length: $len,
                    }),
                )+
            ].iter().cloned().collect()
        });

        $(
            #[doc = concat!("The Control Table for the ", stringify!($model), " models.")]
            pub struct $model(&'static std::collections::HashMap<$crate::Register, $crate::RegisterData>);

            impl $model {
                #[doc = concat!("Constructs the control table for the ", stringify!($model), " models.")]
                #[doc = concat!("The control table is statically allocated to reduce memory usage.")]
                pub fn new() -> Self {
                    Self(&*TABLE)
                }

                /// Get the register data for a specific register.
                pub fn get(&self, register: $crate::Register) -> Option<&$crate::RegisterData> {
                    self.0.get(&register)
                }

                #[doc = concat!("Acquire a static reference to the control table for the ", stringify!($model), " models.")]
                pub(crate) fn table() -> &'static std::collections::HashMap<$crate::Register, $crate::RegisterData> {
                    &*TABLE
                }
            }

            impl Default for $model {
                fn default() -> Self {
                    Self::new()
                }
            }
        )+
    }
}

pub(crate) use model;
