/***********************************************************************************************************************
 * Copyright (c) 2019 by the authors
 *
 * Author: André Borrmann
 * License: Apache 2.0
 **********************************************************************************************************************/

//! # CPACR_EL1 Register
//!

use crate::register::*;
use crate::{define_aarch64_register, impl_system_register_rw, register_field};

define_aarch64_register! {
    @cpacr_el1<u64> {
        // trap floating point instractions in EL0/1
        fpen OFFSET(20) BITS(2) [
            TRAP_ALL: 0b00,
            TRAP_EL0: 0b01,
            TRAP_EL0_OR_1: 0b10,
            NO_TRAP:  0b11
        ],
        // trap trace functions in EL0/1
        tta OFFSET(28) [
            NO_TRAP: 0b0,
            TRAP:    0b1
        ]
    }
}
