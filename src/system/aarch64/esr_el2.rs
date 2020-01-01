/***********************************************************************************************************************
 * Copyright (c) 2019 by the authors
 *
 * Author: André Borrmann
 * License: Apache 2.0
 **********************************************************************************************************************/

//! # ESR_EL2 Register
//! Exception Syndrom Register EL2

use crate::register::*;
use crate::{define_aarch64_register, impl_system_register_rw, register_field};

define_aarch64_register! {
    @esr_el2<u32> {
        // Syndrome information
        ISS OFFSET(0) BITS(24),
        // instruction length for sync exceptions
        IL  OFFSET(25) [
            _16Bit: 0,
            _32Bit: 1
        ],
        // Exception Class
        EC OFFSET(26) BITS(6)
    }
}
