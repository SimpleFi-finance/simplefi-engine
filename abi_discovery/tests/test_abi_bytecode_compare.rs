// Create a test where two abis gets created with some parameters and if they are not in a set, they get added
#[cfg(test)]
mod tests {
    use shared_types::abi::{ Abi };
    use std::collections::HashSet;

    #[test]
    fn test_abi_bytecode_compare() {
        let json1 = r#"
        [
            {
                "inputs": [
                    {
                        "internalType": "contract IERC20",
                        "name": "token",
                        "type": "address"
                    },
                    {
                        "internalType": "address[3]",
                        "name": "recipients",
                        "type": "address[3]"
                    },
                    {
                        "internalType": "uint256[3]",
                        "name": "values",
                        "type": "uint256[3]"
                    }
                ],
                "name": "disperseTokenSimple",
                "outputs": [],
                "stateMutability": "nonpayable",
                "type": "function"
            }
        ]"#;

        let json2 = r#"
        [
            {
                "inputs": [
                    {
                        "internalType": "contract IERC20",
                        "name": "token",
                        "type": "address"
                    },
                    {
                        "internalType": "address[3]",
                        "name": "recipients",
                        "type": "address[3]"
                    },
                    {
                        "internalType": "uint256[3]",
                        "name": "values",
                        "type": "uint256[3]"
                    }
                ],
                "name": "disperseTokenSimple",
                "outputs": [],
                "stateMutability": "nonpayable",
                "type": "function"
            }
        ]"#;

        let abi1 = serde_json::from_str::<Vec<Abi>>(json1).unwrap();
        let abi2 = serde_json::from_str::<Vec<Abi>>(json2).unwrap();

        let mut bytecode_set: HashSet<Vec<u8>> = HashSet::new();


        let bytecode1 = bincode::serialize(&abi1).unwrap();
        let bytecode2 = bincode::serialize(&abi2).unwrap();

        if !bytecode_set.contains(&bytecode1) {
            bytecode_set.insert(bytecode1);
        }

        if !bytecode_set.contains(&bytecode2) {
            bytecode_set.insert(bytecode2);
        }


        assert_eq!(bytecode_set.len(), 1);
    }

    #[test]
    fn test_abi_bytecode_order_matters() {
        let json1 = r#"
        [
            {
                "inputs": [
                    {
                        "internalType": "contract IERC20",
                        "name": "token",
                        "type": "address"
                    },
                    {
                        "internalType": "address[3]",
                        "name": "recipients",
                        "type": "address[3]"
                    },
                    {
                        "internalType": "uint256[3]",
                        "name": "values",
                        "type": "uint256[3]"
                    }
                ],
                "name": "disperseTokenSimple",
                "outputs": [],
                "stateMutability": "nonpayable",
                "type": "function"
            }
        ]"#;

        let json2 = r#"
        [
            {
                "inputs": [
                    {
                        "internalType": "contract IERC20",
                        "name": "token",
                        "type": "address"
                    },
                    {
                        "internalType": "uint256[3]",
                        "name": "values",
                        "type": "uint256[3]"
                    },
                    {
                        "internalType": "address[3]",
                        "name": "recipients",
                        "type": "address[3]"
                    }
                ],
                "name": "disperseTokenSimple",
                "outputs": [],
                "stateMutability": "nonpayable",
                "type": "function"
            }
        ]"#;

        let abi1 = serde_json::from_str::<Vec<Abi>>(json1).unwrap();
        let abi2 = serde_json::from_str::<Vec<Abi>>(json2).unwrap();

        let mut bytecode_set: HashSet<Vec<u8>> = HashSet::new();


        let bytecode1 = bincode::serialize(&abi1).unwrap();
        let bytecode2 = bincode::serialize(&abi2).unwrap();

        if !bytecode_set.contains(&bytecode1) {
            bytecode_set.insert(bytecode1);
        }

        if !bytecode_set.contains(&bytecode2) {
            bytecode_set.insert(bytecode2);
        }

        assert_eq!(bytecode_set.len(), 2);

    }

