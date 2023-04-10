use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct AbiParam {
    #[serde(rename = "type")]
    pub param_type: String,
    pub name: Option<String>,
    pub internal_type: Option<String>,
    pub indexed: Option<bool>,
    pub components: Option<Vec<AbiParam>>,
}

impl Ord for AbiParam {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (
            self.param_type.cmp(&other.param_type),
            self.name.cmp(&other.name),
            self.internal_type.cmp(&other.internal_type),
        ) {
            (std::cmp::Ordering::Equal, std::cmp::Ordering::Equal, internal_type_ordering) => {
                internal_type_ordering
            }
            (std::cmp::Ordering::Equal, name_ordering, _) => name_ordering,
            (param_type_ordering, _, _) => param_type_ordering,
        }
    }
}

impl PartialOrd for AbiParam {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct Abi {
    #[serde(rename = "type")]
    pub abi_type: String,
    pub name: Option<String>,
    pub inputs: Option<Vec<AbiParam>>,
    pub outputs: Option<Vec<AbiParam>>,
    pub state_mutability: Option<String>,
    pub payable: Option<bool>,
    pub constant: Option<bool>,
    pub anonymous: Option<bool>,
    pub signature: Option<String>,
}

impl Abi {
    pub fn sort_parameters(&mut self) {
        if let Some(inputs) = &mut self.inputs {
            inputs.sort();
        }

        if let Some(outputs) = &mut self.outputs {
            outputs.sort();
        }
    }

    pub fn sort_abi_elements(abis: &mut Vec<Abi>) {
        abis.sort_by(|a, b| {
            match (a.abi_type.cmp(&b.abi_type), a.name.cmp(&b.name)) {
                (std::cmp::Ordering::Equal, name_ordering) => name_ordering,
                (abi_type_ordering, _) => abi_type_ordering,
            }
        });
    }

    pub fn get_signature(&self) -> String {
        let mut signature = String::new();
        signature.push_str(&self.name.clone().unwrap());
        signature.push('(');
        if let Some(inputs) = &self.inputs {
            for (i, input) in inputs.iter().enumerate() {
                if i > 0 {
                    signature.push(',');
                }
                signature.push_str(&input.param_type);
            }
        }
        signature.push(')');
        signature
    }
}
