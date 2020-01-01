/***********************************************************************************************************************
 * Copyright (c) 2019 by the authors
 *
 * Author: André Borrmann
 * License: Apache 2.0
 **********************************************************************************************************************/

//! # MPIDR_EL1 Register
//!

use crate::register::*;
use crate::{define_aarch64_register, impl_system_register_rw, register_field};

define_aarch64_register! {
    // ReadOnly MPIDR_EL1 register
    @mpidr_el1<u64> {
        // Affinity level 0, indicates the core number in a processor
        AFF0 OFFSET(0) BITS(8),
        AFF1 OFFSET(8) BITS(8),
        AFF2 OFFSET(16) BITS(8),
        MT OFFSET(24),
        U OFFSET(30),
        AFF3 OFFSET(32) BITS(8)
    }
}
