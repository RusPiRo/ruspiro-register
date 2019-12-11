/***********************************************************************************************************************
 * Copyright (c) 2019 by the authors
 *
 * Author: André Borrmann
 * License: Apache License 2.0
 **********************************************************************************************************************/

//! # Unit Tests

// this list of unit tests need some re-work as the actuall implementations are only
// roughly tested as some of the HW accessing methods could not be mocked properly
use super::*;

#[test]
fn register_field_mask() {
    let field = RegisterField::<u32>::new(0x3, 6);
    assert_eq!(field.mask(), 0x3 << 6);
}

#[test]
fn register_field_shift() {
    let field = RegisterField::<u16>::new(0x3, 3);
    assert_eq!(field.shift(), 3);
}

#[test]
fn register_field_value() {
    let value = RegisterFieldValue::<u32>::new(RegisterField::<u32>::new(0x3, 6), 2);
    assert_eq!(value.value(), 2);
    assert_eq!(value.raw_value(), 2 << 6);
}

#[test]
fn register_field_value_or() {
    let value1 = RegisterFieldValue::<u16>::new(RegisterField::<u16>::new(0xF, 0), 0xA);
    let value2 = RegisterFieldValue::<u16>::new(RegisterField::<u16>::new(0x3, 4), 0x2);
    let value_or = value1 | value2;

    assert_eq!(value_or.value(), 0xA | (0x2 << 4));
    assert_eq!(value_or.raw_value(), 0xA | (0x2 << 4));
}

#[test]
fn register_field_value_and() {
    let value1 = RegisterFieldValue::<u8>::new(RegisterField::<u8>::new(0xF, 0), 0xA);
    let value2 = RegisterFieldValue::<u8>::new(RegisterField::<u8>::new(0x3, 2), 0x2);
    let value_and = value1 & value2;

    assert_eq!(value_and.value(), 0xA & (0x2 << 2));
    assert_eq!(value_and.raw_value(), 0xA & (0x2 << 2));
}

#[test]
fn register_value_update() {
    let field_value = RegisterFieldValue::<u32>::new(RegisterField::<u32>::new(0x3, 8), 0b01);
    let register_value: u32 = 0b1_1010_0011_0010;
    let new_value: u32 = (register_value & !field_value.mask()) | field_value.raw_value();

    assert_eq!(new_value, 0b1_1001_0011_0010);
}

#[test]
fn mmio_register_read() {
    const MMIO: [u32; 1] = [0xff00_ab00];

    let field = RegisterField::<u32>::new(0xff, 8);
    let value = (MMIO[0] & field.mask()) >> field.shift();
    assert_eq!(value, 0xab);
}

#[test]
fn mmio_register_read_value() {
    const MMIO: [u32; 1] = [0xff00_ab00];

    let field = RegisterField::<u32>::new(0xff, 8);
    let value = (MMIO[0] & field.mask()) >> field.shift();
    let field_value = RegisterFieldValue::<u32>::new(field, value);
    assert_eq!(field_value.value(), 0xab);
    assert_eq!(field_value.raw_value(), 0xab00);
}

#[test]
fn mmio_register_modify_value() {
    const MMIO: [u32; 1] = [0xff00_ab00];

    let field = RegisterField::<u32>::new(0xff, 8);
    let old_value = MMIO[0];
    let new_value = (old_value & !field.mask()) | ((0x0f << field.shift()) & field.mask());
    assert_eq!(new_value, 0xff00_0f00);
}
