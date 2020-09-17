/***********************************************************************************************************************
 * Copyright (c) 2019 by the authors
 * 
 * Author: 2ndTaleStudio <43264484+2ndTaleStudio@users.noreply.github.com>
 * License: Apache License 2.0 / MIT
 **********************************************************************************************************************/
#![doc(html_root_url = "https://docs.rs/ruspiro-register/||VERSION||")]
// we require to run with 'std' in unit tests and doc tests to have an allocator in place
#![cfg_attr(not(any(test, doctest)), no_std)]

//! # RusPiRo Register
//! 
//! The crate provides the definitions to conviniently work with register field values that are typically presented by 
//! a set of bit fields. This crate will likely be used in other crates whichspecifies the actual registers and their
//! structure using macros. Examples are [ruspiro-mmio-register](https://crates.io/crates/ruspiro-mmio-register) and
//! [ruspiro-arch-aarch64](https://crates.io/crates/ruspiro-arch-aarch64)

use core::cmp::PartialEq;
use core::ops::{BitAnd, BitOr, Not, Shl, Shr};
use core::fmt;

mod macros;

/// This trait is used to describe the register size/length as type specifier. The trait is only implemented for the
/// internal types **u8**, **u16**, **u32** and **u64** to ensure safe register access sizes with compile time checking
pub trait RegisterType:
    Copy
    + Clone
    + PartialEq
    + BitOr<Output = Self>
    + BitAnd<Output = Self>
    + Not<Output = Self>
    + Shl<Self, Output = Self>
    + Shr<Self, Output = Self>
{
}

// Internal macro to ease the assignment of the custom trait to supported register sizes
#[doc(hidden)]
macro_rules! registertype_impl {
    // invoke the macro for a given type t as often as types are provided when invoking the macro
    ($( $t:ty ),*) => ($(
        impl RegisterType for $t { }
    )*)
}

// implement the type trait for specific unsigned types to enable only those register types/sizes
registertype_impl![u8, u16, u32, u64];

/// Definition of a field contained inside of a register. Each field is defined by a mask and the bit shift value
/// when constructing the field definition the stored mask is already shifted by the shift value
#[derive(Copy, Clone)]
pub struct RegisterField<T: RegisterType> {
    mask: T,
    shift: T,
}

/// Definition of a specific fieldvalue of a regiser. This structure allows to combine field values with bit operators
/// like ``|`` and ``&`` to build the final value that should be written to a register
#[derive(Copy, Clone)]
pub struct RegisterFieldValue<T: RegisterType> {
    /// register field definition
    field: RegisterField<T>,
    /// register field value
    value: T,
}

