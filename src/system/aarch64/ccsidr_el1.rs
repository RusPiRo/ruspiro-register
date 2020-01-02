/***********************************************************************************************************************
 * Copyright (c) 2019 by the authors
 *
 * Author: André Borrmann
 * License: Apache 2.0
 **********************************************************************************************************************/

//! # CCSIDR_EL1 - Cache Size ID Register
//!
//! This is a read-only register
//!
//! ## Usage Constraints
//! EL0 | EL1 (NS) | EL1(S) | EL2 | EL3(NS) | EL3(S)
//! ----|----------|--------|-----|---------|-------
//!  -  | R        | R      | R   | R       | R
//!

use crate::register::*;
use crate::{define_aarch64_register, impl_system_register_rw, register_field};

define_aarch64_register! {
    @ccsidr_el1<u32> {
        /// Indicates the (log2(number of words in cache line)) - 2
        LINESIZE OFFSET(0) BITS(3) [
            /// 16 words per line
            _16 = 0b010
        ],
        /// Indicates the associativity of the cache - 1. A value of 0 indicates an associativity of
        /// 1. It does not need to be a power of 2.
        ASSOC OFFSET(3) BITS(10),
        /// Indeicates the number of sets in cache - 1. A value of 0 indeicates 1 set in the cache.
        /// It does not need to be a power of 2.
        NUMSETS OFFSET(13) BITS(15),
        /// Indicates support for write-allocation
        WA OFFSET(28) [
            /// cache level supports no write-allocation
            CLNWA = 0,
            /// cache level supports write-allocation
            CLWA = 1
        ],
        /// Indicates support for reas-allocation
        RA OFFSET(29) [
            /// cache level supports no read-allocation
            CLNRA = 0,
            /// cache level supports read-allocation
            CLRA = 1
        ],
        /// Indicates support for write-back
        WB OFFSET(30) [
            // cache level supports no write-back
            CLNWB = 0,
            // cache level supports write-back
            CLWB = 1
        ],
        /// Indicates support for write-through
        WT OFFSET(31) [
            // cache level supports no write-through
            CLNWT = 0,
            // cache level supports write-through
            CLWT = 1
        ]
    }
}
