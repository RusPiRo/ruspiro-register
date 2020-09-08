/***********************************************************************************************************************
 * Copyright (c) 2019 by the authors
 *
 * Author: André Borrmann
 * License: ???
 **********************************************************************************************************************/

//! # System register definitions
//!

#[cfg(any(target_arch = "aarch64", test, doc, doctest))]
#[doc(cfg(target_arch = "aarch64"))]
pub mod aarch64;
#[cfg(any(target_arch = "aarch64", test, doc, doctest))]
pub use aarch64::*;

#[cfg(any(target_arch = "arm", test, doc, doctest))]
#[doc(cfg(target_arch = "arm"))]
pub mod aarch32;
#[cfg(any(target_arch = "arm", test, doc, doctest))]
pub use aarch32::*;

/// assembly NOP instruction
#[inline(always)]
pub fn nop() {
    unsafe { llvm_asm!("nop"::::"volatile") };
}

/// assembly WFE instruction
#[inline(always)]
pub fn wfe() {
    unsafe { llvm_asm!("wfe") };
}

/// assembly ISB instruction
#[inline(always)]
pub fn isb() {
    unsafe { llvm_asm!("isb sy") };
}

/// assembly DSB instruction
#[inline(always)]
pub fn dsb() {
    unsafe { llvm_asm!("dsb sy") };
}

/// assembly DSB instruction
#[inline(always)]
pub fn dmb() {
    unsafe { llvm_asm!("dmb sy") };
}
