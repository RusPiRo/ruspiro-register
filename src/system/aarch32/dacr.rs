/***********************************************************************************************************************
 * Copyright (c) 2019 by the authors
 *
 * Author: André Borrmann
 * License: Apache 2.0
 **********************************************************************************************************************/

//! # DACR Register
//! Domain Access Control Register
//!

use crate::register::*;
use crate::{define_aarch32_register, impl_system_register_rw, register_field};

define_aarch32_register! {
    @dacr c3, 0, c0, 0 {
        // MMU domain access permissions
        D0  OFFSET(0) BITS(2) [
            // no access - any access generaits a domain fault
            NONE =       0b00,
            // client access checks permission bits against translation table entries
            CLIENT =     0b01,
            // manager access treats every access as granted without checking permission bits in ttlb
            MANAGER =    0b11
        ],
        D1  OFFSET(2) BITS(2) [
            // no access - any access generaits a domain fault
            NONE =       0b00,
            // client access checks permission bits against translation table entries
            CLIENT =     0b01,
            // manager access treats every access as granted without checking permission bits in ttlb
            MANAGER =    0b11
        ],
        D2  OFFSET(4) BITS(2) [
            // no access - any access generaits a domain fault
            NONE =       0b00,
            // client access checks permission bits against translation table entries
            CLIENT =     0b01,
            // manager access treats every access as granted without checking permission bits in ttlb
            MANAGER =    0b11
        ],
        D3  OFFSET(6) BITS(2) [
            // no access - any access generaits a domain fault
            NONE =       0b00,
            // client access checks permission bits against translation table entries
            CLIENT =     0b01,
            // manager access treats every access as granted without checking permission bits in ttlb
            MANAGER =    0b11
        ],
        D4  OFFSET(8) BITS(2) [
            // no access - any access generaits a domain fault
            NONE =       0b00,
            // client access checks permission bits against translation table entries
            CLIENT =     0b01,
            // manager access treats every access as granted without checking permission bits in ttlb
            MANAGER =    0b11
        ],
        D5  OFFSET(10) BITS(2) [
            // no access - any access generaits a domain fault
            NONE =       0b00,
            // client access checks permission bits against translation table entries
            CLIENT =     0b01,
            // manager access treats every access as granted without checking permission bits in ttlb
            MANAGER =    0b11
        ],
        D6  OFFSET(12) BITS(2) [
            // no access - any access generaits a domain fault
            NONE =       0b00,
            // client access checks permission bits against translation table entries
            CLIENT =     0b01,
            // manager access treats every access as granted without checking permission bits in ttlb
            MANAGER =    0b11
        ],
        D7  OFFSET(14) BITS(2) [
            // no access - any access generaits a domain fault
            NONE =       0b00,
            // client access checks permission bits against translation table entries
            CLIENT =     0b01,
            // manager access treats every access as granted without checking permission bits in ttlb
            MANAGER =    0b11
        ],
        D8  OFFSET(16) BITS(2) [
            // no access - any access generaits a domain fault
            NONE =       0b00,
            // client access checks permission bits against translation table entries
            CLIENT =     0b01,
            // manager access treats every access as granted without checking permission bits in ttlb
            MANAGER =    0b11
        ],
        D9  OFFSET(18) BITS(2) [
            // no access - any access generaits a domain fault
            NONE =       0b00,
            // client access checks permission bits against translation table entries
            CLIENT =     0b01,
            // manager access treats every access as granted without checking permission bits in ttlb
            MANAGER =    0b11
        ],
        D10 OFFSET(20) BITS(2) [
            // no access - any access generaits a domain fault
            NONE =       0b00,
            // client access checks permission bits against translation table entries
            CLIENT =     0b01,
            // manager access treats every access as granted without checking permission bits in ttlb
            MANAGER =    0b11
        ],
        D11 OFFSET(22) BITS(2) [
            // no access - any access generaits a domain fault
            NONE =       0b00,
            // client access checks permission bits against translation table entries
            CLIENT =     0b01,
            // manager access treats every access as granted without checking permission bits in ttlb
            MANAGER =    0b11
        ],
        D12 OFFSET(24) BITS(2) [
            // no access - any access generaits a domain fault
            NONE =       0b00,
            // client access checks permission bits against translation table entries
            CLIENT =     0b01,
            // manager access treats every access as granted without checking permission bits in ttlb
            MANAGER =    0b11
        ],
        D13 OFFSET(26) BITS(2) [
            // no access - any access generaits a domain fault
            NONE =       0b00,
            // client access checks permission bits against translation table entries
            CLIENT =     0b01,
            // manager access treats every access as granted without checking permission bits in ttlb
            MANAGER =    0b11
        ],
        D14 OFFSET(28) BITS(2) [
            // no access - any access generaits a domain fault
            NONE =       0b00,
            // client access checks permission bits against translation table entries
            CLIENT =     0b01,
            // manager access treats every access as granted without checking permission bits in ttlb
            MANAGER =    0b11
        ],
        D15 OFFSET(30) BITS(2) [
            // no access - any access generaits a domain fault
            NONE =       0b00,
            // client access checks permission bits against translation table entries
            CLIENT =     0b01,
            // manager access treats every access as granted without checking permission bits in ttlb
            MANAGER =    0b11
        ]
    }
}
