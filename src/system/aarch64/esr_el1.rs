/***********************************************************************************************************************
 * Copyright (c) 2019 by the authors
 *
 * Author: André Borrmann
 * License: Apache 2.0
 **********************************************************************************************************************/

//! # ESR_EL1 - Exception Syndrom Register EL1
//!
//! Holds syndrome information for an exception taken to EL1
//!
//! ## Usage Constraints
//! EL0 | EL1 (NS) | EL1(S) | EL2 | EL3(NS) | EL3(S)
//! ----|----------|--------|-----|---------|-------
//!  -  | R/W      | R/W    | R/W | R/W     | R/W
//!

use crate::register::*;
use crate::{define_aarch64_register, impl_system_register_rw, register_field};

define_aarch64_register! {
    @esr_el2<u32> {
        /// Syndrome information
        ISS OFFSET(0) BITS(24),
        /// Syndrome valid flag
        VALID OFFSET(24) [
            /// ISS is not valid
            INVALID = 0,
            /// ISS is valid
            VALID = 1
        ],
        /// Instruction length for sync exceptions
        IL  OFFSET(25) [
            _16Bit = 0,
            _32Bit = 1
        ],
        /// Exception Class, indicating the reason for the exception
        EC OFFSET(26) BITS(6)
    }
}
