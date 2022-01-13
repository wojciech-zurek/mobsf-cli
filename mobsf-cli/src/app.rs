use mobsf_core::error::MobsfError;
use mobsf_core::Mobsf;
use crate::{AppError, NAME};

pub struct App {
    inner: Mobsf,
}

impl App {
    pub async fn new(api_key: String, server: String) -> Result<Self, MobsfError> {
        Ok(App {
            inner: Mobsf::new(api_key, server).await?
        })
    }

    pub async fn ci(&self, file_path: &str, re_scan: bool, path_to_save: &str, cvss: f32, trackers: u16, security: u8) -> Result<(), AppError> {
        println!("{}", "Uploading...");
        let upload_response = self.inner.upload(file_path).await?;
        print!("{}", upload_response);

        println!("{}", "Scanning. It takes some time...");
        let scan_response = self.inner.scan(upload_response.scan_type(), upload_response.file_name(), upload_response.hash(), re_scan).await?;
        print!("{}", scan_response);

        let path = format!("{}/report_{}", path_to_save.trim_end_matches("/"), scan_response.file_name());
        let pdf_path = format!("{}.pdf", &path);
        println!("{}", "Downloading reports...");
        self.inner.report_pdf(upload_response.hash(), pdf_path.as_str()).await?;
        println!("Pdf report saved: {}", pdf_path);

        let json_path = format!("{}.json", &path);
        let _ = self.inner.write_report_json(upload_response.hash(), json_path.as_str()).await?;
        println!("Json report saved: {}", json_path);

        println!("{}", "Validating scan scores...");
        if scan_response.average_cvss() > cvss {
            return Err(AppError {
                message: format!("CVSS score [{}] is to high. Max: {}!", scan_response.average_cvss(), cvss),
            });
        } else {
            println!("CVSS score: {}/{}. OK", scan_response.average_cvss(), cvss);
        }

        if scan_response.security_score() < security {
            return Err(AppError {
                message: format!("Security score [{}] is to low. Min: {}!", scan_response.security_score(), security),
            });
        } else {
            println!("Security score: {}/{}. OK", scan_response.security_score(), security);
        }

        if let Some(r) = scan_response.trackers() {
            if r.detected_trackers() > trackers {
                return Err(AppError {
                    message: format!("Trackers score [{}] is to high. Max: {}!", r.detected_trackers(), trackers),
                });
            } else {
                println!("Trackers score: {}/{}. OK", r.detected_trackers(), trackers);
            }
        }

        Ok(())
    }

    pub async fn upload_file(&self, file_path: &str) -> Result<(), AppError> {
        let response = self.inner.upload(file_path).await?;
        print!("{}", response);
        println!("Start scan command : {} scan {} {} {}", NAME, response.scan_type(), response.file_name(), response.hash());

        Ok(())
    }

    pub async fn scans(&self) -> Result<(), AppError> {
        let response = self.inner.scans().await?;
        print!("{}", response);

        Ok(())
    }

    pub async fn scan(&self, scan_type: &str, file_name: &str, hash: &str, re_scan: bool) -> Result<(), AppError> {
        let response = self.inner.scan(scan_type, file_name, hash, re_scan).await?;
        print!("{}", response);

        Ok(())
    }

    pub async fn delete_scan(&self, hash: &str) -> Result<(), AppError> {
        let response = self.inner.delete_scan(hash).await?;
        print!("{}", response);
        Ok(())
    }

    pub async fn play(&self, file_path: &str, re_scan: bool) -> Result<(), AppError> {
        println!("{}", "Uploading...");
        let response = self.inner.upload(file_path).await?;
        print!("{}", response);

        println!("{}", "Scanning. It takes some time...");
        let response = self.inner.scan(response.scan_type(), response.file_name(), response.hash(), re_scan).await?;
        print!("{}", response);

        Ok(())
    }

    pub async fn report_pdf(&self, hash: &str, file_path: &str) -> Result<(), AppError> {
        self.inner.report_pdf(hash, file_path).await?;
        println!("Pdf report saved: {}", file_path);

        Ok(())
    }

    pub async fn write_report_json(&self, hash: &str, file_path: &str) -> Result<(), AppError> {
        let _ = self.inner.write_report_json(hash, file_path).await?;
        println!("Json report saved: {}", file_path);
        Ok(())
    }

    pub async fn print_report_json(&self, hash: &str) -> Result<(), AppError> {
        let response = self.inner.report_json(hash).await?;
        println!("{}", response);

        Ok(())
    }

    pub async fn view_source(&self, scan_type: &str, file_path: &str, hash: &str) -> Result<(), AppError> {
        let response = self.inner.view_source(scan_type, file_path, hash).await?;
        print!("{}", response);

        Ok(())
    }
}