pub mod error;
pub mod response;

use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;
use std::time::Duration;
use reqwest::{Client, multipart};
use reqwest::header::HeaderMap;
use crate::error::{AppError, Cause};
use futures_util::StreamExt;

use crate::error::Cause::InvalidHttpResponse;
use crate::response::{DeleteScanResponse, ErrorResponse, ScanResponse, ScansResponse, UploadResponse, ViewSourceResponse};

const UPLOAD_API: &'static str = "api/v1/upload";
const SCANS_API: &'static str = "api/v1/scans";
const SCAN_API: &'static str = "api/v1/scan";
const DELETE_SCAN_API: &'static str = "api/v1/delete_scan";
const REPORT_PDF_API: &'static str = "api/v1/download_pdf";
const REPORT_JSON_API: &'static str = "api/v1/report_json";
const VIEW_SOURCE_API: &'static str = "api/v1/view_source";

pub struct Mobsf {
    server: String,
    client: Client,
}

impl Mobsf {
    pub async fn new(api_key: String, server: String) -> Result<Self, reqwest::Error> {
        Ok(Mobsf::new_with_client(server, client(api_key).await?))
    }

    pub fn new_with_client(server: String, client: Client) -> Self {
        Mobsf {
            server: server.trim_end_matches("/").to_owned(),
            client,
        }
    }

    pub async fn upload(&self, file_path: &str) -> Result<UploadResponse, AppError> {
        let path = Path::new(file_path);
        let file_name = path.file_name().ok_or(AppError {
            cause: Cause::IoError,
            message: "Invalid file name or path".to_string(),
        })?.to_str().unwrap();

        let mut file = BufReader::new(File::open(&path)?);

        let file_size = path.metadata()?.len();

        let mut v = Vec::with_capacity(file_size as usize);
        file.read_to_end(&mut v)?;

        let file_part = multipart::Part::bytes(v)
            .file_name(file_name.to_owned())
            .mime_str("application/octet-stream")?;

        let form = multipart::Form::new().part("file", file_part);

        let response = self.client
            .post(self.url(UPLOAD_API))
            .multipart(form)
            .send()
            .await?;

        if response.status().as_u16() != 200 {
            return Err(AppError {
                cause: InvalidHttpResponse(response.status().as_u16()),
                message: format!("{}", &response.json::<ErrorResponse>().await?),
                // message: format!("{}", &response.text().await?),
            });
        }

        Ok(response.json().await?)
    }

    pub async fn scans(&self) -> Result<ScansResponse, AppError> {
        let response = self.client
            .get(self.url(SCANS_API))
            .send()
            .await?;

        if response.status().as_u16() != 200 {
            return Err(AppError {
                cause: InvalidHttpResponse(response.status().as_u16()),
                message: format!("{}", &response.json::<ErrorResponse>().await?),
            });
        }

        Ok(response.json().await?)
    }

    pub async fn scan(&self, scan_type: &str, file_name: &str, hash: &str) -> Result<ScanResponse, AppError> {
        let mut params = HashMap::new();
        params.insert("scan_type", scan_type);
        params.insert("file_name", file_name);
        params.insert("hash", hash);

        let response = self.client
            .post(self.url(SCAN_API))
            .form(&params)
            .send()
            .await?;

        if response.status().as_u16() != 200 {
            return Err(AppError {
                cause: InvalidHttpResponse(response.status().as_u16()),
                message: format!("{}", &response.json::<ErrorResponse>().await?),
            });
        }

        Ok(response.json().await?)
    }

    pub async fn delete_scan(&self, hash: &str) -> Result<DeleteScanResponse, AppError> {
        let mut params = HashMap::new();
        params.insert("hash", hash);

        let response = self.client
            .post(self.url(DELETE_SCAN_API))
            .form(&params)
            .send()
            .await?;

        if response.status().as_u16() != 200 {
            return Err(AppError {
                cause: InvalidHttpResponse(response.status().as_u16()),
                message: format!("{}", &response.json::<ErrorResponse>().await?),
            });
        }

        Ok(response.json().await?)
    }

    pub async fn report_pdf(&self, hash: &str, file_path: &str) -> Result<(), AppError> {
        let mut params = HashMap::new();
        params.insert("hash", hash);

        let response = self.client
            .post(self.url(REPORT_PDF_API))
            .form(&params)
            .send()
            .await?;

        let mut file = BufWriter::new(fs::File::create(file_path)?);
        let mut stream = response.bytes_stream();

        while let Some(item) = stream.next().await {
            let chunk = item?;
            file.write(&chunk)?;
        }

        Ok(())
    }

    pub async fn report_json(&self, hash: &str) -> Result<String, AppError> {
        let mut params = HashMap::new();
        params.insert("hash", hash);

        let response = self.client
            .post(self.url(REPORT_JSON_API))
            .form(&params)
            .send()
            .await?;

        let txt_json = response.text().await?;

        Ok(txt_json)
    }

    pub async fn write_report_json(&self, hash: &str, file_path: &str) -> Result<String, AppError> {
        let txt_json = self.report_json(hash).await?;

        let mut file = BufWriter::new(fs::File::create(file_path)?);

        file.write_all((&txt_json).as_ref())?;

        Ok(txt_json)
    }

    pub async fn view_source(&self, scan_type: &str, file_path: &str, hash: &str) -> Result<ViewSourceResponse, AppError> {
        let mut params = HashMap::new();
        params.insert("hash", hash);
        params.insert("file", file_path);
        params.insert("type", scan_type);

        let response = self.client
            .post(self.url(VIEW_SOURCE_API))
            .form(&params)
            .send()
            .await?;

        Ok(response.json().await?)
    }

    fn url(&self, api: &str) -> String {
        format!("{}/{}", &self.server, api)
    }
}

async fn client(api_key: String) -> Result<Client, reqwest::Error> {
    let mut header_map = HeaderMap::new();
    header_map.insert("Accept", "application/json".parse().unwrap());
    header_map.insert("Authorization", api_key.parse().unwrap());
    //header_map.insert("X-Mobsf-Api-Key", api_key.parse().unwrap());

    Client::builder()
        .connect_timeout(Duration::from_secs(10))
        .default_headers(header_map)
        .build()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
