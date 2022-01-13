use std::fmt::{Display, Formatter};
use chrono::{DateTime, Local, Utc};
use serde::Deserialize;
use cli_table::{print_stdout, Table, WithTitle};
use cli_table::format::{Border, Separator};

#[derive(Debug, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

impl Display for ErrorResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.error)
    }
}

#[derive(Debug, Deserialize)]
pub struct UploadResponse {
    analyzer: String,
    status: String,
    hash: String,
    scan_type: String,
    file_name: String,
}

impl UploadResponse {
    pub fn analyzer(&self) -> &str {
        &self.analyzer
    }
    pub fn status(&self) -> &str {
        &self.status
    }
    pub fn hash(&self) -> &str {
        &self.hash
    }
    pub fn scan_type(&self) -> &str {
        &self.scan_type
    }
    pub fn file_name(&self) -> &str {
        &self.file_name
    }
}

impl Display for UploadResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Status: {}", self.status())?;
        writeln!(f, "File name: {}", self.file_name())?;
        writeln!(f, "Hash: {}", self.hash())?;
        writeln!(f, "Scan type: {}", self.scan_type())?;
        writeln!(f, "Analyzer: {}", self.analyzer())
    }
}

#[derive(Debug, Deserialize)]
pub struct ScansResponse {
    content: Vec<ScanItem>,
    count: u16,
    num_pages: u16,
}

impl ScansResponse {
    pub fn content(&self) -> &Vec<ScanItem> {
        &self.content
    }

    pub fn count(&self) -> u16 {
        self.count
    }

    pub fn num_pages(&self) -> u16 {
        self.num_pages
    }
}

impl Display for ScansResponse {
    fn fmt(&self, _: &mut Formatter<'_>) -> std::fmt::Result {
        // writeln!(f, "{: <4} | {: <25} | {: <32} | {: <32} | {: <10} | {: <30} | {: <19} | {}",
        //          "Type",
        //          "File name",
        //          "Hash",
        //          "App name",
        //          "Version",
        //          "Package Name",
        //          "Analyzer",
        //          "Timestamp",
        // )?;
        //
        // self.content().into_iter().try_for_each(|it| {
        //     writeln!(f, "{: <4} | {: <25} | {: <32} | {: <32} | {: <10} | {: <30} | {: <19} | {}",
        //              it.scan_type(),
        //              it.file_name(),
        //              it.md5(),
        //              it.app_name(),
        //              it.version_name(),
        //              it.package_name(),
        //              it.analyzer(),
        //              it.timestamp()
        //     )
        // })


        // writeln!(f, "{}", self.content.with_title().table().);

        let table = self.content.with_title()
            .separator(Separator::builder().build())
            .border(Border::builder().build());

        print_stdout(table).map_err(|_| {
            core::fmt::Error::default()
        })
    }
}

#[derive(Debug, Deserialize, Table)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct ScanItem {
    #[table(title = "Type")]
    scan_type: String,

    #[table(title = "Analyzer")]
    analyzer: String,

    #[table(title = "Time", display_fn = "date_time_format")]
    timestamp: DateTime<Utc>,

    #[table(title = "Hash")]
    md5: String,

    #[table(title = "Version")]
    version_name: String,

    #[table(title = "App name")]
    app_name: String,

    #[table(title = "Package name")]
    package_name: String,

    #[table(title = "File name")]
    file_name: String,
}

impl ScanItem {
    pub fn analyzer(&self) -> &str {
        &self.analyzer
    }
    pub fn scan_type(&self) -> &str {
        &self.scan_type
    }
    pub fn file_name(&self) -> &str {
        &self.file_name
    }
    pub fn app_name(&self) -> &str {
        &self.app_name
    }
    pub fn package_name(&self) -> &str {
        &self.package_name
    }
    pub fn version_name(&self) -> &str {
        &self.version_name
    }
    pub fn md5(&self) -> &str {
        &self.md5
    }
    pub fn timestamp(&self) -> &DateTime<Utc> {
        &self.timestamp
    }
}

