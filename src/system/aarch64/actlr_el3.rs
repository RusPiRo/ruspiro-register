/***********************************************************************************************************************
 * Copyright (c) 2019 by the authors
 *
 * Author: André Borrmann
 * License: Apache 2.0
 **********************************************************************************************************************/

//! # ACTLR_EL3 - Auxiliry Control Register EL3
//!
//! This controls write access to *implementation defined* registers in EL2 modes.
//!
//! ## Usage Constraints
//! EL0 | EL1 (NS) | EL1(S) | EL2 | EL3(NS) | EL3(S)
//! ----|----------|--------|-----|---------|-------
//!  -  | -        | -      | -   | R/W     | R/W
//!

use crate::register::*;
use crate::{define_aarch64_register, impl_system_register_rw, register_field};

define_aarch64_register! {
    @actlr_el3<u32> {
        /// Write access control to CPUACTLR_EL1
        CPUACTLR_EL1 OFFSET(0) [
            /// No write access to this register in EL2
            READONLY = 0,
            /// Write access to this register in EL2
            READWRITE = 1
        ],
        /// Write access control to CPUECTLR_EL1
        CPUECTLR_EL1 OFFSET(1) [
            /// No write access to this register in EL2
            READONLY = 0,
            /// Write access to this register in EL2
            READWRITE = 1
        ],
        /// Write access control to L2CTLR_EL1
        L2CTLR_EL1 OFFSET(4) [
            /// No write access to this register in EL2
            READONLY = 0,
            /// Write access to this register in EL2
            READWRITE = 1
        ],
        /// Write access control to L2ECTLR_EL1
        L2ECTLR_EL1 OFFSET(5) [
            /// No write access to this register in EL2
            READONLY = 0,
            /// Write access to this register in EL2
            READWRITE = 1
        ],
        /// Write access control to L2ACTLR_EL1
        L2ACTLR_EL1 OFFSET(6) [
            /// No write access to this register in EL2
            READONLY = 0,
            /// Write access to this register in EL2
            READWRITE = 1
        ]
    }
}
