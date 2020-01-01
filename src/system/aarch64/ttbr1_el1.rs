/***********************************************************************************************************************
 * Copyright (c) 2019 by the authors
 *
 * Author: André Borrmann
 * License: Apache 2.0
 **********************************************************************************************************************/

//! # TTBR1_EL1 Register
//!

use crate::register::*;
use crate::{define_aarch64_register, impl_system_register_rw, register_field};

define_aarch64_register! {
    @ttbr1_el1<u64> {
        BADDR OFFSET(0) BITS(48),
        ASID OFFSET(48) BITS(16)
    }
}
