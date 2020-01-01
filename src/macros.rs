/***********************************************************************************************************************
 * Copyright (c) 2019 by the authors
 *
 * Author: André Borrmann
 * License: Apache License 2.0
 **********************************************************************************************************************/

//! # Register definition macros
//!
//! The macros are used to simplify the definition of system registers as well as MMIO register.
//!

/// Helper macro to define the fields a register may contain of.<br>
/// This is typically part of the register definition and will be applied there. It's not intended for use outside
/// of a register definition.
#[doc(hidden)]
#[macro_export]
macro_rules! register_field {
    ($t:ty, $field:ident, $offset:expr) => {
        #[allow(unused_variables, dead_code)]
        pub const $field: RegisterField<$t> = RegisterField::<$t>::new(1, $offset);
    };
    ($t:ty, $field:ident, $offset:expr, $bits:expr) => {
        #[allow(unused_variables, dead_code)]
        pub const $field: RegisterField<$t> = RegisterField::<$t>::new((1 << $bits) - 1, $offset);
    };


    ($t:ty, $field:ident, $offset:expr, $bits:expr, [$(enum:ident : value:expr),*]) => {
        #[allow(unused_variables, dead_code)]
        //pub const $field: RegisterField<$t> = RegisterField::<$t>::new((1 << $bits) - 1, $offset);
    };
    ($t:ty, $field:ident, $offset:expr, [$(enum:ident : value:expr),*]) => {
        //register_field!($t, $field, $offset, 1, $(enum:ident : value:expr),*);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! register_field_values {
    ($field:ident, $t:ty, $($enum:ident : $value:expr),*) => {
        $(
            pub const $enum:RegisterFieldValue::<$t> = RegisterFieldValue::<$t>::new($field, $value);
        )*
    };
}

/// Macro to define a MMIO register with specific defined access mode.<br>
/// The access mode could one of: **ReadOnly**, **WriteOnly**, **ReadWrite**.<br>
/// The register size/width could be one of: **u8**, **u16**, **u32**, **u64**
///
/// # Examples
///
/// Define a simple MMIO register that might only be accessed with it's raw value
/// ```no_run
/// # use ruspiro_register::*;
/// define_mmio_register!(
///     FOO<ReadWrite<u32>@(0x3F20_0000+0x10)>
/// );
/// ```
///
/// Define a MMIO register containing a single field at a given offset with 1 bit length.
/// ```no_run
/// # use ruspiro_register::*;
/// define_mmio_register!(
///     FOO<ReadWrite<u32>@(0x3F20_0000)> {
///         BAR OFFSET(0)
///     }
/// );
/// ```
///
/// Define a MMIO register containing several fields with different offsets and bit length
/// ```no_run
/// # use ruspiro_register::*;
/// define_mmio_register!(
///     FOO<ReadWrite<u32>@(0x3F20_0000)> {
///         BAR OFFSET(0),
///         BAZ OFFSET(3) BITS(3)
///     }
/// );
/// ```
///
/// Define multiple MMIO register at once
/// ```no_run
/// # use ruspiro_register::*;
/// define_mmio_register!(
///     FOO<ReadWrite<u32>@(0x3F20_0000)>,
///     BAR<ReadOnly<u32>@(0x3F20_0010)> {
///         BAZ OFFSET(0) BITS(2) [
///             VAL1: 0b10
///         ]
///     }
/// );
/// ```
///
/// Define a MMIO register where one field has defined specific values to be choosen from when
/// writing to or updating this specific register field
/// ```no_run
/// # use ruspiro_register::*;
/// define_mmio_register!(
///     FOO<ReadWrite<u32>@(0x3F20_0000)> {
///         BAR OFFSET(3) BITS(3),
///         BAZ OFFSET(6) BITS(3) [
///             VAL1:  0b000,
///             VAL2:  0b010
///         ],
///         BAL OFFSET(9) BITS(2) [
///             VAL1: 0b01,
///             VAL2: 0b11
///         ]
///     }
/// );
///
/// fn main() {
///     // write a specific value for one field of the MMIO register
///     FOO::Register.write_value(
///         FOO::BAL::VAL1
///     );
///
///     // combine field values of different fields to update the MMIO register
///     FOO::Register.write_value(
///         FOO::BAZ::VAL1 | FOO::BAL::VAL2
///     );
///
///     // write a specif value to a register field that does not provide default/enum values
///     FOO::Register.write_value(
///         FOO::BAR::with_value(0b010)
///     );
/// }
/// ```
#[macro_export]
macro_rules! define_mmio_register {
    // REGISTER_NAME<ReadWrite<TYPE>@ADDRESS> { FIELD OFFSET(num) BITS(num) [ VALUE: val ] }
    ($($vis:vis $name:ident<$access:ident<$t:ty>@($addr:expr)> $(
        { $(
                $field:ident OFFSET($offset:literal) $(BITS($bits:literal))?
                $([$($enum:ident : $value:expr),*])?
        ),* }
    )?),*) => {
        $(
            #[allow(non_snake_case)]
            #[allow(non_upper_case_globals)]
            $vis mod $name {
                use $crate::register::*;
                use super::*;
                pub const Register: $access<$t> = $access::<$t>::new($addr);
                $(
                    $(
                        $crate::register_field!($t, $field, $offset $(, $bits)?);
                        pub mod $field {
                            use super::*;
                            pub const fn with_value(value: $t) -> RegisterFieldValue<$t> {
                                RegisterFieldValue::<$t>::new($field, value)
                            }
                            $(
                                $crate::register_field_values!($field, $t, $($enum:$value),*);
                            )*
                        }
                    )*
                )*
            }
        )*
    };
}

///
/// # Examples
///
/// ```no_run
/// # use ruspiro_register::*;
/// define_register!( GPFSEL0: ReadWrite<u32> @ 0x3F20_0000 );
///
/// # fn main() {
/// let _ = GPFSEL0::Register.get();
/// # }
/// ```
///
/// ```no_run
/// # use ruspiro_register::*;
/// const GPIO_BASE:u32 = 0x3F00_0000;
///
/// define_register!( GPFSEL1: ReadWrite<u32> @ GPIO_BASE + 0x04 );
///
/// # fn main() {
/// let _ = GPFSEL1::Register.get();
/// # }
/// ```
///
/// To pass a more specific definition of the fields the register represents they could be added in the [] of the definition
/// like so:
/// ```no_run
/// # use ruspiro_register::*;
///
/// define_register!( GPFSEL2: ReadWrite<u32> @ 0x3F20_0000 => [
///     FSEL20 OFFSET(0) BITS(3),
///     FSEL21 OFFSET(3) BITS(3),
///     FSEL22 OFFSET(6) BITS(3)
/// ] );
///
/// # fn main() {
/// GPFSEL2::Register.modify(GPFSEL2::FSEL21, 0b001);
/// # }
/// ```
#[deprecated(
    since = "0.3.0",
    note = "this macro definition syntax is deprectated to align \
            the mmio syntax with the system register syntax. \
            Please use `define_mmio_register` instead"
)]
#[macro_export]
macro_rules! define_register {
    // REGISTER_NAME: ReadWrite<TYPE> @ ADDRESS
    ($name:ident : ReadWrite<$t:ty> @ $addr:expr) => {
        $crate::define_register!($name : ReadWrite<$t> @ $addr => []);
    };

    // REGISTER_NAME: ReadWrite<TYPE> @ ADDRESS => []
    ($name:ident : ReadWrite<$t:ty> @ $addr:expr => [ $($field:ident OFFSET($offset:expr) $(BITS($bits:expr))?),* ]) => {
        #[allow(non_snake_case)]
        #[allow(non_upper_case_globals)]
        mod $name {
            use $crate::register::*;
            use super::*;
            pub const Register: ReadWrite<$t> = ReadWrite::<$t>::new($addr);
            $(
                $crate::register_field!($t, $field, $offset $(, $bits)?);
            )*
        }
    };

    // REGISTER_NAME: ReadOnly<TYPE> @ ADDRESS
    ($name:ident : ReadOnly<$t:ty> @ $addr:expr) => {
        $crate::define_register!($name : ReadOnly<$t> @ $addr => []);
    };

    // REGISTER_NAME: ReadOnly<type> @ ADDRESS => []
    ($name:ident : ReadOnly<$t:ty> @ $addr:expr => [ $($field:ident OFFSET($offset:expr) $(BITS($bits:expr))?),* ]) => {
        #[allow(non_snake_case)]
        #[allow(non_upper_case_globals)]
        mod $name {
            use $crate::register::*;
            use super::*;
            pub const Register: ReadOnly<$t> = ReadOnly::<$t>::new($addr);
            $(
                $crate::register_field!($t, $field, $offset $(, $bits)?);
            )*
        }
    };

    // REGISTER_NAME: WriteOnly<TYPE> @ ADDRESS
    ($name:ident : WriteOnly<$t:ty> @ $addr:expr) => {
        $crate::define_register!($name : WriteOnly<$t> @ $addr => []);
    };

    // REGISTER_NAME: WriteOnly<type> @ ADDRESS => []
    ($name:ident : WriteOnly<$t:ty> @ $addr:expr => [ $($field:ident OFFSET($offset:expr) $(BITS($bits:expr))?),* ]) => {
        #[allow(non_snake_case)]
        #[allow(non_upper_case_globals)]
        mod $name {
            use $crate::register::*;
            use super::*;
            pub const Register: WriteOnly<$t> = WriteOnly::<$t>::new($addr);
            $(
                $crate::register_field!($t, $field, $offset $(, $bits)?);
            )*
        }
    };
}

