/***********************************************************************************************************************
 * Copyright (c) 2019 by the authors
 *
 * Author: André Borrmann
 * License: Apache 2.0
 **********************************************************************************************************************/

//! # TCR_EL2 - Translation Control Register EL2
//!
//! Controls translation table walks required for stage 1 translation of a memory access from EL2
//! and holds cacheability and shareability information.
//!
//! ## Usage Constraints
//! EL0 | EL1 (NS) | EL1(S) | EL2 | EL3(NS) | EL3(S)
//! ----|----------|--------|-----|---------|-------
//!  -  | -        | -      | R/W | R/W     | R/W
//!

use crate::register::*;
use crate::{define_aarch64_register, impl_system_register_rw, register_field};

define_aarch64_register! {
    @tcr_el2<u32> {
        /// Size offset of the memory reagion addressed by ttbr0_el2 (size = 2^(64-t0sz))
        T0SZ    OFFSET(0) BITS(6),
        /// Inner cacheability attribute for memory associated with tlb walks using ttbr0_el2
        IRGN0   OFFSET(8) BITS(2) [
            /// normal memory, inner non-cacheable
            NM_INC =         0b00,
            /// normal memory, inner write-back, read-allocate, write-allocate, cacheable
            NM_IWB_RA_WA =   0b01,
            /// normal memory, inner write-through, read-allocate, no write-allocate, cacheable
            NM_IWT_RA_NWA =  0b10,
            /// normal memory, inner write-back, read-allocate, no write-allocate, caheable
            NM_IWB_RA_NWA =  0b11
        ],
        /// Outer cacheability attribute for memory associated with tlb walks using ttbr0_el2
        ORGN0   OFFSET(10) BITS(2) [
            /// normal memory, outer non-cacheable
            NM_ONC =         0b00,
            /// normal memory, outer write-back, read-allocate, write-allocate, cacheable
            NM_OWB_RA_WA =   0b01,
            /// normal memory, outer write-through, read-allocate, no write-allocate, cacheable
            NM_OWT_RA_NWA =  0b10,
            /// normal memory, outer write-back, read-allocate, no write-allocate, caheable
            NM_OWB_RA_NWA =  0b11
        ],
        /// Shareability attribute for memory associated with tlb walks using ttbr0_el2
        SH0     OFFSET(12) BITS(2) [
            /// non shareable
            NS =     0b00,
            /// outer shareable
            OS =     0b10,
            /// inner shareable
            IS =     0b11
        ],
        /// Granule size for the ttbr0_el2
        TG0     OFFSET(14) BITS(2) [
            _4KB =   0b00,
            _64KB =  0b10
        ],
        /// Physical address size
        PS      OFFSET(16) BITS(3) [
            /// 4GB address size
            _32BITS =    0b000,
            /// 64GB address size
            _36BITS =    0b001,
            /// 1TB address size
            _40BITS =    0b010
        ],
        /// Top Byte Ignored
        TBI     OFFSET(20) [
            /// Typ byte is used for address calculation
            USE =        0b0,
            /// Top byte is ignored for address calculation
            IGNORE =     0b1
        ]
    }
}
