# Apple Health Kit CSV Converter

Convert Apple Health export XML files to CSV format with filtering and grouping capabilities.

## Usage

```bash
# Export all records to CSV
cargo run -- -i export.xml -o output.csv

# Export only BodyMassIndex records
cargo run -- -i export.xml -o output.csv -t BodyMassIndex

# Export multiple record types
cargo run -- -i export.xml -o output.csv -t BodyMassIndex -t HeartRate -t StepCount

# Export summary statistics grouped by date
cargo run -- -i export.xml -o output.csv -t BodyMassIndex --summary
```

## Options

- `-i, --input <PATH>`: Input XML file path (required)
- `-o, --output <PATH>`: Output CSV file path (required)
- `-t, --type <TYPE>`: Filter by record types (can be specified multiple times)
- `-s, --summary`: Export daily summary statistics instead of raw data

## Record Types

Record types can be specified using either their full HealthKit identifier or short form:
- `BodyMassIndex` or `HKQuantityTypeIdentifierBodyMassIndex`
- `BodyMass` or `HKQuantityTypeIdentifierBodyMass`
- `HeartRate` or `HKQuantityTypeIdentifierHeartRate`
- `StepCount` or `HKQuantityTypeIdentifierStepCount`
- `ActiveEnergyBurned` or `HKQuantityTypeIdentifierActiveEnergyBurned`

## Output Format

### Raw Data CSV
```
Date,Time,Type,Value,Unit,Source
2024-06-26,00:03:06,BodyMassIndex,26.1,count,ヘルスケア
```

### Summary CSV
```
Date,Count,Average,Min,Max
2024-06-26,3,26.10,25.80,26.40
```