/// Export metadata containing the timestamp of when the export was created
///
/// # Attributes
/// - `value`: ISO 8601 formatted datetime with timezone (e.g., "2025-06-24 21:50:56 +0900")
#[derive(Debug)]
pub struct ExportDate {
    pub value: String,
}
