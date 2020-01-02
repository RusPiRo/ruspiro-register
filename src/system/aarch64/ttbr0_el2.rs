/***********************************************************************************************************************
 * Copyright (c) 2019 by the authors
 *
 * Author: André Borrmann
 * License: Apache 2.0
 **********************************************************************************************************************/

//! # TTBR0_EL2 - Translation Table Base Register 0 EL2
//!
//! Holds the base address of translation table 0, and information about the memory it occupies.
//! This is one of the translation tables for the stage 1 translation of memory accesses from
//! EL2 or HYP mode
//!
//! ## Usage Constraints
//! EL0 | EL1 (NS) | EL1(S) | EL2 | EL3(NS) | EL3(S)
//! ----|----------|--------|-----|---------|-------
//!  -  | -        | -      | R/W | R/W     | R/W
//!

use crate::register::*;
use crate::{define_aarch64_register, impl_system_register_rw, register_field};

define_aarch64_register! {
    @ttbr0_el2<u64> {
        /// Translation table base address bits\[47:x\]. **x** is based on the value of ``TCR_EL2::T0SZ``,
        /// the stage of translation and the memory translation granule size
        BADDR OFFSET(0) BITS(48),
        /// An ASID for the translation table base address.
        ASID OFFSET(48) BITS(16)
    }
}
