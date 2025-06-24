use ahkc_model::record_type::RecordType;
use ahkc_model::{HealthData, Record};
use anyhow::Result;
use chrono::NaiveDateTime;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn export_to_csv(
    health_data: &HealthData,
    output_path: &Path,
    record_types: Vec<RecordType>,
) -> Result<()> {
    let filtered_records: Vec<&Record> = health_data
        .records
        .iter()
        .filter(|r| {
            if record_types.is_empty() {
                true
            } else {
                record_types.contains(&r.r#type)
            }
        })
        .collect();

    if filtered_records.is_empty() {
        anyhow::bail!("No records found for the specified record types");
    }

    // Group records by date
    let mut grouped_records: BTreeMap<String, Vec<&Record>> = BTreeMap::new();

    for record in &filtered_records {
        // Extract date from startDate
        let date_str = extract_date(&record.start_date)?;
        grouped_records.entry(date_str).or_default().push(record);
    }

    // Write to CSV
    let mut file = File::create(output_path)?;

    // Write CSV header
    writeln!(file, "Date,Time,Type,Value,Unit,Source")?;

    // Write records grouped by date
    for (date, records) in grouped_records {
        for record in records {
            let time = extract_time(&record.start_date)?;
            let record_type = format!("{:?}", record.r#type);
            let value = record.value.as_deref().unwrap_or("");
            let unit = record.unit.as_deref().unwrap_or("");
            let source = &record.source_name;

            writeln!(
                file,
                "{},{},{},{},{},{}",
                date, time, record_type, value, unit, source
            )?;
        }
    }

    Ok(())
}

fn extract_date(datetime_str: &str) -> Result<String> {
    // Parse datetime string like "2024-06-26 00:03:06 +0900"
    let naive_dt = NaiveDateTime::parse_from_str(
        &datetime_str[..19], // Take only the datetime part
        "%Y-%m-%d %H:%M:%S",
    )?;

    Ok(naive_dt.format("%Y-%m-%d").to_string())
}

fn extract_time(datetime_str: &str) -> Result<String> {
    // Parse datetime string like "2024-06-26 00:03:06 +0900"
    let naive_dt = NaiveDateTime::parse_from_str(
        &datetime_str[..19], // Take only the datetime part
        "%Y-%m-%d %H:%M:%S",
    )?;

    Ok(naive_dt.format("%H:%M:%S").to_string())
}

pub fn export_summary(
    health_data: &HealthData,
    output_path: &Path,
    record_types: Vec<RecordType>,
) -> Result<()> {
    let filtered_records: Vec<&Record> = health_data
        .records
        .iter()
        .filter(|r| {
            if record_types.is_empty() {
                true
            } else {
                record_types.contains(&r.r#type)
            }
        })
        .collect();

    if filtered_records.is_empty() {
        anyhow::bail!("No records found for the specified record types");
    }

    // Group records by date and calculate daily statistics
    let mut daily_stats: BTreeMap<String, DailyStats> = BTreeMap::new();

    for record in &filtered_records {
        let date_str = extract_date(&record.start_date)?;
        let stats = daily_stats.entry(date_str).or_insert_with(DailyStats::new);

        if let Some(value_str) = &record.value {
            if let Ok(value) = value_str.parse::<f64>() {
                stats.add_value(value);
            }
        }
    }

    // Write summary to CSV
    let mut file = File::create(output_path)?;

    // Write CSV header
    writeln!(file, "Date,Count,Average,Min,Max")?;

    // Write daily statistics
    for (date, stats) in daily_stats {
        if stats.count > 0 {
            writeln!(
                file,
                "{},{},{:.2},{:.2},{:.2}",
                date,
                stats.count,
                stats.sum / stats.count as f64,
                stats.min,
                stats.max
            )?;
        }
    }

    Ok(())
}

struct DailyStats {
    count: usize,
    sum: f64,
    min: f64,
    max: f64,
}

impl DailyStats {
    fn new() -> Self {
        Self {
            count: 0,
            sum: 0.0,
            min: f64::MAX,
            max: f64::MIN,
        }
    }

    fn add_value(
        &mut self,
        value: f64,
    ) {
        self.count += 1;
        self.sum += value;
        self.min = self.min.min(value);
        self.max = self.max.max(value);
    }
}
