//! The [`model!`] macro definition.
//!

/// The [`model!`] macro is used to define the control table for a specific model.
/// It creates a static HashMap of RegisterData for each register in the model.
macro_rules! model {
    (@BASE_MODEL {$($reg:ident : $addr:expr, $len:expr,)+}) => {
        pastey::paste!{
            #[cfg(feature = "std")]
            pub(crate) static TABLE: std::sync::LazyLock<std::collections::HashMap<Register, RegisterData>> = std::sync::LazyLock::new(|| {
                [
                    $(
                        (Register::$reg, [<BASE_ $reg:snake:upper>]),
                    )+
                ].iter().cloned().collect()
            });

            const fn base_get(register: Register) -> Option<RegisterData> {
                match register {
                    $(
                        Register::$reg => Some([<BASE_ $reg:snake:upper>]),
                    )+
                    _ => None,
                }
            }

            $(
                const [< BASE_ $reg:snake:upper>]: RegisterData =
                    RegisterData {
                        address: $addr,
                        length: $len,
                    };

            )+
        }
    };
    (@MODEL $model:ident {$($reg:ident : $addr:expr, $len:expr,)+}) => {
        pastey::paste! {
            #[doc = "The Control Table for the " $model " models."]
            pub struct $model;

            impl $model {

                #[cfg(feature = "std")]
                pub(crate) fn table() -> &'static std::collections::HashMap<Register, RegisterData> {
                    &*TABLE
                }

                #[doc = "return the [`RegisterData`] for this register. Returns an `Option` as the register may not present for this model"]
                pub const fn get(register: Register) -> Option<RegisterData> {
                    base_get(register)
                }

                $(
                    #[doc = "[`RegisterData`] for [`Register::" $reg "`]"]
                    pub const [<$reg:snake:upper>]: RegisterData = {
                        [<BASE_ $reg:snake:upper>]
                    };
                )+
            }
        }

    };
    ($($model:ident)+ => $registers:tt  ) => {
        use $crate::RegisterData;
        use $crate::Register;
        model!(@BASE_MODEL $registers);

        $(
            model!(@MODEL $model $registers);
        )+
    }
}

pub(crate) use model;
