/*********************************************************************************************************************** 
 * Copyright (c) 2019 by the authors
 * 
 * Author: André Borrmann 
 * License: ???
 **********************************************************************************************************************/

//! # System register definitions
//! 

#[cfg(target_arch="aarch64")]
pub mod aarch64;
#[cfg(target_arch="aarch64")]
pub use aarch64::*;

#[cfg(target_arch="arm")]
pub mod aarch32;
#[cfg(target_arch="arm")]
pub use aarch32::*;

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

/// assembly ISB instruction
#[inline(always)]
pub fn isb() {
    unsafe { asm!("isb sy") };
}

/// assembly DSB instruction
#[inline(always)]
pub fn dsb() {
    unsafe { asm!("dsb") };
}