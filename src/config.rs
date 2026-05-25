#[derive(Debug, Clone)]
pub struct Config {
    pub table_name: String,
    pub ddb_endpoint: Option<String>,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            table_name: std::env::var("DDB_TABLE").unwrap_or_else(|_| "system-calls".into()),
            ddb_endpoint: std::env::var("DDB_ENDPOINT").ok(),
        }
    }
}
