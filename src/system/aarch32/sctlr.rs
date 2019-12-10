/*********************************************************************************************************************** 
 * Copyright (c) 2019 by the authors
 * 
 * Author: André Borrmann 
 * License: Apache 2.0
 **********************************************************************************************************************/

//! # SCTLR Register
//! 

use crate::{define_aarch32_register, register_field};
use crate::register::*;

define_aarch32_register!{
    @sctlr c1, 0, c0, 0 {
        M   OFFSET(0) [
            ENABLED:    1,
            DISABLED:   0
        ],
        A   OFFSET(1) [
            ENABLED:    1,
            DISABLED:   0
        ],
        C   OFFSET(2) [
            ENABLED:    1,
            DISABLED:   0
        ],
        CP15BEN OFFSET(5) [
            ENABLED:    1,
            DISABLED:   0
        ],
        ITD     OFFSET(7) [
            UNAVAILABLE: 1,
            AVAILABLE:   0
        ],
        SED     OFFSET(8) [
            AVAILABLE:   1,
            UNAVAILABLE: 0
        ],
        I       OFFSET(12) [
            ENABLED:    1,
            DISABLED:   0
        ],
        V       OFFSET(13) [
            HIGH_VBAR:    1,
            NORMAL_VBAR:  0
        ],
        NTWI    OFFSET(16) [
            NO_TRAP:    1,
            TRAP:       0
        ],
        NTWE    OFFSET(18) [
            NO_TRAP:    1,
            TRAP:       0
        ],
        WXN    OFFSET(19) [
            WRITE_FORCE_XN:     1,
            WRITE_NOT_FORCE_XN: 0
        ],
        UWXN   OFFSET(20) [
            WRITE_FORCE_XN:     1,
            WRITE_NOT_FORCE_XN: 0
        ],
        EE      OFFSET(25) [
            LE: 0,
            BE: 1
        ],
        TRE     OFFSET(28) [
            DISABLED: 0,
            ENABLED:  1
        ],
        AFE     OFFSET(29) [
            FULL_PERMISSION: 0,
            LIMITED_PERMISION: 1
        ],
        TE      OFFSET(30) [
            EXCPETION_A32: 0,
            EXCPETION_T32: 1
        ]
    }
}