// Internal helper macro to implement:
// - ``RegisterField``struct for all relevant basic types
// - ``FieldValue`` struct for all relevant basic types
// - the operators for ``FieldValue``struct for all relevant basic types
#[doc(hidden)]
macro_rules! registerfield_impl {
    ($($t:ty),*) => ($(
        impl RegisterField<$t> {
            /// Create a new register field definition with the mask and the shift offset for this
            /// mask. The offset is the bit offset this field begins.
            #[inline]
            #[allow(dead_code)]
            pub const fn new(mask: $t, shift: $t) -> RegisterField<$t> {
                Self {
                    mask,
                    shift,
                }
            }

            /// retrieve the current mask of the field shifted to its correct position
            #[inline]
            #[allow(dead_code)]
            pub fn mask(&self) -> $t {
                self.mask.checked_shl(self.shift as u32).unwrap_or(0)
            }

            /// retrieve the current shift of the field
            #[allow(dead_code)]
            #[inline]
            pub fn shift(&self) -> $t {
                self.shift
            }
        }

        impl fmt::Debug for RegisterField<$t> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let mut high_bit = self.shift;
                let mut mask = self.mask;
                while mask > 0 {
                    high_bit += 1;
                    mask >>= 1;
                };
                high_bit -= 1;
                write!(f, "RegisterField {{\n    Bits: [{}:{}]\n    Mask: {:#b}\n}}",
                    high_bit, self.shift, self.mask.checked_shl(self.shift as u32).unwrap_or(0))
            }
        }

        impl RegisterFieldValue<$t> {
            /// Create a new fieldvalue based on the field definition and the value given
            #[inline]
            #[allow(dead_code)]
            pub const fn new(field: RegisterField<$t>, value: $t) -> Self {
                RegisterFieldValue {
                    field,
                    value: value & field.mask
                }
            }

            /// Create a new fieldvalue based on the field definition and the raw value given
            #[inline]
            #[allow(dead_code)]
            pub const fn from_raw(field: RegisterField<$t>, raw_value: $t) -> Self {
                RegisterFieldValue {
                    field,
                    value: (raw_value >> field.shift) & field.mask
                }
            }

            /// Retrieve the register field value
            #[inline]
            #[allow(dead_code)]
            pub fn value(&self) -> $t {
                self.value //>> self.field.shift()
            }

            /// Retrieve the register field raw value, means the value is returned in it's position
            /// as it appears in the register when read with the field mask applied but not
            /// shifted
            #[inline]
            #[allow(dead_code)]
            pub fn raw_value(&self) -> $t {
                self.value.checked_shl(self.field.shift as u32).unwrap_or(0)
            }

            /// Retrieve the field mask used with this register field. The mask is shifted to it's
            /// corresponding field position
            #[inline]
            #[allow(dead_code)]
            pub fn mask(&self) -> $t {
                self.field.mask()
            }
        }

        impl fmt::Debug for RegisterFieldValue<$t> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_struct("RegisterFieldValue")
                 .field("field", &self.field)
                 .field("value", &self.value)
                 .field("raw_value", &self.raw_value())
                 .finish()
            }
        }

        impl PartialEq for RegisterFieldValue<$t> {
            fn eq(&self, other: &Self) -> bool {
                self.value() == other.value()
            }
        }

        impl BitOr for RegisterFieldValue<$t> {
            type Output = RegisterFieldValue<$t>;

            #[inline]
            #[allow(dead_code)]
            fn bitor(self, rhs: RegisterFieldValue<$t>) -> Self {
                let field = RegisterField::<$t>::new( self.field.mask() | rhs.field.mask(), 0);
                RegisterFieldValue {
                    field,
                    value: (self.raw_value() | rhs.raw_value()),
                }
            }
        }

        impl BitAnd for RegisterFieldValue<$t> {
            type Output = RegisterFieldValue<$t>;
            #[inline]
            #[allow(dead_code)]
            fn bitand(self, rhs: RegisterFieldValue<$t>) -> Self {
                let field = RegisterField::<$t>::new( self.field.mask() & rhs.field.mask(), 0);
                RegisterFieldValue {
                    field,
                    value: (self.raw_value() & rhs.raw_value()),
                }
            }
        }
    )*);
}

registerfield_impl![u8, u16, u32, u64];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn register_field_mask() {
        let field = RegisterField::<u32>::new(0x3, 6);
        println!("{:?}", field);
        assert_eq!(field.mask(), 0x3 << 6);
    }

    #[test]
    fn register_field_shift() {
        let field = RegisterField::<u16>::new(0x3, 3);
        println!("{:?}", field);
        assert_eq!(field.shift(), 3);
    }

    #[test]
    fn register_field_value() {
        let value = RegisterFieldValue::<u32>::new(RegisterField::<u32>::new(0x3, 6), 2);
        println!("{:?}", value);
        assert_eq!(value.value(), 2);
        assert_eq!(value.raw_value(), 2 << 6);
    }

    #[test]
    fn register_field_value_eq() {
        let value1 = RegisterFieldValue::<u32>::new(
            RegisterField::<u32>::new(3, 0), 7
        );
        let value2 = RegisterFieldValue::<u32>::new(
            RegisterField::<u32>::new(3, 0), 7
        );
        println!("value1: {:?} =? value2: {:?}", value1, value2);
        assert_eq!(value1, value2);
    }

    #[test]
    fn register_field_value_or() {
        let value1 = RegisterFieldValue::<u16>::new(RegisterField::<u16>::new(0xF, 0), 0xA);
        let value2 = RegisterFieldValue::<u16>::new(RegisterField::<u16>::new(0x3, 4), 0x2);
        let value_or = value1 | value2;

        println!("{:?}", value_or);
        assert_eq!(value_or.value(), 0xA | (0x2 << 4));
        assert_eq!(value_or.raw_value(), 0xA | (0x2 << 4));
    }

    #[test]
    fn register_field_value_and() {
        let value1 = RegisterFieldValue::<u8>::new(RegisterField::<u8>::new(0xF, 0), 0xA);
        let value2 = RegisterFieldValue::<u8>::new(RegisterField::<u8>::new(0x3, 2), 0x2);
        let value_and = value1 & value2;

        println!("{:?}", value_and);
        assert_eq!(value_and.value(), 0xA & (0x2 << 2));
        assert_eq!(value_and.raw_value(), 0xA & (0x2 << 2));
    }

    #[test]
    fn register_value_update() {
        let field_value = RegisterFieldValue::<u32>::new(RegisterField::<u32>::new(0x3, 8), 0b01);
        let register_value: u32 = 0b1_1010_0011_0010;
        let new_value: u32 = (register_value & !field_value.mask()) | field_value.raw_value();

        assert_eq!(new_value, 0b1_1001_0011_0010);
    }
}