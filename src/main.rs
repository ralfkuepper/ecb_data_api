use std::io::Cursor;
use reqwest::Error;
use polars::prelude::*;


#[tokio::main]
async fn main() -> Result<(), Error> {
    // Define the base URL for the ECB API
    let base_url = "https://data-api.ecb.europa.eu/service/data/";

    let series_key = "ILM/M.U2.C.L020200.U2.EUR";
    //let series_key = "ICP.M.DE.N.000000.4.INX";

    let format_type = "?format=csvdata";

    let start = "&startPeriod=2007-01";
    let end = "&endPeriod=2022-12";

    let url = format!("{base_url}{series_key}{format_type}{start}{end}");
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
