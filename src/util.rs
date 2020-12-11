//! Some IO utility methods.
//!
//! **TODO:
//! - Error checking.**

use std::fs::File;
use std::io::Read;

/// Read one byte.
pub fn read_u8(file: &mut File) -> u8 {
    let mut buf = [0u8; 1];
    file.read(&mut buf).unwrap();
    u8::from_ne_bytes(buf)
}
/// Read two bytes.
pub fn read_u16(file: &mut File) -> u16 {
    let mut buf = [0u8; 2];
    file.read(&mut buf).unwrap();
    u16::from_ne_bytes(buf)
}
/// Read four bytes.
pub fn read_u32(file: &mut File) -> u32 {
    let mut buf = [0u8; 4];
    file.read(&mut buf).unwrap();
    u32::from_ne_bytes(buf)
}
/// Read eight bytes.
pub fn read_u64(file: &mut File) -> u64 {
    let mut buf = [0u8; 8];
    file.read(&mut buf).unwrap();
    u64::from_ne_bytes(buf)
}

