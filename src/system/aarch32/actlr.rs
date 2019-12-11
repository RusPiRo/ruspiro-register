/***********************************************************************************************************************
 * Copyright (c) 2019 by the authors
 *
 * Author: André Borrmann
 * License: Apache 2.0
 **********************************************************************************************************************/

//! # ACTLR Register
//!

use crate::register::*;
use crate::{define_aarch32_register, impl_system_register_rw, register_field};

define_aarch32_register! {
    @actlr c1, 0, c0, 1 {
        // accessibility for CPUACTLR from lower exception levels
        CPUACTLR    OFFSET(0) [
            READONLY: 0,
            READWRITE: 1
        ],
        // accessibility for CPUECTLR from lower exception levels
        CPUECTLR    OFFSET(1) [
            READONLY: 0,
            READWRITE: 1
        ],
        // accessibility for L2CTLR from lower exception levels
        L2CTLR      OFFSET(4) [
            READONLY: 0,
            READWRITE: 1
        ],
        // accessibility for L2ECTLR from lower exception levels
        L2ECTLR     OFFSET(5) [
            READONLY: 0,
            READWRITE: 1
        ],
        // accessibility for L2ACTLR from lower exception levels
        L2ACTLR     OFFSET(6) [
            READONLY: 0,
            READWRITE: 1
        ]
    }
}
