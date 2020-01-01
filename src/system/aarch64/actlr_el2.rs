/***********************************************************************************************************************
 * Copyright (c) 2019 by the authors
 *
 * Author: André Borrmann
 * License: Apache 2.0
 **********************************************************************************************************************/

//! # ACTLR_EL2 Register
//! Auxiliry Control Register EL2

use crate::register::*;
use crate::{define_aarch64_register, impl_system_register_rw, register_field};

define_aarch64_register! {
    @actlr_el2<u32> {
        CPUACTLR_EL1 OFFSET(0),
        CPUECTLR_EL1 OFFSET(1),
        L2CTLR_EL1 OFFSET(4),
        L2ECTLR_EL1 OFFSET(5),
        L2ACTLR_EL1 OFFSET(6)
    }
}
