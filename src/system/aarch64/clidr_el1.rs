/***********************************************************************************************************************
 * Copyright (c) 2019 by the authors
 *
 * Author: André Borrmann
 * License: Apache 2.0
 **********************************************************************************************************************/

//! # CLIDR_EL1 - Cache Level ID Register
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
    @clidr_el1<u64> {
        /// Type of cache implemented at L1
        CTYPE1 OFFSET(0) BITS(3),
        /// Type of cache implemented at L2
        CTYPE2 OFFSET(3) BITS(3),
        /// Type of cache implemented at L3
        CTYPE3 OFFSET(6) BITS(3),
        /// Level of Unification inner sharable for cache hierarchy
        LOUIS OFFSET(21) BITS(3),
        /// Level of Coherency for cache hierarchy
        LOC OFFSET(24) BITS(3),
        /// Level of Unification Uniprocessor for cache hierarchy
        LOUU OFFSET(27) BITS(3),
        /// Inner cache boundary
        ICB OFFSET(30) BITS(3)
    }
}
