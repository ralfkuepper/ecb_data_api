# Rust-Based ECB Data Downloader
<img src="https://www.ecb.europa.eu/shared/img/logo/logo_only.svg"  width="120">

## About 
This CLI-tool allows for easy and fast downloading of data from the European Central Bank's [Data Portal](https://data.ecb.europa.eu/help/data/overview).

The tool aims at simplifying the usage of the [official API interface](https://data.ecb.europa.eu/help/api/data) (on which it is built). The official interface requires to collect all parameters into a single URL string, which is error-prone if done manually. Instead the parameters can be given as argument to the CLI-tool and the appropriate query will be build for the user.

## Installation
Download the compiled binary from the target directory matching your OS and system archicture (_curently not available but planned_).

If your OS is not included, clone this repository and use the Rust compiler to build for your OS and system.

```
git clone https://github.com/tom-walter/ecb_data_api.git
cargo build --release
```

## Usage
The `--help` command provides an overview on how to use the CLI-tool and which arguments are currently available.

```
A CLI-Tool for Querying Data from the ECB's Public API

Usage: ecb_data_api [OPTIONS] --series-key <SERIES_KEY>

Options:
  -k, --series-key <SERIES_KEY>      Data Series Key from ECB Website
      --start-period <START_PERIOD>  Starting Point matching Date Time format of Series Key
      --end-period <END_PERIOD>      Ending Point matching Date Time format of Series Key
      --detail <DETAIL>              Detail Level of Query Reponse, Options: "full", "dataonly", "serieskeysonly", "nodata"
      --file-format <FILE_FORMAT>    File Format, Options: "CSV", "JSON", "JSONL" [default: CSV]
  -h, --help                         Print help
  -V, --version                      Print version
```

## Example
A minimal working example:
```
ecb_data_api --series-key ILM.M.U2.C.L020200.U2.EUR --start-period 2007-01 --end-period 2022-12 --detail dataonly
```

For convenience, the constructed URL query is printed to the console for the user. 

## TODOs
This project is ongoing and more features will be added. The 
* add all possible parameters available in the [offical API](https://data.ecb.europa.eu/help/api/data)
    * `series-key` ✔
    * `start-period` ✔
    * `end-period` ✔
    * `detail` ✔
    * `update-after`
    * `first-n-observations`
    * `last-n-observations`
    * `include-history`
* support various, common file formats
    * CSV ✔
    * JSON ✔
    * JSONL ✔
    * XLS
    * XLSX
* include compiled binaries
    * Windows (`x86_64-pc-windows-msvc`)
    * Linux (`x86_64-unknown-linux-gnu`)
    * Apple (`x86_64-apple-darwin`)
* write detailed documentation on usage
* implement proper error handling and logging

