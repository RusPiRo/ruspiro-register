/***********************************************************************************************************************
 * Copyright (c) 2019 by the authors
 *
 * Author: André Borrmann
 * License: Apache 2.0
 **********************************************************************************************************************/

//! # TTBR0_EL1 Register
//!

use crate::register::*;
use crate::{define_aarch64_register, impl_system_register_rw, register_field};

define_aarch64_register! {
    @ttbr0_el1<u64> {
        baddr OFFSET(0) BITS(48),
        asid OFFSET(48) BITS(16)
    }
}
