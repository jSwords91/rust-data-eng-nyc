mod anomaly_detector;
mod config;
mod data_fetcher;
mod data_processor;
mod errors;
use std::time::Instant;
use std::cmp::Ordering;

use anyhow::Result;
use log::{info, warn, error, LevelFilter};
use env_logger::Builder;

fn main() -> Result<()> {
    // Initialize the logger
    Builder::new()
        .filter_level(LevelFilter::Info)
        .format_timestamp_secs()
        .init();


    info!("Starting Travel Time Analysis");
    let start_time = Instant::now();

    let config = config::Settings::new()?;

    let raw_data = data_fetcher::fetch_data(&config.app.data_url)?;

    if raw_data.is_empty() {
        error!("No data fetched from the URL. Exiting.");
        return Ok(());
    }

    let processed_data = data_processor::process_data(&raw_data)?;

    if processed_data.is_empty() {
        warn!("No data after processing. Skipping saving and anomaly detection.");
        return Ok(());
    }

    data_processor::save_data(&processed_data, &config.app.output_file)?;

    let mut anomalies = anomaly_detector::detect_anomalies(
        &processed_data,
        config.anomaly_detection.iqr_multiplier,
    );

    // Sort anomalies by date
    anomalies.sort_by(|a, b| a.date.cmp(&b.date));


    println!("Anomalies:");
    for anomaly in anomalies {
        println!("Date: {}, Value: {:.2}", anomaly.date, anomaly.avg_value);
    }

    let duration = start_time.elapsed();
    info!("Travel Time Analysis completed successfully in {:.2?}", duration);
    Ok(())
}