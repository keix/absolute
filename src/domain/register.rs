use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterConvention {
    pub os: String,
    pub arch: String,
    pub instruction: String,
    pub number_register: String,
    pub argument_registers: Vec<String>,
    pub return_register: String,
}
