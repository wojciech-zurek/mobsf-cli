use mobsf_core::error::AppError;
use mobsf_core::Mobsf;
use crate::NAME;

pub struct App {
    inner: Mobsf,
}

impl App {
    pub async fn new(api_key: String, host: String) -> Result<Self, AppError> {
        Ok(App {
            inner: Mobsf::new(api_key, host).await?
        })
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

    pub async fn scan(&self, scan_type: &str, file_name: &str, hash: &str) -> Result<(), AppError> {
        let response = self.inner.scan(scan_type, file_name, hash).await?;
        print!("{}", response);

        Ok(())
    }

    pub async fn delete_scan(&self, hash: &str) -> Result<(), AppError> {
        let response = self.inner.delete_scan(hash).await?;
        print!("{}", response);
        Ok(())
    }

    pub async fn play(&self, file_path: &str) -> Result<(), AppError> {
        let response = self.inner.upload(file_path).await?;
        print!("{}", response);

        let response = self.inner.scan(response.scan_type(), response.file_name(), response.hash()).await?;
        print!("{}", response);

        Ok(())
    }

    pub async fn report_pdf(&self, hash: &str, file_path: &str) -> Result<(), AppError> {
        self.inner.report_pdf(hash, file_path).await
    }

    pub async fn report_json(&self, hash: &str, _: &str) -> Result<(), AppError> {
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