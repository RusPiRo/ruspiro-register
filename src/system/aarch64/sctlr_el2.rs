/***********************************************************************************************************************
 * Copyright (c) 2019 by the authors
 *
 * Author: André Borrmann
 * License: Apache 2.0
 **********************************************************************************************************************/

//! # SCTLR_EL2 Register
//!

use crate::register::*;
use crate::{define_aarch64_register, impl_system_register_rw, register_field};

define_aarch64_register! {
    @sctlr_el2<u64> {
        // globally enable MMU
        M   OFFSET(0) [
            DISABLE: 0b0,
            ENABLE: 0b1
        ],
        // alignment fault check
        A   OFFSET(1) [
            DISABLE: 0b0,
            ENABLE: 0b1
        ],
        // global data cache
        C   OFFSET(2) [
            DISABLE: 0b0,
            ENABLE: 0b1
        ],
        // stack alignment checks
        SA  OFFSET(3) [
            DISABLE: 0b0,
            ENABLE: 0b1
        ],
        // instruction cache
        I   OFFSET(12) [
            DISABLE: 0b0,
            ENABLE: 0b1
        ],
        // Force all memory regions with write permissions as XN
        WXN     OFFSET(19) [
            DONT_FORCE: 0b0,
            FORCE: 0b1
        ],
        // exception endiannes
        EE      OFFSET(25) [
            LTL_ENDIAN: 0b0,
            BIG_ENDIAN: 0b1
        ]
    }
}