    #[test]
    fn test_abi_bytecode_order_not_matters_on_fields() {
        let json1 = r#"
        [
            {
                "inputs": [
                    {
                        "internalType": "contract IERC20",
                        "name": "token",
                        "type": "address"
                    },
                    {
                        "internalType": "address[3]",
                        "name": "recipients",
                        "type": "address[3]"
                    },
                    {
                        "internalType": "uint256[3]",
                        "name": "values",
                        "type": "uint256[3]"
                    }
                ],
                "name": "disperseTokenSimple",
                "outputs": [],
                "stateMutability": "nonpayable",
                "type": "function"
            }
        ]"#;

        let json2 = r#"
        [
            {
                "inputs": [
                    {
                        "internalType": "contract IERC20",
                        "name": "token",
                        "type": "address"
                    },
                    {
                        "internalType": "address[3]",
                        "name": "recipients",
                        "type": "address[3]"
                    },
                    {
                        "type": "uint256[3]",
                        "name": "values",
                        "internalType": "uint256[3]"
                    }
                ],
                "outputs": [],
                "name": "disperseTokenSimple",
                "stateMutability": "nonpayable",
                "type": "function"
            }
        ]"#;

        let abi1 = serde_json::from_str::<Vec<Abi>>(json1).unwrap();
        let abi2 = serde_json::from_str::<Vec<Abi>>(json2).unwrap();

        let mut bytecode_set: HashSet<Vec<u8>> = HashSet::new();


        let bytecode1 = bincode::serialize(&abi1).unwrap();
        let bytecode2 = bincode::serialize(&abi2).unwrap();

        if !bytecode_set.contains(&bytecode1) {
            bytecode_set.insert(bytecode1);
        }

        if !bytecode_set.contains(&bytecode2) {
            bytecode_set.insert(bytecode2);
        }


        assert_eq!(bytecode_set.len(), 1);
    }

    #[test]
    fn test_sprting_makes_them_equal() {
        let json1 = r#"
        [
            {
                "inputs": [
                    {
                        "internalType": "contract IERC20",
                        "name": "token",
                        "type": "address"
                    },
                    {
                        "internalType": "address[3]",
                        "name": "recipients",
                        "type": "address[3]"
                    },
                    {
                        "internalType": "uint256[3]",
                        "name": "values",
                        "type": "uint256[3]"
                    }
                ],
                "name": "disperseTokenSimple",
                "stateMutability": "nonpayable",
                "type": "function",
                "outputs": []
            }
        ]"#;

        let json2 = r#"
        [
            {
                "inputs": [
                    {
                        "internalType": "contract IERC20",
                        "name": "token",
                        "type": "address"
                    },
                    {
                        "internalType": "uint256[3]",
                        "name": "values",
                        "type": "uint256[3]"
                    },
                    {
                        "internalType": "address[3]",
                        "name": "recipients",
                        "type": "address[3]"
                    }
                ],
                "name": "disperseTokenSimple",
                "outputs": [],
                "stateMutability": "nonpayable",
                "type": "function"
            }
        ]"#;

        let mut abi1 = serde_json::from_str::<Vec<Abi>>(json1).unwrap();
        let mut abi2 = serde_json::from_str::<Vec<Abi>>(json2).unwrap();

        Abi::sort_abi_elements(&mut abi1);
        Abi::sort_abi_elements(&mut abi2);

        for abi in &mut abi1 {
            abi.sort_parameters();
        }

        for abi in &mut abi2 {
            abi.sort_parameters();
        }

        let mut bytecode_set: HashSet<Vec<u8>> = HashSet::new();

        let bytecode1 = bincode::serialize(&abi1).unwrap();
        let bytecode2 = bincode::serialize(&abi2).unwrap();

        if !bytecode_set.contains(&bytecode1) {
            bytecode_set.insert(bytecode1);
        }

        if !bytecode_set.contains(&bytecode2) {
            bytecode_set.insert(bytecode2);
        }

        assert_eq!(bytecode_set.len(), 1)
    }
}
