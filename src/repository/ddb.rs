use std::collections::HashMap;

use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::Client;
use serde_dynamo::{from_item, from_items};

use crate::domain::{RegisterConvention, Syscall};
use crate::error::{AppError, Result};

#[derive(Debug)]
pub struct SyscallRepository {
    client: Client,
    table: String,
}

impl SyscallRepository {
    pub fn new(client: Client, table: String) -> Self {
        Self { client, table }
    }

    fn pk(os: &str, arch: &str) -> String {
        format!("{}#{}", os.to_uppercase(), arch)
    }

    pub async fn get_by_name(
        &self,
        os: &str,
        arch: &str,
        name: &str,
    ) -> Result<Option<Syscall>> {
        let resp = self
            .client
            .get_item()
            .table_name(&self.table)
            .key("pk", AttributeValue::S(Self::pk(os, arch)))
            .key("sk", AttributeValue::S(format!("SYSCALL#NAME#{}", name)))
            .send()
            .await
            .map_err(|e| AppError::Ddb(e.to_string()))?;

        match resp.item {
            Some(item) => Ok(Some(from_item(item)?)),
            None => Ok(None),
        }
    }

    pub async fn get_by_number(
        &self,
        os: &str,
        arch: &str,
        number: u32,
    ) -> Result<Option<Syscall>> {
        let resp = self
            .client
            .get_item()
            .table_name(&self.table)
            .key("pk", AttributeValue::S(Self::pk(os, arch)))
            .key("sk", AttributeValue::S(format!("SYSCALL#NR#{}", number)))
            .send()
            .await
            .map_err(|e| AppError::Ddb(e.to_string()))?;

        match resp.item {
            Some(item) => Ok(Some(from_item(item)?)),
            None => Ok(None),
        }
    }

    pub async fn list(&self, os: &str, arch: &str) -> Result<Vec<Syscall>> {
        let mut acc: Vec<Syscall> = Vec::new();
        let mut last_key: Option<HashMap<String, AttributeValue>> = None;

        loop {
            let mut req = self
                .client
                .query()
                .table_name(&self.table)
                .key_condition_expression("pk = :pk AND begins_with(sk, :sk_prefix)")
                .expression_attribute_values(":pk", AttributeValue::S(Self::pk(os, arch)))
                .expression_attribute_values(
                    ":sk_prefix",
                    AttributeValue::S("SYSCALL#NAME#".to_string()),
                );

            if let Some(esk) = last_key.take() {
                req = req.set_exclusive_start_key(Some(esk));
            }

            let resp = req
                .send()
                .await
                .map_err(|e| AppError::Ddb(e.to_string()))?;

            if let Some(items) = resp.items {
                let chunk: Vec<Syscall> = from_items(items)?;
                acc.extend(chunk);
            }

            match resp.last_evaluated_key {
                Some(k) => last_key = Some(k),
                None => break,
            }
        }

        Ok(acc)
    }

    pub async fn get_register_convention(
        &self,
        os: &str,
        arch: &str,
        instruction: &str,
    ) -> Result<Option<RegisterConvention>> {
        let resp = self
            .client
            .get_item()
            .table_name(&self.table)
            .key("pk", AttributeValue::S(Self::pk(os, arch)))
            .key(
                "sk",
                AttributeValue::S(format!("REGISTERS#{}", instruction.to_uppercase())),
            )
            .send()
            .await
            .map_err(|e| AppError::Ddb(e.to_string()))?;

        match resp.item {
            Some(item) => Ok(Some(from_item(item)?)),
            None => Ok(None),
        }
    }
}
