use std::io::Cursor;
use reqwest::Error;
use polars::prelude::*;
use clap::Parser;

#[derive(Parser, Debug)]
# [command(version)]
# [command(about = "A CLI-Tool for Querying Data from the ECB's Public API")]
struct  CliArgs {
    /// Data Series Key from ECB Website
    #[arg(long)]
    series_key: String,
    /// Starting Point matching Date Time format of Series Key
    #[arg(long)]
    start_period: Option<String>,
    /// Ending Point matching Date Time format of Series Key
    #[arg(long)]
    end_period: Option<String>,
    /// File Format, Options: "CSV", "JSON",
    #[arg(long, default_value = "CSV")]
    file_format: String,
}

const ENTRYPOINT: &str = "https://data-api.ecb.europa.eu";

fn construct_request_url(
    series_key: String,
    start_period: Option<String>,
    end_period: Option<String>,
    detail: Option<String>,
    updated_after: Option<String>,
    first_n_observations: Option<String>,
    last_n_observations: Option<String>,
    include_history: bool,
) -> String {
    let (db, ticker) = series_key.split_once('.').unwrap();

    let mut url = format!("{ENTRYPOINT}/service/data/{db}/{ticker}?format=csvdata");

    if let Some(start) = start_period {
        url.push_str(&format!("&startPeriod={}", start));
    }

    if let Some(end) = end_period {
        url.push_str(&format!("&endPeriod={}", end));
    }

    if let Some(dtl) = detail {
        url.push_str(&format!("&detail={}", dtl));
    }

    if let Some(updated) = updated_after {
        url.push_str(&format!("&updatedAfter={}", updated));
    }

    if let Some(first_n) = first_n_observations {
        url.push_str(&format!("&firstNObservations={}", first_n));
    }

    if let Some(last_n) = last_n_observations {
        url.push_str(&format!("&lastNObservations={}", last_n));
    }

    if include_history {
    url.push_str("&includeHistory=true");
    };

    url
}


#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = CliArgs::parse();

    println!("Given Params: {:#?}", args);

    let url = construct_request_url(args.series_key, args.start_period, args.end_period, None, None, None, None, false);
    println!("Full Request: {}", &url);

    // Make the HTTP GET request
    let response = reqwest::get(&url).await?;

    // Check if the response is successful
    if response.status().is_success() {
        let csv_raw: String  = response.text().await?;
        let csv_df = CsvReader::new(
            Cursor::new(csv_raw))
            .infer_schema(None)
            .has_header(true)
            .finish();

        println!("{:?}", csv_df);
        
    } else {
        println!("Failed to fetch data: {}", response.status());
    }

    Ok(())
}
