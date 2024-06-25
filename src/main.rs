extern crate anyhow;
extern crate csv;
extern crate reqwest;
extern crate tokio;

use anyhow::Result;
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs::DirBuilder;
use std::fs::File;
use std::io;
use std::process;
use url::Url;

async fn fetch_file(url: Url, output: &OsString) -> Result<()> {
    let file = url
        .path_segments()
        .expect("Could not slice into segments")
        .last()
        .expect("Could not find last char");
    let filename = format!("output/{file}");
    let response = reqwest::get(url).await?;
    DirBuilder::new().recursive(true).create(output)?;
    let bytes = response.bytes().await?;
    let mut out = File::create(filename)?;
    io::copy(&mut bytes.as_ref(), &mut out)?;

    Ok(())
}

fn get_first_arg() -> OsString {
    match env::args_os().nth(1) {
        None => OsString::from("data.csv"),
        Some(file_path) => file_path,
    }
}

fn get_second_arg() -> OsString {
    match env::args_os().nth(2) {
        None => OsString::from("output"),
        Some(output_path) => output_path,
    }
}

fn fopen(path: &OsString) -> Result<File, Box<dyn Error>> {
    let file = File::open(path)?;
    Ok(file)
}

fn csv_parse(file: File) -> Result<Vec<String>, Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(file);
    let mut urls = Vec::new();
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", &record);
        urls.push(record.as_slice().to_string());
    }
    Ok(urls)
}

#[tokio::main]
async fn main() {
    let file_path = get_first_arg();
    let output_path = get_second_arg();

    match csv_parse(fopen(&file_path).unwrap()) {
        Ok(urls) => {
            for (_, url) in urls.iter().enumerate() {
                let url = Url::parse(url).expect("invalid url format");
                let _ = fetch_file(url, &output_path).await;
            }
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            process::exit(1);
        }
    }
}
