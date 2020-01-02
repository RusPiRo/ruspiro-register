/***********************************************************************************************************************
 * Copyright (c) 2019 by the authors
 *
 * Author: André Borrmann
 * License: Apache 2.0
 **********************************************************************************************************************/

//! # SCTLR_EL1 - System Control Register EL1
//!
//! Provides top level control of the system, including its memory system at EL1.
//!
//! ## Usage Constraints
//! EL0 | EL1 (NS) | EL1(S) | EL2 | EL3(NS) | EL3(S)
//! ----|----------|--------|-----|---------|-------
//!  -  | R/W      | R/W    | R/W | R/W     | R/W
//!

use crate::register::*;
use crate::{define_aarch64_register, impl_system_register_rw, register_field};

define_aarch64_register! {
    @sctlr_el1<u64> {
        /// globally enable MMU
        M   OFFSET(0) [
            DISABLE = 0,
            ENABLE = 1
        ],
        /// alignment fault check
        A   OFFSET(1) [
            DISABLE = 0,
            ENABLE = 1
        ],
        /// global data cache
        C   OFFSET(2) [
            DISABLE = 0,
            ENABLE = 1
        ],
        /// stack alignment checks
        SA  OFFSET(3) [
            DISABLE = 0,
            ENABLE = 1
        ],
        /// El0 stack alignment checks
        SA0 OFFSET(4) [
            DISABLE = 0,
            ENABLE = 1
        ],
        /// CP15 barrier operations enabled ?
        CP15EN OFFSET(5) [
            DISABLE = 0,
            ENABLE = 1
        ],
        /// IT instructions disabled ?
        ITD OFFSET(7) [
            DISABLED = 1,
            ENABLED = 0
        ],
        /// SETEND instructions disabled ?
        SED OFFSET(8) [
            DISABLED = 1,
            ENABLED = 0
        ],
        /// Controls access to interrupt masks from EL0 if EL0 is using Aarch64
        UMA OFFSET(9) [
            DISABLED = 0,
            ENABLED = 1
        ],
        /// instruction cache
        I   OFFSET(12) [
            DISABLE = 0,
            ENABLE = 1
        ],
        /// Enables access to the DC ZVA instruction at EL0
        DZE OFFSET(14) [
            DISABLED = 0,
            ENABLED = 1
        ],
        /// Enables EL0 access to the CTR_EL0 register in Aacrh64 mode
        UCT OFFSET(15) [
            DISABLED = 0,
            ENABLED = 1
        ],
        /// Non-trapping WFI instruction
        NTWI OFFSET(17) [
            /// WFI executions in EL0 are trapped to EL1
            TRAP_EL1 = 0,
            /// WFI executions in EL0 are executed as normal
            NO_TRAP = 1
        ],
        /// Non-trapping WFE instruction
        NTWE OFFSET(18) [
            /// WFE executions in EL0 are trapped to EL1
            TRAP_EL1 = 0,
            /// WFE executions in EL0 are executed as normal
            NO_TRAP = 1
        ],
        /// Force all memory regions with write permissions as XN
        WXN     OFFSET(19) [
            DONT_FORCE = 0,
            FORCE = 1
        ],
        /// explicit data access endiannes at EL0
        E0E     OFFSET(24) [
            /// data accesses at EL0 are little-endian
            LTL_ENDIAN = 0,
            /// data accesses at EL0 are big-endian
            BIG_ENDIAN = 1
        ],
        /// exception endiannes
        EE      OFFSET(25) [
            LTL_ENDIAN = 0,
            BIG_ENDIAN = 1
        ],
        /// Enable EL0 access to cache maintenance instructions:
        /// DC CVAU, DC CIVAC, DC CVAC and IC IVAU in Aarch64 mode
        UCI OFFSET(26) [
            DISABLED = 0,
            ENABLED = 1
        ]
    }
}
