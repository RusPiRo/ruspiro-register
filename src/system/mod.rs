/*********************************************************************************************************************** 
 * Copyright (c) 2019 by the authors
 * 
 * Author: André Borrmann 
 * License: ???
 **********************************************************************************************************************/

//! # System register definitions
//! 

pub mod hcr_el2;
pub mod mair_el2;
pub mod sctlr_el2;
pub mod ttbr0_el2;
pub mod tcr_el2;

/// assembly NOP instruction
#[inline(always)]
pub fn nop() {
    unsafe { asm!("nop"::::"volatile") };
}

/// assembly WFE instruction
#[inline(always)]
pub fn wfe() {
    unsafe { asm!("wfe") };
}