///
/// Macro to provide multiple register definitions at once
///
/// # Examples
///
/// ```no_run
/// # use ruspiro_register::*;
///
/// define_registers! [
///     TIMERCLO: ReadOnly<u32> @ 0x3F00_3004,
///     TIMERCHI: ReadOnly<u32> @ 0x3F00_3008,
///     GPPUD: ReadWrite<u32> @ 0x3F20_0094 => [
///         PUD OFFSET(0) BITS(2)
///     ]
/// ];
///
/// # fn main() {
/// let tclo = TIMERCLO::Register.get();
/// let tchi = TIMERCHI::Register.get();
/// # }
/// ```
///
#[deprecated(
    since = "0.3.0",
    note = "this macro definition syntax is deprectated to align \
            the mmio syntax with the system register syntax. \
            Please use `define_mmio_register` instead"
)]
#[macro_export]
macro_rules! define_registers {
    ( $($name:ident : $access:ident<$t:ty> @ $addr:expr $(=> $fields:tt)?),* ) => {
        $(
            $crate::define_register!($name : $access<$t> @ $addr $(=> $fields)?);
        )*
    }
}

/// Helper macro to implement shared register functions for aarch32/64 system registers
#[doc(hidden)]
#[macro_export]
macro_rules! impl_system_register_rw {
    ($t:ty) => {
        #[inline]
        pub fn write(field_value: RegisterFieldValue<$t>) {
            let raw_value = (get() & !field_value.mask()) | field_value.raw_value();
            set(raw_value);
        }

        #[inline]
        pub fn read(field: RegisterField<$t>) -> RegisterFieldValue<$t> {
            let raw_value = get() & field.mask();
            RegisterFieldValue::<$t>::new(field, raw_value >> field.shift())
        }
    };
}
/// Macro to define an Aarch64 system register and its fields
///
/// # Examples
/// ```no_run
/// # use ruspiro_register::*;
///
/// # #[cfg(target_arch="aarch64")]
/// define_aarch64_register! {
///     foo<u32> {
///         BAR OFFSET(0) [
///             VAL1: 0b1,
///             VAL0: 0b0
///         ],
///         BAZ OFFSET(1) BITS(2) [
///             VAL1: 0b01,
///             VAL2: 0b10,
///             VAL3: 0b11
///         ]
///     }
/// }
///
/// # fn main() {
/// # #[cfg(target_arch="aarch64")]
///     foo::write(
///         foo::BAR::VAL1 | foo::BAZ::VAL2
///     );
/// # }
///
#[macro_export]
macro_rules! define_aarch64_register {
    (@$name:ident<$t:ty> { $($field:ident OFFSET($offset:expr) $(BITS($bits:expr))? $([ $($enum:ident : $value:expr),* ])?),* }) => {
        $(
            #[allow(non_snake_case)]
            #[allow(non_upper_case_globals)]
            pub mod $field {
                use super::*;

                register_field!($t, Field, $offset $(, $bits)?);

                pub fn with_value(value: $t) -> RegisterFieldValue<$t> {
                    RegisterFieldValue::<$t>::new(Field, value)
                }

                $(
                    $(pub const $enum: RegisterFieldValue::<$t> = RegisterFieldValue::<$t>::new(Field, $value);)*
                )*
            }
        )*

        #[inline]
        pub fn get() -> $t {
            let raw_value: $t;
            unsafe {
                asm!(concat!("mrs $0,", stringify!($name)):"=r"(raw_value):::"volatile")
            };
            raw_value
        }

        #[inline]
        pub fn set(raw_value: $t) {
            unsafe {
                asm!(concat!("msr ", stringify!($name) , ", $0 ")::"r"(raw_value)::"volatile")
            }
        }

        impl_system_register_rw!($t);

    };
/*
    ($name:ident<$t:ty> { $($field:ident OFFSET($offset:expr) $(BITS($bits:expr))? $([ $($enum:ident : $value:expr),* ])?),* }) => {
        #[allow(non_snake_case)]
        #[allow(non_upper_case_globals)]
        pub mod $name {
            use super::*;
            define_system_register!{
                @$name<$t> { $($field OFFSET($offset) $(BITS($bits))? $([ $($enum : $value),* ])?),* }
            }
        }
    };
    */
}

