mod exporter;
mod parser;

use ahkc_model::record_type::RecordType;
use clap::Parser;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Input file path for Apple Healthcare data XML
    #[arg(short, long)]
    input: PathBuf,

    /// Output file path for the processed CSV data
    #[arg(short, long)]
    output: PathBuf,

    /// Filter by record types (can be specified multiple times)
    /// Example: -t BodyMassIndex -t HeartRate
    #[arg(short = 't', long = "type", value_name = "TYPE")]
    record_types: Vec<String>,

    /// Export summary statistics instead of raw data
    #[arg(short, long)]
    summary: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // Parse record types if provided
    let mut record_types = Vec::new();
    for type_str in &args.record_types {
        record_types.push(RecordType::from_str(type_str)?);
    }

    // Parse the XML file
    println!("Parsing health data from: {:?}", args.input);
    let health_data = parser::parse_health_data(&args.input)?;

    println!("Found {} records", health_data.records.len());

    // Export to CSV
    if args.summary {
        println!("Exporting summary to: {:?}", args.output);
        exporter::export_summary(&health_data, &args.output, record_types)?;
    } else {
        println!("Exporting data to: {:?}", args.output);
        exporter::export_to_csv(&health_data, &args.output, record_types)?;
    }

    println!("Export completed successfully!");

    Ok(())
}
