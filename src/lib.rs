//========================================================================
// conio_rs | lib.rs - https://overtimecoder.github.io
//------------------------------------------------------------------------
// Lib
//------------------------------------------------------------------------
//
// Author LatteS
//
// File was created in 2022/12/29
//
//========================================================================

//! conio_rs
//! Windows conio API wrapper for rust

#![allow(dead_code)]

// Link api-ms-win-crt-conio-l1-1-0.dll
#[link(name="api-ms-win-crt-conio-l1-1-0", kind="raw-dylib")]
extern{
    fn _getch() -> i32;
    fn _getche() -> i32;

    fn _putch(ch:char) -> i32;
}

// Constants
// Key codes
const NUM_0: i32 = 48;
const NUM_1: i32 = 49;
const NUM_2: i32 = 50;
const NUM_3: i32 = 51;
const NUM_4: i32 = 52;
const NUM_5: i32 = 53;
const NUM_6: i32 = 54;
const NUM_7: i32 = 55;
const NUM_8: i32 = 56;
const NUM_9: i32 = 57;

// Alphabets
const LOWER_A: i32 = 97;
const UPPER_A: i32 = 65;
const LOWER_B: i32 = 98;
const UPPER_B: i32 = 66;
const LOWER_C: i32 = 99;
const UPPER_C: i32 = 67;
const LOWER_D: i32 = 100;
const UPPER_D: i32 = 68;
const LOWER_E: i32 = 101;
const UPPER_E: i32 = 69;
const LOWER_F: i32 = 102;
const UPPER_F: i32 = 70;
const LOWER_G: i32 = 103;
const UPPER_G: i32 = 71;
const LOWER_H: i32 = 104;
const UPPER_H: i32 = 72;
const LOWER_I: i32 = 105;
const UPPER_I: i32 = 73;
const LOWER_J: i32 = 106;
const UPPER_J: i32 = 74;
const LOWER_K: i32 = 107;
const UPPER_K: i32 = 75;
const LOWER_L: i32 = 108;
const UPPER_L: i32 = 76;
const LOWER_M: i32 = 109;
const UPPER_M: i32 = 77;
const LOWER_N: i32 = 110;
const UPPER_N: i32 = 78;
const LOWER_O: i32 = 111;
const UPPER_O: i32 = 79;
const LOWER_P: i32 = 112;
const UPPER_P: i32 = 80;
const LOWER_Q: i32 = 113;
const UPPER_Q: i32 = 81;
const LOWER_R: i32 = 114;
const UPPER_R: i32 = 82;
const LOWER_S: i32 = 115;
const UPPER_S: i32 = 83;
const LOWER_T: i32 = 116;
const UPPER_T: i32 = 84;
const LOWER_U: i32 = 117;
const UPPER_U: i32 = 85;
const LOWER_V: i32 = 118;
const UPPER_V: i32 = 86;
const LOWER_W: i32 = 119;
const UPPER_W: i32 = 87;
const LOWER_X: i32 = 120;
const UPPER_X: i32 = 88;
const LOWER_Y: i32 = 121;
const UPPER_Y: i32 = 89;
const LOWER_Z: i32 = 122;
const UPPER_Z: i32 = 90;

// Implementation
/// Enter 1 byte from the console.
pub fn getch() -> i32 {
    unsafe {
        _getch()
    }
}

/// Enter 1 byte from console with Echo.
pub fn getche() -> i32 {
    unsafe {
        _getche()
    }
}

/// Output 1 byte to the console
pub fn putch(ch:char) -> i32 {
    unsafe {
        _putch(ch)
    }
}

/// Converts i32 to Ascii characters.
pub fn to_ascii(code:i32) -> char {
    code as u8 as char
}