/// Macro to define an Aarch32 (CP15) system register and its fields
///
/// # Examples
/// ```
/// # use ruspiro_register::*;
/// # #[cfg(target_arch="arm")]
/// define_aarch32_register! {
///     foo {
///         BAR OFFSET(0) [
///             VAL1: 0b1,
///             VAL0: 0b0
///         ],
///         BAZ OFFSET(1) BITS(2) [
///             VAL1: 0b01,
///             VAL2: 0b10,
///             VAL3: 0b11
///         ]
///     }
/// }
///
/// # fn main() {
/// # #[cfg(target_arch="arm")]
///     foo::write(
///         foo::BAR::VAL1 | foo::BAZ::VAL2
///     );
/// # }
///
#[macro_export]
macro_rules! define_aarch32_register {
    (@$name:ident $crn:ident, $op1:tt, $crm:ident, $op2:tt { $($field:ident OFFSET($offset:expr) $(BITS($bits:expr))? $([ $($enum:ident : $value:expr),* ])?),* }) => {
        $(
            #[allow(non_snake_case)]
            #[allow(non_upper_case_globals)]
            pub mod $field {
                use super::*;

                register_field!(u32, Field, $offset $(, $bits)?);

                #[inline]
                pub fn with_value(value: u32) -> RegisterFieldValue<u32> {
                    RegisterFieldValue::<u32>::new(Field, value)
                }

                $(
                    $(pub const $enum: RegisterFieldValue::<u32> = RegisterFieldValue::<u32>::new(Field, $value);)*
                )*
            }
        )*

        #[inline]
        pub fn get() -> u32 {
            let raw_value: u32;
            unsafe {
                asm!(concat!("mrc p15, ",
                             stringify!($op1),
                             ", $0 , ",
                             stringify!($crn), ", ",
                             stringify!($crm), ", ",
                             stringify!($op2)):"=r"(raw_value):::"volatile")
            };
            raw_value
        }

        #[inline]
        pub fn set(raw_value: u32) {
            unsafe {
                asm!(concat!("mcr p15, ",
                             stringify!($op1),
                             ", $0 , ",
                            stringify!($crn), ", ",
                            stringify!($crm), ", ",
                            stringify!($op2))::"r"(raw_value)::"volatile")
            }
        }

        impl_system_register_rw!(u32);
    };
/*
    ($name:ident $crn:ident, $op1:tt, $crm:ident, $op2:tt { $($field:ident OFFSET($offset:expr) $(BITS($bits:expr))? $([ $($enum:ident : $value:expr),* ])?),* }) => {
        #[allow(non_snake_case)]
        #[allow(non_upper_case_globals)]
        pub mod $name {
            use super::*;
            define_system_register!{
                @$name $crn, $op1, $crm, $op2 { $($field OFFSET($offset) $(BITS($bits))? $([ $($enum : $value),* ])?),* }
            }
        }
    };
    */
}
