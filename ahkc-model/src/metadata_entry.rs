/// Metadata associated with a health record
///
/// # Common Keys
/// - `HKWasUserEntered`: "1" indicates manually entered data
/// - `HKMetadataKeySyncIdentifier`: UUID for syncing
/// - `HKMetadataKeySyncVersion`: Version for syncing
#[derive(Debug)]
pub struct MetadataEntry {
    pub key: String,
    pub value: String,
}
