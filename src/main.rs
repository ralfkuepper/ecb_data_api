use log::{info, error};
use std::fs::File;
use std::io::Cursor;
use std::env;
use reqwest::Error;
use polars::prelude::*;
use clap::Parser;
use chrono::Local;

#[derive(Parser, Debug, Clone)]
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
    /// Detail Level of Query Reponse, Options: "full", "dataonly", "serieskeysonly", "nodata"
    #[arg(long)]
    detail: Option<String>,
    /// File Format, Options: "CSV", "JSON"
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
        // TODO: match on enum Detail
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

fn save_dataframe(df: &mut DataFrame, series_key: String, file_format: String) -> Result<(), PolarsError> {
    // TODO match statement depending on file_format
    // option 1: CSV
    // option 2: JSON
    
    let datetime = Local::now().format("%Y-%m-%d_%H-%M-%S");
    let path = format!("{}_{}.{}", datetime, series_key, file_format.to_lowercase());

    match file_format.to_lowercase().as_str() {
        "csv" => {
            let file = File::create(path).unwrap();
            CsvWriter::new(file)
                .with_separator(b';') // separator must be as Byte not String
                .finish(df)
        },
        "json" => {
            let file = File::create(path).unwrap();
            JsonWriter::new(file)
                .with_json_format(JsonFormat::Json)
                .finish(df)
        },
        _ => {
            let err = Err(PolarsError::ComputeError(
                format!("Unsupported file format: {}", file_format).into(), 
            ));
            error!("Unsupported file format: {}", file_format);
            err
        },
     }
    
}


#[tokio::main]
async fn main() -> Result<(), Error> {
    // initialize logger
    if env::var("RUST_LOG").is_err() { env::set_var("RUST_LOG", "info") }
    env_logger::init();

    // read CLI args
    let args = CliArgs::parse();
    let args1 = args.clone();

    info!("Given Params: {:#?}", args);

    let url = construct_request_url(args.series_key, args.start_period, args.end_period, args.detail, None, None, None, false);
    info!("Full Request: {}", &url);

    // make the HTTP GET request
    let response = reqwest::get(&url).await?;

    // check if the response is successful
    if response.status().is_success() {
        info!("Request successful!");

        let csv_raw: String  = response.text().await?;
        let mut csv_df = CsvReader::new(
            Cursor::new(csv_raw))
            .infer_schema(None)
            .has_header(true)
            .finish()
            .unwrap()
            ;

        println!("{:?}", &csv_df);
        let _ = save_dataframe(&mut csv_df, args1.series_key, args1.file_format);
        
    } else {
        error!("Failed to fetch data: {}", response.status());
    }

    Ok(())
}
