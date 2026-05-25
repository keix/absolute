use async_trait::async_trait;

use crate::domain::{RegisterConvention, Syscall};
use crate::error::Result;

mod ddb;

pub use ddb::SyscallRepository;

#[async_trait]
pub trait SyscallStore: Send + Sync {
    async fn get_by_name(&self, os: &str, arch: &str, name: &str) -> Result<Option<Syscall>>;
    async fn get_by_number(&self, os: &str, arch: &str, number: u32) -> Result<Option<Syscall>>;
    async fn list(&self, os: &str, arch: &str) -> Result<Vec<Syscall>>;
    async fn get_register_convention(
        &self,
        os: &str,
        arch: &str,
        instruction: &str,
    ) -> Result<Option<RegisterConvention>>;
}
