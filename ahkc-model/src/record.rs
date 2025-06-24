use crate::metadata_entry::MetadataEntry;
use crate::record_type::RecordType;

/// # Attributes
/// - `type`: HealthKit identifier for the data type (e.g., "HKQuantityTypeIdentifierBodyMass")
/// - `sourceName`: App or device that recorded the data (e.g., "ヘルスケア", "Apple Watch")
/// - `sourceVersion`: Version of the recording app
/// - `unit`: Measurement unit (e.g., "kg", "%", "count")
/// - `value`: The actual measurement value
/// - `creationDate`: When the record was created in the system
/// - `startDate`: Start time of the measurement
/// - `endDate`: End time of the measurement (usually same as startDate for instant measurements)
/// - `device`: Optional device information
///
/// # Child Elements
/// - `MetadataEntry`: Optional metadata key-value pairs
///
/// # Example
/// ```xml
/// <Record type="HKQuantityTypeIdentifierBodyMass"
///         sourceName="ヘルスケア"
///         sourceVersion="17.5.1"
///         unit="kg"
///         creationDate="2024-06-26 00:03:06 +0900"
///         startDate="2024-06-26 00:03:06 +0900"
///         endDate="2024-06-26 00:03:06 +0900"
///         value="86.1">
///   <MetadataEntry key="HKWasUserEntered" value="1"/>
/// </Record>
/// ```
#[derive(Debug)]
pub struct Record {
    pub r#type: RecordType,
    pub source_name: String,
    pub source_version: Option<String>,
    pub unit: Option<String>,
    pub value: Option<String>,
    pub creation_date: Option<String>,
    pub start_date: String,
    pub end_date: String,
    pub device: Option<String>,
    pub metadata_entries: Vec<MetadataEntry>,
}
