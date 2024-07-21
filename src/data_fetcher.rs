use crate::errors::AppError;
use log::info;

pub fn fetch_data(url: &str) -> Result<String, AppError> {
    info!("Fetching data from {}", url);
    let response = reqwest::blocking::get(url)?;
    let text = response.text()?;
    info!("Data fetched successfully");
    Ok(text)
}