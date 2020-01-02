/***********************************************************************************************************************
 * Copyright (c) 2019 by the authors
 *
 * Author: André Borrmann
 * License: Apache 2.0
 **********************************************************************************************************************/

//! # MPIDR_EL1 - Multiprocessor Affinity Register
//!
//! Provides an additional core identification mechanism for scheduling purposes in a cluster system.
//! This is a read-only register.
//!
//! ## Usage Constraints
//! EL0 | EL1 (NS) | EL1(S) | EL2 | EL3(NS) | EL3(S)
//! ----|----------|--------|-----|---------|-------
//!  -  | R        | R      | R   | R       | R
//!

use crate::register::*;
use crate::{define_aarch64_register, impl_system_register_rw, register_field};

define_aarch64_register! {
    @mpidr_el1<u64> {
        /// Affinity level 0, indicates the core number in a processor
        AFF0 OFFSET(0) BITS(8),
        /// Affinity level 1
        AFF1 OFFSET(8) BITS(8),
        /// Affinity level 2
        AFF2 OFFSET(16) BITS(8),
        /// Indicates whether the lowest level of affinity consists of logical cores that are
        /// implemented using a multithreading type approach
        MT OFFSET(24),
        /// Indicates a single core system, as distinct from core 0 in a cluster.
        U OFFSET(30) [
            /// Core is part of a cluster
            POC = 0
        ],
        /// Affinity level 3
        AFF3 OFFSET(32) BITS(8)
    }
}
