use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Syscall {
    pub os: String,
    pub arch: String,
    pub number: u32,
    pub abi: String,
    pub name: String,
    pub entry: String,
    pub args: Vec<SyscallArg>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyscallArg {
    pub index: u8,
    pub register: String,
    pub name: String,
    #[serde(rename = "type")]
    pub ty: String,
}
