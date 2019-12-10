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
        // MMU enable flag
        M   OFFSET(0) [
            ENABLE:    0b1,
            DISABLE:   0b0
        ],
        // Alignment check enable flag
        A   OFFSET(1) [
            ENABLE:    0b1,
            DISABLE:   0b0
        ],
        // Data cache enable flag
        C   OFFSET(2) [
            ENABLE:    0b1,
            DISABLE:   0b0
        ],
        // CP15 barrier enable flag
        CP15BEN OFFSET(5) [
            ENABLE:    0b1,
            DISABLE:   0b0
        ],
        // IT instruction disable flag
        ITD     OFFSET(7) [
            UNAVAILABLE: 0b1,
            AVAILABLE:   0b0
        ],
        // SETEND instruction disable flag
        SED     OFFSET(8) [
            AVAILABLE:   0b0,
            UNAVAILABLE: 0b1
        ],
        // Instruction cache enable bit
        I       OFFSET(12) [
            ENABLE:    0b1,
            DISABLE:   0b0
        ],
        // Exception vector table bits
        V       OFFSET(13) [
            // base address starts higher than 0xFFFF_0000
            HIGH_VBAR:    0b1,
            // base address starts higher than 0x0000_0000
            NORMAL_VBAR:  0b0
        ],
        // Do not trap WFI instructions
        NTWI    OFFSET(16) [
            NO_TRAP:    0b1,
            TRAP:       0b0
        ],
        // Do not trap WFE instruction
        NTWE    OFFSET(18) [
            NO_TRAP:    0b1,
            TRAP:       0b0
        ],
        // Flag whether write permission implies ExecuteNever
        WXN    OFFSET(19) [
            WRITE_FORCE_XN:     0b1,
            WRITE_NOT_FORCE_XN: 0b0
        ],
        // Flag whether unprivileged write permission implies EL1 ExecuteNever
        UWXN   OFFSET(20) [
            WRITE_FORCE_XN:     0b1,
            WRITE_NOT_FORCE_XN: 0b0
        ],
        // Exception endianness bit
        EE      OFFSET(25) [
            LE: 0b0,
            BE: 0b1
        ],
        // Enable remapping of the TEX[2:1] bits for use as two translation table bits
        TRE     OFFSET(28) [
            DISABLED: 0b0,
            ENABLED:  0b1
        ],
        // Flag to enable use of the AP[0] bit in translation descriptors
        AFE     OFFSET(29) [
            // Use AP[0] access permission bit
            FULL_PERMISSION:    0b0,
            // Don't use AP[0] access permission bit
            LIMITED_PERMISSION: 0b1
        ],
        // Control whether exceptions are taken in A32 or T32
        TE      OFFSET(30) [
            EXCEPTION_A32: 0b0,
            EXCEPTION_T32: 0b1
        ]
    }
}