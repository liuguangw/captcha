///captcha
pub struct Captcha {
    raw_data: Vec<u8>,
    ///the text of this captcha
    pub phrase: String,
}

impl Captcha {
    ///construct
    pub fn new(raw_data: Vec<u8>, phrase: String) -> Self {
        Self { raw_data, phrase }
    }
    ///image binary data
    pub fn data(&self) -> &[u8] {
        &self.raw_data
    }
}
