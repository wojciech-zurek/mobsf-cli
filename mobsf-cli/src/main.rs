use clap::{App, AppSettings, Arg, ArgMatches};
use mobsf_core::error::AppError;
use mobsf_core::Mobsf;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const AUTHOR: &'static str = env!("CARGO_PKG_AUTHORS");
const NAME: &'static str = env!("CARGO_PKG_NAME");
const HOST: &'static str = "http://localhost:8000";
const API_KEY: &'static str = "ed8321579ceea37128dd9f8c85b2940d861f78f81b60bb43effe54c31a207ec4";

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let matches = cli();

    execute(matches).await?;

    Ok(())
}

fn cli() -> ArgMatches {
    let matches = App::new(NAME)
        .about("mobsf-cli app")
        .version(VERSION)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .author(AUTHOR)
        .subcommand(
            App::new("upload")
                .about("Upload a file.")
                .arg(Arg::new("file_path")
                    // .name("File path")
                    .required(true)
                    .index(1)
                    .takes_value(true))
        )
        .subcommand(
            App::new("scan")
                .about("Scan a file.")
                .arg(Arg::new("scan_type")
                    .possible_values(["xapk", "apk", "zip", "ipa", "appx"])
                    .required(true)
                    .index(1)
                    .takes_value(true))
                .arg(Arg::new("file_name")
                    .alias("File name")
                    .required(true)
                    .index(2)
                    .takes_value(true))
                .arg(Arg::new("file_hash")
                    .required(true)
                    .index(3)
                    .takes_value(true))
        )
        .subcommand(
            App::new("scans")
                .about("Display recent scans.")
        )
        .subcommand(
            App::new("report")
                .about("Get report.")
                .arg(Arg::new("report_type")
                    .possible_values(["pdf", "json"])
                    .required(true)
                    .index(1)
                    .takes_value(true))
                .arg(Arg::new("file_hash")
                    .required(true)
                    .index(2)
                    .takes_value(true))
        )
        .subcommand(
            App::new("delete")
                .about("Delete scan.")
                .arg(Arg::new("file_hash")
                    .required(true)
                    .index(1)
                    .takes_value(true))
        ).
        subcommand(
            App::new("play")
                .about("Upload a file and auto start scan.")
                .arg(Arg::new("file_path")
                    // .name("File path")
                    .required(true)
                    .index(1)
                    .takes_value(true))
        )
        .get_matches();

    matches
}

async fn execute(matches: ArgMatches) -> Result<(), AppError> {
    let subcommand = matches.subcommand();
    match subcommand {
        Some(("upload", upload_matches)) => {
            let file_path = upload_matches.value_of("file_path").unwrap();
            upload_file(file_path).await?;
        }
        Some(("scan", scan_matches)) => {
            let scan_type = scan_matches.value_of("scan_type").unwrap();
            let file_name = scan_matches.value_of("file_name").unwrap();
            let hash = scan_matches.value_of("file_hash").unwrap();

            scan(scan_type, file_name, hash).await?;
        }
        Some(("scans", scans_matches)) => {
            scans().await?;
        }
        Some(("report", report_matches)) => {
            let report_type = report_matches.value_of("report_type").unwrap();
            let hash = report_matches.value_of("file_hash").unwrap();

            match report_type {
                "pdf" => {
                    report_pdf(hash, "report.pdf").await;
                }
                "json" => {}
                _ => {}
            };
        }
        Some(("delete", delete_matches)) => {
            let hash = delete_matches.value_of("file_hash").unwrap();
            delete_scan(hash).await?;
        }
        Some(("play", play_matches)) => {
            let file_path = play_matches.value_of("file_path").unwrap();
            play(file_path).await?;
        }
        Some(_) => {}
        None => {}
    }

    Ok(())
}


async fn upload_file(file_path: &str) -> Result<(), AppError> {
    let mobsf = Mobsf::new(API_KEY.to_owned(), HOST.to_owned()).await?;
    let response = mobsf.upload(file_path).await?;
    print!("{}", response);
    println!("Start scan command : {} scan {} {} {}", NAME, response.scan_type(), response.file_name(), response.hash());

    Ok(())
}

async fn scans() -> Result<(), AppError> {
    let mobsf = Mobsf::new(API_KEY.to_owned(), HOST.to_owned()).await?;
    let response = mobsf.scans().await?;

    print!("{}", response);

    Ok(())
}

async fn scan(scan_type: &str, file_name: &str, hash: &str) -> Result<(), AppError> {
    let mobsf = Mobsf::new(API_KEY.to_owned(), HOST.to_owned()).await?;
    let response = mobsf.scan(scan_type, file_name, hash).await?;
    print!("{}", response);

    Ok(())
}

async fn delete_scan(hash: &str) -> Result<(), AppError> {
    let mobsf = Mobsf::new(API_KEY.to_owned(), HOST.to_owned()).await?;
    let response = mobsf.delete_scan(hash).await?;
    print!("{}", response);
    Ok(())
}

async fn play(file_path: &str) -> Result<(), AppError> {
    let mobsf = Mobsf::new(API_KEY.to_owned(), HOST.to_owned()).await?;
    let response = mobsf.upload(file_path).await?;
    print!("{}", response);

    let response = mobsf.scan(response.scan_type(), response.file_name(), response.hash()).await?;
    print!("{}", response);

    Ok(())
}

async fn report_pdf(hash: &str, file_path: &str) -> Result<(), AppError> {
    let mobsf = Mobsf::new(API_KEY.to_owned(), HOST.to_owned()).await?;
    mobsf.report_pdf(hash, file_path).await?;

    Ok(())
}
