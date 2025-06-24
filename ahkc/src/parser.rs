use ahkc_model::record_type::RecordType;
use ahkc_model::{ExportDate, HealthData, Me, MetadataEntry, Record};
use anyhow::Result;
use quick_xml::Reader;
use quick_xml::events::Event;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::str::FromStr;

pub fn parse_health_data(path: &Path) -> Result<HealthData> {
    let file = File::open(path)?;
    let file = BufReader::new(file);
    let mut reader = Reader::from_reader(file);
    reader.config_mut().trim_text(true);

    let mut health_data = HealthData {
        locale: String::new(),
        export_date: ExportDate {
            value: String::new(),
        },
        me: Me {
            date_of_birth: String::new(),
            biological_sex: String::new(),
            blood_type: String::new(),
            fitzpatrick_skin_type: String::new(),
            cardio_fitness_medications_use: String::new(),
        },
        records: Vec::new(),
    };

    let mut buf = Vec::new();
    let mut current_record: Option<Record> = None;

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(ref e) | Event::Empty(ref e) => {
                match e.name().as_ref() {
                    b"HealthData" => {
                        for attr in e.attributes() {
                            let attr = attr?;
                            if attr.key.as_ref() == b"locale" {
                                health_data.locale =
                                    String::from_utf8_lossy(&attr.value).to_string();
                            }
                        }
                    },
                    b"ExportDate" => {
                        for attr in e.attributes() {
                            let attr = attr?;
                            if attr.key.as_ref() == b"value" {
                                health_data.export_date.value =
                                    String::from_utf8_lossy(&attr.value).to_string();
                            }
                        }
                    },
                    b"Me" => {
                        for attr in e.attributes() {
                            let attr = attr?;
                            match attr.key.as_ref() {
                                b"HKCharacteristicTypeIdentifierDateOfBirth" => {
                                    health_data.me.date_of_birth =
                                        String::from_utf8_lossy(&attr.value).to_string();
                                },
                                b"HKCharacteristicTypeIdentifierBiologicalSex" => {
                                    health_data.me.biological_sex =
                                        String::from_utf8_lossy(&attr.value).to_string();
                                },
                                b"HKCharacteristicTypeIdentifierBloodType" => {
                                    health_data.me.blood_type =
                                        String::from_utf8_lossy(&attr.value).to_string();
                                },
                                b"HKCharacteristicTypeIdentifierFitzpatrickSkinType" => {
                                    health_data.me.fitzpatrick_skin_type =
                                        String::from_utf8_lossy(&attr.value).to_string();
                                },
                                b"HKCharacteristicTypeIdentifierCardioFitnessMedicationsUse" => {
                                    health_data.me.cardio_fitness_medications_use =
                                        String::from_utf8_lossy(&attr.value).to_string();
                                },
                                _ => {},
                            }
                        }
                    },
                    b"Record" => {
                        let mut record = Record {
                            r#type: RecordType::BodyMass, // Temporary default
                            source_name: String::new(),
                            source_version: None,
                            unit: None,
                            value: None,
                            creation_date: None,
                            start_date: String::new(),
                            end_date: String::new(),
                            device: None,
                            metadata_entries: Vec::new(),
                        };

                        for attr in e.attributes() {
                            let attr = attr?;
                            match attr.key.as_ref() {
                                b"type" => {
                                    let type_str = String::from_utf8_lossy(&attr.value);
                                    record.r#type = RecordType::from_str(&type_str)?;
                                },
                                b"sourceName" => {
                                    record.source_name =
                                        String::from_utf8_lossy(&attr.value).to_string();
                                },
                                b"sourceVersion" => {
                                    record.source_version =
                                        Some(String::from_utf8_lossy(&attr.value).to_string());
                                },
                                b"unit" => {
                                    record.unit =
                                        Some(String::from_utf8_lossy(&attr.value).to_string());
                                },
                                b"value" => {
                                    record.value =
                                        Some(String::from_utf8_lossy(&attr.value).to_string());
                                },
                                b"creationDate" => {
                                    record.creation_date =
                                        Some(String::from_utf8_lossy(&attr.value).to_string());
                                },
                                b"startDate" => {
                                    record.start_date =
                                        String::from_utf8_lossy(&attr.value).to_string();
                                },
                                b"endDate" => {
                                    record.end_date =
                                        String::from_utf8_lossy(&attr.value).to_string();
                                },
                                b"device" => {
                                    record.device =
                                        Some(String::from_utf8_lossy(&attr.value).to_string());
                                },
                                _ => {},
                            }
                        }

                        if matches!(e.name().as_ref(), b"Record") && e.is_empty() {
                            health_data.records.push(record);
                        } else {
                            current_record = Some(record);
                        }
                    },
                    b"MetadataEntry" => {
                        if let Some(ref mut record) = current_record {
                            let mut metadata_entry = MetadataEntry {
                                key: String::new(),
                                value: String::new(),
                            };

                            for attr in e.attributes() {
                                let attr = attr?;
                                match attr.key.as_ref() {
                                    b"key" => {
                                        metadata_entry.key =
                                            String::from_utf8_lossy(&attr.value).to_string();
                                    },
                                    b"value" => {
                                        metadata_entry.value =
                                            String::from_utf8_lossy(&attr.value).to_string();
                                    },
                                    _ => {},
                                }
                            }

                            record.metadata_entries.push(metadata_entry);
                        }
                    },
                    _ => {},
                }
            },
            Event::End(ref e) => {
                if e.name().as_ref() == b"Record" {
                    if let Some(record) = current_record.take() {
                        health_data.records.push(record);
                    }
                }
            },
            Event::Eof => break,
            _ => {},
        }
        buf.clear();
    }

    Ok(health_data)
}