#[derive(Debug, Deserialize)]
pub struct ScanResponse {
    title: String,
    version: String,
    file_name: String,
    app_name: String,
    app_type: String,
    package_name: Option<String>,
    size: String,
    md5: String,
    sha1: String,
    sha256: String,
    average_cvss: f32,
    security_score: u8,
    trackers: Option<Trackers>,
}

impl ScanResponse {
    pub fn title(&self) -> &str {
        &self.title
    }
    pub fn version(&self) -> &str {
        &self.version
    }
    pub fn file_name(&self) -> &str {
        &self.file_name
    }
    pub fn app_name(&self) -> &str {
        &self.app_name
    }
    pub fn app_type(&self) -> &str {
        &self.app_type
    }
    pub fn package_name(&self) -> &Option<String> {
        &self.package_name
    }
    pub fn size(&self) -> &str {
        &self.size
    }
    pub fn md5(&self) -> &str {
        &self.md5
    }
    pub fn sha1(&self) -> &str {
        &self.sha1
    }
    pub fn sha256(&self) -> &str {
        &self.sha256
    }
    pub fn average_cvss(&self) -> f32 {
        self.average_cvss
    }
    pub fn security_score(&self) -> u8 {
        self.security_score
    }
    pub fn trackers(&self) -> &Option<Trackers> {
        &self.trackers
    }
}

impl Display for ScanResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Title: {}", self.title())?;
        writeln!(f, "File name: {}", self.file_name())?;
        writeln!(f, "Version: {}", self.version())?;
        writeln!(f, "App name: {}", self.app_name())?;
        writeln!(f, "App type: {}", self.app_type())?;
        writeln!(f, "MD5: {}", self.md5())?;
        writeln!(f, "SHA1: {}", self.sha1())?;
        writeln!(f, "SHA256: {}", self.sha256())?;
        writeln!(f, "Size: {}", self.size())?;
        self.package_name().as_ref().map(|pn| writeln!(f, "Package name: {}", &pn));
        writeln!(f, "Average CVSS: {}", self.average_cvss())?;
        writeln!(f, "Security score: {}/100", self.security_score())?;
        self.trackers().as_ref().map(|tr| writeln!(f, "Trackers detection: {}/{}", tr.detected_trackers(), tr.total_trackers()));
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct Trackers {
    detected_trackers: u16,
    total_trackers: u16,
}

impl Trackers {
    pub fn detected_trackers(&self) -> u16 {
        self.detected_trackers
    }
    pub fn total_trackers(&self) -> u16 {
        self.total_trackers
    }
}

#[derive(Debug, Deserialize)]
pub struct DeleteScanResponse {
    deleted: String,
}

impl DeleteScanResponse {
    pub fn deleted(&self) -> &str {
        &self.deleted
    }
}

impl Display for DeleteScanResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Deleted: {}", self.deleted)
    }
}

#[derive(Debug, Deserialize)]
pub struct ViewSourceResponse {
    title: String,
    file: String,
    #[serde(rename(deserialize = "type"))]
    file_type: String,
    data: String,
    // sqlite: String,
    version: String,
}

impl ViewSourceResponse {
    pub fn title(&self) -> &str {
        &self.title
    }
    pub fn file(&self) -> &str {
        &self.file
    }
    pub fn file_type(&self) -> &str {
        &self.file_type
    }
    pub fn data(&self) -> &str {
        &self.data
    }
    pub fn version(&self) -> &str {
        &self.version
    }
}

impl Display for ViewSourceResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Title: {}", self.title)?;
        writeln!(f, "File: {}", self.file)?;
        writeln!(f, "Type: {}", self.file_type)?;
        writeln!(f, "Version: {}", self.version)?;
        writeln!(f, "{}", self.data)
    }
}

fn date_time_format(value: &DateTime<Utc>) -> impl Display {
    let local = Local::now();
    let dt = value.with_timezone(&local.timezone());

    dt.format("%Y-%m-%d %H:%M:%S").to_string()
}
