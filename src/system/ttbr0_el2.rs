/*********************************************************************************************************************** 
 * Copyright (c) 2019 by the authors
 * 
 * Author: André Borrmann 
 * License: Apache 2.0
 **********************************************************************************************************************/

//! # TTBR0_EL2 Register
//! 

use crate::{ define_system_register, register_field };
use crate::register::*;

define_system_register! {
    @ttbr0_el2<u64> {
        baddr OFFSET(0) BITS(48),
        asid OFFSET(48) BITS(16)
    }
}