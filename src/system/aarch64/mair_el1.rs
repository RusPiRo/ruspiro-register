/***********************************************************************************************************************
 * Copyright (c) 2019 by the authors
 *
 * Author: André Borrmann
 * License: Apache 2.0
 **********************************************************************************************************************/

//! # MAIR_EL1 - Memory Attribute Indirection Register EL1
//!
//! Provides the memory attribute encodings corresponding to the possible AttrIndx values in a
//! Long-descriptor format translation table entry for stage 1 translations at EL1.
//!
//! ## Usage Constraints
//! EL0 | EL1 (NS) | EL1(S) | EL2 | EL3(NS) | EL3(S)
//! ----|----------|--------|-----|---------|-------
//!  -  | R/W      | R/W    | R/W | R/W     | R/W
//!

use crate::register::*;
use crate::{define_aarch64_register, impl_system_register_rw, register_field};

define_aarch64_register! {
    @mair_el1<u64> {
        MAIR0 OFFSET(0) BITS(8) [
            NGNRNE = 0x00,
            NGNRE = 0x04,
            GRE = 0x0C,
            NC = 0x44,
            NORM = 0xFF
        ],
        MAIR1 OFFSET(8) BITS(8) [
            NGNRNE = 0x00,
            NGNRE = 0x04,
            GRE = 0x0C,
            NC = 0x44,
            NORM = 0xFF
        ],
        MAIR2 OFFSET(16) BITS(8) [
            NGNRNE = 0x00,
            NGNRE = 0x04,
            GRE = 0x0C,
            NC = 0x44,
            NORM = 0xFF
        ],
        MAIR3 OFFSET(24) BITS(8) [
            NGNRNE = 0x00,
            NGNRE = 0x04,
            GRE = 0x0C,
            NC = 0x44,
            NORM = 0xFF
        ],
        MAIR4 OFFSET(32) BITS(8) [
            NGNRNE = 0x00,
            NGNRE = 0x04,
            GRE = 0x0C,
            NC = 0x44,
            NORM = 0xFF
        ]
    }
}
