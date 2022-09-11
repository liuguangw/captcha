///captcha
pub struct Captcha {
    ///image binary data
    pub raw_data: Vec<u8>,
    ///the text of this captcha
    pub phrase: String,
}

impl Captcha {
    ///image binary data
    pub fn data(&self) -> &[u8] {
        &self.raw_data
    }
    ///get base64 image data url
    #[cfg(feature = "base64")]
    pub fn base64_url(&self) -> String {
        let data = base64::encode(&self.raw_data);
        format!("data:image/png;base64,{}", data)
    }
}
