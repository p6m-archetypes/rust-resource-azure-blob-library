pub mod settings;

use anyhow::Result;
use azure_storage::{CloudLocation, StorageCredentials};
use azure_storage_blobs::prelude::ClientBuilder;
use settings::StorageAzureSettings;

pub use azure_storage_blobs::prelude::BlobServiceClient as AzureBlobClient;

pub fn connect(settings: &StorageAzureSettings) -> Result<AzureBlobClient> {
    let credentials = StorageCredentials::access_key(
        &settings.account_name,
        &settings.account_key,
    );
    let client = ClientBuilder::with_location(
        CloudLocation::Custom {
            account: settings.account_name.clone(),
            uri: settings.endpoint.clone(),
        },
        credentials,
    )
    .blob_service_client();
    tracing::info!("Azure Blob client connected to {}", settings.endpoint);
    Ok(client)
}
