/*********************************************************************************************************************** 
 * Copyright (c) 2019 by the authors
 * 
 * Author: André Borrmann 
 * License: Apache 2.0
 **********************************************************************************************************************/

//! # TTBR1 Register
//! 

use crate::{define_aarch32_register, register_field};
use crate::register::*;

define_aarch32_register!{
    @ttbr1 c2, 0, c0, 1 {
        // the IRGN field is 2 bits but splitted accross different offset locations
        // their meanings:
        // 0b00 -> normal, inner non-cacheable
        // 0b01 -> normal, inner write-back write-allocate cacheable
        // 0b10 -> normal, inner write-through cacheable
        // 0b11 -> normal, inner write-back no write-allocate cacheable
        IRGN1   OFFSET(0),
        IRGN0   OFFSET(6),
        // shareable flag for memory associated with ttlb walks
        S       OFFSET(1) [
            NON_SHAREABLE: 0,
            SHAREABLE: 1
        ],
        // region bits for outer cachability attributes
        RGN     OFFSET(3) BITS(2) [
            // normal memory, outer non-caheable
            NORM_O_NC:      0b00,
            // normal memory, outer write-back write-allocate cacheable
            NORM_OWB_WAC:   0b01,
            // normal memory, outer write-through cacheable
            NORM_OWT_C:     0b10,
            // normal memory, outer write-back no write-allocate cacheable
            NORM_OWB_NWAC:  0b11
        ],
        // not outer shareable bit
        NOS     OFFSET(5) [
            OUTER:  0b0,
            INNER:  0b1
        ],
        // bits 31:14 of translation table base register
        // the address value passed to this field need to shifted right by 14 bits
        // the real used bits are defined by TTBCR.N [31:14-TTBCR.N]
        TTB1    OFFSET(14) BITS(18)
    }
}