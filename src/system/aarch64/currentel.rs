/*********************************************************************************************************************** 
 * Copyright (c) 2019 by the authors
 * 
 * Author: André Borrmann 
 * License: Apache 2.0
 **********************************************************************************************************************/

//! # CurrentEL Register
//! 

use crate::{ define_aarch64_register, register_field };
use crate::register::*;

define_aarch64_register! {
    @currentEl<u64> {
        el OFFSET(2) BITS(2)
    }
}