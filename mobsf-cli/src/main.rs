mod app;
mod cli;

use std::env;
use clap::ArgMatches;
use mobsf_core::error::AppError;
use crate::app::App;
use crate::cli::cli;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const AUTHOR: &'static str = env!("CARGO_PKG_AUTHORS");
const NAME: &'static str = env!("CARGO_PKG_NAME");
const HOST: &'static str = "http://localhost:8000";
const API_KEY: &'static str = "";

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let matches = cli();

    let api_key = env::var("MOBSF_API_KEY").unwrap_or(API_KEY.to_owned());
    let host = env::var("MOBSF_HOST").unwrap_or(HOST.to_owned());

    let app = App::new(api_key, host).await?;
    execute(app, matches).await?;

    Ok(())
}

async fn execute(app: App, matches: ArgMatches) -> Result<(), AppError> {
    let subcommand = matches.subcommand();

    match subcommand {
        Some(("upload", upload_matches)) => {
            let file_path = upload_matches.value_of("file_path").unwrap();
            app.upload_file(file_path).await?;
        }
        Some(("scan", scan_matches)) => {
            let scan_type = scan_matches.value_of("scan_type").unwrap();
            let file_name = scan_matches.value_of("file_name").unwrap();
            let hash = scan_matches.value_of("file_hash").unwrap();

            app.scan(scan_type, file_name, hash).await?;
        }
        Some(("scans", _)) => {
            app.scans().await?;
        }
        Some(("report", report_matches)) => {
            match report_matches.subcommand() {
                Some(("pdf", pdf_matches)) => {
                    let hash = pdf_matches.value_of("file_hash").unwrap();
                    let output_file_path = pdf_matches.value_of("output_file_path").unwrap();
                    app.report_pdf(hash, output_file_path).await?;
                }
                Some(("json", json_matches)) => {
                    let hash = json_matches.value_of("file_hash").unwrap();

                    if json_matches.is_present("stdout_output") {
                        app.print_report_json(hash).await?;
                    } else {
                        let output_file_path = json_matches.value_of("output_file_path").unwrap();
                        app.write_report_json(hash, output_file_path).await?;
                    }
                }
                Some(_) => {}
                None => {}
            }
        }
        Some(("delete", delete_matches)) => {
            let hash = delete_matches.value_of("file_hash").unwrap();
            app.delete_scan(hash).await?;
        }
        Some(("play", play_matches)) => {
            let file_path = play_matches.value_of("file_path").unwrap();
            app.play(file_path).await?;
        }
        Some(("source", source_matches)) => {
            let scan_type = source_matches.value_of("scan_type").unwrap();
            let file_path = source_matches.value_of("file_path").unwrap();
            let hash = source_matches.value_of("file_hash").unwrap();
            app.view_source(scan_type, file_path, hash).await?;
        }
        Some(_) => {}
        None => {}
    }

    Ok(())
}
