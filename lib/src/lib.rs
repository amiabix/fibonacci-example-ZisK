use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct FibResult {
    pub n: u32,
    pub value: u64,
}
