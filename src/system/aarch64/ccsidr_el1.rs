/***********************************************************************************************************************
 * Copyright (c) 2019 by the authors
 *
 * Author: André Borrmann
 * License: Apache 2.0
 **********************************************************************************************************************/

//! # CCSIDR_EL1 Register
//! Cache Size ID Register

use crate::register::*;
use crate::{define_aarch64_register, impl_system_register_rw, register_field};

define_aarch64_register! {
    // ReadOnly CCSIDR_EL1 register
    @ccsidr_el1<u32> {
        LINESIZE OFFSET(0) BITS(3),
        ASSOC OFFSET(3) BITS(10),
        NUMSETS OFFSET(13) BITS(15),
        WA OFFSET(28) [
            // cache level supports no write-allocation
            CLNWA: 0,
            // cache level supports write-allocation
            CLWA: 1
        ],
        RA OFFSET(29) [
            // cache level supports no read-allocation
            CLNRA: 0,
            // cache level supports read-allocation
            CLRA: 1
        ],
        WB OFFSET(30) [
            // cache level supports no write-back
            CLNWB: 0,
            // cache level supports write-back
            CLWB: 1
        ],
        WT OFFSET(31) [
            // cache level supports no write-through
            CLNWT: 0,
            // cache level supports write-through
            CLWT: 1
        ]
    }
}
