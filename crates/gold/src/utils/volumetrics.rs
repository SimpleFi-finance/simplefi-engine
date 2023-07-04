use super::balance_strings::merge_bal_string_vecs;
use super::big_number::{add_big_from_strs, sub_big_from_strs};
use crate::types::volumetrics::Volumetric;

pub fn amalgamate_volumetrics(
    volume_a: &Volumetric,
    volume_b: &Volumetric,
) -> Volumetric {
    Volumetric {
        block_number: volume_a.block_number,
        timestamp: volume_a.timestamp,
        swaps_out: merge_bal_string_vecs(&volume_a.swaps_out, &volume_b.swaps_out),
        swaps_in: merge_bal_string_vecs(&volume_a.swaps_in, &volume_b.swaps_in),
        withdrawal: merge_bal_string_vecs(&volume_a.withdrawal, &volume_b.withdrawal),
        mint: merge_bal_string_vecs(&volume_a.mint, &volume_b.mint),
        transfer: add_big_from_strs(&volume_a.transfer, &volume_b.transfer),
    }
}

pub fn unformat_balance_string(balance_string: &str) -> (std::string::String, std::string::String) {
    let s = balance_string.split('|').collect::<Vec<&str>>();
    (String::from(s[0]), String::from(s[1]))
}

pub fn format_balance_string(
    address: &str,
    balance: &str,
) -> std::string::String {
    format!("{}|{}", address, balance)
}

pub fn add_to_balance_string(
    bal_string: &str,
    num_string: &str,
) -> String {
    let (address, balance) = unformat_balance_string(bal_string);
    let new_bal = add_big_from_strs(&balance, num_string);
    format_balance_string(&address, &new_bal)
}
pub fn sub_from_balance_string(
    bal_string: &str,
    num_string: &str,
) -> String {
    let (address, balance) = unformat_balance_string(&bal_string);
    let new_bal = sub_big_from_strs(&balance, num_string);

    format_balance_string(&address, &new_bal)
}

pub fn add_bal_string_to_bal_string(
    bal_1: &str,
    bal_2: &str,
) -> String {
    add_to_balance_string(bal_1, &unformat_balance_string(bal_2).1)
}
