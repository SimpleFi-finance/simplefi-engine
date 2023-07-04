use super::big_number::{add_big_from_strs, sub_big_from_strs};

pub fn merge_bal_string_vecs(
    vec_a: &Vec<String>,
    vec_b: &Vec<String>,
) -> Vec<String> {
    let mut new_vec: Vec<String> = vec![];

    vec_a.iter().for_each(|val| {
        let matching_token = new_vec
            .iter()
            .position(|x| unformat_balance_string(x).0 == unformat_balance_string(val).0);

        match matching_token {
            Some(x) => new_vec[x] = add_bal_string_to_bal_string(val, &new_vec[x]),
            _ => new_vec.push(val.to_string().clone()),
        }
    });
    vec_b.iter().for_each(|val| {
        let matching_token = new_vec
            .iter()
            .position(|x| unformat_balance_string(x).0 == unformat_balance_string(val).0);

        match matching_token {
            Some(x) => new_vec[x] = add_bal_string_to_bal_string(val, &new_vec[x]),
            _ => new_vec.push(val.to_string().clone()),
        }
    });

    new_vec
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
