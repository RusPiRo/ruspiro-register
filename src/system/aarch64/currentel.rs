/***********************************************************************************************************************
 * Copyright (c) 2019 by the authors
 *
 * Author: André Borrmann
 * License: Apache 2.0
 **********************************************************************************************************************/

//! # CurrentEL - Current Exception Level
//!
//! Hold the current exception level
//!

use crate::register::*;
use crate::{define_aarch64_register, impl_system_register_rw, register_field};

define_aarch64_register! {
    @currentEl<u64> {
        /// The current exception level
        EL OFFSET(2) BITS(2) [
            EL0 = 0b00,
            EL1 = 0b01,
            EL2 = 0b10,
            EL3 = 0b11
        ]
    }
}
