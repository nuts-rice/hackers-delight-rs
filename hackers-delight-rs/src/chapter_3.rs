//Rounding up or down
// clp2 defined as similar to 'ceiling' but directed roundings to closest intergal power of 2
//right-propogation rounding up to next ^2
pub fn clp2_0(mut x: i32) -> i32 {
    x = x - 1;
    x = x | (x >> 1);
    x = x | (x >> 2);
    x = x | (x >> 4);
    x = x | (x >> 8);
    x = x | (x >> 16);
    return x + 1;
}

#[cfg_attr(not(target_arch = "x86_64"), test_case)]
#[cfg_attr(not(target_arch = "riscv64"), test)]
fn test_clp2() {
    assert_eq!(clp2_0(0), 0);
}
