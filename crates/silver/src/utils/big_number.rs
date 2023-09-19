use ethers::types::U256;
use std::ops::{Add, Sub};

pub fn bn_gt(
    bn: &str,
    num: u64,
) -> bool {
    U256::from_dec_str(&bn).unwrap().gt(&U256::from(num))
}
pub fn bn_lt(
    bn: &str,
    num: u64,
) -> bool {
    U256::from_dec_str(&bn).unwrap().lt(&U256::from(num))
}
pub fn bn_eq(
    bn: &str,
    num: u64,
) -> bool {
    U256::from_dec_str(&bn).unwrap().eq(&U256::from(num))
}

pub fn add_big_from_strs(
    x: &str,
    y: &str,
) -> String {
    U256::from_dec_str(&x)
        .unwrap()
        .add(U256::from_dec_str(&y).unwrap())
        .to_string()
}
pub fn sub_big_from_strs(
    x: &str,
    y: &str,
) -> String {
    U256::from_dec_str(&x)
        .unwrap()
        .sub(U256::from_dec_str(&y).unwrap())
        .to_string()
}
