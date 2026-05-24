use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Syscall {
    pub os: String,
    pub arch: String,
    pub number: u32,
    pub abi: String,
    pub name: String,
    pub entry: String,
    #[serde(default, skip_deserializing)]
    pub man_url: Option<String>,
}

pub fn man_url(os: &str, name: &str) -> Option<String> {
    match os.to_lowercase().as_str() {
        "linux" => Some(format!(
            "https://man7.org/linux/man-pages/man2/{}.2.html",
            name
        )),
        _ => None,
    }
}
