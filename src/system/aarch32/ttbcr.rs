/***********************************************************************************************************************
 * Copyright (c) 2019 by the authors
 *
 * Author: André Borrmann
 * License: Apache 2.0
 **********************************************************************************************************************/

//! # TTBCR Register
//!

use crate::register::*;
use crate::{define_aarch32_register, impl_system_register_rw, register_field};

define_aarch32_register! {
    @ttbcr c2, 0, c0, 2 {
        // width of ttlb base address stored in TTBR0 - means using bits [31:14-N]
        // this value also drives the alignment requirement for the TTBR0 base address
        N   OFFSET(0) BITS(3),
        // Disable translation table walk for TTBR0
        PD0 OFFSET(4) [
            ENABLED = 0,
            DISABLED = 0
        ],
        // Disable translation table walk for TTBR1
        PD1 OFFSET(5) [
            ENABLED = 0,
            DISABLED = 0
        ],
        // extended address enable flag, if enabled it uses 64Bit long descriptor format
        EAE OFFSET(31)
    }
}
