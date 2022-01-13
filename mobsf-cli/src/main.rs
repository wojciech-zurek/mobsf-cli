mod app;
mod cli;
mod error;

use std::env;
use clap::ArgMatches;
use mobsf_core::error::MobsfError;
use crate::app::App;
use crate::cli::cli;
use crate::error::AppError;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const AUTHOR: &'static str = env!("CARGO_PKG_AUTHORS");
const NAME: &'static str = env!("CARGO_PKG_NAME");
const SERVER: &'static str = "http://localhost:8000";
const API_KEY: &'static str = "";

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let matches = cli();

    let api_key = env::var("MOBSF_API_KEY").unwrap_or(API_KEY.to_owned());
    let server = env::var("MOBSF_SERVER").unwrap_or(SERVER.to_owned());

    let app = init_app(api_key, server, &matches).await?;
    execute(app, matches).await?;

    Ok(())
}

async fn init_app(api_key: String, server: String, matches: &ArgMatches) -> Result<App, MobsfError> {
    let api_key = if let Some(ak) = matches.value_of("api_key") {
        ak.to_owned()
    } else {
        api_key
    };

    let server = if let Some(s) = matches.value_of("server") {
        s.to_owned()
    } else {
        server
    };

    App::new(api_key, server).await
}

async fn execute(app: App, matches: ArgMatches) -> Result<(), AppError> {
    match matches.subcommand() {
        Some(("ci", ci_matches)) => {
            let file_path = ci_matches.value_of("file_path").unwrap();
            let path_to_save = ci_matches.value_of("path_to_save").unwrap();
            let cvss: f32 = ci_matches.value_of("cvss").unwrap().parse().unwrap();
            let trackers: u16 = ci_matches.value_of("trackers").unwrap().parse().unwrap();
            let security: u8 = ci_matches.value_of("security").unwrap().parse().unwrap();
            let re_scan = ci_matches.is_present("re_scan");
            app.ci(file_path, re_scan, path_to_save, cvss, trackers, security).await?;
        }

        Some(("upload", upload_matches)) => {
            let file_path = upload_matches.value_of("file_path").unwrap();
            app.upload_file(file_path).await?;
        }
        Some(("scan", scan_matches)) => {
            let scan_type = scan_matches.value_of("scan_type").unwrap();
            let file_name = scan_matches.value_of("file_name").unwrap();
            let hash = scan_matches.value_of("file_hash").unwrap();
            let re_scan = scan_matches.is_present("re_scan");

            app.scan(scan_type, file_name, hash, re_scan).await?;
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
            let re_scan = play_matches.is_present("re_scan");
            app.play(file_path, re_scan).await?;
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
