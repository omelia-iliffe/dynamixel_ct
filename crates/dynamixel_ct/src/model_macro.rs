//! The [`crate::model!`] macro definition.
//!

/// The [`model!`] macro is used to define the control table for a specific model.
/// It creates a static HashMap of RegisterData for each register in the model.
macro_rules! model {
    (@BASE_MODEL {$($reg:ident : $addr:expr, $len:expr,)+}) => {
        paste::paste!{
            pub(crate) static TABLE: std::sync::LazyLock<std::collections::HashMap<$crate::Register, $crate::RegisterData>> = std::sync::LazyLock::new(|| {
                [
                    $(
                        ($crate::Register::$reg, [<base_ $reg:snake>]()),
                    )+
                ].iter().cloned().collect()
            });

            fn base_get(register: $crate::Register) -> Option<$crate::RegisterData> {
                match register {
                    $(
                        $crate::Register::$reg => Some([<base_ $reg:snake>]()),
                    )+
                    _ => None,
                }
            }


            $(
                fn [< base_ $reg:snake>] () -> $crate::RegisterData {
                    $crate::RegisterData {
                        address: $addr,
                        length: $len,
                    }
                }
            )+
        }
    };
    (@MODEL $model:ident {$($reg:ident : $addr:expr, $len:expr,)+}) => {
        #[doc = concat!("The Control Table for the ", stringify!($model), " models.")]
        pub struct $model;

        impl $model {

            #[doc = concat!("Acquire a static reference to the control table for the ", stringify!($model), " models.")]
            pub(crate) fn table() -> &'static std::collections::HashMap<$crate::Register, $crate::RegisterData> {
                &*TABLE
            }

            pub fn get(register: $crate::Register) -> Option<$crate::RegisterData> {
                base_get(register)
            }

            paste::paste! {
                $(
                    pub fn [<$reg:snake>]() -> $crate::RegisterData {
                        [<base_ $reg:snake>]()
                    }
                )+
            }
        }

    };
    ($($model:ident)+ => $registers:tt  ) => {
        model!(@BASE_MODEL $registers);


        $(
            model!(@MODEL $model $registers);
        )+
    }
}

pub(crate) use model;
