use clap::{App, AppSettings, Arg, ArgMatches};
use crate::{AUTHOR, NAME, VERSION};

pub fn cli() -> ArgMatches {
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
                .subcommand(
                    App::new("pdf")
                        .about("Pdf report format")
                        .arg(Arg::new("file_hash")
                            .required(true)
                            .index(1)
                            .takes_value(true))
                        .arg(Arg::new("output_file_path")
                            .short('o')
                            .help("File path to save a report")
                            .required(false)
                            .default_value("report.pdf")
                        )
                )
                .subcommand(
                    App::new("json")
                        .about("JSON report format")
                        .arg(Arg::new("file_hash")
                            .required(true)
                            .index(1)
                            .takes_value(true))
                        .arg(Arg::new("output_file_path")
                            .short('o')
                            .help("File path to save a report")
                            .required(false)
                            .default_value("report.json")
                        )
                        .arg(Arg::new("stdout_output")
                            .short('p')
                            .help("Print to stdout instead of saving a file")
                            .required(false)
                            .takes_value(false)
                        )
                )
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
        .subcommand(
            App::new("source")
                .about("View source files.")
                .arg(Arg::new("scan_type")
                    .possible_values(["apk", "ipa", "studio", "eclipse", "ios"])
                    .required(true)
                    .index(1)
                    .takes_value(true))
                .arg(Arg::new("file_path")
                    .alias("Relative file path")
                    .required(true)
                    .index(2)
                    .takes_value(true))
                .arg(Arg::new("file_hash")
                    .required(true)
                    .index(3)
                    .takes_value(true))
        )
        .get_matches();

    matches
}