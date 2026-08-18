use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StorageAzureSettings {
    pub endpoint: String,
    pub container: String,
    pub account_name: String,
    pub account_key: String,
}

impl Default for StorageAzureSettings {
    fn default() -> Self {
        Self {
            endpoint: "http://localhost:10000/devstoreaccount1".to_string(),
            container: String::new(),
            account_name: "devstoreaccount1".to_string(),
            account_key: String::new(),
        }
    }
}
