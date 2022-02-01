use crate::*;

//2-1 manipulating rightmost bits

//use formula: 'x&(x -1)' to turn off the rightmost 1 bit in a word, producing 0 if none
// this can be used to determine if an unsigned integer is power of 2 or is 0
pub fn formula_get_and(x: c8) -> c8 {
    x & (x - c8(1))
}

//use formula 'x | (x+1)' to turn on the rightmost 0 bit in word, producing 1s if none
pub fn formula_get_or(x: c8) -> c8 {
    x | (x + c8(1))
}

//formula: 'x&(x+1)' to turn off trailing 1s in word, producing x if none
// can be used to determin if unsigned integer is the form 2^n -1, 0, or all 1s
pub fn formula_trailing_1_to_0(x: c8) -> c8 {
    x & (x + c8(1))
}

//formula: 'x | (x -1)' to turn on all the trailing 0s in word, producing x if none
pub fn formula_trailing_0_to_1(x: c8) -> c8 {
    x | (x - c8(1))
}

//formula '!x & (x + 1)' to create a word with single 1 bit at position of the rightmost 0-bit
// producing 0 if none
pub fn formula_rightmost_0_to_1(x: c8) -> c8 {
    !x (x + c8(1))
}

#[cfg_attr(not(target_arch = "x86_64"), test_case)]
#[cfg_attr(not(target_arch = "riscv64"),test)]
fn test_2_1(){
    assert_eq!(formula_get_and(c8(0b01011000)), c8(0b001010000));
    assert_eq!(formula_get_or(c8(0b10100111)), c8(0b10101111));
    assert_eq!(formula_trailing_1_to_0(c8(0b10100111)), c8(0b10100000));
    assert_eq!(formula_trailing_0_to_1(c8(0b10101000)), c8(0b10101111));
}
