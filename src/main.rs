use std::io::Cursor;
use reqwest::Error;
use polars::prelude::*;

const ENTRYPOINT: &str = "https://data-api.ecb.europa.eu";

fn construct_request_url(
    series_key: &str,
    start_period: Option<&str>,
    end_period: Option<&str>,
    detail: Option<&str>,
    updated_after: Option<&str>,
    first_n_observations: Option<&str>,
    last_n_observations: Option<&str>,
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
    // Define the base URL for the ECB API
    // let base_url = "https://data-api.ecb.europa.eu/service/data/";

    let series_key = "ILM.M.U2.C.L020200.U2.EUR";
    //let series_key = "ICP.M.DE.N.000000.4.INX";

    // let format_type = "?format=csvdata";

    let start_period = Some("2007-01");
    let end_period = Some("2022-12");

    let url = construct_request_url(series_key, start_period, end_period, None, None, None, None, false);
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
