pub mod export_date;
pub mod me;
pub mod metadata_entry;
pub mod record;
pub mod record_type;

pub use crate::export_date::ExportDate;
pub use crate::me::Me;
pub use crate::metadata_entry::MetadataEntry;
pub use crate::record::Record;

// Apple Health Export XML Structure
//
// The Apple Health export file is a large XML document containing all health data
// recorded by the Health app and connected devices/apps.
//
// ## Document Structure
//
// ```xml
// <?xml version="1.0" encoding="UTF-8"?>
// <!DOCTYPE HealthData [...]>
// <HealthData locale="ja_JP">
//   <ExportDate value="2025-06-24 21:50:56 +0900"/>
//   <Me HKCharacteristicTypeIdentifierDateOfBirth="1996-01-27" .../>
//   <Record type="..." .../>
//   <Record type="..." .../>
//   <!-- ... millions of records ... -->
// </HealthData>
// ```

/// Root element of the Apple Health export
///
/// # Attributes
/// - `locale`: The locale of the export (e.g., "ja_JP", "en_US")
#[derive(Debug)]
pub struct HealthData {
    pub locale: String,
    pub export_date: ExportDate,
    pub me: Me,
    pub records: Vec<Record>,
}
