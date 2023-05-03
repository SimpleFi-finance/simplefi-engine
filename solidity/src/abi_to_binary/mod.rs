use serde_json::Value;
use shared_types::abi::Abi;
use std::io::Cursor;

pub fn abi_to_binary(abi: &Value) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut binary_buffer = Vec::new();
    let mut cursor = Cursor::new(&mut binary_buffer);
    serde_json::to_writer(&mut cursor, abi)?;

    Ok(binary_buffer)
}

pub fn binary_to_abi(binary: &[u8]) -> Result<Value, Box<dyn std::error::Error>> {
    let mut cursor = Cursor::new(binary);
    let abi = serde_json::from_reader(&mut cursor)?;

    Ok(abi)
}

pub fn abi_to_bytecode(abi_string: &String) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let abi: Vec<Abi> = serde_json::from_str(&abi_string).unwrap();

    let abi_bytecode = bincode::serialize(&abi).unwrap();

    Ok(abi_bytecode)
}

pub fn bytecode_to_abi(abi_bytecode: &Vec<u8>) -> Result<String, Box<dyn std::error::Error>> {
    let abi: Vec<Abi> = bincode::deserialize(&abi_bytecode).unwrap();
    let abi_string = serde_json::to_string(&abi).unwrap();

    Ok(abi_string)
}

/*
#[cfg(test)]
mod tests {
    #[allow(unused)]
    use bson::Binary;
    use serde_json::json;

    use super::*;
    use serde_json::json;

    #[test]
    fn test_abi_to_binary() {
        let abi = json!({
            "name": "test",
            "type": "function",
            "inputs": [
                {
                    "name": "a",
                    "type": "uint256"
                },
                {
                    "name": "b",
                    "type": "uint256"
                }
            ],
            "outputs": [
                {
                    "name": "c",
                    "type": "uint256"
                }
            ]
        });

        let binary = abi_to_binary(&abi).unwrap();

        assert_eq!(
            binary,
            vec![
                123, 34, 110, 97, 109, 101, 34, 58, 34, 116, 101, 115, 116, 34, 44, 34, 116, 121,
                112, 101, 34, 58, 34, 102, 117, 110, 99, 116, 105, 111, 110, 34, 44, 34, 105, 110,
                112, 117, 116, 115, 34, 58, 91, 123, 34, 110, 97, 109, 101, 34, 58, 34, 97, 34, 44,
                34, 116, 121, 112, 101, 34, 58, 34, 117, 105, 110, 116, 50, 53, 54, 34, 125, 44,
                123, 34, 110, 97, 109, 101, 34, 58, 34, 98, 34, 44, 34, 116, 121, 112, 101, 34, 58,
                34, 117, 105, 110, 116, 50, 53, 54, 34, 125, 93, 44, 34, 111, 117, 116, 112, 117,
                116, 115, 34, 58, 91, 123, 34, 110, 97, 109, 101, 34, 58, 34, 99, 34, 44, 34, 116,
                121, 112, 101, 34, 58, 34, 117, 105, 110, 116, 50, 53, 54, 34, 125, 93, 125
            ]
        );

        let abi2 = binary_to_abi(&binary).unwrap();

        assert!(abi == abi2);
    }

    #[test]
    fn test_abi_to_bytecode() {
        let abi_string = String::from(
            r#"[
            {
                "name": "test",
                "type": "function",
                "inputs": [
                    {
                        "name": "a",
                        "type": "uint256"
                    },
                    {
                        "name": "b",
                        "type": "uint256"
                    }
                ],
                "outputs": [
                    {
                        "name": "c",
                        "type": "uint256"
                    }
                ]
            }]"#,
        );

        let mut abi: Vec<Abi> = serde_json::from_str(&abi_string).unwrap();

        Abi::sort_abi_elements(&mut abi);

        // sort parameters
        for abi in &mut abi {
            abi.sort_parameters();
        }

        let abi_ordered_string = serde_json::to_string(&abi).unwrap();

        let abi_bytecode = abi_to_bytecode(&abi_ordered_string).unwrap();

        print!("{:?}", abi_bytecode);

        assert_eq!(
            abi_bytecode,
            vec![
                1, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 102, 117, 110, 99, 116, 105, 111,
                110, 1, 4, 0, 0, 0, 0, 0, 0, 0, 116, 101, 115, 116, 1, 2, 0, 0, 0, 0, 0, 0, 0, 7,
                0, 0, 0, 0, 0, 0, 0, 117, 105, 110, 116, 50, 53, 54, 1, 1, 0, 0, 0, 0, 0, 0, 0, 97,
                0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 117, 105, 110, 116, 50, 53, 54, 1, 1, 0, 0, 0, 0,
                0, 0, 0, 98, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 117, 105,
                110, 116, 50, 53, 54, 1, 1, 0, 0, 0, 0, 0, 0, 0, 99, 0, 0, 0, 0, 0, 0, 0, 0
            ]
        );

        let abi_string2 = bytecode_to_abi(&abi_bytecode).unwrap();

        assert!(abi_ordered_string == abi_string2);
    }
    #[allow(unused)]
    fn test_decode() {
         let bin = Binary {

        }

         let bin = doc! {
            "timestamp": 1680278397706,
            "index": 11574,
            "abi": {
                "$binary": {
                    "base64": "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",
                    "subType": "00"
                }
            }}

        }
    }

}
 */
