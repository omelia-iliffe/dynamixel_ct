//! The [`model!`] macro definition.
//!

/// The [`model!`] macro is used to define the control table for a specific model.
/// It creates a static HashMap of RegisterData for each register in the model.
macro_rules! model {
    (@BASE_MODEL {$($reg:ident : $addr:expr, $len:expr,)+}) => {
        paste::paste!{
            #[cfg(feature = "std")]
            pub(crate) static TABLE: std::sync::LazyLock<std::collections::HashMap<Register, RegisterData>> = std::sync::LazyLock::new(|| {
                [
                    $(
                        (Register::$reg, [<base_ $reg:snake>]()),
                    )+
                ].iter().cloned().collect()
            });

            const fn base_get(register: Register) -> Option<RegisterData> {
                match register {
                    $(
                        Register::$reg => Some([<base_ $reg:snake>]()),
                    )+
                    _ => None,
                }
            }

            $(
                const fn [< base_ $reg:snake>] () -> RegisterData {
                    RegisterData {
                        address: $addr,
                        length: $len,
                    }
                }
            )+
        }
    };
    (@MODEL $model:ident {$($reg:ident : $addr:expr, $len:expr,)+}) => {
        paste::paste! {
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
                    #[doc = "returns the [`RegisterData`] for [`Register::" $reg "`]"]
                    pub const fn [<$reg:snake>]() -> RegisterData {
                        [<base_ $reg:snake>]()
                    }
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
