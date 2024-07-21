use crate::data_processor::DailyAggregate;
use log::info;

pub fn detect_anomalies(data: &[DailyAggregate], iqr_multiplier: f64) -> Vec<DailyAggregate> {
    info!("Detecting anomalies");

    // Sort the data by date
    let mut sorted_data = data.to_vec();
    sorted_data.sort_by(|a, b| a.date.cmp(&b.date));

    let values: Vec<f64> = sorted_data.iter().map(|r| r.avg_value).collect();

    let q1 = percentile(&values, 0.25);
    let q3 = percentile(&values, 0.75);
    let iqr = q3 - q1;
    let lower_bound = q1 - iqr_multiplier * iqr;
    let upper_bound = q3 + iqr_multiplier * iqr;

    let anomalies: Vec<DailyAggregate> = sorted_data
        .into_iter()
        .filter(|r| r.avg_value < lower_bound || r.avg_value > upper_bound)
        .collect();

    info!("Detected {} anomalies", anomalies.len());
    anomalies
}

fn percentile(data: &[f64], percentile: f64) -> f64 {
    let index = (percentile * (data.len() - 1) as f64).floor() as usize;
    data[index]
}
