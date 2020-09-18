/***********************************************************************************************************************
 * Copyright (c) 2019 by the authors
 *
 * Author: André Borrmann <pspwizard@gmx.de>
 * License: Apache License 2.0 / MIT
 **********************************************************************************************************************/

/// Helper macro to define the fields a register may contain of.
///
/// This is typically part of the register definition and will be applied there. It's not intended for use outside
/// of a register definition.
#[doc(hidden)]
#[macro_export]
macro_rules! register_field {
    ($t:ty, $field:ident, $offset:expr) => {
        #[allow(unused_variables, dead_code)]
        #[doc(hidden)]
        pub const $field: RegisterField<$t> = RegisterField::<$t>::new(1, $offset);
    };
    ($t:ty, $field:ident, $offset:expr, $bits:expr) => {
        #[allow(unused_variables, dead_code)]
        #[doc(hidden)]
        pub const $field: RegisterField<$t> = RegisterField::<$t>::new((1 << $bits) - 1, $offset);
    };
}

/// Helper macro to define constant register field values a specific register field can contain.
///
/// This macro is typically used as part of a register field definition and not intended for the use outside of this
/// context
#[doc(hidden)]
#[macro_export]
macro_rules! register_field_values {
    ($field:ident, $t:ty, $($($fvdoc:expr)?, $enum:ident = $value:expr),*) => {
        $(
            $(#[doc = $fvdoc])?
            #[allow(unused_variables, dead_code)]
            pub const $enum:RegisterFieldValue::<$t> = RegisterFieldValue::<$t>::new($field, $value);
        )*
    };
}
