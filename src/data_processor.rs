use crate::errors::AppError;
use chrono::NaiveDateTime;
use csv::Writer;
use log::{info, warn};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;


#[derive(Debug, Deserialize)]
pub struct RawRecord {
    pub timestamp: String,
    pub value: Option<i32>,
}

#[derive(Debug, Serialize, Clone)]
pub struct DailyAggregate {
    pub date: String,
    pub avg_value: f64,
}

pub fn process_data(data: &str) -> Result<Vec<DailyAggregate>, AppError> {
    info!("Processing data");
    let records = parse_data(data)?;
    info!("Parsed {} raw records", records.len());
    let clean_records = clean_data(records);
    info!("Cleaned data, resulting in {} records", clean_records.len());
    let daily_aggregates = aggregate_daily(clean_records);
    info!("Aggregated data into {} daily records", daily_aggregates.len());
    Ok(daily_aggregates)
}

fn parse_data(data: &str) -> Result<Vec<RawRecord>, AppError> {
    let mut rdr = csv::Reader::from_reader(data.as_bytes());
    let records: Result<Vec<RawRecord>, _> = rdr.deserialize().collect();
    Ok(records?)
}

fn clean_data(records: Vec<RawRecord>) -> Vec<(NaiveDateTime, i32)> {
    let mut cleaned = Vec::new();
    let mut dropped = 0;

    for record in records {
        if let (Ok(timestamp), Some(value)) = (
            NaiveDateTime::parse_from_str(&record.timestamp, "%Y-%m-%d %H:%M:%S"),
            record.value,
        ) {
            cleaned.push((timestamp, value));
        } else {
            dropped += 1;
        }
    }

    if dropped > 0 {
        warn!("Dropped {} records due to parsing errors or null values", dropped);
    }

    cleaned
}

fn aggregate_daily(data: Vec<(NaiveDateTime, i32)>) -> Vec<DailyAggregate> {
    let mut daily_data: HashMap<String, Vec<i32>> = HashMap::new();

    for (timestamp, value) in data {
        let date = timestamp.format("%Y-%m-%d").to_string();
        daily_data.entry(date).or_insert_with(Vec::new).push(value);
    }

    daily_data
        .into_iter()
        .map(|(date, values)| {
            let avg_value = values.iter().sum::<i32>() as f64 / values.len() as f64;
            DailyAggregate { date, avg_value }
        })
        .collect()
}

pub fn save_data(data: &[DailyAggregate], file_path: &str) -> Result<(), AppError> {
    info!("Saving data to {}", file_path);
    let file = File::create(file_path)?;
    let mut wtr = Writer::from_writer(file);

    for record in data {
        wtr.serialize(record)?;
    }

    wtr.flush()?;
    info!("Data saved successfully");
    Ok(())
}