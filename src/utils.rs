use std::{collections::HashMap, fs, path::Path, time::Duration};

use anyhow::{Context, Result};
use chrono::NaiveDate;
use reqwest::Client;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use tokio::time::sleep;


/// Custom function to serialize a chrono::NaiveDate into a date string
pub fn serialize_date<S>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = date.format("%Y-%m-%d").to_string();
    serializer.serialize_str(&s)
}

/// Custom function to deserialize a date string into a chrono::NaiveDate
pub fn deserialize_date<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    NaiveDate::parse_from_str(s, "%Y-%m-%d").map_err(de::Error::custom)
}

/// Function to check if cache exists and read from it
pub fn read_from_cache<T>(cache_path: &str) -> Result<Option<T>>
where
    T: for<'de> Deserialize<'de>,
{
    let cache_file = Path::new(cache_path);

    print!("Checking for cache file at: {} | ", cache_path);

    if cache_file.exists() {
        println!("Cache found. Reading from file...");

        let file_content =
            fs::read_to_string(cache_file).context("Failed to read the cache file")?;

        let data: T =
            serde_json::from_str(&file_content).context("Failed to deserialize JSON from cache")?;
        return Ok(Some(data));
    }

    Ok(None)
}

/// Function to write API response to cache
pub fn write_to_cache<T>(cache_path: &str, data: &T) -> Result<()>
where
    T: Serialize,
{
    let json_string =
        serde_json::to_string_pretty(data).context("Failed to serialize data to JSON")?;

    fs::write(cache_path, json_string).context("Failed to write the cache file")?;

    Ok(())
}

/// Function to request data from the API
pub async fn request_data<T>(
    client: &Client,
    url: &str,
    query_params: &HashMap<&str, &str>,
    headers: &HashMap<&str, &str>,
    access_level: &str,
) -> Result<T>
where
    T: for<'de> Deserialize<'de>,
{
    println!("Cache not found. Making HTTP request...");
    let mut request = client.get(url).query(query_params);

    for (key, value) in headers {
        request = request.header(*key, *value);
    }

    if access_level == "trial" {
        // Setting a sleep time to avoid the 1 query per second rate limiting
        sleep(Duration::from_secs(2)).await;
    }

    let response = request.send().await.context("Failed to send request")?;

    if !response.status().is_success() {
        anyhow::bail!("Request failed with status: {}", response.status());
    }

    let data = response
        .json::<T>()
        .await
        .context("Failed to parse JSON response")?;

    Ok(data)
}

// TODO update query params to use proper types
// TODO update for urls to be enums
/// Function to fetch data, either from cache or by requesting
pub async fn fetch_data<T>(
    client: &Client,
    url: &str,
    query_params: &HashMap<&str, &str>,
    headers: &HashMap<&str, &str>,
    cache_path: &str,
    access_level: &str,
) -> Result<T>
where
    T: for<'de> Deserialize<'de> + Serialize,
{
    // Read from cache if it exists
    if let Some(data) = read_from_cache(cache_path)? {
        return Ok(data);
    }

    // Request data from the API
    let data = request_data(client, url, query_params, headers, access_level).await?;

    // Write data to cache
    write_to_cache(cache_path, &data)?;

    Ok(data)
}
