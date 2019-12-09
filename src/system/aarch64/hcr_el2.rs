/*********************************************************************************************************************** 
 * Copyright (c) 2019 by the authors
 * 
 * Author: André Borrmann 
 * License: Apache 2.0
 **********************************************************************************************************************/

//! # HCR_EL2 Register
//! 

use crate::{ define_aarch64_register, register_field };
use crate::register::*;

define_aarch64_register! {
    @hcr_el2<u64> {
        // enable second stage of translation
        VM      OFFSET(0) [
            DISABLE:    0b0,
            ENABLE:     0b1
        ],
        PTW     OFFSET(2),
        // physical FIQ routing to EL2
        FMO     OFFSET(3) [
            DISABLE:   0b0,
            ENABLE:    0b1
        ],
        // phyiscal IRQ routing to EL2
        IMO     OFFSET(4) [
            DISABLE:   0b0,
            ENABLE:    0b1
        ],
        // Async Abort and Error exception routing to EL2
        AMO     OFFSET(5) [
            DISABLE:   0b0,
            ENABLE:    0b1
        ],
        // virtual FIQ pending
        VF      OFFSET(6),
        // virtual IRQ pending
        VI      OFFSET(7),
        // virtual system error/async abort pending
        VSE     OFFSET(8),
        // Forces broadcast
        FB      OFFSET(9),
        // Barrier sharability upgrade
        BSU     OFFSET(10) BITS(2),
        // default cacheable
        DC      OFFSET(12) [
            DISABLE:  0b0,
            ENABLE:   0b1
        ],
        // trap WFI if there is no pending WFI event
        TWI     OFFSET(13) [
            NO_TRAP: 0b0,
            TRAP:    0b1
        ],
        // trap WFE if there is no pending WFE event
        TWE     OFFSET(14) [
            NO_TRAP: 0b0,
            TRAP:    0b1
        ],
        // trap ID group 0 registers
        TID0     OFFSET(15) [
            NO_TRAP: 0b0,
            TRAP:    0b1
        ],
        // trap ID group 1 registers
        TID1     OFFSET(16) [
            NO_TRAP: 0b0,
            TRAP:    0b1
        ],
        // trap ID group 2 registers
        TID2     OFFSET(17) [
            NO_TRAP: 0b0,
            TRAP:    0b1
        ],
        // trap ID group 3 registers
        TID3     OFFSET(18) [
            NO_TRAP: 0b0,
            TRAP:    0b1
        ],
        // trap SMC instruction
        TSC     OFFSET(19) [
            NO_TRAP: 0b0,
            TRAP:    0b1
        ],
        // trap implementation depended instructions
        TIDCP   OFFSET(20) [
            NO_TRAP: 0b0,
            TRAP:    0b1
        ],
        // trap auxiliry control registers
        TACR    OFFSET(21) [
            NO_TRAP: 0b0,
            TRAP:    0b1
        ],
        // trap data or unified cache maintenenace instructions by set or way
        TSW     OFFSET(22) [
            NO_TRAP: 0b0,
            TRAP:    0b1
        ],
        // trap data or unified cache maintenance instructions to Point of Coherency
        TPC     OFFSET(23) [
            NO_TRAP: 0b0,
            TRAP:    0b1
        ],
        // trap data or nuified cache maintenance instructions to Point of Unification
        TPU     OFFSET(24) [
            NO_TRAP: 0b0,
            TRAP:    0b1
        ],
        // trap ttlb maintenance instructions
        TTLB    OFFSET(25) [
            NO_TRAP: 0b0,
            TRAP:    0b1
        ],
        // trap virtual memory control
        TVM     OFFSET(26) [
            NO_TRAP: 0b0,
            TRAP:    0b1
        ],
        // trap generel exceptions
        TGE     OFFSET(27) [
            NO_TRAP: 0b0,
            TRAP:    0b1
        ],
        // trap dc zva instruction
        TDZ     OFFSET(28) [
            NO_TRAP: 0b0,
            TRAP:    0b1
        ],
        // trap reads of virtual memory controls
        TRVM    OFFSET(30) [
            NO_TRAP: 0b0,
            TRAP:    0b1
        ],
        // register width control for lower exception levels
        RW      OFFSET(31) [
            ALL_A32: 0b0,
            EL1_A64: 0b1
        ],
        // disable stage 2 data cache
        CD      OFFSET(32) [
            INACTIVE: 0b0,
            DISABLE_CACHE: 0b1
        ],
        // disable stage 2 instruction cache
        ID      OFFSET(33) [
            INACTIVE: 0b0,
            DISABLE_CACHE: 0b1
        ]
    }
}